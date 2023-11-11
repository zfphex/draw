use dx2::*;
use win_window::*;

fn main() {
    let dx11 = D3D11::new();

    let window = create_window("dx2", 1280, 800);

    let size = window.size();
    let d3window = D3D11Window::new(
        window.hwnd,
        size.width() as u32,
        size.height() as u32,
        &dx11,
        None,
    );

    loop {
        match event() {
            Event::Quit => break,
            _ => {}
        }
    }
}
