use crate::numerics::FromVector2;
use crate::windows::{
    foundation::numerics::{Vector2, Vector3},
    graphics::SizeInt32,
    ui::{
        composition::{Compositor, ContainerVisual, SpriteVisual},
        Colors,
    },
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct TileCoordinate {
    pub x: i32,
    pub y: i32,
}

pub struct VisualGrid {
    compositor: Compositor,
    root: ContainerVisual,

    tiles: Vec<SpriteVisual>,
    selection_visual: SpriteVisual,

    grid_width_in_tiles: i32,
    grid_height_in_tiles: i32,
    tile_size: Vector2,
    margin: Vector2,

    current_selection: Option<TileCoordinate>,
}

impl VisualGrid {
    pub fn new(
        compositor: &Compositor,
        grid_size_in_tiles: &SizeInt32,
        tile_size: &Vector2,
        margin: &Vector2,
    ) -> winrt::Result<Self> {
        let compositor = compositor.clone();
        let root = compositor.create_container_visual()?;

        let selection_visual = compositor.create_sprite_visual()?;
        let color_brush = compositor.create_color_brush_with_color(Colors::red()?)?;
        let nine_grid_brush = compositor.create_nine_grid_brush()?;
        nine_grid_brush.set_insets_with_values(margin.x, margin.y, margin.x, margin.y)?;
        nine_grid_brush.set_is_center_hollow(true)?;
        nine_grid_brush.set_source(color_brush)?;
        selection_visual.set_brush(nine_grid_brush)?;
        selection_visual.set_offset(Vector3::from_vector2(margin * -1.0, 0.0))?;
        selection_visual.set_is_visible(false)?;
        selection_visual.set_size(tile_size + margin * 2.0)?;

        let mut result = Self {
            compositor,
            root,

            tiles: Vec::new(),
            selection_visual,

            grid_width_in_tiles: grid_size_in_tiles.width,
            grid_height_in_tiles: grid_size_in_tiles.height,
            tile_size: tile_size.clone(),
            margin: margin.clone(),

            current_selection: None,
        };

        result.reset(grid_size_in_tiles)?;

        Ok(result)
    }

    pub fn reset(&mut self, grid_size_in_tiles: &SizeInt32) -> winrt::Result<()> {
        let children = self.root.children()?;
        children.remove_all()?;
        self.tiles.clear();

        self.grid_width_in_tiles = grid_size_in_tiles.width;
        self.grid_height_in_tiles = grid_size_in_tiles.height;
        self.select_tile(None)?;

        self.root.set_size(
            (&self.tile_size + &self.margin)
                * Vector2 {
                    x: self.grid_width_in_tiles as f32,
                    y: self.grid_height_in_tiles as f32,
                },
        )?;

        for x in 0..self.grid_width_in_tiles {
            for y in 0..self.grid_height_in_tiles {
                let visual = self.compositor.create_sprite_visual()?;
                visual.set_size(&self.tile_size)?;
                visual.set_center_point(Vector3::from_vector2(&self.tile_size / 2.0, 0.0))?;
                visual.set_offset(Vector3::from_vector2(
                    (&self.margin / 2.0)
                        + ((&self.tile_size + &self.margin)
                            * Vector2 {
                                x: x as f32,
                                y: y as f32,
                            }),
                    0.0,
                ))?;

                children.insert_at_top(&visual)?;
                self.tiles.push(visual);
            }
        }

        Ok(())
    }

    pub fn tiles_iter(&self) -> impl std::iter::Iterator<Item = &SpriteVisual> {
        self.tiles.iter()
    }

    pub fn root(&self) -> &ContainerVisual {
        &self.root
    }

    pub fn selection_visual(&self) -> &SpriteVisual {
        &self.selection_visual
    }

    pub fn size(&self) -> winrt::Result<Vector2> {
        self.root.size()
    }

    pub fn hit_test(&self, point: &Vector2) -> Option<TileCoordinate> {
        let x = (point.x / (self.tile_size.x + self.margin.x)) as i32;
        let y = (point.y / (self.tile_size.y + self.margin.y)) as i32;

        if self.is_in_bounds(x, y) {
            Some(TileCoordinate { x, y })
        } else {
            None
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&SpriteVisual> {
        if self.is_in_bounds(x, y) {
            Some(&self.tiles[self.compute_index(x, y)])
        } else {
            None
        }
    }

    pub fn select_tile(&mut self, tile_coordinate: Option<TileCoordinate>) -> winrt::Result<()> {
        self.current_selection = tile_coordinate;
        if let Some(tile_coordinate) = tile_coordinate {
            let visual = &self.tiles[self.compute_index(tile_coordinate.x, tile_coordinate.y)];
            self.selection_visual.set_parent_for_transform(visual)?;
            self.selection_visual.set_is_visible(true)?;
        } else {
            self.selection_visual.set_is_visible(false)?;
        }

        Ok(())
    }

    pub fn current_selected_tile(&self) -> Option<TileCoordinate> {
        self.current_selection
    }

    fn compute_index(&self, x: i32, y: i32) -> usize {
        (x * self.grid_height_in_tiles + y) as usize
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        (x >= 0 && x < self.grid_width_in_tiles) && (y >= 0 && y < self.grid_height_in_tiles)
    }
}
