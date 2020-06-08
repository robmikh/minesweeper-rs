use crate::comp_assets::CompAssets;
use crate::minesweeper::{IndexHelper, MineState};
use crate::visual_grid::{TileCoordinate, VisualGrid};
use bindings::windows::{
    foundation::{
        numerics::{Vector2, Vector3},
        TimeSpan,
    },
    graphics::SizeInt32,
    ui::{
        composition::{
            AnimationIterationBehavior, CompositionBatchTypes, CompositionBorderMode, Compositor,
            ContainerVisual, SpriteVisual,
        },
        Colors,
    },
};
use std::collections::VecDeque;
use std::time::Duration;

pub struct CompUI {
    compositor: Compositor,
    _root: SpriteVisual,
    parent_size: Vector2,
    game_board_margin: Vector2,
    index_helper: IndexHelper,

    game_board: VisualGrid,
    assets: CompAssets,

    mine_animation_playing: bool,
}

impl CompUI {
    pub fn new(
        parent_visual: &ContainerVisual,
        parent_size: &Vector2,
        grid_size_in_tiles: &SizeInt32,
    ) -> winrt::Result<Self> {
        let compositor = parent_visual.compositor()?;
        let root = compositor.create_sprite_visual()?;

        root.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
        root.set_brush(compositor.create_color_brush_with_color(Colors::white()?)?)?;
        root.set_border_mode(CompositionBorderMode::Hard)?;
        parent_visual.children()?.insert_at_top(&root)?;

        let tile_size = Vector2 { x: 25.0, y: 25.0 };
        let game_board = VisualGrid::new(
            &compositor,
            grid_size_in_tiles,
            &tile_size,
            &Vector2 { x: 2.5, y: 2.5 },
        )?;
        let game_board_margin = Vector2 { x: 100.0, y: 100.0 };

        let game_board_visual = game_board.root();
        game_board_visual.set_relative_offset_adjustment(Vector3 {
            x: 0.5,
            y: 0.5,
            z: 0.0,
        })?;
        game_board_visual.set_anchor_point(Vector2 { x: 0.5, y: 0.5 })?;
        root.children()?.insert_at_top(game_board_visual)?;

        let selection_visual = game_board.selection_visual();
        root.children()?.insert_at_top(selection_visual)?;

        let assets = CompAssets::new(&compositor, &tile_size)?;

        Ok(Self {
            compositor,
            _root: root,
            parent_size: parent_size.clone(),
            game_board_margin,
            index_helper: IndexHelper::new(grid_size_in_tiles.width, grid_size_in_tiles.height),

            game_board,
            assets,
            mine_animation_playing: false,
        })
    }

    pub fn hit_test(&self, point: &Vector2) -> winrt::Result<Option<TileCoordinate>> {
        let window_size = &self.parent_size;
        let scale = self.compute_scale_factor()?;
        let real_board_size = self.game_board.size()? * scale;
        let real_offset = (window_size - real_board_size) / 2.0;

        let point = (point - real_offset) / scale;
        Ok(self.game_board.hit_test(&point))
    }

    pub fn resize(&mut self, new_size: &Vector2) -> winrt::Result<()> {
        self.parent_size = new_size.clone();
        self.update_board_scale(new_size)?;
        Ok(())
    }

    pub fn select_tile(&mut self, tile_coordinate: Option<TileCoordinate>) -> winrt::Result<()> {
        self.game_board.select_tile(tile_coordinate)
    }

    pub fn current_selected_tile(&self) -> Option<TileCoordinate> {
        self.game_board.current_selected_tile()
    }

    pub fn update_tile_with_state(
        &self,
        tile_coordinate: &TileCoordinate,
        mine_state: MineState,
    ) -> winrt::Result<()> {
        let visual = self
            .game_board
            .get_tile(tile_coordinate.x, tile_coordinate.y)
            .unwrap();

        visual.set_brush(self.assets.get_color_brush_from_mine_state(mine_state))?;
        Ok(())
    }

    pub fn reset(&mut self, grid_size_in_tiles: &SizeInt32) -> winrt::Result<()> {
        self.game_board.reset(grid_size_in_tiles)?;
        self.index_helper = IndexHelper::new(grid_size_in_tiles.width, grid_size_in_tiles.height);

        for visual in self.game_board.tiles_iter() {
            visual.set_brush(
                self.assets
                    .get_color_brush_from_mine_state(MineState::Empty),
            )?;
        }

        self.update_board_scale(&self.parent_size.clone())?;
        self.mine_animation_playing = false;

        Ok(())
    }

