#![allow(unused_imports)]
use makepad_windows::{
    core::{ComInterface, Interface, PCSTR},
    Win32::{
        Foundation::{BOOL, HANDLE, HINSTANCE, HWND, S_FALSE},
        Graphics::{
            Direct3D::{
                Fxc::D3DCompile, ID3DBlob, D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
                D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_0,
            },
            Direct3D11::{
                D3D11CreateDevice, ID3D11BlendState, ID3D11Buffer, ID3D11DepthStencilState,
                ID3D11DepthStencilView, ID3D11Device, ID3D11DeviceContext, ID3D11InputLayout,
                ID3D11PixelShader, ID3D11Query, ID3D11RasterizerState, ID3D11RenderTargetView,
                ID3D11Resource, ID3D11ShaderResourceView, ID3D11Texture2D, ID3D11VertexShader,
                D3D11_BIND_CONSTANT_BUFFER, D3D11_BIND_DEPTH_STENCIL, D3D11_BIND_FLAG,
                D3D11_BIND_INDEX_BUFFER, D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE,
                D3D11_BIND_VERTEX_BUFFER, D3D11_BLEND_DESC, D3D11_BLEND_INV_SRC_ALPHA,
                D3D11_BLEND_ONE, D3D11_BLEND_OP_ADD, D3D11_BUFFER_DESC, D3D11_CLEAR_DEPTH,
                D3D11_CLEAR_STENCIL, D3D11_COLOR_WRITE_ENABLE_ALL, D3D11_COMPARISON_ALWAYS,
                D3D11_COMPARISON_LESS_EQUAL, D3D11_CPU_ACCESS_WRITE, D3D11_CREATE_DEVICE_FLAG,
                D3D11_CULL_NONE, D3D11_DEPTH_STENCILOP_DESC, D3D11_DEPTH_STENCIL_DESC,
                D3D11_DEPTH_STENCIL_VIEW_DESC, D3D11_DEPTH_WRITE_MASK_ALL,
                D3D11_DSV_DIMENSION_TEXTURE2D, D3D11_FILL_SOLID, D3D11_INPUT_ELEMENT_DESC,
                D3D11_INPUT_PER_INSTANCE_DATA, D3D11_INPUT_PER_VERTEX_DATA,
                D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_WRITE_DISCARD, D3D11_QUERY_DESC,
                D3D11_QUERY_EVENT, D3D11_RASTERIZER_DESC, D3D11_RENDER_TARGET_BLEND_DESC,
                D3D11_RESOURCE_MISC_FLAG, D3D11_SDK_VERSION, D3D11_STENCIL_OP_REPLACE,
                D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
                D3D11_USAGE_DYNAMIC, D3D11_VIEWPORT,
            },
            Dxgi::{
                Common::{
                    DXGI_ALPHA_MODE_IGNORE,
                    DXGI_FORMAT,
                    DXGI_FORMAT_B8G8R8A8_UNORM,
                    //DXGI_FORMAT_D32_FLOAT_S8X 24_UINT,
                    DXGI_FORMAT_D32_FLOAT,
                    DXGI_FORMAT_R16_FLOAT,
                    DXGI_FORMAT_R32G32B32A32_FLOAT,
                    DXGI_FORMAT_R32G32B32_FLOAT,
                    DXGI_FORMAT_R32G32_FLOAT,
                    DXGI_FORMAT_R32_FLOAT,
                    DXGI_FORMAT_R32_UINT,
                    DXGI_FORMAT_R8G8_UNORM,
                    DXGI_FORMAT_R8_UNORM,
                    DXGI_SAMPLE_DESC,
                },
                CreateDXGIFactory2, IDXGIFactory2, IDXGIResource, IDXGISwapChain1, DXGI_RGBA,
                DXGI_SCALING_NONE, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT_FLIP_DISCARD,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
    },
};

use glam::DVec2;
use window::Window;

#[derive(Clone)]
pub struct D3D11 {
    pub device: ID3D11Device,
    pub context: ID3D11DeviceContext,
    pub query: ID3D11Query,
    pub factory: IDXGIFactory2,
    pub hwnd: isize,
    pub is_in_resize: bool,
    pub render_target_view: Option<ID3D11RenderTargetView>,
    pub swap_texture: Option<ID3D11Texture2D>,
    pub alloc_size: DVec2,
    pub first_draw: bool,
    pub swap_chain: IDXGISwapChain1,
}

impl D3D11 {
    pub fn new(hwnd: isize, width: u32, height: u32) -> D3D11 {
        unsafe {
            let factory: IDXGIFactory2 = CreateDXGIFactory2(0).unwrap();
            let adapter = factory.EnumAdapters(0).unwrap();
            let mut device: Option<ID3D11Device> = None;
            let mut context: Option<ID3D11DeviceContext> = None;
            let mut query: Option<ID3D11Query> = None;
            D3D11CreateDevice(
                &adapter,
                D3D_DRIVER_TYPE_UNKNOWN,
                HINSTANCE(0),
                D3D11_CREATE_DEVICE_FLAG(0),
                Some(&[D3D_FEATURE_LEVEL_11_0]),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )
            .unwrap();

            let device = device.unwrap();
            let context = context.unwrap();

            device
                .CreateQuery(
                    &D3D11_QUERY_DESC {
                        Query: D3D11_QUERY_EVENT,
                        MiscFlags: 0,
                    },
                    Some(&mut query),
                )
                .unwrap();

            let query = query.unwrap();

            let sc_desc = DXGI_SWAP_CHAIN_DESC1 {
                AlphaMode: DXGI_ALPHA_MODE_IGNORE,
                BufferCount: 2,
                Width: width,
                Height: height,
                Format: DXGI_FORMAT_B8G8R8A8_UNORM,
                Flags: 0,
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Scaling: DXGI_SCALING_NONE,
                Stereo: BOOL(0),
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            };

            let swap_chain = factory
                .CreateSwapChainForHwnd(&device, HWND(hwnd), &sc_desc, None, None)
                .unwrap();

            let swap_texture = swap_chain.GetBuffer(0).unwrap();
            let mut render_target_view = None;
            device
                .CreateRenderTargetView(&swap_texture, None, Some(&mut render_target_view))
                .unwrap();
            swap_chain
                .SetBackgroundColor(&mut DXGI_RGBA {
                    r: 0.3,
                    g: 0.3,
                    b: 0.3,
                    a: 1.0,
                })
                .unwrap();

            D3D11 {
                device,
                context,
                factory,
                query,
                first_draw: true,
                is_in_resize: false,
                hwnd,
                // alloc_size: wg.inner_size,
                alloc_size: DVec2::default(),
                swap_texture: Some(swap_texture),
                render_target_view,
                swap_chain,
            }
        }
    }
    pub fn start_querying(&self) {
        // QUERY_EVENT signals when rendering is complete
        unsafe { self.context.End(&self.query) };
    }

    pub fn is_gpu_done(&self) -> bool {
        let hresult = unsafe {
            (Interface::vtable(&self.context).GetData)(
                Interface::as_raw(&self.context),
                Interface::as_raw(&self.query),
                std::ptr::null_mut(),
                0,
                0,
            )
        };
        hresult != S_FALSE
    }

    pub fn clear(&self) {
        unsafe {
            self.context.ClearRenderTargetView(
                self.render_target_view.as_ref().unwrap(),
                &[1.0, 0.6, 0.6, 1.0],
            );
        }
    }
    pub fn start_resize(&mut self) {
        self.is_in_resize = true;
    }

    // switch back to swapchain
    pub fn stop_resize(&mut self) {
        self.is_in_resize = false;
        self.alloc_size = DVec2::default();
    }

    // pub fn resize_buffers(&mut self, d3d11_cx: &D3d11Cx) {
    //     if self.alloc_size == self.window_geom.inner_size {
    //         return;
    //     }
    //     self.alloc_size = self.window_geom.inner_size;
    //     self.swap_texture = None;
    //     self.render_target_view = None;

    //     unsafe {
    //         let wg = &self.window_geom;
    //         self.swap_chain
    //             .ResizeBuffers(
    //                 2,
    //                 (wg.inner_size.x * wg.dpi_factor) as u32,
    //                 (wg.inner_size.y * wg.dpi_factor) as u32,
    //                 DXGI_FORMAT_B8G8R8A8_UNORM,
    //                 0,
    //             )
    //             .unwrap();

    //         let swap_texture = self.swap_chain.GetBuffer(0).unwrap();
    //         let mut render_target_view = None;
    //         d3d11_cx
    //             .device
    //             .CreateRenderTargetView(&swap_texture, None, Some(&mut render_target_view))
    //             .unwrap();

    //         self.swap_texture = Some(swap_texture);
    //         self.render_target_view = render_target_view;
    //     }
    // }

    pub fn present(&mut self, vsync: bool) {
        unsafe {
            self.swap_chain
                .Present(if vsync { 1 } else { 0 }, 0)
                .unwrap()
        };
    }
}
