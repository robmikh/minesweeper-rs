use crate::minesweeper::MineState;
use crate::windows::{
    foundation::{
        numerics::Vector2,
    },
    ui::{
        composition::{
            CompositionColorBrush, CompositionGeometry, CompositionShape, CompositionSpriteShape,
            Compositor,
        },
        Colors,
    },
};
use std::collections::HashMap;
use winrt::TryInto;

fn get_dot_shape(
    compositor: &Compositor,
    geometry: &CompositionGeometry,
    brush: &CompositionColorBrush,
    offset: Vector2,
) -> winrt::Result<CompositionSpriteShape> {
    let shape = compositor
        .create_sprite_shape_with_geometry(geometry)?;
    shape.set_fill_brush(brush)?;
    shape.set_offset(offset)?;
    Ok(shape)
}

pub struct CompAssets {
    mine_brush: CompositionColorBrush,
    mine_state_brushes: HashMap<MineState, CompositionColorBrush>,
    mine_count_background_brushes: HashMap<i32, CompositionColorBrush>,
    mine_count_shapes: HashMap<i32, CompositionShape>,
}

impl CompAssets {
    pub fn new(compositor: &Compositor, tile_size: &Vector2) -> winrt::Result<Self> {
        let mine_brush = compositor.create_color_brush_with_color(Colors::red()?)?;

        let mut result = Self {
            mine_brush: mine_brush,
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

    fn generate_assets(&mut self, compositor: &Compositor, tile_size: &Vector2) -> winrt::Result<()> {
        self.mine_state_brushes.clear();
        self.mine_state_brushes.insert(
            MineState::Empty,
            compositor
                .create_color_brush_with_color(Colors::blue()?)?,
        );
        self.mine_state_brushes.insert(
            MineState::Flag,
            compositor
                .create_color_brush_with_color(Colors::orange()?)?,
        );
        self.mine_state_brushes.insert(
            MineState::Question,
            compositor
                .create_color_brush_with_color(Colors::lime_green()?)?,
        );

        self.mine_count_background_brushes.clear();
        self.mine_count_background_brushes.insert(
            1,
            compositor
                .create_color_brush_with_color(Colors::light_blue()?)?,
        );
        self.mine_count_background_brushes.insert(
            2,
            compositor
                .create_color_brush_with_color(Colors::light_green()?)?,
        );
        self.mine_count_background_brushes.insert(
            3,
            compositor
                .create_color_brush_with_color(Colors::light_salmon()?)?,
        );
        self.mine_count_background_brushes.insert(
            4,
            compositor
                .create_color_brush_with_color(Colors::light_steel_blue()?)?,
        );
        self.mine_count_background_brushes.insert(
            5,
            compositor
                .create_color_brush_with_color(Colors::medium_purple()?)?,
        );
        self.mine_count_background_brushes.insert(
            6,
            compositor
                .create_color_brush_with_color(Colors::light_cyan()?)?,
        );
        self.mine_count_background_brushes.insert(
            7,
            compositor
                .create_color_brush_with_color(Colors::maroon()?)?,
        );
        self.mine_count_background_brushes.insert(
            8,
            compositor
                .create_color_brush_with_color(Colors::dark_sea_green()?)?,
        );
        self.mine_count_background_brushes.insert(
            0,
            compositor
                .create_color_brush_with_color(Colors::white_smoke()?)?,
        );

        self.mine_count_shapes.clear();
        let circle_geometry = compositor.create_ellipse_geometry()?;
        circle_geometry.set_radius(tile_size / 12.0)?;
        let circle_geometry: CompositionGeometry = circle_geometry.try_into()?;
        let dot_brush = compositor
            .create_color_brush_with_color(Colors::black()?)?;

        // 1
        {
            let container_shape = compositor.create_container_shape()?;
            let shapes = container_shape.shapes()?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                tile_size / 2.0,
            )?)?;
            self.mine_count_shapes
                .insert(1, container_shape.try_into()?);
        }
        // 2
        {
            let container_shape = compositor.create_container_shape()?;
            let shapes = container_shape.shapes()?;
            let third_x = tile_size.x / 3.0;
            let half_y = tile_size.y / 2.0;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: third_x,
                    y: half_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: third_x * 2.0,
                    y: half_y,
                },
            )?)?;
            self.mine_count_shapes
                .insert(2, container_shape.try_into()?);
        }
        // 3
        {
            let container_shape = compositor.create_container_shape()?;
            let shapes = container_shape.shapes()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                tile_size / 2.0,
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y * 3.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y,
                },
            )?)?;
            self.mine_count_shapes
                .insert(3, container_shape.try_into()?);
        }
        // 4
        {
            let container_shape = compositor.create_container_shape()?;
            let shapes = container_shape.shapes()?;
            let third_x = tile_size.x / 3.0;
            let third_y = tile_size.y / 3.0;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: third_x,
                    y: third_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: third_x * 2.0,
                    y: third_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: third_x,
                    y: third_y * 2.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: third_x * 2.0,
                    y: third_y * 2.0,
                },
            )?)?;
            self.mine_count_shapes
                .insert(4, container_shape.try_into()?);
        }
        // 5
        {
            let container_shape = compositor.create_container_shape()?;
            let shapes = container_shape.shapes()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                tile_size / 2.0,
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y * 3.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y * 3.0,
                },
            )?)?;
            self.mine_count_shapes
                .insert(5, container_shape.try_into()?);
        }
        // 6
        {
            let container_shape = compositor.create_container_shape()?;
            let shapes = container_shape.shapes()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y * 2.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y * 3.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y * 3.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y * 2.0,
                },
            )?)?;
            self.mine_count_shapes
                .insert(6, container_shape.try_into()?);
        }
        // 7
        {
            let container_shape = compositor.create_container_shape()?;
            let shapes = container_shape.shapes()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y * 2.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y * 3.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y * 3.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y * 2.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                tile_size / 2.0,
            )?)?;
            self.mine_count_shapes
                .insert(7, container_shape.try_into()?);
        }
        // 8
        {
            let container_shape = compositor.create_container_shape()?;
            let shapes = container_shape.shapes()?;
            let fourth_x = tile_size.x / 4.0;
            let fourth_y = tile_size.y / 4.0;
            let half_x = tile_size.x / 2.0;
            let third_y = tile_size.y / 3.0;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y * 2.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y * 3.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x,
                    y: fourth_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y * 3.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: fourth_x * 3.0,
                    y: fourth_y * 2.0,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: half_x,
                    y: third_y,
                },
            )?)?;
            shapes.append(get_dot_shape(
                compositor,
                &circle_geometry,
                &dot_brush,
                Vector2 {
                    x: half_x,
                    y: third_y * 2.0,
                },
            )?)?;
            self.mine_count_shapes
                .insert(8, container_shape.try_into()?);
        }

        Ok(())
    }
}