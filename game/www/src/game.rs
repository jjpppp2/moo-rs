use macroquad::prelude::*;

use crate::{config::{BackgroundColors, MapConfig}, ws::WsClient};

pub struct Camera {
    pub x: f32,
    pub y: f32,
}

pub struct RenderUtil {
    pub camera: Camera,
    pub x_offset: f32,
    pub y_offset: f32,
    pub map_config: MapConfig,
}

pub struct Game {
    pub render: RenderUtil,
    //pub ws: WsClient,
}

impl Game {
    pub fn new() -> Self {
        Game {
            render: RenderUtil::new(),
            //ws: WsClient::new()
        }
    }
}

impl RenderUtil {
    pub fn new() -> Self {
        RenderUtil {
            camera: Camera::new(),
            x_offset: 7200.0,
            y_offset: 6700.0,
            map_config: MapConfig::default(),
        }
    }

    pub fn render_background(&self) {
        // winter biome
        draw_rectangle(
            0.0 - self.x_offset,
            0.0 - self.y_offset,
            self.map_config.size,
            self.map_config.snow_biome_y,
            BackgroundColors::winter(),
        );

        // grasslands biome
        draw_rectangle(
            0.0 - self.x_offset,
            self.map_config.snow_biome_y - self.y_offset,
            self.map_config.size,
            self.map_config.size - self.map_config.snow_biome_y,
            BackgroundColors::grassland(),
        );

        // desert
        draw_rectangle(
            0.0 - self.x_offset,
            self.map_config.size - self.map_config.snow_biome_y - self.y_offset,
            self.map_config.size,
            self.map_config.snow_biome_y,
            BackgroundColors::desert(),
        );

        // river stuff
        let mid_y = self.map_config.size / 2.0;
        // setup a padding first
        draw_rectangle(
            0.0 - self.x_offset,
            mid_y
                - self.y_offset
                - (self.map_config.river_size / 2.0)
                - self.map_config.river_padding,
            self.map_config.size,
            self.map_config.river_size + self.map_config.river_padding,
            BackgroundColors::riverbed(),
        );
        // actual juicy river
        draw_rectangle(0.0 - self.x_offset, mid_y - self.y_offset - (self.map_config.river_size / 2.0) + self.map_config.river_padding, self.map_config.size, self.map_config.river_size - (self.map_config.river_padding * 3.0), BackgroundColors::river());
    }

    pub fn render_grid_lines(&self) {
        // replace grid later, is bad
        let mut i = 0.0;
        while i < self.map_config.size {
            draw_line(
                0.0,
                i,
                self.map_config.size,
                i,
                4.0,
                BackgroundColors::grid_color(),
            );

            i = i + self.map_config.grid_spacing as f32;
        }

        let mut i = 0.0;
        while i < self.map_config.size {
            draw_line(
                i,
                0.0,
                i,
                self.map_config.size,
                4.0,
                BackgroundColors::grid_color(),
            );

            i = i + self.map_config.grid_spacing as f32;
        }
    }
}

impl Camera {
    pub fn new() -> Self {
        Camera { x: 0.0, y: 0.0 }
    }
}
