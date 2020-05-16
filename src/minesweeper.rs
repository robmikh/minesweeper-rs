use crate::comp_assets::CompAssets;
use crate::visual_grid::VisualGrid;
use crate::windows::{
    foundation::{
        numerics::{Vector2, Vector3},
        TimeSpan,
    },
    graphics::SizeInt32,
    ui::{
        composition::{
            AnimationIterationBehavior, CompositionBatchTypes, CompositionBorderMode,
            Compositor, ContainerVisual, SpriteVisual,
        },
        Colors,
    },
};
use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;
use std::time::Duration;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum MineState {
    Empty,
    Flag,
    Question,
    Revealed,
}
impl MineState {
    fn cycle(self) -> Self {
        match self {
            MineState::Empty => MineState::Flag,
            MineState::Flag => MineState::Question,
            MineState::Question => MineState::Empty,
            MineState::Revealed => panic!("We shouldn't be cycling a revealed tile!"),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum MineGenerationState {
    Deferred,
    Generated,
}

pub struct Minesweeper {
    compositor: Compositor,
    _root: SpriteVisual,

    game_board: VisualGrid,

    game_board_width: i32,
    game_board_height: i32,
    game_board_margin: Vector2,

    mine_states: Vec<MineState>,
    mines: Vec<bool>,
    neighbor_counts: Vec<i32>,
    parent_size: Vector2,
    mine_generation_state: MineGenerationState,
    num_mines: i32,

    mine_animation_playing: bool,
    game_over: bool,

    assets: CompAssets,
}

impl Minesweeper {
    pub fn new(parent_visual: &ContainerVisual, parent_size: &Vector2) -> winrt::Result<Self> {
        let compositor = parent_visual.compositor()?;
        let root = compositor.create_sprite_visual()?;

        root.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
        root.set_brush(compositor.create_color_brush_with_color(Colors::white()?)?)?;
        root.set_border_mode(CompositionBorderMode::Hard)?;
        parent_visual.children()?.insert_at_top(&root)?;

        let tile_size = Vector2 { x: 25.0, y: 25.0 };
        let game_board = VisualGrid::new(
            &compositor,
            &SizeInt32 {
                width: 16,
                height: 16,
            },
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

        let mut result = Self {
            compositor: compositor,
            _root: root,

            game_board: game_board,

            game_board_width: 0,
            game_board_height: 0,
            game_board_margin: game_board_margin,

            mine_states: Vec::new(),
            mines: Vec::new(),
            neighbor_counts: Vec::new(),
            parent_size: parent_size.clone(),
            mine_generation_state: MineGenerationState::Deferred,
            num_mines: 0,

            mine_animation_playing: false,
            game_over: false,

            assets: assets,
        };

        result.new_game(16, 16, 40)?;
        result.on_parent_size_changed(parent_size)?;

        Ok(result)
    }

    pub fn on_pointer_moved(&mut self, point: &Vector2) -> winrt::Result<()> {
        if self.game_over || self.mine_animation_playing {
            return Ok(());
        }

        let window_size = &self.parent_size;
        let scale = self.compute_scale_factor()?;
        let real_board_size = self.game_board.size()? * scale;
        let real_offset = (window_size - real_board_size) / 2.0;

        let point = (point - real_offset) / scale;

        let selected_tile = if let Some(tile) = self.game_board.hit_test(&point) {
            if self.mine_states[self.compute_index(tile.x, tile.y)] != MineState::Revealed {
                Some(tile)
            } else {
                None
            }
        } else {
            None
        };
        self.game_board.select_tile(selected_tile)?;

        Ok(())
    }

    pub fn on_parent_size_changed(&mut self, new_size: &Vector2) -> winrt::Result<()> {
        self.parent_size = new_size.clone();
        self.update_board_scale(new_size)?;
        Ok(())
    }

    pub fn on_pointer_pressed(
        &mut self,
        is_right_button: bool,
        is_eraser: bool,
    ) -> winrt::Result<()> {
        // TODO: Switch the condition back once we can subscribe to events.
        //if self.game_over && !self.mine_animation_playing {
        if self.game_over && self.mine_animation_playing {
            self.new_game(
                self.game_board_width,
                self.game_board_height,
                self.num_mines,
            )?;
        }

        let current_selection = self.game_board.current_selected_tile();
        if let Some(current_selection) = current_selection {
            let index = self.compute_index(current_selection.x, current_selection.y);
            let visual = self
                .game_board
                .get_tile(current_selection.x, current_selection.y)
                .unwrap();

            if self.mine_states[index] != MineState::Revealed {
                if is_right_button || is_eraser {
                    let state = self.mine_states[index].cycle();
                    self.mine_states[index] = state;
                    visual.set_brush(self.assets.get_color_brush_from_mine_state(state))?;
                } else if self.mine_states[index] == MineState::Empty {
                    if self.sweep(current_selection.x, current_selection.y)? {
                        // We hit a mine! Setup and play an animation whiel locking any input.
                        let hit_x = current_selection.x;
                        let hit_y = current_selection.y;

                        // First, hide the selection visual and reset the selection
                        self.game_board.select_tile(None)?;

                        // Create an animation batch so that we can know when the animations complete
                        let batch = self
                            .compositor
                            .create_scoped_batch(CompositionBatchTypes::Animation)?;

                        self.play_animation_on_all_mines(hit_x, hit_y)?;

                        // Subscribe to the completion event and complete the batch
                        // TODO: events
                        batch.end()?;

                        self.mine_animation_playing = true;
                        self.game_over = true;
                    }
                    // TODO: Detect that the player has won
                }
            }
        }
        Ok(())
    }

    fn new_game(&mut self, board_width: i32, board_height: i32, mines: i32) -> winrt::Result<()> {
        self.game_board_width = board_width;
        self.game_board_height = board_height;

        self.game_board.reset(&SizeInt32 {
            width: board_width,
            height: board_height,
        })?;
        self.mine_states.clear();

        for visual in self.game_board.tiles_iter() {
            visual.set_brush(self.assets.get_color_brush_from_mine_state(MineState::Empty))?;
            self.mine_states.push(MineState::Empty);
        }

        self.mine_animation_playing = false;
        self.game_over = false;
        self.mine_generation_state = MineGenerationState::Deferred;
        self.num_mines = mines;

        self.update_board_scale(&self.parent_size.clone())?;

        Ok(())
    }

    fn compute_scale_factor_from_size(&self, window_size: &Vector2) -> winrt::Result<f32> {
        let board_size = self.game_board.size()?;
        let board_size = board_size + &self.game_board_margin;

        let window_ratio = window_size.x / window_size.y;
        let board_ratio = board_size.x / board_size.y;

        let mut scale_factor = window_size.x / board_size.x;
        if window_ratio > board_ratio {
            scale_factor = window_size.y / board_size.y;
        }

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

    fn sweep(&mut self, x: i32, y: i32) -> winrt::Result<bool> {
        if self.mine_generation_state == MineGenerationState::Deferred {
            // We don't want the first thing that the user clicks to be a mine.
            // Generate mines but avoid putting it where the user clicked.
            self.generate_mines(self.num_mines, x, y);
            self.mine_generation_state = MineGenerationState::Generated;
        }

        let mut hit_mine = false;
        let mut sweeps: VecDeque<usize> = VecDeque::new();
        sweeps.push_back(self.compute_index(x, y));
        self.reveal(*sweeps.front().unwrap())?;

        while !sweeps.is_empty() {
            let index = *sweeps.front().unwrap();
            let current_x = self.compute_x_from_index(index);
            let current_y = self.compute_y_from_index(index);

            if self.mines[index] {
                // We hit a mine, game over
                hit_mine = true;
                break;
            }

            if self.neighbor_counts[index] == 0 {
                self.push_if_unmarked(&mut sweeps, current_x - 1, current_y - 1)?;
                self.push_if_unmarked(&mut sweeps, current_x, current_y - 1)?;
                self.push_if_unmarked(&mut sweeps, current_x + 1, current_y - 1)?;
                self.push_if_unmarked(&mut sweeps, current_x + 1, current_y)?;
                self.push_if_unmarked(&mut sweeps, current_x + 1, current_y + 1)?;
                self.push_if_unmarked(&mut sweeps, current_x, current_y + 1)?;
                self.push_if_unmarked(&mut sweeps, current_x - 1, current_y + 1)?;
                self.push_if_unmarked(&mut sweeps, current_x - 1, current_y)?;
            }

            sweeps.pop_front().unwrap();
        }

        Ok(hit_mine)
    }

    fn reveal(&mut self, index: usize) -> winrt::Result<()> {
        let visual = self
            .game_board
            .get_tile(
                self.compute_x_from_index(index),
                self.compute_y_from_index(index),
            )
            .unwrap();

        if self.mines[index] {
            visual.set_brush(&self.assets.get_mine_brush())?;
        } else {
            let count = self.neighbor_counts[index];
            visual.set_brush(self.assets.get_color_brush_from_mine_count(count))?;

            if count > 0 {
                let shape = self.assets.get_shape_from_mine_count(count);
                let shape_visual = self.compositor.create_shape_visual()?;
                shape_visual.set_relative_size_adjustment(Vector2 { x: 1.0, y: 1.0 })?;
                shape_visual.shapes()?.append(shape)?;
                shape_visual.set_border_mode(CompositionBorderMode::Soft)?;
                visual.children()?.insert_at_top(shape_visual)?;
            }
        }

        self.mine_states[index] = MineState::Revealed;
        Ok(())
    }

    fn is_in_bounds_and_unmarked(&self, x: i32, y: i32) -> bool {
        let index = self.compute_index(x, y);
        self.is_in_bounds(x, y) && self.mine_states[index] == MineState::Empty
    }

    fn push_if_unmarked(
        &mut self,
        sweeps: &mut VecDeque<usize>,
        x: i32,
        y: i32,
    ) -> winrt::Result<()> {
        if self.is_in_bounds_and_unmarked(x, y) {
            let index = self.compute_index(x, y);
            self.reveal(index)?;
            sweeps.push_back(index);
        }

        Ok(())
    }

    fn generate_mines(&mut self, num_mines: i32, exclude_x: i32, exclude_y: i32) {
        self.mines.clear();
        for _x in 0..self.game_board_width {
            for _y in 0..self.game_board_height {
                self.mines.push(false);
            }
        }

        let between = Uniform::from(0..(self.game_board_width * self.game_board_height) as usize);
        let mut rng = rand::thread_rng();
        for _i in 0..num_mines {
            let mut index: usize;
            let exclude_index = self.compute_index(exclude_x, exclude_y);
            // do while loops look weird in rust...
            while {
                index = between.sample(&mut rng);
                index == exclude_index || self.mines[index]
            } {}

            self.mines[index] = true;
        }

        self.neighbor_counts.clear();
        for i in 0..self.mines.len() {
            let x = self.compute_x_from_index(i);
            let y = self.compute_y_from_index(i);

            if self.mines[i] {
                // -1 means a mine
                self.neighbor_counts.push(-1);
            } else {
                let count = self.get_surrounding_mine_count(x, y);
                self.neighbor_counts.push(count);
            }
        }
    }

    fn compute_index(&self, x: i32, y: i32) -> usize {
        (x * self.game_board_height + y) as usize
    }

    fn compute_x_from_index(&self, index: usize) -> i32 {
        index as i32 / self.game_board_height
    }

    fn compute_y_from_index(&self, index: usize) -> i32 {
        index as i32 % self.game_board_height
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        (x >= 0 && x < self.game_board_width) && (y >= 0 && y < self.game_board_height)
    }

    fn test_spot(&self, x: i32, y: i32) -> bool {
        self.is_in_bounds(x, y) && self.mines[self.compute_index(x, y)]
    }

    fn get_surrounding_mine_count(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;

        if self.test_spot(x + 1, y) {
            count = count + 1;
        }

        if self.test_spot(x - 1, y) {
            count = count + 1;
        }

        if self.test_spot(x, y + 1) {
            count = count + 1;
        }

        if self.test_spot(x, y - 1) {
            count = count + 1;
        }

        if self.test_spot(x + 1, y + 1) {
            count = count + 1;
        }

        if self.test_spot(x - 1, y - 1) {
            count = count + 1;
        }

        if self.test_spot(x - 1, y + 1) {
            count = count + 1;
        }

        if self.test_spot(x + 1, y - 1) {
            count = count + 1;
        }

        count
    }

    fn play_mine_animation(&self, index: usize, delay: &TimeSpan) -> winrt::Result<()> {
        let visual = self
            .game_board
            .get_tile(
                self.compute_x_from_index(index),
                self.compute_y_from_index(index),
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

    fn check_tile_for_mine_for_animation(
        &self,
        x: i32,
        y: i32,
        mine_indices: &mut VecDeque<usize>,
        visited_tiles: &mut i32,
        mines_in_ring: &mut i32,
    ) {
        if self.is_in_bounds(x, y) {
            let tile_index = self.compute_index(x, y);
            if self.mines[tile_index] {
                mine_indices.push_back(tile_index);
                *mines_in_ring = *mines_in_ring + 1;
            }
            *visited_tiles = *visited_tiles + 1;
        }
    }

    fn play_animation_on_all_mines(&mut self, center_x: i32, center_y: i32) -> winrt::Result<()> {
        // Build a queue that contains the indices of the mines in a spiral starting from the clicked mine.
        let mut mine_indices: VecDeque<usize> = VecDeque::new();
        let mut mines_per_ring: VecDeque<i32> = VecDeque::new();
        let mut visited_tiles: i32 = 0;
        let mut ring_level: i32 = 0;
        while visited_tiles < self.game_board.num_tiles() as i32 {
            if ring_level == 0 {
                let hit_mine_index = self.compute_index(center_x, center_y);
                mine_indices.push_back(hit_mine_index);
                mines_per_ring.push_back(1);
                visited_tiles = visited_tiles + 1;
            } else {
                let mut current_mines_in_ring = 0;

                // Check the top side
                for x in (center_x - ring_level)..=(center_x + ring_level) {
                    let y = center_y - ring_level;
                    self.check_tile_for_mine_for_animation(
                        x,
                        y,
                        &mut mine_indices,
                        &mut visited_tiles,
                        &mut current_mines_in_ring,
                    );
                }

                // Check the right side
                for y in (center_y - ring_level + 1)..=(center_y + ring_level) {
                    let x = center_x + ring_level;
                    self.check_tile_for_mine_for_animation(
                        x,
                        y,
                        &mut mine_indices,
                        &mut visited_tiles,
                        &mut current_mines_in_ring,
                    );
                }

                // Check the bottom side
                for x in (center_x - ring_level)..(center_x + ring_level) {
                    let y = center_y + ring_level;
                    self.check_tile_for_mine_for_animation(
                        x,
                        y,
                        &mut mine_indices,
                        &mut visited_tiles,
                        &mut current_mines_in_ring,
                    );
                }

                // Check the left side
                for y in (center_y - ring_level + 1)..(center_y + ring_level) {
                    let x = center_x - ring_level;
                    self.check_tile_for_mine_for_animation(
                        x,
                        y,
                        &mut mine_indices,
                        &mut visited_tiles,
                        &mut current_mines_in_ring,
                    );
                }

                if current_mines_in_ring > 0 {
                    mines_per_ring.push_back(current_mines_in_ring);
                }
            }
            ring_level = ring_level + 1;
        }

        // Iterate and animate each mine
        let animation_delay_step = Duration::from_millis(100);
        let mut current_delay = Duration::from_millis(0);
        let mut current_mines_count = 0;
        while !mine_indices.is_empty() {
            let mine_index = *mine_indices.front().unwrap();
            self.play_mine_animation(mine_index, &TimeSpan::from(current_delay))?;
            current_mines_count = current_mines_count + 1;

            let mines_on_current_level = *mines_per_ring.front().unwrap();
            if current_mines_count == mines_on_current_level {
                current_mines_count = 0;
                mines_per_ring.pop_front().unwrap();
                current_delay = current_delay + animation_delay_step;
            }
            mine_indices.pop_front().unwrap();
        }

        Ok(())
    }
}
