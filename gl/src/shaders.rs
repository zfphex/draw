use std::mem::size_of;

use crate::*;

/// Limitations: https://registry.khronos.org/OpenGL-Refpages/gl4/html/glLinkProgram.xhtml
/// Cannot exceed the limit for attributes, uniforms or have any compile issues.
/// This should not be created more than once.
pub unsafe fn shader<S: AsRef<str>>(gl: &Context, vertex: S, fragment: S) -> NativeProgram {
    let program = gl.create_program().unwrap();

    let create = |shader_type: u32, source: &str| -> NativeShader {
        let shader = gl.create_shader(shader_type).unwrap();
        let error = gl.get_shader_info_log(shader);
        if !error.is_empty() {
            panic!("{}", error);
        }
        gl.shader_source(shader, source.as_ref());
        gl.compile_shader(shader);
        gl.attach_shader(program, shader);
        shader
    };

    let v = create(glow::VERTEX_SHADER, vertex.as_ref());
    let f = create(glow::FRAGMENT_SHADER, fragment.as_ref());

    //Links the program and checks the shader for issues.
    gl.link_program(program);
    if !gl.get_program_link_status(program) {
        panic!("{}", gl.get_program_info_log(program));
    }

    //Cleanup
    gl.delete_shader(v);
    gl.delete_shader(f);

    program
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

pub fn color(r: f32, g: f32, b: f32) -> Color {
    Color { r, g, b }
}

pub const TRIANGLE_V: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 pos;
        layout (location = 1) in vec3 color;

        out vec3 input_color;

        void main() {
            gl_Position = vec4(pos, 1.0);
            input_color = color;
        }
    "#;

pub const TRIANGLE_F: &str = r#"
        #version 330 core
        out vec4 color;
        in vec4 input_color;

        void main() {
            color = input_color;
        }
    "#;

pub const RECTANGLE_V: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 pos;
        layout (location = 1) in vec3 color;

        out vec3 input_color;

        void main() {
            gl_Position = vec4(pos, 1.0);
            input_color = color;
        }
    "#;

pub const RECTANGLE_F: &str = r#"
        #version 330 core
        out vec4 color;
        in vec4 input_color;

        void main() {
            color = input_color;
        }
    "#;

pub struct TriangleBuffer {
    pub buffer: NativeBuffer,
    pub shader: NativeProgram,
    pub data: Vec<f32>,
}

impl TriangleBuffer {
    pub fn new(gl: &Context) -> Self {
        unsafe {
            Self {
                buffer: gl.create_buffer().unwrap(),
                shader: shader(gl, TRIANGLE_V, TRIANGLE_F),
                data: Vec::new(),
            }
        }
    }
    pub fn bind(&mut self, gl: &Context) {
        unsafe { gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer)) };
    }
    pub fn upload(&mut self, gl: &Context) {
        self.bind(gl);
        unsafe {
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(&self.data), glow::STATIC_DRAW);
        }
    }
    pub fn extend(&mut self, vertex: &[f32]) {
        self.data.extend(vertex);
    }
    pub fn clear(&mut self, gl: &Context) {
        self.data.clear();
        self.bind(gl);
        unsafe {
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                buffer(&self.data),
                glow::MAP_INVALIDATE_BUFFER_BIT,
            );
        }
        self.upload(gl);
    }
    pub fn draw(&self, gl: &Context) {
        unsafe {
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 6 * 4, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 6 * 4, 3 * 4);
            gl.enable_vertex_attrib_array(1);

            gl.use_program(Some(self.shader));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}

