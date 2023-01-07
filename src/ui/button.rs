extern crate glium;

pub mod button {
    use glium::{implement_vertex, uniform, Display, DrawParameters, Frame, Surface};

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
    }

    implement_vertex!(Vertex, position);

    pub struct Button {
        pub x: f32,
        pub y: f32,
        pub width: f32,
        pub height: f32,
        VertexBuffer: glium::VertexBuffer<Vertex>,
        program: glium::Program,
    }

    impl Button {
        pub fn new(display: &Display) -> Button {
            let program = glium::Program::from_source(
                display,
                r#"
              #version 140

              in vec3 position;
              uniform float x;
              uniform float y;

              void main() {
                vec3 pos = position;
                pos.x += x;
                pos.y += y;
                gl_Position = vec4(pos, 1.0);
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

            Button {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                VertexBuffer: glium::VertexBuffer::new(
                    display,
                    &[
                        Vertex {
                            position: [0.0, 0.0, 0.0],
                        },
                        Vertex {
                            position: [0.1, 0.0, 0.0],
                        },
                        Vertex {
                            position: [0.1, -0.4, 0.0],
                        },
                        Vertex {
                            position: [0.0, -0.4, 0.0],
                        },
                        Vertex {
                            position: [0.0, 0.0, 0.0],
                        },
                    ],
                )
                .unwrap(),
                program: program,
            }
        }

        pub fn move_x(&mut self, number: f32) -> &mut Button {
            self.x += number;
            self
        }

        pub fn move_y(&mut self, number: f32) -> &mut Button {
            self.y += number;
            self
        }

        pub fn draw(&mut self, frame: &mut Frame) -> () {
            println!("Drawing button");
            frame.draw(
                &self.VertexBuffer,
                glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
                &self.program,
                &uniform! { x: self.x, y: self.y },
                &Default::default(),
            );
        }
    }
}
