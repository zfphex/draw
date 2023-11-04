#![feature(const_maybe_uninit_zeroed)]
use glfw::{Action, Key, Monitor, WindowEvent};
use glm::{Vec2, Vec4};
use glow::*;
use std::mem::{size_of, MaybeUninit};

extern crate nalgebra_glm as glm;

pub mod font;
pub use font::*;

#[track_caller]
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

pub fn create_basic_shader(gl: &Context) -> NativeProgram {
    unsafe {
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

        create_program(
            &gl,
            include_str!("../shaders/simple.vert"),
            include_str!("../shaders/text.frag"),
        )
    }
}

static mut GL: MaybeUninit<Context> = MaybeUninit::uninit();

//I think rust packed my struct in a weird way.
//So align won't work unless you use `repr(C)`.
#[repr(C)]
pub struct Vertex {
    pub position: Vec2,
    pub color: Vec4,
    pub uv: Vec2,
}

pub struct Renderer {
    pub gl: &'static glow::Context,
    pub vertices: Vec<Vertex>,
    pub vao: NativeVertexArray,
    pub vbo: NativeBuffer,
}

impl Renderer {
    pub fn new(gl: &'static glow::Context) -> Self {
        unsafe {
            // gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::DEBUG_OUTPUT);
            gl.enable(glow::DEBUG_OUTPUT_SYNCHRONOUS);
            gl.debug_message_callback(|source, ty, id, severity, msg| {
                if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
                    return;
                }

                println!("---------------");
                println!("Debug message ({}): {}", id, msg);

                match source {
                    glow::DEBUG_SOURCE_API => println!("Source: API"),
                    glow::DEBUG_SOURCE_WINDOW_SYSTEM => println!("Source: Window System"),
                    glow::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader Compiler"),
                    glow::DEBUG_SOURCE_THIRD_PARTY => println!("Source: Third Party"),
                    glow::DEBUG_SOURCE_APPLICATION => println!("Source: Application"),
                    glow::DEBUG_SOURCE_OTHER => println!("Source: Other"),
                    _ => println!("Source: Unknown"),
                }

                match ty {
                    glow::DEBUG_TYPE_ERROR => println!("Type: Error"),
                    glow::DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Type: Deprecated Behaviour"),
                    glow::DEBUG_TYPE_UNDEFINED_BEHAVIOR => println!("Type: Undefined Behaviour"),
                    glow::DEBUG_TYPE_PORTABILITY => println!("Type: Portability"),
                    glow::DEBUG_TYPE_PERFORMANCE => println!("Type: Performance"),
                    glow::DEBUG_TYPE_MARKER => println!("Type: Marker"),
                    glow::DEBUG_TYPE_PUSH_GROUP => println!("Type: Push Group"),
                    glow::DEBUG_TYPE_POP_GROUP => println!("Type: Pop Group"),
                    glow::DEBUG_TYPE_OTHER => println!("Type: Other"),
                    _ => println!("Type: Unknown"),
                }

                match severity {
                    glow::DEBUG_SEVERITY_HIGH => println!("Severity: High"),
                    glow::DEBUG_SEVERITY_MEDIUM => println!("Severity: Medium"),
                    glow::DEBUG_SEVERITY_LOW => println!("Severity: Low"),
                    glow::DEBUG_SEVERITY_NOTIFICATION => println!("Severity: Notification"),
                    _ => println!("Severity: Unknown"),
                }

                println!();
            });

            gl.clear_color(0.2, 0.2, 0.2, 1.0);

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            // gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &[], glow::DYNAMIC_DRAW);

            Self {
                gl,
                vao,
                vbo,
                vertices: Vec::new(),
            }
        }
    }

    pub fn vertex(&mut self, position: Vec2, color: Vec4, uv: Vec2) {
        self.vertices.push(Vertex {
            position,
            color,
            uv,
        });
    }

    pub fn triangle(
        &mut self,
        p0: Vec2,
        p1: Vec2,
        p2: Vec2,
        c0: Vec4,
        c1: Vec4,
        c2: Vec4,
        uv0: Vec2,
        uv1: Vec2,
        uv2: Vec2,
    ) {
        self.vertex(p0, c0, uv0);
        self.vertex(p1, c1, uv1);
        self.vertex(p2, c2, uv2);
    }

    pub fn quad(
        &mut self,
        p0: Vec2,
        p1: Vec2,
        p2: Vec2,
        p3: Vec2,
        c0: Vec4,
        c1: Vec4,
        c2: Vec4,
        c3: Vec4,
        uv0: Vec2,
        uv1: Vec2,
        uv2: Vec2,
        uv3: Vec2,
    ) {
        self.triangle(p0, p1, p2, c0, c1, c2, uv0, uv1, uv2);
        self.triangle(p1, p2, p3, c1, c2, c3, uv1, uv2, uv3);
    }

    pub fn texture(&mut self, pos: Vec2, size: Vec2, uvp: Vec2, uvs: Vec2, color: Vec4) {
        self.quad(
            pos,
            pos + Vec2::new(size.x, 0.0),
            pos + Vec2::new(0.0, size.y),
            pos + size,
            color,
            color,
            color,
            color,
            uvp,
            uvp + Vec2::new(uvs.x, 0.0),
            uvp + Vec2::new(0.0, uvs.y),
            uvp + uvs,
        );
    }

    pub fn use_shader(&self, program: NativeProgram) {
        unsafe { self.gl.use_program(Some(program)) };
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

    #[allow(unused)]
    unsafe {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
        let monitor = Monitor::from_primary();
        let video_mode = monitor.get_video_mode().unwrap();
        let (width, height) = (
            video_mode.width as f32 / 1.5,
            video_mode.height as f32 / 1.5,
        );
        //TODO: Handle window re-sizing.
        let (mut window, events) = glfw
            .create_window(
                width as u32,
                height as u32,
                "Triangle",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");
        window.make_current();

        assert!(window.is_opengl_debug_context());

        GL = MaybeUninit::new(glow::Context::from_loader_function(|s| {
            window.get_proc_address(s) as *const _
        }));
        let gl = GL.assume_init_ref();

        let mut rd = Renderer::new(gl);

        let basic = create_basic_shader(gl);
        rd.use_shader(basic);

        // let font = create_program(
        //     &gl,
        //     include_str!("../shaders/simple.vert"),
        //     include_str!("../shaders/text.frag"),
        // );
        // rd.use_shader(font);

        let atlas = load_font(&rd, include_bytes!("../JetBrainsMono.ttf"));

        let text_position = Vec2::new(-2.0, 0.0);
        //FIXME: Here the font is really big because it expect that I'm drawing to a viewport.
        draw_line(
            &atlas,
            &mut rd,
            "hi there.",
            text_position,
            Vec4::new(0.3, 0.7, 0.6, 1.0),
        );

        // let color = Vec4::new(0.8, 0.8, 0.8, 1.0);
        // rd.texture(
        //     Vec2::new(0.0, 0.0),
        //     Vec2::new(0.5, 0.5),
        //     Vec2::new(0.0, 0.0),
        //     Vec2::new(1.0, 1.0),
        //     color,
        // );

        // let p0 = Vec2::new(0.5, 0.5);
        // let p1 = Vec2::new(0.5, -0.5);
        // let p2 = Vec2::new(-0.5, 0.5);
        // let p3 = Vec2::new(-0.5, -0.5);
        // let color = Vec4::new(1.0, 0.5, 1.0, 1.0);
        // let uv0 = Vec2::new(0.0, 0.0);
        // let uv1 = Vec2::new(1.0, 0.0);
        // let uv2 = Vec2::new(1.0, 1.0);
        // let uv3 = Vec2::new(0.0, 1.0);
        // rd.quad(
        //     p0, p1, p2, p3, color, color, color, color, uv0, uv1, uv2, uv3,
        // );

        // let color = Vec4::new(0.5, 0.5, 0.5, 1.0);
        // let p0 = Vec2::new(0.5, 0.5);
        // let p1 = Vec2::new(-0.5, 0.5);
        // let p2 = Vec2::new(-0.5, -0.5);
        // rd.triangle(p0, p1, p2, color, color, color, uv, uv, uv);

        dbg!(rd.vertices.len());

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
