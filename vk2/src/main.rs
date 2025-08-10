// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
///!https://github.com/ash-rs/ash/blob/master/examples/src/lib.rs
///!https://github.com/zX3no/vk/blob/main/src/main.rs
///!https://vkguide.dev/
///!https://vulkan-tutorial.com/
///!https://docs.rs/ash/latest/ash/index.html
use vk2::Vulkan;
use window::*;

fn main() {
    mini::defer_results!();
    mini::profile!();
    let mut vk = Vulkan::new(800, 600, true);

    let mut frame_number = 0.0;

    loop {
        unsafe { vk2::draw(&mut vk, &mut frame_number) };

        match vk.window.event() {
            Some(Event::Quit) => {
                break;
            }
            _ => {}
        }

        if vk.resize_requested {
            unsafe { vk2::resize_swapchain(&mut vk) };
        }
    }
}
