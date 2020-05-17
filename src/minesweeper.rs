use crate::comp_ui::CompUI;
use crate::visual_grid::TileCoordinate;
use crate::windows::{
    foundation::numerics::Vector2, graphics::SizeInt32, ui::composition::ContainerVisual,
};
use rand::distributions::{Distribution, Uniform};
use std::collections::VecDeque;

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

pub struct IndexHelper {
    width: i32,
    height: i32,
}

impl IndexHelper {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn compute_index(&self, x: i32, y: i32) -> usize {
        (x * self.height + y) as usize
    }

    pub fn compute_x_from_index(&self, index: usize) -> i32 {
        index as i32 / self.height
    }

    pub fn compute_y_from_index(&self, index: usize) -> i32 {
        index as i32 % self.height
    }

    pub fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        (x >= 0 && x < self.width) && (y >= 0 && y < self.height)
    }
}

pub struct Minesweeper {
    ui: CompUI,

    game_board_width: i32,
    game_board_height: i32,
    index_helper: IndexHelper,

    mine_states: Vec<MineState>,
    mines: Vec<bool>,
    neighbor_counts: Vec<i32>,
    mine_generation_state: MineGenerationState,
    num_mines: i32,

    game_over: bool,
}

impl Minesweeper {
    pub fn new(parent_visual: &ContainerVisual, parent_size: &Vector2) -> winrt::Result<Self> {
        let game_board_size_in_tiles = SizeInt32 {
            width: 16,
            height: 16,
        };
        let ui = CompUI::new(parent_visual, parent_size, &game_board_size_in_tiles)?;

        let mut result = Self {
            ui,

            game_board_width: game_board_size_in_tiles.width,
            game_board_height: game_board_size_in_tiles.height,
            index_helper: IndexHelper::new(
                game_board_size_in_tiles.width,
                game_board_size_in_tiles.height,
            ),

            mine_states: Vec::new(),
            mines: Vec::new(),
            neighbor_counts: Vec::new(),
            mine_generation_state: MineGenerationState::Deferred,
            num_mines: 0,

            game_over: false,
        };

        result.new_game(16, 16, 40)?;
        result.on_parent_size_changed(parent_size)?;

        Ok(result)
    }

    pub fn on_pointer_moved(&mut self, point: &Vector2) -> winrt::Result<()> {
        if self.game_over || self.ui.is_animation_playing() {
            return Ok(());
        }

        let selected_tile = if let Some(tile) = self.ui.hit_test(&point)? {
            if self.mine_states[self.index_helper.compute_index(tile.x, tile.y)]
                != MineState::Revealed
            {
                Some(tile)
            } else {
                None
            }
        } else {
            None
        };
        self.ui.select_tile(selected_tile)?;

        Ok(())
    }

    pub fn on_parent_size_changed(&mut self, new_size: &Vector2) -> winrt::Result<()> {
        self.ui.resize(new_size)?;
        Ok(())
    }

    pub fn on_pointer_pressed(
        &mut self,
        is_right_button: bool,
        is_eraser: bool,
    ) -> winrt::Result<()> {
        // TODO: Switch the condition back once we can subscribe to events.
        //if self.game_over && !self.ui.is_animation_playing() {
        if self.game_over {
            self.new_game(
                self.game_board_width,
                self.game_board_height,
                self.num_mines,
            )?;
        }

        let current_selection = self.ui.current_selected_tile();
        if let Some(current_selection) = current_selection {
            let index = self
                .index_helper
                .compute_index(current_selection.x, current_selection.y);

            if self.mine_states[index] != MineState::Revealed {
                if is_right_button || is_eraser {
                    let state = self.mine_states[index].cycle();
                    self.mine_states[index] = state;
                    self.ui.update_tile_with_state(&current_selection, state)?;
                } else if self.mine_states[index] == MineState::Empty {
                    if self.sweep(current_selection.x, current_selection.y)? {
                        // We hit a mine! Setup and play an animation while locking any input.
                        let hit_x = current_selection.x;
                        let hit_y = current_selection.y;

                        // First, hide the selection visual and reset the selection
                        self.ui.select_tile(None)?;

                        self.play_animation_on_all_mines(hit_x, hit_y)?;

                        self.game_over = true;
                    } else if self.check_if_won() {
                        self.ui.select_tile(None)?;
                        // TODO: Play a win animation
                        self.game_over = true;
                    }
                }
            }
        }
        Ok(())
    }

