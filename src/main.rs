use std::fs::File;
use std::io::Read;
use std::path::Path;

use glow::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

struct Program {
    native_program: NativeProgram,
}

impl Program {
    pub fn new(
        gl: &Context,
        vertex_path: impl AsRef<Path>,
        fragment_path: impl AsRef<Path>,
    ) -> Self {
        unsafe {
            let v_source = Self::open(vertex_path);
            let f_source = Self::open(fragment_path);
            let program = gl.create_program().unwrap();

            //Vertex shader
            let v = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            let error = gl.get_shader_info_log(v);
            if !error.is_empty() {
                panic!("{}", error);
            }
            gl.shader_source(v, &v_source);
            gl.compile_shader(v);
            gl.attach_shader(program, v);

            //Fragment shader
            let f = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            let error = gl.get_shader_info_log(f);
            if !error.is_empty() {
                panic!("{}", error);
            }
            gl.shader_source(f, &f_source);
            gl.compile_shader(f);
            gl.attach_shader(program, f);

            //Link program
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            gl.use_program(Some(program));

            //Cleanup
            gl.delete_shader(v);
            gl.delete_shader(f);

            Self {
                native_program: program,
            }
        }
    }
    pub fn open(path: impl AsRef<Path>) -> String {
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

        let program = Program::new(&gl, "src/vertex.glsl", "src/fragment.glsl");

        {
            let square: &[f32] = &[
                -0.5, 0.5, 0.0, //Top Left
                1.0, 0.0, 0.0, //Color
                0.5, 0.5, 0.0, //Top Right
                0.0, 1.0, 0.0, //Color
                0.0, -0.5, 0.0, //Position
                0.0, 0.0, 1.0, //Color
            ];

            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(square), glow::STATIC_DRAW);

            //Position
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 6 * 4, 0);
            gl.enable_vertex_attrib_array(0);

            //Color
            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 6 * 4, 3 * 4);
            gl.enable_vertex_attrib_array(1);
        }

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

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
                    //Clear must come first
                    gl.clear(glow::COLOR_BUFFER_BIT);
                    gl.draw_arrays(glow::TRIANGLES, 0, 3);

                    window.swap_buffers().unwrap();
                }
                Event::WindowEvent { ref event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        window.resize(*physical_size);
                    }
                    WindowEvent::CloseRequested => {
                        gl.delete_program(program.native_program);
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
