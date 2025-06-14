use macroquad::prelude::Color;

pub enum BackgroundColors {
    Winter(Color),
    Grassland(Color),
    Riverbed(Color),
    Desert(Color),
}

impl BackgroundColors {
    pub fn winter() -> Color {
        Color::new(230.0 / 255.0, 230.0 / 255.0, 230.0 / 255.0, 1.0)
    }

    pub fn grassland() -> Color {
        Color::new(110.0 / 255.0, 140.0 / 255.0, 90.0 / 255.0, 1.0)
    }

    pub fn river() -> Color {
        Color::new(70.0 / 255.0, 130.0 / 255.0, 180.0 / 255.0, 1.0)
    }

    pub fn riverbed() -> Color {
        Color::new(220.0 / 255.0, 180.0 / 255.0, 120.0 / 255.0, 1.0)
    }

    pub fn desert() -> Color {
        Color::new(220.0 / 255.0, 180.0 / 255.0, 120.0 / 255.0, 1.0)
    }

    pub fn grid_color() -> Color {
        Color::new(0.0, 0.0, 0.0, 0.06)
    }
}

pub struct MapConfig {
    pub size: f32,
    pub grid_spacing: u8,
    pub snow_biome_y: f32,
    pub river_size: f32,
    pub river_padding: f32,
}

impl Default for MapConfig {
    fn default() -> Self {
        MapConfig {
            size: 14400.0,
            grid_spacing: 69,
            snow_biome_y: 2400.0,
            river_size: 768.0,
            river_padding: 30.0
        }
    }
}
