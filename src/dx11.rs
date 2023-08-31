use std::{mem::transmute, ptr::null_mut};

use glfw::*;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winapi::{
    shared::{
        dxgi::*, dxgi1_2::*, dxgiformat::*, dxgitype::*, minwindef::*, ntdef::HRESULT,
        windef::HWND, winerror::S_OK,
    },
    um::{d3d11::*, d3dcommon::*},
    Interface,
};

pub trait Result {
    fn unwrap(self);
}

impl Result for HRESULT {
    #[track_caller]
    fn unwrap(self) {
        if self != S_OK {
            panic!("{self:#x}")
        }
    }
}

// pub fn create_texture<'a>(
//     device: &GraphicsDevice,
//     texture_desc: D3D11_TEXTURE2D_DESC,
//     subresources_data: Vec<D3D11_SUBRESOURCE_DATA>,
// ) -> Result<Texture<'a>, ()> {
//     let mut texture: *mut winapi::um::d3d11::ID3D11Texture2D = null_mut();
//     let mut texture_view: *mut winapi::um::d3d11::ID3D11ShaderResourceView = null_mut();

//     unsafe {
//         let hr =
//             device
//                 .native
//                 .CreateTexture2D(&texture_desc, subresources_data.as_ptr(), &mut texture);

//         if hr != S_OK {
//             return Err(());
//         }

//         // create a resource view
//         let hr = device.native.CreateShaderResourceView(
//             texture as *mut winapi::um::d3d11::ID3D11Resource,
//             null_mut(),
//             &mut texture_view,
//         );

//         if hr != S_OK {
//             return Err(());
//         }
//     }

//     Ok(Texture {
//         native_texture: unsafe { texture.as_mut().unwrap() },
//         srv: ShaderResourceView {
//             native_view: unsafe { texture_view.as_mut().unwrap() },
//         },
//     })
// }

//https://github.com/jendrikillner/RustMatch3/blob/master/graphics_device/src/graphics_device_lib.rs#L201
//https://www.jendrikillner.com/post/rust-game-part-6/
pub fn dx11() {
    unsafe {
        let adapter: *mut IDXGIAdapter = null_mut();
        let mut device: *mut ID3D11Device = null_mut();
        let mut immediate_context: *mut ID3D11DeviceContext = null_mut();

        D3D11CreateDevice(
            adapter,
            D3D_DRIVER_TYPE_HARDWARE,
            null_mut(),
            D3D11_CREATE_DEVICE_DEBUG | D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT,
            &D3D_FEATURE_LEVEL_11_1,
            1,
            D3D11_SDK_VERSION,
            &mut device,
            null_mut(),
            &mut immediate_context,
        )
        .unwrap();

        let mut dxgi_device: *mut IDXGIDevice = null_mut();

        (*device)
            .QueryInterface(
                &IDXGIDevice::uuidof(),
                std::mem::transmute(&mut dxgi_device),
            )
            .unwrap();

        let mut dxgi_adapter: *mut IDXGIAdapter = null_mut();
        (*dxgi_device).GetAdapter(&mut dxgi_adapter).unwrap();

        let mut dxgi_factory: *mut IDXGIFactory1 = null_mut();
        (*dxgi_adapter)
            .GetParent(
                &IDXGIFactory1::uuidof(),
                std::mem::transmute(&mut dxgi_factory),
            )
            .unwrap();

        let mut dxgi_factory_2: *mut IDXGIFactory2 = null_mut();

        (*dxgi_factory)
            .QueryInterface(
                &IDXGIFactory2::uuidof(),
                std::mem::transmute(&mut dxgi_factory_2),
            )
            .unwrap();

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        let handle = window.raw_window_handle();
        let win32 = match handle {
            RawWindowHandle::Win32(handle) => handle,
            _ => unreachable!(),
        };

        let mut swapchain: *mut IDXGISwapChain1 = null_mut();

        (*dxgi_factory_2)
            .CreateSwapChainForHwnd(
                device as *mut winapi::um::unknwnbase::IUnknown,
                std::mem::transmute(win32.hwnd),
                &DXGI_SWAP_CHAIN_DESC1 {
                    Width: 0,
                    Height: 0,
                    Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                    BufferCount: 2,
                    AlphaMode: DXGI_ALPHA_MODE_UNSPECIFIED,
                    Flags: 0,
                    Scaling: DXGI_SCALING_STRETCH,
                    SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
                    Stereo: 0,
                },
                null_mut(),
                null_mut(),
                &mut swapchain,
            )
            .unwrap();
        assert!(!swapchain.is_null());

        let mut framebuffer: *mut ID3D11Texture2D = null_mut();

        (*swapchain)
            .GetBuffer(0, &ID3D11Texture2D::uuidof(), transmute(&mut framebuffer))
            .unwrap();

        let mut render_target_view: *mut ID3D11RenderTargetView = null_mut();

        (*device).CreateRenderTargetView(
            transmute(framebuffer),
            null_mut(),
            transmute(&mut render_target_view),
        );

        let ctx = immediate_context.as_mut().unwrap();

        while !window.should_close() {
            glfw.poll_events();

            ctx.ClearRenderTargetView(render_target_view, &[0.5, 0.6, 0.6, 1.0]);
            ctx.OMSetRenderTargets(1, transmute(&mut render_target_view), null_mut());

            (*swapchain).Present(1, 0);

            for (_, event) in glfw::flush_messages(&events) {
                //TODO
            }
        }
    }
}
