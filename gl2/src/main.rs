#![windows_subsystem = "windows"]
use glfw::*;
use glow::HasContext;
use mini::{defer_results, profile};

pub fn load_gl(window: &mut Window) -> glow::Context {
    profile!();
    unsafe {
        let gl = glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _);
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
        gl
    }
}

fn main() {
    defer_results!();
    profile!();
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw
        .create_window(800, 600, "gl2", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();

    let gl = load_gl(&mut window);

    while !window.should_close() {
        unsafe { gl.clear(glow::COLOR_BUFFER_BIT) };

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Close => window.set_should_close(true),
                _ => {}
            };
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
