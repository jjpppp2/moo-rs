use log::info;
use notan::draw::*;
use notan::prelude::*;
use std::cmp::min;

use crate::class::player::Player;

pub struct RenderUtil {
    pub x_offset: f32,
    pub y_offset: f32,
    pub camera: Camera,
}

impl RenderUtil {
    pub fn new() -> Self {
        RenderUtil {
            x_offset: 7200.0,
            y_offset: 7200.0,
            camera: Camera::new(),
        }
    }

    pub fn draw(
        &mut self,
        mut app: &mut App,
        gfx: &mut Graphics,
        my_player_id: u64,
        players: Vec<Player>,
    ) {
        let mut draw = gfx.create_draw();

        draw.clear(Color::BLACK);

        let my_player = players.iter().find(|x| x.id == my_player_id);

        match my_player {
            Some(player) => {
                self.x_offset = player.x - (draw.width() / 2.0);
                self.y_offset = player.y - (draw.height() / 2.0);

                info!("{} {}", player.x, player.y);
            }

            None => {}
        }

        self.render_background(&mut draw);
        draw.set_blend_mode(Some(BlendMode::NORMAL));

        draw.rect((-self.x_offset, -self.y_offset), (14400.0, 14400.0))
            .color(Color::from_rgba(
                0.0 / 255.0,
                0.0 / 255.0,
                70.0 / 255.0,
                0.35 / 1.0,
            ));

            draw.set_alpha(0.06);
        self.render_grid_lines(&mut draw, my_player);
        draw.set_alpha(1.0);

        self.render_players(&mut app, &mut draw, players);

        gfx.render(&draw);
    }

    fn render_background(&mut self, draw: &mut Draw) {
        draw.rect((-self.x_offset, -self.y_offset), (14400.0, 2400.0))
            .color(Color::from_rgb(255.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0));

        draw.rect(
            (-self.x_offset, -self.y_offset + 2400.0),
            (14400.0, 14400.0 - (2400.0 * 2.0)),
        )
        .color(Color::from_rgb(182.0 / 255.0, 219.0 / 255.0, 102.0 / 255.0));

        draw.rect(
            (-self.x_offset, -self.y_offset + (14400.0 - 2400.0)),
            (14400.0, 2400.0),
        )
        .color(Color::from_rgb(219.0 / 255.0, 198.0 / 255.0, 102.0 / 255.0));
    }

    fn render_grid_lines(&mut self, draw: &mut Draw, my_player: Option<&Player>) {
        match my_player {
            Some(my_player) => {
    let gap = draw.width() / 18.0;
    
    // Draw vertical lines
    let mut x = -my_player.x % gap; // Start at the first grid line relative to player position
    while x < draw.width() {
        if x >= 0.0 {
            draw.line((x, 0.0), (x, draw.height()))
                .width(4.0)
                .color(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                });
        }
        x += gap;
    }

    // Draw horizontal lines
    let mut y = -my_player.y % gap; // Start at the first grid line relative to player position
    while y < draw.height() {
        if y >= 0.0 {
            draw.line((0.0, y), (draw.width(), y))
                .width(4.0)
                .color(Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                });
        }
        y += gap;
    }
}

            None => {}
        }
    }

    fn render_players(&mut self, app: &mut App, draw: &mut Draw, players: Vec<Player>) {
        //draw.circle(35.0).position(0.0, 0.0).color(Color::RED);

        let delta = app.timer.delta_f32() * 1000.0;
        let elapsed_time = delta - (1000.0 / 103.0);
        for mut player in players {
            // update the player's coords and stuff
            let time_delta = player.time_2 - player.time_1;
            let interporation_factor = (elapsed_time - player.time_1) / time_delta;
            let smoothing_duration = 170.0;

            player.delta += delta;
            let smoothing_factor = (player.delta / smoothing_duration).min(1.7);

            player.lerp_x = player.last_lerp_x + (player.x - player.last_lerp_x) * smoothing_factor;
            player.lerp_y = player.last_lerp_y + (player.y - player.last_lerp_y) * smoothing_factor;

            // player.dir = lerp_angle(player.dir, player.last_dir, min(1.2, interporation_factor));

            /*
                const smoothingFactor = Math.min(1.7, entity.dt / smoothingDuration);

            // interpolate position
            entity.x = entity.x1 + (entity.x2 - entity.x1) * smoothingFactor;
            entity.y = entity.y1 + (entity.y2 - entity.y1) * smoothingFactor;

            // interpolate direction
            entity.dir = Math.lerpAngle(entity.d2, entity.d1, Math.min(1.2, interpolationFactor));
            */

            draw.circle(35.0)
                .position(player.x - self.x_offset, player.y - self.y_offset)
                .color(Color::RED);
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
