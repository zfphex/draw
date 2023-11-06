#![feature(const_maybe_uninit_zeroed)]
use font::*;
use glfw::{Action, Key, Monitor, WindowEvent};
use glow::HasContext;
use std::mem::{size_of, MaybeUninit};
extern crate nalgebra_glm as glm;

//https://www.khronos.org/opengl/wiki/Face_Culling
//By default OpenGL uses counter-clockwise winding order.
fn main() {
    use glfw::Context;

    #[allow(unused)]
    unsafe {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));
        let monitor = Monitor::from_primary();
        let video_mode = monitor.get_video_mode().unwrap();
        let (width, height) = (
            (video_mode.width as f32 / 1.5) as i32,
            (video_mode.height as f32 / 1.5) as i32,
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
        window.set_resizable(true);
        window.set_key_polling(true);
        window.make_current();

        assert!(window.is_opengl_debug_context());
        assert!(window.is_resizable());

        GL = MaybeUninit::new(glow::Context::from_loader_function(|s| {
            window.get_proc_address(s) as *const _
        }));
        let gl = GL.assume_init_ref();

        let projection = glm::ortho(0.0, width as f32, 0.0, height as f32, 0.0, 0.0);
        let mut rd = Renderer::new(gl);

        // let font = create_program(
        //     &gl,
        //     include_str!("../shaders/simple.vert"),
        //     include_str!("../shaders/text.frag"),
        // );
        // rd.use_shader(font);

        let basic = shader! {
            include_str!("../shaders/simple.vert"),
            include_str!("../shaders/text.frag"),
            Vec2 => 0,
            Vec2 => 1,
            Vec4 => 2
        };
        rd.use_shader(basic);

        let location = gl.get_uniform_location(basic, "projection").unwrap();
        gl.uniform_matrix_4_f32_slice(Some(&location), false, projection.as_slice());

        let atlas = load_font(&rd, include_bytes!("../JetBrainsMono.ttf"));
        // let tex = texture();

        atlas.draw_text(
            &mut rd,
            "This is sample text",
            25.0,
            25.0,
            1.0,
            (0.0, 0.0, 0.0, 0.0).into(),
        );

        // draw_character(
        //     &atlas,
        //     &mut rd,
        //     'g',
        //     0.0,
        //     0.0,
        //     Vec4::new(1.0, 1.0, 1.0, 1.0),
        // );

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
        // let p3 = Vec2::new(-0.5, 0.5);
        // let color = Vec4::new(1.0, 0.5, 1.0, 1.0);
        // let uv0 = Vec2::new(1.0, 1.0);
        // let uv1 = Vec2::new(1.0, 0.0);
        // let uv2 = Vec2::new(0.0, 1.0);
        // rd.triangle(p0, p1, p2, color, color, color, uv0, uv1, uv2);
        // rd.triangle(p3, p2, p0, color, color, color, uv0, uv1, uv2);

        let color = Vec4::new(0.5, 0.5, 0.5, 1.0);
        //TODO: What order can vertecies be created in.
        //does it need to go around in a circle or top to bottom?
        let uv0 = Vec2::new(1.0, 1.0);
        let uv1 = Vec2::new(0.0, 1.0);
        let uv2 = Vec2::new(0.0, 0.0);
        let uv3 = Vec2::new(1.0, 0.0);

        rd.draw_rectangle(-0.5, -0.5, 1.0, 1.0, color);

        // rd.triangle(
        //     TOP_RIGHT,
        //     TOP_LEFT,
        //     BOTTOM_LEFT,
        //     color,
        //     color,
        //     color,
        //     UV_TOP_RIGHT,
        //     UV_TOP_LEFT,
        //     UV_BOTTOM_LEFT,
        // );

        // rd.triangle(
        //     BOTTOM_LEFT,
        //     BOTTOM_RIGHT,
        //     TOP_RIGHT,
        //     color,
        //     color,
        //     color,
        //     UV_BOTTOM_LEFT,
        //     UV_BOTTOM_RIGHT,
        //     UV_TOP_RIGHT,
        // );

        // rd.quad(
        //     TOP_RIGHT,
        //     TOP_LEFT,
        //     BOTTOM_LEFT,
        //     BOTTOM_RIGHT,
        //     color,
        //     color,
        //     color,
        //     color,
        //     uv0,
        //     uv1,
        //     uv2,
        //     uv3,
        // );

        // let mut tex = [0.0; 8];
        // let pos = [0.5, 0.5, -0.5, 0.5, -0.5, -0.5, 0.5, -0.5];
        // generate_texture_coordinates(&pos, &mut tex);

        // return;

        let (mut width, mut height) = window.get_framebuffer_size();

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

            let (w, h) = window.get_framebuffer_size();
            if width != w || height != h {
                rd.update(w, h);
            }

            rd.clear();
            rd.draw();

            window.swap_buffers();
            glfw.poll_events();
        }
    }
}
