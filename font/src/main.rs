#![feature(const_maybe_uninit_zeroed)]
use font::*;
use glfw::{Action, Key, Monitor, WindowEvent};
use glow::HasContext;
use std::mem::MaybeUninit;
extern crate nalgebra_glm as glm;

pub unsafe fn texture() -> glow::NativeTexture {
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

//https://www.khronos.org/opengl/wiki/Face_Culling
//By default OpenGL uses counter-clockwise winding order.
fn main() {
    use glfw::Context;

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

        let mut rd = Renderer::new(gl, width, height);

        // let font = create_program(
        //     &gl,
        //     include_str!("../shaders/simple.vert"),
        //     include_str!("../shaders/text.frag"),
        // );
        // rd.use_shader(font);

        let atlas = load_font(&rd, include_bytes!("../JetBrainsMono.ttf"));
        // let tex = texture();

        atlas.draw_text(
            &mut rd,
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            25.0,
            25.0,
            (1.0, 1.0, 1.0, 1.0).into(),
        );

        let (width, height) = window.get_framebuffer_size();

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
