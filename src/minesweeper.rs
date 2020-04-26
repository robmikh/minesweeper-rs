use crate::windows::{
    foundation::numerics::{Vector2, Vector3},
    ui::{
        Colors,
        composition::{Compositor, ContainerVisual, SpriteVisual},
    },
};

enum MineState {
    Empty,
    Flag,
    Question,
}

pub struct Minesweeper {
    compositor: Compositor,
    root: SpriteVisual,

    game_board: ContainerVisual,
    tiles: Vec<SpriteVisual>,
    selection_visual: SpriteVisual,

    game_board_width: i32,
    game_board_height: i32,
    tile_size: Vector2,
    margin: Vector2,
    game_board_margin: Vector2,
    current_selection_x: i32,
    current_selection_y: i32,
    mine_states: Vec<MineState>,
    mines: Vec<bool>,
    neighbor_counts: Vec<i32>,
    parent_size: Vector2,
}

impl Minesweeper {
    pub fn new(parent_visual: &ContainerVisual, parent_size: Vector2) -> winrt::Result<Self> {
        let compositor = parent_visual.compositor()?;
        let root = compositor.create_sprite_visual()?;

        root.set_relative_size_adjustment(Vector2{ x: 1.0, y: 1.0 })?;
        root.set_brush(compositor.create_color_brush_with_color(Colors::white()?)?)?;
        parent_visual.children()?.insert_at_top(&root)?;

        let tile_size = Vector2{ x: 25.0, y: 25.0 };
        let margin = Vector2{ x: 2.5, y: 2.5 };
        let game_board_margin = Vector2{ x: 100.0, y: 100.0 };

        let game_board = compositor.create_container_visual()?;
        game_board.set_relative_offset_adjustment(Vector3{ x: 0.5, y: 0.5, z: 0.0 })?;
        game_board.set_anchor_point(Vector2{ x: 0.5, y: 0.5 })?;
        root.children()?.insert_at_top(&game_board)?;

        let selection_visual = compositor.create_sprite_visual()?;
        let color_brush = compositor.create_color_brush_with_color(Colors::red()?)?;
        let nine_grid_brush = compositor.create_nine_grid_brush()?;
        nine_grid_brush.set_insets_with_values(margin.x, margin.y, margin.x, margin.y)?;
        nine_grid_brush.set_is_center_hollow(true)?;
        nine_grid_brush.set_source(color_brush)?;
        selection_visual.set_brush(nine_grid_brush)?;
        selection_visual.set_offset(Vector3{ x: margin.x * -1.0, y: margin.y * -1.0, z: 0.0 })?;
        selection_visual.set_is_visible(false)?;
        selection_visual.set_size(Vector2{ x: tile_size.x + margin.x * 2.0, y: tile_size.y + margin.y * 2.0 })?;
        root.children()?.insert_at_top(&selection_visual)?;
        let current_selection_x = -1;
        let current_selection_y = -1;

        let mut result = Self {
            compositor: compositor,
            root: root,

            game_board: game_board,
            tiles: Vec::new(),
            selection_visual: selection_visual,

            game_board_width: 0,
            game_board_height: 0,
            tile_size: tile_size,
            margin: margin,
            game_board_margin: game_board_margin,
            current_selection_x: current_selection_x,
            current_selection_y: current_selection_y,
            mine_states: Vec::new(),
            mines: Vec::new(),
            neighbor_counts: Vec::new(),
            parent_size: parent_size.clone(),
        };

        result.new_game(16, 16, 40)?;
        result.on_parent_size_changed(parent_size)?;

        Ok(result)
    }

    pub fn on_pointer_moved(&mut self, point: Vector2) -> winrt::Result<()> {
        Ok(())
    }

    pub fn on_parent_size_changed(&mut self, new_size: Vector2) -> winrt::Result<()> {
        Ok(())
    }

    pub fn on_pointer_pressed(&mut self, is_right_button: bool, is_eraser: bool) -> winrt::Result<()> {
        Ok(())
    }

    fn new_game(&mut self, board_width: i32, board_height: i32, mines: i32) -> winrt::Result<()> {
        Ok(())
    }
}