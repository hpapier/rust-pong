extern crate glium;

pub mod button {
    use glium::{implement_vertex, Display, Frame, Surface};

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    pub struct Button {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
        translation_x: f32,
        translation_y: f32,
        vertex_buffer: glium::VertexBuffer<Vertex>,
        program: glium::Program,
    }

    impl Button {
        pub fn new(display: &Display, x: f32, y: f32, width: f32, height: f32) -> Button {
            if width < 0.0 {
                panic!("Width must be positive");
            }

            if height < 0.0 {
                panic!("Height must be positive");
            }

            let program = glium::Program::from_source(
                display,
                r#"
              #version 140

              in vec2 position;
              uniform float x;
              uniform float y;

              void main() {
                vec2 pos = position;
                pos.x += x;
                pos.y += y;
                gl_Position = vec4(pos, 0.0, 1.0);
              }
            "#,
                r#"
              #version 140

              out vec4 color;

              void main() {
                color = vec4(1.0, 1.0, 1.0, 1.0);
              }
            "#,
                None,
            )
            .unwrap();

            // TODO: Find a way to get dynamic window size
            let end_x = x + 2.0 / (1280.0 / width);
            let end_y = y - 2.0 / (720.0 / height);

            println!(
                "Button: x: {}, y: {}, end_x: {}, end_y: {}",
                x, y, end_x, end_y
            );

            Button {
                x: x,
                y: y,
                width: width,
                height: height,
                translation_x: 0.0,
                translation_y: 0.0,
                vertex_buffer: glium::VertexBuffer::new(
                    display,
                    &vec![
                        Vertex { position: [x, y] },
                        Vertex {
                            position: [end_x, y],
                        },
                        Vertex {
                            position: [end_x, end_y],
                        },
                        Vertex {
                            position: [x, end_y],
                        },
                        Vertex { position: [x, y] },
                    ],
                )
                .unwrap(),
                program: program,
            }
        }

        pub fn move_x(&mut self, number: f32) -> &mut Button {
            self.translation_x += number;
            println!("[button:move_x] x: {}", self.x + self.translation_x);
            self
        }

        pub fn move_y(&mut self, number: f32) -> &mut Button {
            self.translation_y += number;
            println!("[button:move_y] y: {}", self.y + self.translation_y);
            self
        }

        pub fn get_y_position(&self) -> f32 {
            self.y + self.translation_y
        }

        pub fn draw(&mut self, frame: &mut Frame) -> () {
            // println!("Drawing button");
            let frame_result = frame.draw(
                &self.vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &self.program,
                &glium::uniform! {
                    x: self.translation_x,
                    y: self.translation_y,
                },
                &Default::default(),
            );

            match frame_result {
                Ok(_) => (),
                Err(e) => println!("[button:draw] Drawing error: {}", e),
            }
        }
    }
}
