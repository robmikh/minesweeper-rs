use crate::windows::{
    foundation::numerics::{Vector2, Vector3},
    ui::{
        Colors,
        composition::{Compositor, ContainerVisual, SpriteVisual, CompositionColorBrush},
    },
};
use crate::numerics::FromVector2;
use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;

// TODO: We allow dead code here because we don't directly construct some of these
//       variants. The way we cycle through mine states in on_pointer_pressed isn't 
//       idiomatic, so we need to find a better way to do it.
#[allow(dead_code)]
#[repr(i32)]
#[derive(Copy, Clone, PartialEq)]
enum MineState {
    Empty = 0,
    Flag = 1,
    Question = 2,
    Last = 3, // ????
}

pub struct Minesweeper {
    compositor: Compositor,
    _root: SpriteVisual,

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
    pub fn new(parent_visual: &ContainerVisual, parent_size: &Vector2) -> winrt::Result<Self> {
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
        selection_visual.set_offset(Vector3::from_vector2(&margin * -1.0, 0.0))?;
        selection_visual.set_is_visible(false)?;
        selection_visual.set_size(&tile_size + &margin * 2.0)?;
        root.children()?.insert_at_top(&selection_visual)?;
        let current_selection_x = -1;
        let current_selection_y = -1;

        let mut result = Self {
            compositor: compositor,
            _root: root,

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

    pub fn on_pointer_moved(&mut self, point: &Vector2) -> winrt::Result<()> {
        let window_size = &self.parent_size;
        let scale = self.compute_scale_factor()?;
        let real_board_size = self.game_board.size()? * scale;
        let real_offset = (window_size - real_board_size) / 2.0;
        
        let point = (point - real_offset) / scale;

        let x = (point.x / (self.tile_size.x + self.margin.x)) as i32;
        let y = (point.y / (self.tile_size.y + self.margin.y)) as i32;
        let index = self.compute_index(x, y);

        if self.is_in_bounds(x, y) && self.mine_states[index] != MineState::Last {
            let visual = &self.tiles[index];
            self.selection_visual.set_parent_for_transform(visual)?;
            self.current_selection_x = x;
            self.current_selection_y = y;
            self.selection_visual.set_is_visible(true)?;
        } else {
            self.current_selection_x = -1;
            self.current_selection_y = -1;
            self.selection_visual.set_is_visible(false)?;
        }

        Ok(())
    }

    pub fn on_parent_size_changed(&mut self, new_size: &Vector2) -> winrt::Result<()> {
        self.parent_size = new_size.clone();
        self.update_board_scale(new_size)?;
        Ok(())
    }

    pub fn on_pointer_pressed(&mut self, is_right_button: bool, is_eraser: bool) -> winrt::Result<()> {
        if self.current_selection_x >= 0 || self.current_selection_y >= 0 {
            let index = self.compute_index(self.current_selection_x, self.current_selection_y);
            let visual = &self.tiles[index];

            if self.mine_states[index] != MineState::Last {
                if is_right_button || is_eraser {
                    let mut state = self.mine_states[index];
                    // TODO: Find a more idiomatic way to do this (see enum comment).
                    state = unsafe {
                        let state: i32 = (state as i32 + 1) % MineState::Last as i32;
                        std::mem::transmute(state)
                    };
                    self.mine_states[index] = state;
                    visual.set_brush(self.get_color_brush_from_mine_state(state)?)?;
                } else if self.mine_states[index] == MineState::Empty {
                    self.sweep(self.current_selection_x, self.current_selection_y)?;
                }
            }
        }
        Ok(())
    }

    fn new_game(&mut self, board_width: i32, board_height: i32, mines: i32) -> winrt::Result<()> {
        self.game_board_width = board_width;
        self.game_board_height = board_height;

        self.game_board.children()?.remove_all()?;
        self.game_board.set_size((&self.tile_size + &self.margin) * Vector2{ x: self.game_board_width as f32, y: self.game_board_height as f32 })?;

        for x in 0..self.game_board_width {
            for y in 0..self.game_board_height {
                let visual = self.compositor.create_sprite_visual()?;
                visual.set_size(&self.tile_size)?;
                visual.set_offset(Vector3::from_vector2(
                    (&self.margin / 2.0) + ((&self.tile_size + &self.margin) * Vector2{ x: x as f32, y: y as f32}),
                    0.0,
                ))?;
                visual.set_brush(self.compositor.create_color_brush_with_color(Colors::blue()?)?)?;

                self.game_board.children()?.insert_at_top(&visual)?;
                self.tiles.push(visual);
                self.mine_states.push(MineState::Empty);
            }
        }

        self.generate_mines(mines);

        self.selection_visual.set_is_visible(false)?;
        self.current_selection_x = -1;
        self.current_selection_y = -1;

        self.update_board_scale(&self.parent_size.clone())?;

        Ok(())
    }

    fn compute_scale_factor_from_size(&self, window_size: &Vector2) -> winrt::Result<f32> {
        let board_size = self.game_board.size()?;
        let board_size = board_size + &self.game_board_margin;
        let mut scale_factor = window_size.y / board_size.y;

        if board_size.x > window_size.x {
            scale_factor = window_size.x / board_size.x;
        }

        Ok(scale_factor)
    }

    fn compute_scale_factor(&self) -> winrt::Result<f32> {
        self.compute_scale_factor_from_size(&self.parent_size)
    }

    fn update_board_scale(&mut self, window_size: &Vector2) -> winrt::Result<()> {
        let scale_factor = self.compute_scale_factor_from_size(window_size)?;
        self.game_board.set_scale(Vector3{ x: scale_factor, y: scale_factor, z: 1.0 })?;
        Ok(())
    }

    fn sweep(&mut self, x: i32, y: i32) -> winrt::Result<bool> {
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
        let visual = &self.tiles[index];

        if self.mines[index] {
            visual.set_brush(self.compositor.create_color_brush_with_color(Colors::red()?)?)?;
        } else {
            let count = self.neighbor_counts[index];
            visual.set_brush(self.get_color_brush_from_mine_count(count)?)?;
        }

        self.mine_states[index] = MineState::Last;
        Ok(())
    }

    fn is_in_bounds_and_unmarked(&self, x: i32, y: i32) -> bool {
        let index = self.compute_index(x, y);
        self.is_in_bounds(x, y) && self.mine_states[index] == MineState::Empty
    }

    fn push_if_unmarked(&mut self, sweeps: &mut VecDeque<usize>, x: i32, y: i32) -> winrt::Result<()> {
        if self.is_in_bounds_and_unmarked(x, y) {
            let index = self.compute_index(x, y);
            self.reveal(index)?;
            sweeps.push_back(index);
        }

        Ok(())
    }

    fn get_color_brush_from_mine_state(&self, state: MineState) -> winrt::Result<CompositionColorBrush> {
        let brush = match state {
            MineState::Empty => self.compositor.create_color_brush_with_color(Colors::blue()?)?,
            MineState::Flag => self.compositor.create_color_brush_with_color(Colors::orange()?)?,
            MineState::Question => self.compositor.create_color_brush_with_color(Colors::lime_green()?)?,
            _ => self.compositor.create_color_brush_with_color(Colors::black()?)?,
        };
        Ok(brush)
    }

    fn get_color_brush_from_mine_count(&self, count: i32) -> winrt::Result<CompositionColorBrush> {
        let brush = match count {
            1 => self.compositor.create_color_brush_with_color(Colors::light_blue()?)?,
            2 => self.compositor.create_color_brush_with_color(Colors::light_green()?)?,
            3 => self.compositor.create_color_brush_with_color(Colors::light_salmon()?)?,
            4 => self.compositor.create_color_brush_with_color(Colors::light_steel_blue()?)?,
            5 => self.compositor.create_color_brush_with_color(Colors::medium_purple()?)?,
            6 => self.compositor.create_color_brush_with_color(Colors::light_cyan()?)?,
            7 => self.compositor.create_color_brush_with_color(Colors::maroon()?)?,
            8 => self.compositor.create_color_brush_with_color(Colors::dark_sea_green()?)?,
            _ => self.compositor.create_color_brush_with_color(Colors::white_smoke()?)?,
        };
        Ok(brush)
    }

    fn generate_mines(&mut self, num_mines: i32) {
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
            // do while loops look weird in rust...
            while {
                index = between.sample(&mut rng);
                self.mines[index]
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
}