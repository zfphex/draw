#![allow(unused)]
use std::io::Read;
use std::path::Path;
use std::{f32::consts::PI, fs::File};

use glfw::{Action, Key, Monitor, WindowEvent};
use glow::*;

pub use shaders::*;
pub mod dx11;
pub mod shaders;

extern crate nalgebra_glm as glm;

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

// &vertices.align_to::<u8>().1
#[inline]
fn buffer(vertices: &[f32]) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * core::mem::size_of::<f32>(),
        )
    }
}

#[inline]
fn radians(input: f32) -> f32 {
    input * PI / 180.0
}

pub fn check_error(gl: &Context) {
    let error = unsafe { gl.get_error() };
    match error {
        glow::INVALID_ENUM => panic!("INVALID_ENUM"),
        glow::INVALID_VALUE => panic!("INVALID_VALUE"),
        glow::INVALID_OPERATION => panic!("INVALID_OPERATION"),
        glow::STACK_OVERFLOW => panic!("STACK_OVERFLOW"),
        glow::STACK_UNDERFLOW => panic!("STACK_UNDERFLOW"),
        glow::OUT_OF_MEMORY => panic!("OUT_OF_MEMORY"),
        glow::INVALID_FRAMEBUFFER_OPERATION => panic!("INVALID_FRAMEBUFFER_OPERATION"),
        0 => {}
        _ => unreachable!(),
    }
}