    fn new_game(&mut self, board_width: i32, board_height: i32, mines: i32) -> winrt::Result<()> {
        self.game_board_width = board_width;
        self.game_board_height = board_height;
        self.index_helper = IndexHelper::new(board_width, board_height);

        self.ui.reset(&SizeInt32 {
            width: board_width,
            height: board_height,
        })?;
        self.mine_states.clear();

        for _ in 0..(board_width * board_height) {
            self.mine_states.push(MineState::Empty);
        }

        self.game_over = false;
        self.mine_generation_state = MineGenerationState::Deferred;
        self.num_mines = mines;

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
        sweeps.push_back(self.index_helper.compute_index(x, y));
        self.reveal(*sweeps.front().unwrap())?;

        while !sweeps.is_empty() {
            let index = *sweeps.front().unwrap();
            let current_x = self.index_helper.compute_x_from_index(index);
            let current_y = self.index_helper.compute_y_from_index(index);

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
        let tile_coordinate = TileCoordinate {
            x: self.index_helper.compute_x_from_index(index),
            y: self.index_helper.compute_y_from_index(index),
        };

        if self.mines[index] {
            self.ui.update_tile_as_mine(&tile_coordinate)?;
        } else {
            let count = self.neighbor_counts[index];
            self.ui
                .update_tile_with_mine_count(&tile_coordinate, count)?;
        }

        self.mine_states[index] = MineState::Revealed;
        Ok(())
    }

    fn is_in_bounds_and_unmarked(&self, x: i32, y: i32) -> bool {
        let index = self.index_helper.compute_index(x, y);
        self.index_helper.is_in_bounds(x, y) && self.mine_states[index] == MineState::Empty
    }

    fn push_if_unmarked(
        &mut self,
        sweeps: &mut VecDeque<usize>,
        x: i32,
        y: i32,
    ) -> winrt::Result<()> {
        if self.is_in_bounds_and_unmarked(x, y) {
            let index = self.index_helper.compute_index(x, y);
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
            let exclude_index = self.index_helper.compute_index(exclude_x, exclude_y);
            // do while loops look weird in rust...
            while {
                index = between.sample(&mut rng);
                index == exclude_index || self.mines[index]
            } {}

            self.mines[index] = true;
        }

        self.neighbor_counts.clear();
        for i in 0..self.mines.len() {
            let x = self.index_helper.compute_x_from_index(i);
            let y = self.index_helper.compute_y_from_index(i);

            if self.mines[i] {
                // -1 means a mine
                self.neighbor_counts.push(-1);
                // DEBUG
                if cfg!(feature = "show-mines") {
                    self.ui
                        .update_tile_with_state(&TileCoordinate { x, y }, MineState::Question)
                        .unwrap();
                }
            } else {
                let count = self.get_surrounding_mine_count(x, y);
                self.neighbor_counts.push(count);
            }
        }
    }

    fn test_spot(&self, x: i32, y: i32) -> bool {
        self.index_helper.is_in_bounds(x, y) && self.mines[self.index_helper.compute_index(x, y)]
    }

    fn get_surrounding_mine_count(&self, x: i32, y: i32) -> i32 {
        let mut count = 0;

        if self.test_spot(x + 1, y) {
            count += 1;
        }

        if self.test_spot(x - 1, y) {
            count += 1;
        }

        if self.test_spot(x, y + 1) {
            count += 1;
        }

        if self.test_spot(x, y - 1) {
            count += 1;
        }

        if self.test_spot(x + 1, y + 1) {
            count += 1;
        }

        if self.test_spot(x - 1, y - 1) {
            count += 1;
        }

        if self.test_spot(x - 1, y + 1) {
            count += 1;
        }

        if self.test_spot(x + 1, y - 1) {
            count += 1;
        }

        count
    }

    fn check_tile_for_mine_for_animation(
        &self,
        x: i32,
        y: i32,
        mine_indices: &mut VecDeque<usize>,
        visited_tiles: &mut i32,
        mines_in_ring: &mut i32,
    ) {
        if self.index_helper.is_in_bounds(x, y) {
            let tile_index = self.index_helper.compute_index(x, y);
            if self.mines[tile_index] {
                mine_indices.push_back(tile_index);
                *mines_in_ring += 1;
            }
            *visited_tiles += 1;
        }
    }

    fn play_animation_on_all_mines(&mut self, center_x: i32, center_y: i32) -> winrt::Result<()> {
        // Build a queue that contains the indices of the mines in a spiral starting from the clicked mine.
        let mut mine_indices: VecDeque<usize> = VecDeque::new();
        let mut mines_per_ring: VecDeque<i32> = VecDeque::new();
        let mut visited_tiles: i32 = 0;
        let mut ring_level: i32 = 0;
        while visited_tiles < (self.game_board_width * self.game_board_height) {
            if ring_level == 0 {
                let hit_mine_index = self.index_helper.compute_index(center_x, center_y);
                mine_indices.push_back(hit_mine_index);
                mines_per_ring.push_back(1);
                visited_tiles += 1;
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
            ring_level += 1;
        }

        // Iterate and animate each mine
        self.ui.play_mine_animations(mine_indices, mines_per_ring)?;

        Ok(())
    }

    fn check_if_won(&self) -> bool {
        // Get the number of non-revealed tiles
        let mut non_revealed_tiles = 0;
        for state in &self.mine_states {
            if *state != MineState::Revealed {
                non_revealed_tiles += 1;
            }
        }

        non_revealed_tiles == self.num_mines
    }
}
