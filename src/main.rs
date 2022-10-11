use macroquad::{prelude::*, window};

use crate::mesh_field::MeshField;
use crate::particle::Particle;
use crate::render::Render;
use crate::math::Vector2D;

mod math;
mod particle;
mod line;
mod mesh_field;
mod render;

const WINDOW_H: f32 = 480.0;
const WINDOW_W: f32 = 640.0;
const MAX_LINE_LEN: f32 = 100.0;
const PARTICLE_COUNT: usize = 100;

#[macroquad::main("Tyler Time")]
async fn main() {
    let size = Vector2D::new(screen_width(), screen_height());
    let mut mesh_field = MeshField::new(PARTICLE_COUNT, size, MAX_LINE_LEN);

    while "tyler" != "fit" {
        clear_background(DARKGRAY);
        //request_new_screen_size(WINDOW_W, WINDOW_H);

        mesh_field.particles().iter().for_each(|x| x.render(RED));
        mesh_field.lines().iter().for_each(|x| x.render(RED));

        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        mesh_field.update_screen_size(Vector2D::new(screen_width(), screen_height()));
        mesh_field.update_all();

        next_frame().await
    }
}