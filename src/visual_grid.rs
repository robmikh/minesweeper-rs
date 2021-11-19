use crate::minesweeper::IndexHelper;
use crate::numerics::FromVector2;
use windows::{
    core::Result,
    Foundation::Numerics::{Vector2, Vector3},
    Graphics::SizeInt32,
    UI::{
        Colors,
        Composition::{Compositor, ContainerVisual, SpriteVisual},
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
    index_helper: IndexHelper,

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
    ) -> Result<Self> {
        let compositor = compositor.clone();
        let root = compositor.CreateContainerVisual()?;

        let selection_visual = compositor.CreateSpriteVisual()?;
        let color_brush = compositor.CreateColorBrushWithColor(Colors::Red()?)?;
        let nine_grid_brush = compositor.CreateNineGridBrush()?;
        nine_grid_brush.SetInsetsWithValues(margin.X, margin.Y, margin.X, margin.Y)?;
        nine_grid_brush.SetIsCenterHollow(true)?;
        nine_grid_brush.SetSource(color_brush)?;
        selection_visual.SetBrush(nine_grid_brush)?;
        selection_visual.SetOffset(Vector3::from_vector2(margin * -1.0, 0.0))?;
        selection_visual.SetIsVisible(false)?;
        selection_visual.SetSize(tile_size + margin * 2.0)?;

        let mut result = Self {
            compositor,
            root,

            tiles: Vec::new(),
            selection_visual,
            index_helper: IndexHelper::new(grid_size_in_tiles.Width, grid_size_in_tiles.Height),

            grid_width_in_tiles: grid_size_in_tiles.Width,
            grid_height_in_tiles: grid_size_in_tiles.Height,
            tile_size: tile_size.clone(),
            margin: margin.clone(),

            current_selection: None,
        };

        result.reset(grid_size_in_tiles)?;

        Ok(result)
    }

    pub fn reset(&mut self, grid_size_in_tiles: &SizeInt32) -> Result<()> {
        let children = self.root.Children()?;
        children.RemoveAll()?;
        self.tiles.clear();

        self.index_helper = IndexHelper::new(grid_size_in_tiles.Width, grid_size_in_tiles.Height);

        self.grid_width_in_tiles = grid_size_in_tiles.Width;
        self.grid_height_in_tiles = grid_size_in_tiles.Height;
        self.select_tile(None)?;

        self.root.SetSize(
            (&self.tile_size + &self.margin)
                * Vector2::new(
                    self.grid_width_in_tiles as f32,
                    self.grid_height_in_tiles as f32,
                ),
        )?;

        for x in 0..self.grid_width_in_tiles {
            for y in 0..self.grid_height_in_tiles {
                let visual = self.compositor.CreateSpriteVisual()?;
                visual.SetSize(&self.tile_size)?;
                visual.SetCenterPoint(Vector3::from_vector2(&self.tile_size / 2.0, 0.0))?;
                visual.SetOffset(Vector3::from_vector2(
                    (&self.margin / 2.0)
                        + ((&self.tile_size + &self.margin) * Vector2::new(x as f32, y as f32)),
                    0.0,
                ))?;

                children.InsertAtTop(&visual)?;
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

    pub fn size(&self) -> Result<Vector2> {
        self.root.Size()
    }

    pub fn hit_test(&self, point: &Vector2) -> Option<TileCoordinate> {
        let x = (point.X / (self.tile_size.X + self.margin.X)) as i32;
        let y = (point.Y / (self.tile_size.Y + self.margin.Y)) as i32;

        if self.index_helper.is_in_bounds(x, y) {
            Some(TileCoordinate { x, y })
        } else {
            None
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&SpriteVisual> {
        if self.index_helper.is_in_bounds(x, y) {
            Some(&self.tiles[self.index_helper.compute_index(x, y)])
        } else {
            None
        }
    }

    pub fn select_tile(&mut self, tile_coordinate: Option<TileCoordinate>) -> Result<()> {
        self.current_selection = tile_coordinate;
        if let Some(tile_coordinate) = tile_coordinate {
            let visual = &self.tiles[self
                .index_helper
                .compute_index(tile_coordinate.x, tile_coordinate.y)];
            self.selection_visual.SetParentForTransform(visual)?;
            self.selection_visual.SetIsVisible(true)?;
        } else {
            self.selection_visual.SetIsVisible(false)?;
        }

        Ok(())
    }

    pub fn current_selected_tile(&self) -> Option<TileCoordinate> {
        self.current_selection
    }
}
