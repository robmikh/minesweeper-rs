use crate::minesweeper::MineState;
use std::collections::HashMap;
use windows::{
    core::{ComInterface, Result},
    Foundation::Numerics::Vector2,
    UI::{
        Colors,
        Composition::{
            CompositionColorBrush, CompositionGeometry, CompositionShape,
            CompositionShapeCollection, CompositionSpriteShape, Compositor,
        },
    },
};

fn get_dot_shape(
    compositor: &Compositor,
    geometry: &CompositionGeometry,
    brush: &CompositionColorBrush,
    offset: Vector2,
) -> Result<CompositionSpriteShape> {
    let shape = compositor.CreateSpriteShapeWithGeometry(geometry)?;
    shape.SetFillBrush(brush)?;
    shape.SetOffset(offset)?;
    Ok(shape)
}

pub struct CompAssets {
    mine_brush: CompositionColorBrush,
    mine_state_brushes: HashMap<MineState, CompositionColorBrush>,
    mine_count_background_brushes: HashMap<i32, CompositionColorBrush>,
    mine_count_shapes: HashMap<i32, CompositionShape>,
}

impl CompAssets {
    pub fn new(compositor: &Compositor, tile_size: &Vector2) -> Result<Self> {
        let mine_brush = compositor.CreateColorBrushWithColor(Colors::Red()?)?;

        let mut result = Self {
            mine_brush,
            mine_state_brushes: HashMap::new(),
            mine_count_background_brushes: HashMap::new(),
            mine_count_shapes: HashMap::new(),
        };

        result.generate_assets(compositor, tile_size)?;

        Ok(result)
    }

    pub fn get_mine_brush(&self) -> CompositionColorBrush {
        self.mine_brush.clone()
    }

    pub fn get_shape_from_mine_count(&self, count: i32) -> CompositionShape {
        self.mine_count_shapes.get(&count).unwrap().clone()
    }

    pub fn get_color_brush_from_mine_state(&self, state: MineState) -> CompositionColorBrush {
        self.mine_state_brushes.get(&state).unwrap().clone()
    }

    pub fn get_color_brush_from_mine_count(&self, count: i32) -> CompositionColorBrush {
        self.mine_count_background_brushes
            .get(&count)
            .unwrap()
            .clone()
    }

    fn generate_assets(&mut self, compositor: &Compositor, tile_size: &Vector2) -> Result<()> {
        self.mine_state_brushes.clear();
        self.mine_state_brushes.insert(
            MineState::Empty,
            compositor.CreateColorBrushWithColor(Colors::Blue()?)?,
        );
        self.mine_state_brushes.insert(
            MineState::Flag,
            compositor.CreateColorBrushWithColor(Colors::Orange()?)?,
        );
        self.mine_state_brushes.insert(
            MineState::Question,
            compositor.CreateColorBrushWithColor(Colors::LimeGreen()?)?,
        );

        self.mine_count_background_brushes.clear();
        self.mine_count_background_brushes.insert(
            1,
            compositor.CreateColorBrushWithColor(Colors::LightBlue()?)?,
        );
        self.mine_count_background_brushes.insert(
            2,
            compositor.CreateColorBrushWithColor(Colors::LightGreen()?)?,
        );
        self.mine_count_background_brushes.insert(
            3,
            compositor.CreateColorBrushWithColor(Colors::LightSalmon()?)?,
        );
        self.mine_count_background_brushes.insert(
            4,
            compositor.CreateColorBrushWithColor(Colors::LightSteelBlue()?)?,
        );
        self.mine_count_background_brushes.insert(
            5,
            compositor.CreateColorBrushWithColor(Colors::MediumPurple()?)?,
        );
        self.mine_count_background_brushes.insert(
            6,
            compositor.CreateColorBrushWithColor(Colors::LightCyan()?)?,
        );
        self.mine_count_background_brushes
            .insert(7, compositor.CreateColorBrushWithColor(Colors::Maroon()?)?);
        self.mine_count_background_brushes.insert(
            8,
            compositor.CreateColorBrushWithColor(Colors::DarkSeaGreen()?)?,
        );
        self.mine_count_background_brushes.insert(
            0,
            compositor.CreateColorBrushWithColor(Colors::WhiteSmoke()?)?,
        );

        self.mine_count_shapes.clear();
        let circle_geometry = compositor.CreateEllipseGeometry()?;
        circle_geometry.SetRadius(tile_size / 12.0)?;
        let circle_geometry: CompositionGeometry = circle_geometry.cast()?;
        let dot_brush = compositor.CreateColorBrushWithColor(Colors::Black()?)?;

        let append_shape = |shapes: &CompositionShapeCollection, vector| {
            shapes.Append(&get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                vector,
            )?)
        };

