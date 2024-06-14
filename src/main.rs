use ffi::Camera2D;
use raylib::math::Vector2;
use raylib::prelude::*;

const AXIS_LENGTH: i32 = 300;
const HANDLE_RADIUS: f32 = 10.0;

struct Handle {
    center_x: i32,
    center_y: i32,
    radius: f32,
}

impl Handle {
    fn new(center_x: i32, center_y: i32, radius: f32) -> Self {
        Self {
            center_x,
            center_y,
            radius,
        }
    }
}

fn main() {
    let bg_color = Color::from_hex("3c3836").unwrap();
    let axis_color: Color = Color::WHITE;
    let handle_color = Color::from_hex("cc241d").unwrap();
    let hover_color = Color::from_hex("d79921").unwrap();
    let selected_color = Color::WHITE;

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

    let mut dragging_handle_idx = None;

    let mut handles = vec![
        Handle::new(0, 0, HANDLE_RADIUS),
        Handle::new((AXIS_LENGTH as f32 * 0.25) as i32, (-AXIS_LENGTH as f32 * 0.25) as i32, HANDLE_RADIUS),
        Handle::new((AXIS_LENGTH as f32 * 0.75) as i32, (-AXIS_LENGTH as f32 * 0.75) as i32, HANDLE_RADIUS),
        Handle::new(AXIS_LENGTH, -AXIS_LENGTH, HANDLE_RADIUS),
    ];

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let mut mode2d = d.begin_mode2D(camera);

        mode2d.clear_background(bg_color);
        mode2d.draw_line(0, 0, AXIS_LENGTH, 0, axis_color);
        mode2d.draw_line(0, 0, 0, -AXIS_LENGTH, axis_color);

        let first_handle = handles.get(0).unwrap();
        let second_handle = handles.get(1).unwrap();
        let third_handle = handles.get(2).unwrap();
        let fourth_handle = handles.get(3).unwrap();
        mode2d.draw_line(first_handle.center_x, first_handle.center_y, second_handle.center_x, second_handle.center_y, Color::ROYALBLUE);
        mode2d.draw_line(third_handle.center_x, third_handle.center_y, fourth_handle.center_x, fourth_handle.center_y, Color::ROYALBLUE);

        for (idx, handle) in handles.iter_mut().enumerate() {
            let mouse = mode2d.get_screen_to_world2D(mode2d.get_mouse_position(), camera);
            let is_handle_on_hover = check_collision_point_circle(
                mouse,
                Vector2::new(handle.center_x as f32, handle.center_y as f32),
                HANDLE_RADIUS,
            );
            
            let is_handle_selected = Some(idx) == dragging_handle_idx;

            let color = if is_handle_selected {
                selected_color
            } else if is_handle_on_hover {
                hover_color
            } else {
                handle_color
            };

            if is_handle_selected {
                handle.center_x = mouse.x as i32;
                handle.center_y = mouse.y as i32;
            }

            mode2d.draw_circle(handle.center_x, handle.center_y, handle.radius, color);

            // if we hover over a handle and click, we should select it
            if is_handle_on_hover && mode2d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                dragging_handle_idx = Some(idx);
            }

            // if we're dragging a handle and we release the mouse, we should unselect it
            if is_handle_selected && mode2d.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
                dragging_handle_idx = None;
            }
        }
    }
}
