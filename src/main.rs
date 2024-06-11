use ffi::Camera2D;
use raylib::math::Vector2;
use raylib::prelude::*;

const AXIS_LENGTH: i32 = 300;
const HANDLE_RADIUS: f32 = 10.0;

fn main() {
    let bg_color = Color::from_hex("3c3836").unwrap();
    let axis_color: Color = Color::WHITE;
    let handle_color = Color::from_hex("cc241d").unwrap();
    let hover_color = Color::from_hex("d79921").unwrap();

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

    let mut handle_pos = Vector2::zero();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mut mode2d = d.begin_mode2D(camera);

        mode2d.clear_background(bg_color);
        mode2d.draw_line(0, 0, AXIS_LENGTH, 0, axis_color);
        mode2d.draw_line(0, 0, 0, -AXIS_LENGTH, axis_color);


        let mouse = mode2d.get_screen_to_world2D(mode2d.get_mouse_position(), camera);
        let is_on_hover = check_collision_point_circle(mouse, handle_pos, HANDLE_RADIUS);

        let circle_color = if is_on_hover {
            hover_color
        } else {
            handle_color
        };

        let is_dragging = is_on_hover && mode2d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        if is_dragging {
            handle_pos = mouse;
        } 

        mode2d.draw_circle_v(handle_pos, HANDLE_RADIUS, circle_color);
    }
}