pub unsafe fn test(gl: &Context) {
    let mut tb = TriangleBuffer::new(gl);
    let (r, g, b) = (1.0, 1.0, 1.0);
    #[rustfmt::skip]
    tb.extend(&[
        0.5, -0.5, 0.0, r, g, b, -0.5, -0.5, 0.0, r, g, b, 0.0, 0.5, 0.0, r, g, b,
    ]);

    // #[rustfmt::skip]
    // tb.extend(&[
    //     0.5 + 0.2, -0.5 + 0.2, 0.0, r, g, b, -0.5, -0.5, 0.0, r, g, b, 0.0, 0.5, 0.0, r, g, b,
    // ]);

    // #[rustfmt::skip]
    // tb.extend(&[
    //     0.5 + 0.4, -0.5 + 0.4, 0.0, r, g, b, -0.5, -0.5, 0.0, r, g, b, 0.0, 0.5, 0.0, r, g, b,
    // ]);

    // #[rustfmt::skip]
    // tb.extend(&[
    //     0.5 + 0.6, -0.5 + 0.6, 0.0, r, g, b, -0.5, -0.5, 0.0, r, g, b, 0.0, 0.5, 0.0, r, g, b,
    // ]);

    tb.upload(gl);

    tb.draw(gl);
}

pub unsafe fn draw_rectangle(gl: &Context, x: f32, y: f32, w: f32, h: f32, color: Color) {
    let program = shader(gl, RECTANGLE_V, RECTANGLE_F);
    gl.use_program(Some(program));

    #[rustfmt::skip]
    let vertices = [
        x,     y + h, 0.0, color.r, color.g, color.b,
        x + w, y + h, 0.0, color.r, color.g, color.b,
        x + w, y,     0.0, color.r, color.g, color.b,
        x,     y,     0.0, color.r, color.g, color.b,
    ];
    let indices = [0, 1, 2, 2, 3, 0];

    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(&vertices), glow::STATIC_DRAW);

    let ebo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
    gl.buffer_data_u8_slice(
        glow::ELEMENT_ARRAY_BUFFER,
        indices.align_to::<u8>().1,
        glow::STATIC_DRAW,
    );

    let stride = (6 * size_of::<f32>()) as i32;

    gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
    gl.enable_vertex_attrib_array(0);

    gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, stride, 3 * 4);
    gl.enable_vertex_attrib_array(1);

    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
    gl.draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_INT, 0);
}

pub unsafe fn draw_rectangle_ortho(gl: &Context, width: f32, height: f32) {
    const V: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 position;
    uniform mat4 projection;

    void main()
    {
        gl_Position = projection * vec4(position, 0.0, 1.0);
    }
"#;

    const F: &str = r#"
    #version 330 core

    void main() {
        gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
    }
"#;

    let program = shader(gl, V, F);
    gl.use_program(Some(program));

    let projection_location = gl.get_uniform_location(program, "projection").unwrap();
    let projection = glm::ortho(0.0, width, height, 0.0, -1.0, 1.0);

    let vertices = [
        100.0, 100.0, // Top-left
        300.0, 100.0, // Top-right
        300.0, 300.0, // Bottom-right
        100.0, 300.0, // Bottom-left
    ];
    let indices = [0, 1, 2, 2, 3, 0];

    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(&vertices), glow::STATIC_DRAW);

    let stride = (2 * size_of::<f32>()) as i32;
    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
    gl.enable_vertex_attrib_array(0);

    let ebo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
    gl.buffer_data_u8_slice(
        glow::ELEMENT_ARRAY_BUFFER,
        indices.align_to::<u8>().1,
        glow::STATIC_DRAW,
    );

    gl.uniform_matrix_4_f32_slice(Some(&projection_location), false, projection.as_slice());

    check_error(gl);
}

const LINE_V: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 position;
    layout (location = 1) in vec3 color;

    out vec3 input_color;

    void main()
    {
        gl_Position = vec4(position, 0.0, 1.0);
        input_color = color;
    }
"#;

const LINE_F: &str = r#"
    #version 330 core
    out vec4 color;
    in vec4 input_color;

    void main() {
        color = input_color;
    }
"#;

pub unsafe fn draw_line(gl: &Context, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
    let program = shader(gl, LINE_V, LINE_F);
    gl.use_program(Some(program));

    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(&[x1, y1]), glow::STATIC_DRAW);

    check_error(gl);

    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 2 * 3, 0);
    gl.enable_vertex_attrib_array(0);

    // gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 3 * 3, 2 * 3);
    // gl.enable_vertex_attrib_array(1);

    gl.draw_arrays(glow::LINES, 0, 2);
}