fn main() {
    return dx11::dx11();

    unsafe {
        use glfw::Context;

        //Window
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let monitor = Monitor::from_primary();
        let video_mode = monitor.get_video_mode().unwrap();
        let (width, height) = (
            video_mode.width as f32 / 1.5,
            video_mode.height as f32 / 1.5,
        );
        let (mut window, events) = glfw
            .create_window(
                width as u32,
                height as u32,
                "Triangle",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_cursor_mode(glfw::CursorMode::Disabled);
        window.make_current();

        //OpenGL
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        gl.enable(glow::DEPTH_TEST);

        let program = program(&gl, "shaders/vertex.glsl", "shaders/fragment.glsl");

        #[rustfmt::skip]
        let vertices: &[f32] = &[
            -0.5, -0.5, -0.5,  0.0, 0.0,
            0.5, -0.5, -0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5,  0.5,  0.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
            0.5, -0.5, -0.5,  1.0, 1.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
            0.5,  0.5, -0.5,  1.0, 1.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
        ];

        #[rustfmt::skip]
        let cube_positions = [
            glm::vec3( 0.0,  0.0,  0.0),
            glm::vec3( 2.0,  5.0, -15.0),
            glm::vec3(-1.5, -2.2, -2.5),
            glm::vec3(-3.8, -2.0, -12.3),
            glm::vec3( 2.4, -0.4, -3.5),
            glm::vec3(-1.7,  3.0, -7.5),
            glm::vec3( 1.3, -2.0, -2.5),
            glm::vec3( 1.5,  2.0, -2.5),
            glm::vec3( 1.5,  0.2, -1.5),
            glm::vec3(-1.3,  1.0, -1.5)
        ];

        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();

        gl.bind_vertex_array(Some(vao));

        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(vertices), glow::STATIC_DRAW);

        // position attribute
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 5 * 4, 0);
        gl.enable_vertex_attrib_array(0);

        // texture coord attribute
        gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 5 * 4, 3 * 4);
        gl.enable_vertex_attrib_array(1);

        //First texture
        let im = image::open("resources/textures/container.jpg").unwrap();
        let texture1 = gl.create_texture().unwrap();

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

        gl.generate_mipmap(glow::TEXTURE_2D);
        gl.active_texture(glow::TEXTURE1);
        gl.bind_texture(glow::TEXTURE_2D, Some(texture1));

        //Second texture
        let im = image::open("resources/textures/awesomeface.png")
            .unwrap()
            .flipv();

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

        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RGB as i32,
            im.width() as i32,
            im.height() as i32,
            0,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            Some(im.as_bytes()),
        );

        gl.generate_mipmap(glow::TEXTURE_2D);
        gl.active_texture(glow::TEXTURE0);

        let ul1 = gl.get_uniform_location(program, "texture1").unwrap();
        gl.uniform_1_i32(Some(&ul1), 0);

        let ul2 = gl.get_uniform_location(program, "texture2").unwrap();
        gl.uniform_1_i32(Some(&ul2), 1);

        let model_location = gl.get_uniform_location(program, "model").unwrap();
        let view_location = gl.get_uniform_location(program, "view").unwrap();

        let projection_location = gl.get_uniform_location(program, "projection").unwrap();
        let projection = glm::perspective(width as f32 / height as f32, 45.0, 0.1, 100.0);
        gl.uniform_matrix_4_f32_slice(Some(&projection_location), false, projection.as_slice());

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        let mut camera_pos = glm::vec3(0.0, 0.0, 3.0);
        let mut camera_front = glm::vec3(0.0, 0.0, -1.0);
        let camera_up = glm::vec3(0.0, 1.0, 0.0);

        let mut delta_time: f32;
        let mut last_frame: f32 = 0.0;

        //Mouse
        let mut first_mouse = true;
        // yaw is initialized to -90.0 degrees since a yaw of 0.0 results in a direction vector pointing to the right so we initially rotate a bit to the left.
        let mut yaw: f32 = -90.0;
        let mut pitch: f32 = 0.0;
        let mut last_x: f32 = 800.0 / 2.0;
        let mut last_y: f32 = 600.0 / 2.0;
        let sensitivity: f32 = 0.1;

        // let mut tb = TriangleBuffer::new(&gl);
        // let (r, g, b) = (1.0, 1.0, 1.0);

        // for x in 1..10 {
        //     tb.extend(&[
        //         0.5 + 0.1 * x as f32,
        //         -0.5 + 0.1 * x as f32,
        //         0.0,
        //         r,
        //         g,
        //         b,
        //         -0.5,
        //         -0.5,
        //         0.0,
        //         r,
        //         g,
        //         b,
        //         0.0,
        //         0.5,
        //         0.0,
        //         r,
        //         g,
        //         b,
        //     ]);
        // }

        // tb.upload(&gl);

        while !window.should_close() {
            let current_frame = glfw.get_time() as f32;
            delta_time = current_frame - last_frame;
            last_frame = current_frame;

            let camera_speed = 5.0 * delta_time;

            //Camera Input
            if window.get_key(Key::W) == Action::Press {
                camera_pos += camera_speed * camera_front;
            }
            if window.get_key(Key::S) == Action::Press {
                camera_pos -= camera_speed * camera_front;
            }
            if window.get_key(Key::A) == Action::Press {
                camera_pos -= glm::normalize(&glm::cross(&camera_front, &camera_up)) * camera_speed;
            }
            if window.get_key(Key::D) == Action::Press {
                camera_pos += glm::normalize(&glm::cross(&camera_front, &camera_up)) * camera_speed;
            }

            //Events
            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    }
                    WindowEvent::Close => window.set_should_close(true),
                    WindowEvent::CursorPos(x, y) => {
                        let (x, y) = (x as f32, y as f32);
                        if first_mouse {
                            last_x = x;
                            last_y = y;
                            first_mouse = false;
                        }
                        let mut xoffset = x - last_x;
                        let mut yoffset = last_y - y;
                        // reversed since y-coordinates go from bottom to top
                        last_x = x;
                        last_y = y;

                        xoffset *= sensitivity;
                        yoffset *= sensitivity;

                        yaw += xoffset;
                        pitch += yoffset;

                        if pitch > 89.9 {
                            pitch = 89.9;
                        }
                        if pitch < -89.9 {
                            pitch = -89.9;
                        }

                        let mut front = glm::vec3(0.0, 0.0, 0.0);
                        // front.x = f32::cos(yaw) * f32::cos(pitch);
                        // front.y = f32::sin(pitch);
                        // front.z = f32::sin(yaw) * f32::cos(pitch);

                        front.x = f32::cos(radians(yaw)) * f32::cos(radians(pitch));
                        front.y = f32::sin(radians(pitch));
                        front.z = f32::sin(radians(yaw)) * f32::cos(radians(pitch));

                        camera_front = glm::normalize(&front);
                    }
                    _ => {}
                }
            }

            //Rendering
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            // tb.draw(&gl);

            // draw_rectangle(&gl, -0.5, 0.0, 0.7, 0.7, color(1.0, 0.5, 0.5));

            // draw_rectangle(&gl, 0.0, 0.0, 0.1, 0.1, color(0.2, 0.5, 0.5));

            // draw_rectangle_ortho(&gl, width, height);

            draw_line(&gl, -0.3, 0.0, 0.3, 0.3, color(0.1, 0.1, 0.1));

            'cubes: {
                break 'cubes;

                //Camera/View transformation
                let view = glm::look_at(&camera_pos, &(camera_pos + camera_front), &camera_up);
                gl.uniform_matrix_4_f32_slice(Some(&view_location), false, view.as_slice());

                for (i, cube) in cube_positions.iter().enumerate() {
                    // calculate the model matrix for each object and pass it to shader before drawing
                    let mut model = glm::translate(&glm::identity(), &cube);
                    model = glm::rotate(&model, 20.0 * i as f32, &glm::vec3(1.0, 0.3, 0.5));
                    model = glm::rotate(
                        &model,
                        (i as f32 + 1.0) * glfw.get_time() as f32 / 4.0,
                        &glm::vec3(0.5, 1.0, 0.0),
                    );
                    gl.uniform_matrix_4_f32_slice(Some(&model_location), false, model.as_slice());

                    gl.draw_arrays(glow::TRIANGLES, 0, 36);
                }
            }

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
