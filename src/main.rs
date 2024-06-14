use ffi::Camera2D;
use raylib::math::Vector2;
use raylib::prelude::*;

const AXIS_LENGTH: i32 = 300;
const HANDLE_RADIUS: f32 = 10.0;
const BEZIER_CURVE_RESOLUTION: i32 = 50;
const BEZIER_CURVE_CIRCLE_RADIUS: f32 = 3.0;

fn main() {
    let bg_color = Color::from_hex("3c3836").unwrap();
    let axis_color: Color = Color::WHITE;
    let bezier_curve_circle_color = Color::from_hex("8ec07c").unwrap();
    let circle_default_color = Color::from_hex("cc241d").unwrap();
    let circle_hover_color = Color::from_hex("d79921").unwrap();
    let selected_circle_color = Color::WHITE;

    let (mut rl, thread) = raylib::init().size(740, 580).title("Bézier").build();
    let offset_width = rl.get_screen_width() as f32 / 2.0 - AXIS_LENGTH as f32 / 2.0;
    let offset_height = rl.get_screen_height() as f32 / 2.0 + AXIS_LENGTH as f32 / 2.0;
    let camera = Camera2D {
        zoom: 1.0,
        offset: Vector2::new(offset_width, offset_height).into(),
        target: Vector2::zero().into(),
        rotation: 0.0,
    };

    let mut selected_circle_idx: Option<usize> = None;

    let mut circles = vec![
        Vector2::zero(),
        Vector2::new(AXIS_LENGTH as f32 * 0.25, -AXIS_LENGTH as f32 * 0.5),
        Vector2::new(AXIS_LENGTH as f32 * 0.75, -AXIS_LENGTH as f32 * 0.5),
        Vector2::new(AXIS_LENGTH as f32, -AXIS_LENGTH as f32),
    ];

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mut mode2d = d.begin_mode2D(camera);

        mode2d.clear_background(bg_color);
        mode2d.draw_line(0, 0, AXIS_LENGTH, 0, axis_color);
        mode2d.draw_line(0, 0, 0, -AXIS_LENGTH, axis_color);

        let c1 = circles.get(0).unwrap().clone();
        let c2 = circles.get(1).unwrap().clone();
        let c3 = circles.get(2).unwrap().clone();
        let c4 = circles.get(3).unwrap().clone();

        mode2d.draw_line_v(c1, c2, Color::ROYALBLUE);
        mode2d.draw_line_v(c3, c4, Color::ROYALBLUE);

        for (idx, circle) in circles.iter_mut().enumerate() {
            let mouse = mode2d.get_screen_to_world2D(mode2d.get_mouse_position(), camera);
            let is_handle_on_hover = check_collision_point_circle(mouse, *circle, HANDLE_RADIUS);

            let is_circle_selected = Some(idx) == selected_circle_idx;

            let circle_color = if is_circle_selected {
                selected_circle_color
            } else if is_handle_on_hover {
                circle_hover_color
            } else {
                circle_default_color
            };

            if is_circle_selected {
                *circle = mouse;
            }

            mode2d.draw_circle_v(*circle, HANDLE_RADIUS, circle_color);

            if selected_circle_idx == None
                && is_handle_on_hover
                && mode2d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT)
            {
                selected_circle_idx = Some(idx);
            }

            if is_circle_selected && mode2d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT)
            {
                selected_circle_idx = None;
            }

            for i in 1..BEZIER_CURVE_RESOLUTION {
                let t = i as f32 / BEZIER_CURVE_RESOLUTION as f32;
                let k = 1.0 - t;

                let e1 = c1.clone().scale_by(k.powf(3.0));
                let e2 = c2.clone().scale_by(3.0 * k.powf(2.0) * t);
                let e3 = c3.clone().scale_by(3.0 * k * t.powf(2.0));
                let e4 = c4.clone().scale_by(t.powf(3.0));
                let b = e1 + e2 + e3 + e4;

                mode2d.draw_circle_v(b, BEZIER_CURVE_CIRCLE_RADIUS, bezier_curve_circle_color);
            }
        }
    }
}
