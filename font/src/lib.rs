#![feature(const_maybe_uninit_zeroed)]
use glow::*;
use std::mem::MaybeUninit;

extern crate nalgebra_glm as glm;

pub mod glyph;
pub mod math;

pub use glyph::*;
pub use math::*;

#[cfg(test)]
mod tests;

pub static mut GL: MaybeUninit<Context> = MaybeUninit::uninit();

pub const TOP_LEFT: Vec2 = Vec2::new(-0.5, 0.5);
pub const BOTTOM_LEFT: Vec2 = Vec2::new(-0.5, -0.5);
pub const TOP_RIGHT: Vec2 = Vec2::new(0.5, 0.5);
pub const BOTTOM_RIGHT: Vec2 = Vec2::new(0.5, -0.5);
pub const UV_TOP_LEFT: Vec2 = Vec2::new(0.0, 1.0);
pub const UV_BOTTOM_LEFT: Vec2 = Vec2::new(0.0, 0.0);
pub const UV_TOP_RIGHT: Vec2 = Vec2::new(1.0, 1.0);
pub const UV_BOTTOM_RIGHT: Vec2 = Vec2::new(1.0, 0.0);

pub fn create_window() -> (
    i32,
    i32,
    glfw::Window,
    std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    glfw::Glfw,
    &'static glow::Context,
) {
    use glfw::Context;
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
    let monitor = glfw::Monitor::from_primary();
    let video_mode = monitor.get_video_mode().unwrap();
    let (width, height) = (
        (video_mode.width as f32 / 1.5) as i32,
        (video_mode.height as f32 / 1.5) as i32,
    );
    let (mut window, events) = glfw
        .create_window(
            width as u32,
            height as u32,
            "Triangle",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");
    window.set_resizable(true);
    window.set_key_polling(true);
    window.make_current();

    assert!(window.is_opengl_debug_context());
    assert!(window.is_resizable());

    let gl = unsafe {
        GL = MaybeUninit::new(glow::Context::from_loader_function(|s| {
            window.get_proc_address(s) as *const _
        }));
        GL.assume_init_ref()
    };

    (width, height, window, events, glfw, gl)
}

//TODO: Remove
pub unsafe fn container() -> glow::NativeTexture {
    let gl = GL.assume_init_ref();
    let bytes = include_bytes!("../container.jpg");
    let im = image::load_from_memory(bytes).unwrap();
    let texture = gl.create_texture().unwrap();

    gl.active_texture(glow::TEXTURE0);
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
    texture
}

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

//TODO: Uniforms?
/// Macro for creating shaders.
/// ```rs
/// let program = shader! {
///     include_str!("../shaders/simple.vert"),
///     include_str!("../shaders/text.frag"),
///     Vec2 => 0,
///     Vec4 => 1,
///     Vec2 => 2
/// };
/// ```
#[macro_export]
macro_rules! shader {
    ($vert:expr, $frag:expr, $($type:ident => $position:expr),*$(,)?) => {
        unsafe {
            use glow::HasContext;

            let gl = $crate::GL.assume_init_ref();
            let mut stride = 0;
            let mut _offset = 0;

            $(
                stride += std::mem::size_of::<$type>();
            )*

            $(
                let n = std::mem::size_of::<$type>() / std::mem::size_of::<f32>();
                gl.enable_vertex_attrib_array($position);
                gl.vertex_attrib_pointer_f32($position, n as i32, glow::FLOAT, false, stride as i32, _offset as i32);
                _offset += std::mem::size_of::<$type>();
            )*

            let program = gl.create_program().unwrap();

            let v = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            let error = gl.get_shader_info_log(v);
            if !error.is_empty() {
                panic!("{error}");
            }
            gl.shader_source(v, $vert);
            gl.compile_shader(v);
            gl.attach_shader(program, v);

            let f = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            let error = gl.get_shader_info_log(f);
            if !error.is_empty() {
                panic!("{error}");
            }
            gl.shader_source(f, $frag);
            gl.compile_shader(f);
            gl.attach_shader(program, f);

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            gl.delete_shader(v);
            gl.delete_shader(f);

            program
        }
   };
}

#[macro_export]
macro_rules! vertex {
    () => {
        Vertex::default()
    };
    ($position:expr) => {
        Vertex {
            position: $position.into(),
            uv: Vec2::default(),
            color: Vec4::default(),
        }
    };
    ($position:expr, $color:expr) => {
        Vertex {
            position: $position.into(),
            uv: Vec2::default(),
            color: $color,
        }
    };
    ($position:expr, $color:expr, $uv:expr) => {
        Vertex {
            position: $position.into(),
            color: $color.into(),
            uv: $uv.into(),
        }
    };
}

//I think rust packed my struct in a weird way.
//So align won't work unless you use `repr(C)`.
#[repr(C)]
#[derive(Default, Debug)]
pub struct Vertex {
    pub position: Vec2,
    pub uv: Vec2,
    pub color: Vec4,
}

impl Vertex {
    pub fn position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }
    pub fn uv(mut self, uv: Vec2) -> Self {
        self.uv = uv;
        self
    }
    pub fn color(mut self, color: Vec4) -> Self {
        self.color = color;
        self
    }
}

#[inline]
pub fn buffer(vertices: &[f32]) -> &[u8] {
    unsafe {
        core::slice::from_raw_parts(
            vertices.as_ptr() as *const u8,
            vertices.len() * core::mem::size_of::<f32>(),
        )
    }
}

#[inline]
pub fn hex(hex: u32) -> Vec4 {
    let bytes: [u8; 4] = hex.to_be_bytes();

    Vec4::new(
        bytes[1] as f32 / 255.,
        bytes[2] as f32 / 255.,
        bytes[3] as f32 / 255.,
        1.0,
    )
}

