use std::fs::File;
use std::io::Read;
use std::path::Path;

use glow::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

pub fn open(path: impl AsRef<Path>) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

pub unsafe fn program(
    gl: &Context,
    vertex_path: impl AsRef<Path>,
    fragment_path: impl AsRef<Path>,
) -> NativeProgram {
    let v_source = open(vertex_path);
    let f_source = open(fragment_path);
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

    program
}

fn buffer<T>(vertices: &[T]) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * core::mem::size_of::<T>(),
        )
    }
}

unsafe fn texture(gl: &Context, path: impl AsRef<Path>, png: bool, flip_v: bool) {
    let im = image::open(path).unwrap();
    let im = if flip_v { im.flipv() } else { im };

    let texture = gl.create_texture().unwrap();
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));

    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
    gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::LINEAR as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::LINEAR as i32,
    );

    if png {
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGBA as i32,
            im.width() as i32,
            im.height() as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(im.as_bytes()),
        );
    } else {
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGB as i32,
            im.width() as i32,
            im.height() as i32,
            0,
            glow::RGB,
            glow::UNSIGNED_BYTE,
            Some(im.as_bytes()),
        );
    }

    gl.generate_mipmap(glow::TEXTURE_2D);

    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
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

        let program = program(&gl, "src/vertex.glsl", "src/fragment.glsl");

        {
            let vertices: &[f32] = &[
                // positions          // colors           // texture coords
                0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
                0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
                -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom let
                -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top let
            ];
            let indices = [
                0, 1, 3, // first triangle
                1, 2, 3, // second triangle
            ];
            let vao = gl.create_vertex_array().unwrap();
            let vbo = gl.create_buffer().unwrap();
            let ebo = gl.create_buffer().unwrap();

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(vertices), glow::STATIC_DRAW);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                buffer(&indices),
                glow::STATIC_DRAW,
            );

            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 8 * 4, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 8 * 4, 3 * 4);
            gl.enable_vertex_attrib_array(1);

            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 8 * 4, 6 * 4);
            gl.enable_vertex_attrib_array(2);

            gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);

            let ul = gl.get_uniform_location(program, "texture1").unwrap();
            gl.uniform_1_i32(Some(&ul), 0);

            texture(&gl, "resources/textures/container.jpg", false, false);
            texture(&gl, "resources/textures/awesomeface.png", true, true);
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
