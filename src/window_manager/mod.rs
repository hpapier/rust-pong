extern crate glium;

#[path = "../ui/mod.rs"]
mod ui;

pub mod window_manager {
    use std::{collections::HashMap, time::Instant};

    use glium::{
        glutin::{
            event::{Event, KeyboardInput, WindowEvent},
            event_loop::EventLoop,
            window::WindowBuilder,
        },
        implement_vertex, Display, Frame,
    };

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
    }

    implement_vertex!(Vertex, position);

    pub struct Window {
        frame_rate: u32,
        title: String,
        width: f32,
        height: f32,
        pub display: Option<Display>,
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
    pub enum KeyEnum {
        Up,
        Right,
        Down,
        Left,
    }

    pub struct Events {
        pub key_pressed: HashMap<KeyEnum, bool>,
    }

    impl Events {
        pub fn has_pressed(&self, key: KeyEnum) -> bool {
            match self.key_pressed.get(&key) {
                Some(value) => true,
                None => false,
            }
        }

        pub fn set_key_pressed(&mut self, key: KeyEnum) -> () {
            self.key_pressed.insert(key, true);
        }
    }

    impl Window {
        pub fn new() -> Self {
            Window {
                frame_rate: 60,
                title: String::from("Window"),
                width: 800.0,
                height: 600.0,
                display: None,
            }
        }

        pub fn set_frame_rate(mut self, frame_rate: u32) -> Self {
            println!("Frame rate set to {}", frame_rate);
            self.frame_rate = frame_rate;
            self
        }

        pub fn set_title(mut self, title: String) -> Self {
            println!("Title set to {}", title);
            self.title = title;
            self
        }

        pub fn set_width(mut self, width: f32) -> Self {
            println!("Width set to {}", width);
            self.width = width;
            self
        }

        pub fn set_height(mut self, height: f32) -> Self {
            println!("Height set to {}", height);
            self.height = height;
            self
        }

        pub fn build(mut self, events_loop: &EventLoop<()>) -> Self {
            println!("Building window");

            let windowBuilder = WindowBuilder::new()
                .with_inner_size(glium::glutin::dpi::LogicalSize::new(
                    self.width,
                    self.height,
                ))
                .with_title(self.title.clone());

            let contextBuilder = glium::glutin::ContextBuilder::new();

            let display = glium::Display::new(windowBuilder, contextBuilder, events_loop).unwrap();

            self.display = Some(display);

            self
        }

        pub fn run<F>(mut self, events_loop: EventLoop<()>, mut runner: F) -> ()
        where
            F: FnMut(&mut Self, &Events, &mut Frame) -> () + 'static,
        {
            if self.display.is_none() {
                panic!("Window not built")
            }

            println!("Running window");

            let ms_between_frames: u64 = 1000 / self.frame_rate as u64;

            events_loop.run(move |event, _, control_flow| {
                let mut events = Events {
                    key_pressed: HashMap::new(),
                };

                control_flow.set_wait_until(
                    Instant::now() + std::time::Duration::from_millis(ms_between_frames),
                );

                match event {
                    Event::WindowEvent { window_id, event } => match event {
                        WindowEvent::KeyboardInput {
                            device_id,
                            input,
                            is_synthetic,
                        } => match input {
                            KeyboardInput {
                                virtual_keycode,
                                scancode: _,
                                state: _,
                                modifiers: _,
                            } => match virtual_keycode.unwrap() {
                                glium::glutin::event::VirtualKeyCode::Up => {
                                    events.set_key_pressed(KeyEnum::Up)
                                }
                                glium::glutin::event::VirtualKeyCode::Right => {
                                    events.set_key_pressed(KeyEnum::Right)
                                }
                                glium::glutin::event::VirtualKeyCode::Down => {
                                    events.set_key_pressed(KeyEnum::Down)
                                }
                                glium::glutin::event::VirtualKeyCode::Left => {
                                    events.set_key_pressed(KeyEnum::Left)
                                }
                                _ => (),
                            },
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => (),
                }

                let mut frame = self.display.as_ref().unwrap().draw();
                runner(&mut self, &events, &mut frame);
                frame.finish().unwrap();
            });
        }
    }
}
