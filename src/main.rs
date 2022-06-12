use std::fs::File;
use std::io::Read;
use std::time::Instant;

use glow::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

enum Shader {
    Vertex,
    Fragment,
}
impl Shader {
    pub fn new(path: &str) -> String {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        buf
    }
}

fn buffer<T>(vertices: &[T]) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * core::mem::size_of::<T>(),
        )
    }
}

fn main() {
    unsafe {
        let event_loop = glutin::event_loop::EventLoop::new();

        let window_builder = glutin::window::WindowBuilder::new()
            .with_title("OpenGL Window")
            .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0));

        let window = glutin::ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(window_builder, &event_loop)
            .unwrap()
            .make_current()
            .unwrap();

        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);

        let program = gl.create_program().expect("Cannot create program");
        let create_shader = |source, s_type| -> NativeShader {
            let shader = match s_type {
                Shader::Vertex => gl.create_shader(glow::VERTEX_SHADER),
                Shader::Fragment => gl.create_shader(glow::FRAGMENT_SHADER),
            }
            .unwrap();
            let error = gl.get_shader_info_log(shader);
            if !error.is_empty() {
                panic!("{}", error);
            }
            gl.shader_source(shader, source);
            gl.compile_shader(shader);
            gl.attach_shader(program, shader);
            shader
        };

        let source = Shader::new("src/vertex.glsl");
        let v = create_shader(&source, Shader::Vertex);

        let source = Shader::new("src/fragment.glsl");
        let f = create_shader(&source, Shader::Fragment);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        gl.use_program(Some(program));

        gl.delete_shader(v);
        gl.delete_shader(f);

        let triangle = buffer(&[
            -0.5f32, -0.5f32, 0.0f32, 0.5f32, -0.5f32, 0.0f32, 0.0f32, 0.5f32, 0.0f32,
        ]);

        //TODO: After I draw my triangle I want to draw just the outline
        let vbo = gl.create_buffer().unwrap();
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle, glow::STATIC_DRAW);

        let vao = gl.create_vertex_array().unwrap();
        gl.bind_vertex_array(Some(vao));
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        //Draw square
        {
            let square = buffer(&[
                -0.5f32, 0.5f32, 0.0f32, //top left
                0.5f32, 0.5f32, 0.0f32, //top right
                0.5f32, -0.5f32, 0.0f32, //bottom right
                -0.5f32, -0.5f32, 0.0f32, // bottom left
            ]);

            //top left, top right, bottom right
            //top left, bottom left, top right
            let indices = [
                0, 1, 2, //first triangle
                0, 3, 2, //second triangle
            ];

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(&square), glow::STATIC_DRAW);

            let ebo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                buffer(&indices),
                glow::STATIC_DRAW,
            );
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 0, 0);
            gl.enable_vertex_attrib_array(0);
            // gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
        }

        gl.clear_color(0.1, 0.2, 0.3, 1.0);
        // gl.polygon_mode(glow::FRONT_AND_BACK, glow::LINE);

        let time = Instant::now();
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let green = time.elapsed().as_secs_f32().sin() / 2.0 + 0.5;
                    let location = gl.get_uniform_location(program, "our_color");
                    gl.uniform_4_f32(location.as_ref(), 0.0, green, 0.0, 1.0);

                    gl.clear(glow::COLOR_BUFFER_BIT);
                    gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
                    window.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        window.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => {
                        gl.delete_program(program);
                        // gl.delete_vertex_array(vertex_array);
                        *control_flow = ControlFlow::Exit
                    }
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
