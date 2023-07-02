use std::io::Read;
use std::path::Path;
use std::{fs::File, time::Instant};

use glow::*;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::ControlFlow;

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
        gl.enable(glow::DEPTH_TEST);
        let now = Instant::now();

        let program = program(&gl, "src/vertex.glsl", "src/fragment.glsl");

        // let vertices: &[f32] = &[
        //     // positions          // colors           // texture coords
        //     0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // top right
        //     0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, // bottom right
        //     -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom let
        //     -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, // top let
        // ];

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

        // gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 8 * 4, 6 * 4);
        // gl.enable_vertex_attrib_array(2);

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

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        let camera_pos = glm::vec3(0.0, 0.0, 3.0);
        let camera_target = glm::vec3(0.0, 0.0, 0.0);
        let camera_direction = glm::normalize(&(camera_pos - camera_target));

        let up = glm::vec3(0.0, 1.0, 0.0);
        let cameraRight = glm::normalize(&glm::cross(&glm::vec3(0.0, 1.0, 0.0), &camera_direction));

        let view = glm::look_at(
            &glm::vec3(0.0, 0.0, 3.0),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0),
        );
        let radius = 10.0;
        let camX = f32::sin(now.elapsed().as_secs_f32()) * radius;
        let camZ = f32::cos(now.elapsed().as_secs_f32()) * radius;
        let view = glm::look_at(
            &glm::vec3(camX, 0.0, camZ),
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 1.0, 0.0),
        );

        let mut frame_count = 0;
        let mut start_time = std::time::Instant::now();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            let size = window.window().inner_size();
            let (width, height) = (size.width as f32, size.height as f32);

            // Calculate the elapsed time since the start
            let elapsed = start_time.elapsed().as_secs_f32();

            // Calculate and display the framerate every second
            if elapsed >= 1.0 {
                let _framerate = frame_count as f32 / elapsed;
                // println!("Framerate: {:.2}", _framerate);

                // Reset the frame count and start time
                frame_count = 0;
                start_time = std::time::Instant::now();
            }

            match event {
                Event::LoopDestroyed => {
                    return;
                }
                Event::MainEventsCleared => {
                    window.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    frame_count += 1;
                    //Clear must come first
                    gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

                    // note that we're translating the scene in the reverse direction of where we want to move
                    let view = glm::translate(&glm::identity(), &glm::vec3(0.0, 0.0, -3.0));
                    let projection = glm::perspective(1024.0 / 768.0, 45.0, 0.1, 100.0);
                    gl.uniform_matrix_4_f32_slice(Some(&view_location), false, view.as_slice());
                    gl.uniform_matrix_4_f32_slice(
                        Some(&projection_location),
                        false,
                        projection.as_slice(),
                    );

                    // camera/view transformation
                    let radius = 8.0;
                    let cam_x = f32::sin(now.elapsed().as_secs_f32()) * radius;
                    let cam_z = f32::cos(now.elapsed().as_secs_f32()) * radius;
                    // let cam_y = (1.0 / now.elapsed().as_secs_f32()) * radius;
                    let cam_y = 0.0;
                    let view = glm::look_at(
                        &glm::vec3(cam_x, cam_y, cam_z),
                        &glm::vec3(0.0, 0.0, 0.0),
                        &glm::vec3(0.0, 1.0, 0.0),
                    );
                    gl.uniform_matrix_4_f32_slice(Some(&view_location), false, view.as_slice());

                    for (i, cube) in cube_positions.iter().enumerate() {
                        // calculate the model matrix for each object and pass it to shader before drawing
                        let mut model = glm::translate(&glm::identity(), &cube);
                        model = glm::rotate(&model, 20.0 * i as f32, &glm::vec3(1.0, 0.3, 0.5));
                        model = glm::rotate(
                            &model,
                            (i as f32 + 1.0) * now.elapsed().as_secs_f32() / 4.0,
                            &glm::vec3(0.5, 1.0, 0.0),
                        );
                        gl.uniform_matrix_4_f32_slice(
                            Some(&model_location),
                            false,
                            model.as_slice(),
                        );

                        gl.draw_arrays(glow::TRIANGLES, 0, 36);
                    }

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
