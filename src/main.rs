use std::borrow::Cow;

use wgpu::{
    Backends, DeviceDescriptor, Features, FragmentState, Instance, Limits, MultisampleState,
    PipelineLayout, PipelineLayoutDescriptor, PowerPreference, PresentMode, PrimitiveState,
    RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource,
    SurfaceConfiguration, TextureUsages,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

async fn run(event_loop: EventLoop<()>, window: Window) {
    //the width and height
    let size = window.inner_size();

    //Context for all other wgpu objects. Instance of wgpu.
    let instance = Instance::new(Backends::all());

    //Something to render to
    let surface = unsafe { instance.create_surface(&window) };

    let adapter_options = RequestAdapterOptions {
        power_preference: PowerPreference::HighPerformance,
        //Request an adapter that can render to the surface
        compatible_surface: Some(&surface),
    };

    //Handle to a physical graphics and/or compute device
    let adapter = instance
        .request_adapter(&adapter_options)
        .await
        .expect("no adapters bruh?");

    //Options for the device
    let device_descriptor = DeviceDescriptor {
        label: None,
        features: Features::empty(),
        limits: Limits::downlevel_defaults().using_resolution(adapter.limits()),
    };

    //Requests a connection to a physical device, creating a logical device.
    //The queue handles writting to buffers and textures
    let (device, queue) = adapter
        .request_device(&device_descriptor, None)
        .await
        .expect("couldn't create device and queue");

    //Options for the shader
    let shader_descriptor = ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    };

    let shader = device.create_shader_module(&shader_descriptor);

    let pipeline_layout_descriptor = PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    };

    //A `PipelineLayout` object describes the available binding groups of a pipeline
    let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

    //Gets the optimal texture format
    let swapchain_format = surface
        .get_preferred_format(&adapter)
        .expect("failed to get swapchain format");

    // A `RenderPipeline` object represents a graphics pipeline and its stages, bindings, vertex
    // buffers and targets.
    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[swapchain_format.into()],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multisample: MultisampleState::default(),
    });

    let mut config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: PresentMode::Mailbox,
    };

    surface.configure(&device, &config);

    event_loop.run(move |event, _, control_flow| {
        // Have the closure take ownership of the resources.
        // `event_loop.run` never returns, therefore we must do this to ensure
        // the resources are properly cleaned up.
        let _ = (&instance, &adapter, &shader, &pipeline_layout);

        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                // Reconfigure the surface with the new size
                config.width = size.width;
                config.height = size.height;
                surface.configure(&device, &config);
            }
            Event::RedrawRequested(_) => {
                let frame = surface
                    .get_current_frame()
                    .expect("Failed to acquire next swap chain texture")
                    .output;
                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &[wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                                store: true,
                            },
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
fn main() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();

    env_logger::init();

    pollster::block_on(run(event_loop, window));
}
