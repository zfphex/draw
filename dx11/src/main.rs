#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use error::*;
use glfw::*;
use std::{
    ffi::CString,
    mem::transmute,
    ptr::{null, null_mut},
};
use winapi::{
    shared::{dxgi::*, dxgi1_2::*, dxgiformat::*, dxgitype::*, minwindef::*},
    um::{d3d11::*, d3dcommon::*, d3dcompiler::*},
    Interface,
};

mod error;

pub enum Target {
    Vertex,
    Pixel,
    Fragment,
    Compute,
    Domain,
    Geometry,
    Hull,
}

fn compile_shader(src_data: &str, entry_point: &str, target: Target) -> Result<Vec<u8>, String> {
    unsafe {
        let mut code: *mut ID3DBlob = null_mut();
        let mut error_msgs: *mut ID3DBlob = null_mut();
        let target = match target {
            Target::Vertex => "vs_5_0",
            Target::Pixel | Target::Fragment => "ps_5_0",
            Target::Compute => "cs_5_0",
            Target::Domain => "ds_5_0",
            Target::Geometry => "gs_5_0",
            Target::Hull => "hs_5_0",
        };
        let entry_point = CString::new(entry_point).unwrap();
        let target = CString::new(target).unwrap();

        let hr = D3DCompile(
            src_data.as_bytes().as_ptr() as LPCVOID,
            src_data.as_bytes().len(),
            null(),
            null(),
            null_mut(),
            entry_point.as_ptr(),
            target.as_ptr(),
            0,
            0,
            &mut code,
            &mut error_msgs,
        );

        if hr < 0 {
            if !error_msgs.is_null() {
                let error_msgs = error_msgs.as_ref().unwrap();

                let error_msgs = std::str::from_utf8(std::slice::from_raw_parts(
                    error_msgs.GetBufferPointer() as *const u8,
                    error_msgs.GetBufferSize(),
                ))
                .unwrap();

                Err(error_msgs.to_string())
            } else {
                Err(format!("hresult: {}", hr))
            }
        } else {
            let code = code.as_ref().unwrap();

            Ok(std::slice::from_raw_parts(
                code.GetBufferPointer() as *const u8,
                code.GetBufferSize(),
            )
            .to_vec())
        }
    }
}

//https://learn.microsoft.com/en-us/windows/win32/direct3d11/how-to--compile-a-shader
const V: &str = r#"
float4 main(float2 pos : Position) : SV_Position
{
    return float4(pos.x, pos.y, 0.0f, 1.0f);
}
"#;

const P: &str = r#"
float4 main() : SV_Target
{
    return float4(0.0f, 1.0f, 0.0f, 1.0f);
}
"#;

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
//https://antongerdelan.net/opengl/d3d11.html
pub fn main() {
    unsafe {
        let adapter: *mut IDXGIAdapter = null_mut();
        let mut device: *mut ID3D11Device = null_mut();
        let mut immediate_context: *mut ID3D11DeviceContext = null_mut();
        let mut dxgi_device: *mut IDXGIDevice = null_mut();
        let mut dxgi_adapter: *mut IDXGIAdapter = null_mut();
        let mut dxgi_factory: *mut IDXGIFactory1 = null_mut();
        let mut dxgi_factory_2: *mut IDXGIFactory2 = null_mut();

        let mut framebuffer: *mut ID3D11Texture2D = null_mut();
        let mut swapchain: *mut IDXGISwapChain1 = null_mut();
        let mut render_target_view: *mut ID3D11RenderTargetView = null_mut();

        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .create_window(800, 600, "DX11", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        let win32 = window.get_win32_window();

        window.set_key_polling(true);
        window.make_current();

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

        (*device)
            .QueryInterface(
                &IDXGIDevice::uuidof(),
                std::mem::transmute(&mut dxgi_device),
            )
            .unwrap();

        (*dxgi_device).GetAdapter(&mut dxgi_adapter).unwrap();

        (*dxgi_adapter)
            .GetParent(
                &IDXGIFactory1::uuidof(),
                std::mem::transmute(&mut dxgi_factory),
            )
            .unwrap();

        (*dxgi_factory)
            .QueryInterface(
                &IDXGIFactory2::uuidof(),
                std::mem::transmute(&mut dxgi_factory_2),
            )
            .unwrap();

        (*dxgi_factory_2)
            .CreateSwapChainForHwnd(
                device as *mut winapi::um::unknwnbase::IUnknown,
                std::mem::transmute(win32),
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

        (*swapchain)
            .GetBuffer(0, &ID3D11Texture2D::uuidof(), transmute(&mut framebuffer))
            .unwrap();

        (*device).CreateRenderTargetView(
            transmute(framebuffer),
            null_mut(),
            transmute(&mut render_target_view),
        );

        //TODO: Use deferred context.
        let mut dctx: *mut ID3D11DeviceContext = std::ptr::null_mut();
        let error = (*device).CreateDeferredContext(0, &mut dctx);

        let device = device.as_mut().unwrap();
        let ctx = immediate_context.as_mut().unwrap();
        let swapchain = swapchain.as_mut().unwrap();

        const VERTICIES: [f32; 6] = [0.0, 0.5, 0.5, -0.5, -0.5, -0.5];

        let v = compile_shader(V, "main", Target::Vertex).unwrap();
        let p = compile_shader(P, "main", Target::Pixel).unwrap();

        let vertex_shader: *mut *mut ID3D11VertexShader = null_mut();
        (*device)
            .CreateVertexShader(
                v.as_ptr() as *const winapi::ctypes::c_void,
                v.len(),
                null_mut(),
                vertex_shader,
            )
            .unwrap();

        let pixel_shader: *mut *mut ID3D11PixelShader = null_mut();
        (*device)
            .CreatePixelShader(
                p.as_ptr() as *const winapi::ctypes::c_void,
                p.len(),
                null_mut(),
                pixel_shader,
            )
            .unwrap();

        ctx.OMSetRenderTargets(1, transmute(&mut render_target_view), null_mut());

        while !window.should_close() {
            glfw.poll_events();

            ctx.ClearRenderTargetView(render_target_view, &[1.0, 0.6, 0.6, 1.0]);

            // ctx.IASetVertexBuffers(StartSlot, NumBuffers, ppVertexBuffers, pStrides, pOffsets);
            // ctx.IASetPrimitiveTopology(D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            // ctx.Draw(3, 0);

            swapchain.Present(1, 0);

            for (_, event) in glfw::flush_messages(&events) {
                //TODO
            }
        }
    }
}
