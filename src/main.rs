mod ui;
mod window_manager;

use glium::Surface;
use ui::button::button;
use window_manager::window_manager::{KeyEnum, Window};

fn main() {
    let event_loop = glium::glutin::event_loop::EventLoop::new();

    let window = Window::new()
        .set_frame_rate(60)
        .set_width(1280.0)
        .set_height(720.0)
        .build(&event_loop);

    let mut user = button::Button::new(window.display.as_ref().unwrap(), -0.9, 0.0, 20.0, 80.0);
    let mut ai = button::Button::new(window.display.as_ref().unwrap(), 0.9, 0.0, 20.0, 80.0);

    window.run(event_loop, move |window, events, frame| {
        if events.has_pressed(KeyEnum::W) {
            user.move_y(0.02);
        }

        if events.has_pressed(KeyEnum::S) {
            user.move_y(-0.02);
        }

        if events.has_requested_close() {
            // TODO: Save game state
            println!("Closing window");
        }

        frame.clear_color(0.0, 0.0, 0.0, 0.0);
        user.draw(frame);
        ai.draw(frame);
    });
}