    pub fn update_tile_as_mine(&self, tile_coordinate: &TileCoordinate) -> winrt::Result<()> {
        let visual = self
            .game_board
            .get_tile(tile_coordinate.x, tile_coordinate.y)
            .unwrap();

        visual.set_brush(&self.assets.get_mine_brush())?;
        Ok(())
    }

    pub fn update_tile_with_mine_count(
        &self,
        tile_coordinate: &TileCoordinate,
        num_mines: i32,
    ) -> winrt::Result<()> {
        let visual = self
            .game_board
            .get_tile(tile_coordinate.x, tile_coordinate.y)
            .unwrap();
        visual.set_brush(self.assets.get_color_brush_from_mine_count(num_mines))?;

        if num_mines > 0 {
            let shape = self.assets.get_shape_from_mine_count(num_mines);
            let shape_visual = self.compositor.create_shape_visual()?;
            shape_visual.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
            shape_visual.shapes()?.append(shape)?;
            shape_visual.set_border_mode(CompositionBorderMode::Soft)?;
            visual.children()?.insert_at_top(shape_visual)?;
        }

        Ok(())
    }

    pub fn play_mine_animations(
        &mut self,
        mut mine_indices: VecDeque<usize>,
        mut mines_per_ring: VecDeque<i32>,
    ) -> winrt::Result<()> {
        // Create an animation batch so that we can know when the animations complete
        let batch = self
            .compositor
            .create_scoped_batch(CompositionBatchTypes::Animation)?;

        let animation_delay_step = Duration::from_millis(100);
        let mut current_delay = Duration::from_millis(0);
        let mut current_mines_count = 0;
        while !mine_indices.is_empty() {
            let mine_index = *mine_indices.front().unwrap();
            self.play_mine_animation(mine_index, &TimeSpan::from(current_delay))?;
            current_mines_count += 1;

            let mines_on_current_level = *mines_per_ring.front().unwrap();
            if current_mines_count == mines_on_current_level {
                current_mines_count = 0;
                mines_per_ring.pop_front().unwrap();
                current_delay += animation_delay_step;
            }
            mine_indices.pop_front().unwrap();
        }

        // Subscribe to the completion event and complete the batch
        // TODO: events
        batch.end()?;

        self.mine_animation_playing = true;

        Ok(())
    }

    pub fn is_animation_playing(&self) -> bool {
        self.mine_animation_playing
    }

    fn compute_scale_factor_from_size(&self, window_size: &Vector2) -> winrt::Result<f32> {
        let board_size = self.game_board.size()?;
        let board_size = board_size + &self.game_board_margin;

        let window_ratio = window_size.x / window_size.y;
        let board_ratio = board_size.x / board_size.y;

        let scale_factor = if window_ratio > board_ratio {
            window_size.y / board_size.y
        } else {
            window_size.x / board_size.x
        };

        Ok(scale_factor)
    }

    fn compute_scale_factor(&self) -> winrt::Result<f32> {
        self.compute_scale_factor_from_size(&self.parent_size)
    }

    fn update_board_scale(&mut self, window_size: &Vector2) -> winrt::Result<()> {
        let scale_factor = self.compute_scale_factor_from_size(window_size)?;
        self.game_board.root().set_scale(Vector3 {
            x: scale_factor,
            y: scale_factor,
            z: 1.0,
        })?;
        Ok(())
    }

    fn play_mine_animation(&self, index: usize, delay: &TimeSpan) -> winrt::Result<()> {
        let visual = self
            .game_board
            .get_tile(
                self.index_helper.compute_x_from_index(index),
                self.index_helper.compute_y_from_index(index),
            )
            .unwrap();
        // First, we need to promote the visual to the top.
        let parent_children = visual.parent()?.children()?;
        parent_children.remove(visual)?;
        parent_children.insert_at_top(visual)?;
        // Make sure the visual has the mine brush
        visual.set_brush(&self.assets.get_mine_brush())?;
        // Play the animation
        let animation = self.compositor.create_vector3_key_frame_animation()?;
        animation.insert_key_frame(
            0.0,
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        )?;
        animation.insert_key_frame(
            0.7,
            Vector3 {
                x: 2.0,
                y: 2.0,
                z: 1.0,
            },
        )?;
        animation.insert_key_frame(
            1.0,
            Vector3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        )?;
        animation.set_duration(TimeSpan::from(Duration::from_millis(600)))?;
        animation.set_delay_time(delay)?;
        animation.set_iteration_behavior(AnimationIterationBehavior::Count)?;
        animation.set_iteration_count(1)?;
        visual.start_animation("Scale", animation)?;
        Ok(())
    }
}