        // 1
        {
            let container_shape = compositor.CreateContainerShape()?;
            let shapes = container_shape.Shapes()?;
            append_shape(&shapes, tile_size / 2.0)?;
            self.mine_count_shapes.insert(1, container_shape.cast()?);
        }
        // 2
        {
            let container_shape = compositor.CreateContainerShape()?;
            let shapes = container_shape.Shapes()?;
            let third_x = tile_size.X / 3.0;
            let half_y = tile_size.Y / 2.0;
            append_shape(&shapes, Vector2::new(third_x, half_y))?;
            append_shape(&shapes, Vector2::new(third_x * 2.0, half_y))?;
            self.mine_count_shapes.insert(2, container_shape.cast()?);
        }
        // 3
        {
            let container_shape = compositor.CreateContainerShape()?;
            let shapes = container_shape.Shapes()?;
            let fourth_x = tile_size.X / 4.0;
            let fourth_y = tile_size.Y / 4.0;
            append_shape(&shapes, tile_size / 2.0)?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y))?;
            self.mine_count_shapes.insert(3, container_shape.cast()?);
        }
        // 4
        {
            let container_shape = compositor.CreateContainerShape()?;
            let shapes = container_shape.Shapes()?;
            let third_x = tile_size.X / 3.0;
            let third_y = tile_size.Y / 3.0;
            append_shape(&shapes, Vector2::new(third_x, third_y))?;
            append_shape(&shapes, Vector2::new(third_x * 2.0, third_y))?;
            append_shape(&shapes, Vector2::new(third_x, third_y * 2.0))?;
            append_shape(&shapes, Vector2::new(third_x * 2.0, third_y * 2.0))?;
            self.mine_count_shapes.insert(4, container_shape.cast()?);
        }
        // 5
        {
            let container_shape = compositor.CreateContainerShape()?;
            let shapes = container_shape.Shapes()?;
            let fourth_x = tile_size.X / 4.0;
            let fourth_y = tile_size.Y / 4.0;
            append_shape(&shapes, tile_size / 2.0)?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y))?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y * 3.0))?;
            self.mine_count_shapes.insert(5, container_shape.cast()?);
        }
        // 6
        {
            let container_shape = compositor.CreateContainerShape()?;
            let shapes = container_shape.Shapes()?;
            let fourth_x = tile_size.X / 4.0;
            let fourth_y = tile_size.Y / 4.0;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y * 2.0))?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y))?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y * 3.0))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y * 2.0))?;
            self.mine_count_shapes.insert(6, container_shape.cast()?);
        }
        // 7
        {
            let container_shape = compositor.CreateContainerShape()?;
            let shapes = container_shape.Shapes()?;
            let fourth_x = tile_size.X / 4.0;
            let fourth_y = tile_size.Y / 4.0;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y * 2.0))?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y))?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y * 3.0))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y * 2.0))?;
            append_shape(&shapes, tile_size / 2.0)?;
            self.mine_count_shapes.insert(7, container_shape.cast()?);
        }
        // 8
        {
            let container_shape = compositor.CreateContainerShape()?;
            let shapes = container_shape.Shapes()?;
            let fourth_x = tile_size.X / 4.0;
            let fourth_y = tile_size.Y / 4.0;
            let half_x = tile_size.X / 2.0;
            let third_y = tile_size.Y / 3.0;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y * 2.0))?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y * 3.0))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y))?;
            append_shape(&shapes, Vector2::new(fourth_x, fourth_y))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y * 3.0))?;
            append_shape(&shapes, Vector2::new(fourth_x * 3.0, fourth_y * 2.0))?;
            append_shape(&shapes, Vector2::new(half_x, third_y))?;
            append_shape(&shapes, Vector2::new(half_x, third_y * 2.0))?;
            self.mine_count_shapes.insert(8, container_shape.cast()?);
        }

        Ok(())
    }
}