pub struct Renderer {
    pub gl: &'static glow::Context,
    pub vertices: Vec<Vertex>,
    pub vao: NativeVertexArray,
    pub vbo: NativeBuffer,
    pub buffer_size: usize,
    pub width: i32,
    pub height: i32,
    pub projection: glm::Mat4x4,
    pub projection_location: NativeUniformLocation,
    pub shader: NativeProgram,
}

impl Renderer {
    pub fn new(gl: &'static glow::Context, width: i32, height: i32) -> Self {
        unsafe {
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

            gl.clear_color(0.2, 0.2, 0.2, 0.2);

            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

            #[allow(unused)]
            let basic = shader! {
                include_str!("../shaders/simple.vert"),
                include_str!("../shaders/text.frag"),
                Vec2 => 0,
                Vec2 => 1,
                Vec4 => 2
            };

            gl.use_program(Some(basic));

            //1:1 pixel mapping projection matrix. Bottom left origin.
            let projection = glm::ortho(0.0, width as f32, 0.0, height as f32, -1.0, 1.0);
            let location = gl.get_uniform_location(basic, "projection").unwrap();
            gl.uniform_matrix_4_f32_slice(Some(&location), false, projection.as_slice());

            Self {
                gl,
                vao,
                vbo,
                vertices: Vec::new(),
                buffer_size: 0,
                width,
                height,
                projection,
                projection_location: location,
                shader: basic,
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

    ///Create in counter clockwise order.
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

    /// Draws a solid rectangle with its top-left corner at `[x, y]` with size `[w, h]` (width going to
    /// the right, height going down).
    pub fn texture(&mut self, x: f32, y: f32, w: f32, h: f32, color: Vec4) {
        //Bottom left, bottom right, top right.
        //Top right, top left, bottom left.

        //TODO: I want to mix and match floats and vecs when creating vertex data.
        //Not sure how to do it. Right now it sucks bad.
        // #[rustfmt::skip]
        // let vertices = [
        //     vertex!((x    , y    ), color, (0.0, 0.0)),
        //     vertex!((x + w, y    ), color, (1.0, 0.0)),
        //     vertex!((x + w, y + h), color, (1.0, 1.0)),
        //     vertex!((x + w, y + h), color, (1.0, 1.0)),
        //     vertex!((x    , y + h), color, (0.0, 1.0)),
        //     vertex!((x    , y    ), color, (0.0, 0.0))
        // ];

        #[rustfmt::skip]
        let vertices = [
            vertex!((x    , y    ), color, (0.0, 1.0)),
            vertex!((x + w, y    ), color, (1.0, 1.0)),
            vertex!((x + w, y + h), color, (1.0, 0.0)),
            vertex!((x + w, y + h), color, (1.0, 0.0)),
            vertex!((x    , y + h), color, (0.0, 0.0)),
            vertex!((x    , y    ), color, (0.0, 1.0))
        ];
        self.vertices.extend(vertices);
    }

    pub fn quad(&mut self, x: f32, y: f32, w: f32, h: f32, color: Vec4) {
        //Bottom left, bottom right, top right.
        //Top right, top left, bottom left.
        #[rustfmt::skip]
        let vertices = [
            vertex!((x    , y    ), color, (0.0, 0.0)),
            vertex!((x + w, y    ), color, (0.0, 0.0)),
            vertex!((x + w, y + h), color, (0.0, 0.0)),
            vertex!((x + w, y + h), color, (0.0, 0.0)),
            vertex!((x    , y + h), color, (0.0, 0.0)),
            vertex!((x    , y    ), color, (0.0, 0.0))
        ];
        self.vertices.extend(vertices);
    }

    pub fn use_shader(&mut self, program: NativeProgram) {
        unsafe {
            self.shader = program;
            self.gl.use_program(Some(self.shader));

            //Update the projection matrix location.
            self.projection_location = self
                .gl
                .get_uniform_location(self.shader, "projection")
                .unwrap();

            self.gl.uniform_matrix_4_f32_slice(
                Some(&self.projection_location),
                false,
                self.projection.as_slice(),
            );
        }
    }

    pub fn clear(&self) {
        unsafe {
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            //When replacing the entire data store, consider using glBufferSubData rather than completely recreating the data store with glBufferData. This avoids the cost of reallocating the data store.
            if self.buffer_size != self.vertices.len() {
                self.gl.buffer_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    self.vertices.align_to::<u8>().1,
                    glow::DYNAMIC_DRAW,
                );
                self.buffer_size = self.vertices.len();
            } else {
                self.gl.buffer_sub_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    0,
                    self.vertices.align_to::<u8>().1,
                );
            }

            // self.gl.draw_arrays(glow::LINES, 0, 2);
            self.gl
                .draw_arrays(glow::TRIANGLES, 0, self.vertices.len() as i32);
        }
    }

    pub fn update(&mut self, width: i32, height: i32) {
        unsafe {
            self.projection = glm::ortho(0.0, width as f32, 0.0, height as f32, -1.0, 1.0);
            self.gl.uniform_matrix_4_f32_slice(
                Some(&self.projection_location),
                false,
                self.projection.as_slice(),
            );
            self.gl.viewport(0, 0, width, height);
        }
    }

    pub fn enable_blend(&mut self) {
        unsafe {
            self.gl.enable(glow::BLEND);
            self.gl
                .blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_CONSTANT_ALPHA);
        }
    }

    pub fn disable_blend(&mut self) {
        unsafe {
            self.gl.disable(glow::BLEND);
        }
    }

    pub fn reset(&mut self) {
        self.vertices.clear();
    }
}
