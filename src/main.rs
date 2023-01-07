mod ui;
mod window_manager;

use glium::{glutin::event, Surface};
use ui::button::button;
use window_manager::window_manager::{KeyEnum, Window};

fn main() {
    let mut event_loop = glium::glutin::event_loop::EventLoop::new();

    let window = Window::new()
        .set_frame_rate(60)
        .set_width(1280.0)
        .set_height(720.0)
        .build(&event_loop);

    let mut left_bar = button::Button::new(window.display.as_ref().unwrap());

    window.run(event_loop, move |window, events, frame| {
        if events.has_pressed(KeyEnum::Up) {
            left_bar.move_y(0.02);
        }

        if events.has_pressed(KeyEnum::Right) {
            left_bar.move_x(0.02);
        }

        if events.has_pressed(KeyEnum::Down) {
            left_bar.move_y(-0.02);
        }

        if events.has_pressed(KeyEnum::Left) {
            left_bar.move_x(-0.02);
        }

        frame.clear_color(0.3, 1.0, 0.0, 1.0);
        left_bar.draw(frame);
    });
}
