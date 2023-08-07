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

pub const TRIANGLE_V: &str = r#"
        #version 330 core
        layout (location = 0) in vec3 pos;
        // in vec3 pos;
        layout (location = 1) in vec3 color;

        out vec3 input_color;

        void main() {
            gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
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

pub unsafe fn draw_triangle(
    gl: &Context,
    v1: glm::Vec2,
    v2: glm::Vec2,
    v3: glm::Vec2,
    r: f32,
    g: f32,
    b: f32,
) {
    // pub unsafe fn draw_triangle(gl: &Context) {
    let program = shader(gl, TRIANGLE_V, TRIANGLE_F);
    gl.use_program(Some(program));

    // #[rustfmt::skip]
    // let vertices = [
    //     0.5, -0.5, 0.0,  //Position
    //     r,g,b,
    //     -0.5, -0.5, 0.0, //Position
    //     r,g,b,
    //     0.0, 0.5, 0.0,   //Position
    //     r,g,b
    // ];

    #[rustfmt::skip]
    let vertices = [
        v1.x, v1.y, 0.0, r, g, b,
        v2.x, v2.y, 0.0, r, g, b,
        v3.x, v3.y, 0.0, r, g, b,
    ];

    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
    gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, buffer(&vertices), glow::STATIC_DRAW);

    gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 6 * 4, 0);
    gl.enable_vertex_attrib_array(0);

    gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, 6 * 4, 3 * 4);
    gl.enable_vertex_attrib_array(1);

    gl.draw_arrays(glow::TRIANGLES, 0, 3);
}
