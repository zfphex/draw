use std::{
    ffi::CString,
    mem::transmute,
    panic::Location,
    ptr::{null, null_mut},
};

use glfw::*;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winapi::{
    shared::{
        dxgi::*, dxgi1_2::*, dxgiformat::*, dxgitype::*, minwindef::*, ntdef::HRESULT, winerror::*,
    },
    um::{d3d11::*, d3dcommon::*, d3dcompiler::*},
    Interface,
};

pub trait WinResult {
    fn unwrap(self);
}

impl WinResult for HRESULT {
    #[track_caller]
    fn unwrap(self) {
        if self == S_FALSE {
            let c = Location::caller();
            return println!(
                "{}:{}:{}: Successful but nonstandard completion (the precise meaning depends on context).", c.file(), c.line(), c.column(),
            );
        }
        if self != S_OK {
            let result: u32 = unsafe { transmute(self) };
            match result {
                //D3D11_ERROR_FILE_NOT_FOUND
                0x887C0002 => panic!("D3D11_ERROR_FILE_NOT_FOUND - The file was not found."),
                0x887C0001 => panic!("D3D11_ERROR_TOO_MANY_UNIQUE_STATE_OBJECTS - There are too many unique instances of a particular type of state object."),
                0x887C0003 => panic!("D3D11_ERROR_TOO_MANY_UNIQUE_VIEW_OBJECTS - There are too many unique instances of a particular type of view object."),
                0x887C0004 => panic!("D3D11_ERROR_DEFERRED_CONTEXT_MAP_WITHOUT_INITIAL_DISCARD - The first call to ID3D11DeviceContext::Map after either ID3D11Device::CreateDeferredContext or ID3D11DeviceContext::FinishCommandList per Resource was not D3D11_MAP_WRITE_DISCARD."),
                0x887A0001 => panic!("DXGI_ERROR_INVALID_CALL - The method call is invalid. For example, a method's parameter may not be a valid pointer."),
                0x887A000A => panic!("DXGI_ERROR_WAS_STILL_DRAWING - The previous blit operation that is transferring information to or from this surface is incomplete."),
                0x80004005 => panic!("E_FAIL - Attempted to create a device with the debug layer enabled and the layer is not installed."),
                0x80070057 => panic!("E_INVALIDARG - An invalid parameter was passed to the returning function."),
                0x8007000E => panic!("E_OUTOFMEMORY - Direct3D could not allocate sufficient memory to complete the call."),
                0x80004001 => panic!("E_NOTIMPL - The method call isn't implemented with the passed parameter combination."),
                0x8007000A => panic!("E_PENDING - The data necessary to complete this operation is not yet available."),
                0x80004002 => panic!("E_POINTER - An invalid pointer was used."),
                0x80070006 => panic!("E_HANDLE - The handle is invalid."),
                0x80070005 => panic!("E_ACCESSDENIED - General access denied error."),
                0x800700AA => panic!("E_ABORT - Operation aborted."),
                0x800700DF => panic!("E_UNEXPECTED - Catastrophic failure."),
                0x8007007A => panic!("E_REMCLOSED - The object has been closed."),
                0x80070057 => panic!("E_INVALIDARG - One or more arguments are invalid."),
                0x80070070 => panic!("E_DISK_FULL - The disk is full."),
                0x8007000C => panic!("E_PAGENOTACTIVE - The page is not active."),
                0x80004003 => panic!("E_POINTER - Invalid pointer."),
                0x80070017 => panic!("E_NOTIMPL - Requested function is not implemented."),
                0x800700AA => panic!("E_ABORT - Operation aborted."),
                0x80070006 => panic!("E_HANDLE - Invalid handle."),
                0x8007001F => panic!("E_INVALIDARG - An invalid parameter was passed to the returning function."),
                0x8007000E => panic!("E_OUTOFMEMORY - Out of memory."),
                0x80070057 => panic!("E_INVALIDARG - Invalid argument."),
                0x80004005 => panic!("E_FAIL - Unspecified failure."),
                0x8007000D => panic!("E_PENDING - The data necessary to complete this operation is not yet available."),
                0x887A002B => panic!("DXGI_ERROR_ACCESS_DENIED - You tried to use a resource to which you did not have the required access privileges. This error is most typically caused when you write to a shared resource with read-only access."),
                0x887A0026 => panic!("DXGI_ERROR_ACCESS_LOST - The desktop duplication interface is invalid. The desktop duplication interface typically becomes invalid when a different type of image is displayed on the desktop."),
                0x887A0036 => panic!("DXGI_ERROR_ALREADY_EXISTS - The desired element already exists. This is returned by DXGIDeclareAdapterRemovalSupport if it is not the first time that the function is called."),
                0x887A002A => panic!("DXGI_ERROR_CANNOT_PROTECT_CONTENT - DXGI can't provide content protection on the swap chain. This error is typically caused by an older driver, or when you use a swap chain that is incompatible with content protection."),
                0x887A0006 => panic!("DXGI_ERROR_DEVICE_HUNG - The application's device failed due to badly formed commands sent by the application. This is a design-time issue that should be investigated and fixed."),
                0x887A0005 => panic!("DXGI_ERROR_DEVICE_REMOVED - The video card has been physically removed from the system, or a driver upgrade for the video card has occurred. The application should destroy and recreate the device."),
                0x887A0007 => panic!("DXGI_ERROR_DEVICE_RESET - The device failed due to a badly formed command. This is a run-time issue; The application should destroy and recreate the device."),
                0x887A0020 => panic!("DXGI_ERROR_DRIVER_INTERNAL_ERROR - The driver encountered a problem and was put into the device removed state."),
                0x887A000B => panic!("DXGI_ERROR_FRAME_STATISTICS_DISJOINT - An event (for example, a power cycle) interrupted the gathering of presentation statistics."),
                0x887A000C => panic!("DXGI_ERROR_GRAPHICS_VIDPN_SOURCE_IN_USE - The application attempted to acquire exclusive ownership of an output, but failed because some other application (or device within the application) already acquired ownership."),
                0x887A0001 => panic!("DXGI_ERROR_INVALID_CALL - The application provided invalid parameter data; this must be debugged and fixed before the application is released."),
                0x887A0003 => panic!("DXGI_ERROR_MORE_DATA - The buffer supplied by the application is not big enough to hold the requested data."),
                0x887A002C => panic!("DXGI_ERROR_NAME_ALREADY_EXISTS - The supplied name of a resource in a call to IDXGIResource1::CreateSharedHandle is already associated with some other resource."),
                0x887A0021 => panic!("DXGI_ERROR_NONEXCLUSIVE - A global counter resource is in use, and the Direct3D device can't currently use the counter resource."),
                0x887A0022 => panic!("DXGI_ERROR_NOT_CURRENTLY_AVAILABLE - The resource or request is not currently available, but it might become available later."),
                0x887A0002 => panic!("DXGI_ERROR_NOT_FOUND - When calling IDXGIObject::GetPrivateData, the GUID passed in is not recognized as one previously passed to IDXGIObject::SetPrivateData or IDXGIObject::SetPrivateDataInterface. When calling IDXGIFactory::EnumAdapters or IDXGIAdapter::EnumOutputs, the enumerated ordinal is out of range."),
                0x887A0023 => panic!("DXGI_ERROR_REMOTE_CLIENT_DISCONNECTED - Reserved"),
                0x887A0024 => panic!("DXGI_ERROR_REMOTE_OUTOFMEMORY - Reserved"),
                0x887A0029 => panic!("DXGI_ERROR_RESTRICT_TO_OUTPUT_STALE - The DXGI output (monitor) to which the swap chain content was restricted is now disconnected or changed."),
                0x887A002D => panic!("DXGI_ERROR_SDK_COMPONENT_MISSING - The operation depends on an SDK component that is missing or mismatched."),
                0x887A0028 => panic!("DXGI_ERROR_SESSION_DISCONNECTED - The Remote Desktop Services session is currently disconnected."),
                0x887A0004 => panic!("DXGI_ERROR_UNSUPPORTED - The requested functionality is not supported by the device or the driver."),
                0x887A0027 => panic!("DXGI_ERROR_WAIT_TIMEOUT - The time-out interval elapsed before the next desktop frame was available."),
                0x887A000A => panic!("DXGI_ERROR_WAS_STILL_DRAWING - The GPU was busy at the moment when a call was made to perform an operation, and did not execute or schedule the operation."),
                _ => panic!("{:#x}", self),
            }
        }
    }
}

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
        let handle = window.raw_window_handle();
        let win32 = match handle {
            RawWindowHandle::Win32(handle) => handle,
            _ => unreachable!(),
        };
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
