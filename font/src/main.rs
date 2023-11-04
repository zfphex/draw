use glfw::{Action, Key, Monitor, WindowEvent};
use glow::*;
use std::mem::size_of;

extern crate nalgebra_glm as glm;

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

pub fn create_program(gl: &Context, vertex: &str, fragment: &str) -> NativeProgram {
    unsafe {
        let program = gl.create_program().unwrap();

        let v = gl.create_shader(glow::VERTEX_SHADER).unwrap();
        let error = gl.get_shader_info_log(v);
        if !error.is_empty() {
            panic!("{}", error);
        }
        gl.shader_source(v, vertex);
        gl.compile_shader(v);
        gl.attach_shader(program, v);

        let f = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
        let error = gl.get_shader_info_log(f);
        if !error.is_empty() {
            panic!("{}", error);
        }
        gl.shader_source(f, fragment);
        gl.compile_shader(f);
        gl.attach_shader(program, f);

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        //Cleanup
        gl.delete_shader(v);
        gl.delete_shader(f);

        program
    }
}

//I think rust packed my struct in a weird way.
//So align won't work unless you use `repr(C)`.
#[repr(C)]
pub struct Vertex {
    pub position: glm::Vec2,
    pub color: glm::Vec4,
    pub uv: glm::Vec2,
}

const SIMPLE: [&str; 2] = [
    include_str!("../shaders/simple.vert"),
    include_str!("../shaders/simple.frag"),
];

pub struct Renderer {
    pub gl: glow::Context,
    pub vertices: Vec<Vertex>,
    pub vao: NativeVertexArray,
    pub vbo: NativeBuffer,
    pub shader: NativeProgram,
}

impl Renderer {
    pub fn new(gl: glow::Context) -> Self {
        unsafe {
            gl.enable(glow::DEPTH_TEST);
            gl.clear_color(0.2, 0.2, 0.2, 1.0);

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            // gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &[], glow::DYNAMIC_DRAW);

            //Make sure the stride is correct.
            let stride = size_of::<Vertex>() as i32;
            assert_eq!(stride, size_of::<f32>() as i32 * 8);

            //Position Vec2
            let mut offset = 0;
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, offset);
            offset += (size_of::<f32>() * 2) as i32;

            //Color Vec4
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 4, glow::FLOAT, false, stride, offset);
            offset += (size_of::<f32>() * 4) as i32;

            //UV Vec2
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, stride, offset);

            let shader = create_program(&gl, SIMPLE[0], SIMPLE[1]);
            gl.use_program(Some(shader));

            Self {
                gl,
                vao,
                vbo,
                vertices: Vec::new(),
                shader,
            }
        }
    }

    pub fn vertex(&mut self, position: glm::Vec2, color: glm::Vec4, uv: glm::Vec2) {
        self.vertices.push(Vertex {
            position,
            color,
            uv,
        });
    }

    pub fn triangle(
        &mut self,
        p0: glm::Vec2,
        p1: glm::Vec2,
        p2: glm::Vec2,
        c0: glm::Vec4,
        c1: glm::Vec4,
        c2: glm::Vec4,
        uv0: glm::Vec2,
        uv1: glm::Vec2,
        uv2: glm::Vec2,
    ) {
        self.vertex(p0, c0, uv0);
        self.vertex(p1, c1, uv1);
        self.vertex(p2, c2, uv2);
    }

    pub fn quad(
        &mut self,
        p0: glm::Vec2,
        p1: glm::Vec2,
        p2: glm::Vec2,
        p3: glm::Vec2,
        c0: glm::Vec4,
        c1: glm::Vec4,
        c2: glm::Vec4,
        c3: glm::Vec4,
        uv0: glm::Vec2,
        uv1: glm::Vec2,
        uv2: glm::Vec2,
        uv3: glm::Vec2,
    ) {
        self.triangle(p0, p1, p2, c0, c1, c2, uv0, uv1, uv2);
        self.triangle(p1, p2, p3, c1, c2, c3, uv1, uv2, uv3);
    }

    pub fn clear(&self) {
        unsafe {
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw(&self) {
        unsafe {
            //When replacing the entire data store, consider using glBufferSubData rather than completely recreating the data store with glBufferData. This avoids the cost of reallocating the data store.

            // self.gl.buffer_sub_data_u8_slice(
            //     glow::ARRAY_BUFFER,
            //     (self.vertices.len() * std::mem::size_of::<Vertex>()) as i32,
            //     self.vertices.align_to::<u8>().1,
            // );

            // self.gl.use_program(Some(self.shader));
            // self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));

            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                self.vertices.align_to::<u8>().1,
                glow::DYNAMIC_DRAW,
            );

            self.gl
                .draw_arrays(glow::TRIANGLES, 0, self.vertices.len() as i32);
        }
    }
}

fn main() {
    use glfw::Context;
    unsafe {
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

        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        let mut rd = Renderer::new(gl);

        let uv = glm::Vec2::new(1.0, 1.0);
        let p0 = glm::Vec2::new(0.5, 0.5); //bottom right
        let p1 = glm::Vec2::new(0.5, -0.5); //bottom left
        let p2 = glm::Vec2::new(-0.5, 0.5); //top right
        let p3 = glm::Vec2::new(-0.5, -0.5); //top top

        let color = glm::Vec4::new(1.0, 0.5, 0.0, 1.0);
        rd.quad(p0, p1, p2, p3, color, color, color, color, uv, uv, uv, uv);

        // let color = glm::Vec4::new(0.5, 0.5, 0.5, 1.0);
        // let p0 = glm::Vec2::new(0.5, 0.5);
        // let p1 = glm::Vec2::new(-0.5, 0.5);
        // let p2 = glm::Vec2::new(-0.5, -0.5);
        // rd.triangle(p0, p1, p2, color, color, color, uv, uv, uv);

        while !window.should_close() {
            let _current_frame = glfw.get_time() as f32;

            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        window.set_should_close(true)
                    }
                    WindowEvent::Close => window.set_should_close(true),
                    _ => {}
                }
            }

            rd.clear();
            rd.draw();
            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
