use dx2::*;
use window::*;

fn main() {
    let window = create_window("dx2", 1280, 800);
    let size = window.area();
    let mut dx11 = D3D11::new(window.hwnd, size.width() as u32, size.height() as u32);

    loop {
        match event() {
            Some(Event::Quit) => break,
            _ => {}
        }

        dx11.clear();
        dx11.present(false);
    }
}
