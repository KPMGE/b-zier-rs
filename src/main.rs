use ffi::Camera2D;
use raylib::math::Vector2;
use raylib::prelude::*;

const AXIS_LENGTH: i32 = 300;
const AXIS_COLOR: Color = Color::BLUE;

fn main() {
    let (mut rl, thread) = raylib::init().size(740, 580).title("Bézier").build();

    let screen_width = rl.get_screen_width() as u32;
    let screen_height = rl.get_screen_height() as u32;

    let offset_width = screen_width as f32 / 2.0 - AXIS_LENGTH as f32 / 2.0;
    let offset_height = screen_height as f32 / 2.0 + AXIS_LENGTH as f32 / 2.0;
    let camera = Camera2D {
        zoom: 1.0,
        offset: Vector2::new(offset_width, offset_height).into(),
        target: Vector2::zero().into(),
        rotation: 0.0,
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mut mode2d = d.begin_mode2D(camera);

        mode2d.clear_background(Color::WHITE);
        mode2d.draw_line(0, 0, AXIS_LENGTH, 0, AXIS_COLOR);
        mode2d.draw_line(0, 0, 0, -AXIS_LENGTH, AXIS_COLOR);
    }
}
