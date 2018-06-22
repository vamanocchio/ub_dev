#[macro_use]
extern crate glium;
extern crate glium_graphics;
extern crate graphics;
extern crate piston;
extern crate tiled;

extern crate image;

use glium_graphics::{Flip, Glium2d, GliumWindow, OpenGL, Texture, TextureSettings};
use graphics::Transformed;
use std::io::Cursor;

fn main(){
    use glium::{glutin, Surface};

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let image = image::load(Cursor::new(&include_bytes!("../assets/safari_zone.png")[..]),
                            image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {

        position: [f32; 2],
        tex_coords: [f32; 2],
    }

    implement_vertex!(Vertex, position, tex_coords);

    // struct Tile {
    //     coords: [Vertex; 4],
    // }

    // let tile1 = vec![]

    let vertex1 = Vertex { position: [0.25, 0.25], tex_coords: [0.25, 0.25] };
    let vertex2 = Vertex { position: [-0.25, 0.25], tex_coords: [-0.25, 0.25] };
    let vertex3 = Vertex { position: [0.25, -0.25], tex_coords: [0.25, -0.25] };
    let vertex4 = Vertex { position: [-0.25, -0.25], tex_coords: [-0.25, -0.25] };
    //let shape1 = vec![vertex1, vertex2, vertex3, vertex4];

    let vertexA = Vertex { position: [-0.25, -0.25], tex_coords: [-0.25, -0.25] };
    let vertexB = Vertex { position: [0.25, -0.25], tex_coords: [0.25, -0.25] };
    let vertexC = Vertex { position: [-0.25, 0.25], tex_coords: [-0.25, 0.25] };

    let vertices = vec![vertex1, vertex2, vertex3, vertex4];
    let edges = vec![vertex1, vertex2, vertex3, vertex1, vertex3, vertex4];
    // let shape2 = vec![vertexA, vertexB, vertexC];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    //let vertex_buffer2 = glium::VertexBuffer::new(&display, &shape2).unwrap();

    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &edges).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main(){
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut t = 0.0;
    let mut closed = false;
    while !closed{
        // t += 0.0002;
        // if t > 0.5{
        //     t = -0.5;
        // }
    

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [t , 0.0, 0.0, 1.0f32],
        ],
        tex: &texture,
    };

    //target.draw((&vertex_buffer, &vertex_buffer2), &indices, &program, &uniforms, &Default::default()).unwrap();
    target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
    target.finish().unwrap();

    events_loop.poll_events(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => closed = true,
                _ => ()
            },
            _ => (),
        }
    });
  }
}
