pub use crate::*;
use glfw::{Action, Context, Key, WindowEvent};

#[test]
pub fn pixel_perfect() {
    let (width, height, mut window, events, mut glfw, gl) = create_window();
    let mut rd = Renderer::new(gl, width, height);

    let simple = shader! {
        include_str!("../shaders/simple.vert"),
        include_str!("../shaders/simple.frag"),
        Vec2 => 0,
        Vec2 => 1,
        Vec4 => 2
    };
    rd.use_shader(simple);

    for i in 0..5 {
        rd.quad(
            (300.0 as f32 / 2.0) + i as f32,
            0.0,
            1.0,
            300.0,
            if i % 3 == 0 {
                hex(0xdcdcaa)
            } else if i % 2 == 0 {
                hex(0xcc3e44)
            } else {
                hex(0x328fde)
            },
        );
    }

    let (width, height) = window.get_framebuffer_size();
    while !window.should_close() {
        let _current_frame = glfw.get_time() as f32;

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
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
