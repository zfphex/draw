#![feature(const_maybe_uninit_zeroed)]
use font::*;
use glfw::{Action, Context, Key, WindowEvent};
extern crate nalgebra_glm as glm;

//https://www.khronos.org/opengl/wiki/Face_Culling
//By default OpenGL uses counter-clockwise winding order.
fn main() {
    unsafe {
        let (width, height, mut window, events, mut glfw, gl) = create_window();
        let mut rd = Renderer::new(gl, width, height);

        // let atlas = load_font(&rd, include_bytes!("../JetBrainsMono.ttf"));
        let atlas = load_font(&rd, include_bytes!("../CascadiaMono.ttf"));

        rd.enable_blend();
        // rd.texture(
        //     0.0,
        //     0.0,
        //     width as f32,
        //     atlas.height as f32,
        //     Vec4::new(1.0, 1.0, 1.0, 1.0),
        // );

        atlas.draw_text(
            &mut rd,
            // "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            // "abcdefghijklmnopqrstuvwxyz",
            // "Let's check out this epic text! Wow it works so well.",
            "This is the first line.\nThis is the second line!\n\nThis is the third line.",
            25.0,
            200.0,
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
