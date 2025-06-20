use log::{error, info};
use notan::draw::*;
use notan::prelude::*;

pub struct RenderUtil {
    pub x_offset: f32,
    pub y_offset: f32,
    pub camera: Camera,
}

impl RenderUtil {
    pub fn new() -> Self {
        RenderUtil {
            x_offset: 0.0,
            y_offset: 0.0,
            camera: Camera::new(),
        }
    }

    pub fn draw(&mut self, gfx: &mut Graphics) {
        info!("render!");

        let mut draw = gfx.create_draw();

        draw.clear(Color::BLACK);

        self.render_background(&mut draw);

        draw.set_alpha(0.35);
        draw.rect(
            (-self.x_offset, -self.y_offset),
            (draw.width(), draw.height()),
        )
        .color(Color {
            r: 0.0,
            g: 0.0,
            b: 70.0,
            a: 0.35,
        });

        self.render_grid_lines(&mut draw);

        gfx.render(&draw);
    }

    fn render_background(&mut self, draw: &mut Draw) {
        draw.rect((-self.x_offset, -self.y_offset), (14400.0, 2400.0))
            .color(Color::from_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0));
        draw.rect(
            (-self.x_offset, -self.y_offset + 2400.0),
            (14400.0, 14000.0 - 2400.0),
        )
        .color(Color::from_rgb(149.0 / 255.0, 196.0 / 255.0, 100.0 / 255.0));
    }

    fn render_grid_lines(&mut self, draw: &mut Draw) {
        let mut i = 0.0;
        while i < draw.width() {
            draw.line((i, 0.0), (i, draw.height()))
                .width(2.0)
                .color(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.25,
                });

            i += 30.0;
        }

        let mut i = 0.0;
        while i < draw.height() {
            draw.line((0.0, i), (draw.width(), i))
                .width(2.0)
                .color(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.25,
                });

            i += 30.0;
        }
    }
}

struct Camera {
    pub x: f32,
    pub y: f32,
}

impl Camera {
    fn new() -> Camera {
        Camera { x: 0.0, y: 0.0 }
    }
}
