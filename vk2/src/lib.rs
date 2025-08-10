use std::pin::Pin;

use ash::*;
use mini::*;
use window::*;

pub unsafe fn str_from_i8(slice: &[i8]) -> Result<&str, std::str::Utf8Error> {
    let (end, _) = slice
        .iter()
        .enumerate()
        .find(|(_, c)| **c == b'\0' as i8)
        .unwrap();
    std::str::from_utf8(std::mem::transmute(&slice[..end]))
}

pub unsafe fn create_surface(
    entry: &Entry,
    instance: &Instance,
    width: u32,
    height: u32,
) -> (Pin<Box<window::Window>>, vk::SurfaceKHR) {
    profile!();
    let window = create_window(
        "test window",
        0, 
        0,
        width as i32,
        height as i32,
        WindowStyle::default(),
    );
    let win32_surface_fn = khr::win32_surface::Instance::new(&entry, &instance);
    let surface = win32_surface_fn
        .create_win32_surface(
            &vk::Win32SurfaceCreateInfoKHR::default()
                // .hinstance(window.hinstance as isize)
                .hwnd(window.hwnd as isize),
            None,
        )
        .unwrap();

    (window, surface)
}

///https://vulkan.gpuinfo.org/displayreport.php?id=18463#queuefamilies
pub unsafe fn create_device(instance: &Instance) -> (vk::PhysicalDevice, Device, vk::Queue, u32) {
    profile!();
    let devices = instance.enumerate_physical_devices().unwrap();
    let physical_device = &devices[0];
    let queue = instance.get_physical_device_queue_family_properties(*physical_device);
    let (index, _) = queue
        .iter()
        .enumerate()
        .find(|(_, info)| info.queue_flags.contains(vk::QueueFlags::GRAPHICS))
        .unwrap();

    // let properties = instance.get_physical_device_properties(*physical_device);
    // let name = str_from_i8(&properties.device_name).unwrap();
    // info!("Physical Device: {}", name);

    let device = instance
        .create_device(
            *physical_device,
            &vk::DeviceCreateInfo::default()
                .queue_create_infos(&[vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(index as u32)
                    .queue_priorities(&[1.0])])
                .enabled_extension_names(&[khr::swapchain::NAME.as_ptr()])
                .enabled_features(&vk::PhysicalDeviceFeatures {
                    shader_clip_distance: 1,
                    ..Default::default()
                }),
            None,
        )
        .unwrap();

    let queue = device.get_device_queue(index as u32, 0);

    (*physical_device, device, queue, index as u32)
}

pub const SURFACE_FORMAT: vk::SurfaceFormatKHR = vk::SurfaceFormatKHR {
    format: vk::Format::B8G8R8A8_UNORM,
    color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
};

pub unsafe fn create_swapchain(
    entry: &Entry,
    instance: &Instance,
    surface: &vk::SurfaceKHR,
    physical_device: &vk::PhysicalDevice,
    device: &Device,
    width: u32,
    height: u32,
) -> (
    khr::swapchain::Device,
    vk::SwapchainKHR,
    Vec<vk::Image>,
    Vec<vk::ImageView>,
    khr::surface::Instance,
    vk::SurfaceCapabilitiesKHR,
) {
    profile!();
    let surface_loader = khr::surface::Instance::new(entry, instance);
    let swapchain_loader = khr::swapchain::Device::new(instance, device);

    let surface_capabilities = surface_loader
        .get_physical_device_surface_capabilities(*physical_device, *surface)
        .unwrap();
    let surface_formats = surface_loader
        .get_physical_device_surface_formats(*physical_device, *surface)
        .unwrap();

    if !surface_formats.contains(&SURFACE_FORMAT) {
        panic!(
            "Physical device does not support this format: {:?}",
            SURFACE_FORMAT
        );
    }

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
        .surface(*surface)
        .min_image_count(surface_capabilities.min_image_count + 1)
        .image_color_space(SURFACE_FORMAT.color_space)
        .image_format(SURFACE_FORMAT.format)
        // .image_extent(surface_capabilities.current_extent)
        .image_extent(vk::Extent2D { width, height })
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(vk::PresentModeKHR::FIFO_RELAXED)
        .clipped(true)
        .image_array_layers(1);

    let swapchain = swapchain_loader
        .create_swapchain(&swapchain_create_info, None)
        .unwrap();

    let images = swapchain_loader.get_swapchain_images(swapchain).unwrap();
    let image_views: Vec<vk::ImageView> = images
        .iter()
        .map(|&image| {
            device
                .create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .view_type(vk::ImageViewType::TYPE_2D)
                        .format(SURFACE_FORMAT.format)
                        .components(vk::ComponentMapping {
                            r: vk::ComponentSwizzle::R,
                            g: vk::ComponentSwizzle::G,
                            b: vk::ComponentSwizzle::B,
                            a: vk::ComponentSwizzle::A,
                        })
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        })
                        .image(image),
                    None,
                )
                .unwrap()
        })
        .collect();

    (
        swapchain_loader,
        swapchain,
        images,
        image_views,
        surface_loader,
        surface_capabilities,
    )
}

//TODO: VKGuide has been updated quite a bit, so retrofitting resizing into this shitty code isn't having a great affect.
//Time for vk3 I guess...
pub unsafe fn resize_swapchain(vk: &mut Vulkan) {
    vk.device.device_wait_idle().unwrap();

    vk.swapchain_loader.destroy_swapchain(vk.swapchain, None);

    for image in std::mem::take(&mut vk.swapchain_image_views) {
        vk.device.destroy_image_view(image, None);
    }

    // dbg!(vk.surface_capabilities.current_extent, vk.window.width(), vk.window.height());
    // let (
    //     swapchain_loader,
    //     swapchain,
    //     swapchain_images,
    //     swapchain_image_views,
    //     surface_loader,
    //     surface_capabilities,
    // ) = create_swapchain(
    //     &vk.entry,
    //     &vk.instance,
    //     &vk.surface,
    //     &vk.physical_device,
    //     &vk.device,
    //     vk.surface_capabilities.current_extent.width,
    //     vk.surface_capabilities.current_extent.height
    // );
    let width = vk.surface_capabilities.current_extent.width;
    let height = vk.surface_capabilities.current_extent.height;

    // vk.swapchain_loader = swapchain_loader;
    // vk.swapchain = swapchain;
    // vk.swapchain_images = swapchain_images;
    // vk.swapchain_image_views = swapchain_image_views;
    // vk.surface_loader = surface_loader;
    // vk.surface_capabilities = surface_capabilities;

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
        .surface(vk.surface)
        .min_image_count(vk.surface_capabilities.min_image_count + 1)
        .image_color_space(SURFACE_FORMAT.color_space)
        .image_format(SURFACE_FORMAT.format)
        // .image_extent(surface_capabilities.current_extent)
        .image_extent(vk::Extent2D { width, height })
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(vk::SurfaceTransformFlagsKHR::IDENTITY)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(vk::PresentModeKHR::FIFO_RELAXED)
        .clipped(true)
        .image_array_layers(1);

    // let swapchain_loader = khr::swapchain::Device::new(&vk.instance, &vk.device);
    // let swapchain = swapchain_loader
    //     .create_swapchain(&swapchain_create_info, None)
    //     .unwrap();

    let swapchain = vk
        .swapchain_loader
        .create_swapchain(&swapchain_create_info, None)
        .unwrap();
    vk.swapchain_images = vk.swapchain_loader.get_swapchain_images(swapchain).unwrap();
    let image_views: Vec<vk::ImageView> = vk
        .swapchain_images
        .iter()
        .map(|&image| {
            vk.device
                .create_image_view(
                    &vk::ImageViewCreateInfo::default()
                        .view_type(vk::ImageViewType::TYPE_2D)
                        .format(SURFACE_FORMAT.format)
                        .components(vk::ComponentMapping {
                            r: vk::ComponentSwizzle::R,
                            g: vk::ComponentSwizzle::G,
                            b: vk::ComponentSwizzle::B,
                            a: vk::ComponentSwizzle::A,
                        })
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        })
                        .image(image),
                    None,
                )
                .unwrap()
        })
        .collect();

    vk.swapchain_image_views = image_views;
    vk.swapchain = swapchain;

    vk.resize_requested = false;
}

pub unsafe fn create_commands(device: &Device, index: u32) -> (vk::CommandPool, vk::CommandBuffer) {
    profile!();
    let pool = device
        .create_command_pool(
            &vk::CommandPoolCreateInfo::default()
                .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
                .queue_family_index(index),
            None,
        )
        .unwrap();

    let command_buffers = device
        .allocate_command_buffers(
            &vk::CommandBufferAllocateInfo::default()
                .command_buffer_count(1)
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY),
        )
        .unwrap();
    assert_eq!(command_buffers.len(), 1);

    (pool, command_buffers[0])
}

pub enum ShaderType {
    Vertex,
    Fragment,
}

///https://vkguide.dev/docs/chapter-2/toggling_shaders/
pub unsafe fn create_shader(device: &Device, bytes: &[u8], shader_type: vk::ShaderStageFlags) {
    profile!();
    const MAIN: *const i8 = b"main\0" as *const u8 as *const i8;
    let (_, code, _) = unsafe { bytes.align_to::<u32>() };
    let shader_info = vk::ShaderModuleCreateInfo::default().code(&code);
    let shader_module = device.create_shader_module(&shader_info, None).unwrap();
    let _shader = vk::PipelineShaderStageCreateInfo {
        module: shader_module,
        p_name: MAIN,
        stage: shader_type,
        ..Default::default()
    };
}

pub unsafe fn create_render_pass(
    device: &Device,
    image_views: &[vk::ImageView],
    surface_capabilities: vk::SurfaceCapabilitiesKHR,
) -> (vk::RenderPass, Vec<vk::Framebuffer>) {
    profile!();
    let render_pass = device
        .create_render_pass(
            &vk::RenderPassCreateInfo::default()
                .attachments(&[vk::AttachmentDescription {
                    format: SURFACE_FORMAT.format,
                    samples: vk::SampleCountFlags::TYPE_1,
                    load_op: vk::AttachmentLoadOp::CLEAR,
                    store_op: vk::AttachmentStoreOp::STORE,
                    final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                    ..Default::default()
                }])
                .subpasses(&[vk::SubpassDescription::default()
                    .color_attachments(&[vk::AttachmentReference {
                        attachment: 0,
                        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                    }])
                    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)]),
            None,
        )
        .unwrap();

    let framebuffers: Vec<vk::Framebuffer> = image_views
        .iter()
        .map(|image_view| {
            let attachments = &[*image_view];
            device
                .create_framebuffer(
                    &vk::FramebufferCreateInfo::default()
                        .render_pass(render_pass)
                        .attachments(attachments)
                        .width(surface_capabilities.current_extent.width)
                        .height(surface_capabilities.current_extent.height)
                        .layers(1),
                    None,
                )
                .unwrap()
        })
        .collect();

    (render_pass, framebuffers)
}

pub unsafe fn create_sync(device: &Device) -> (vk::Fence, vk::Semaphore, vk::Semaphore) {
    profile!();
    let render_fence = device
        .create_fence(
            &vk::FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED),
            None,
        )
        .unwrap();
    let present_semaphore = device
        .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
        .unwrap();
    let render_semaphore = device
        .create_semaphore(&vk::SemaphoreCreateInfo::default(), None)
        .unwrap();

    (render_fence, present_semaphore, render_semaphore)
}

pub unsafe fn draw(vk: &mut Vulkan, frame_number: &mut f32) {
    profile!();
    const ONE_SECOND: u64 = 1000000000;

    vk.device
        .wait_for_fences(&[vk.render_fence], true, ONE_SECOND)
        .unwrap();
    vk.device.reset_fences(&[vk.render_fence]).unwrap();

    let (index, is_suboptimal) = match vk.swapchain_loader.acquire_next_image(
        vk.swapchain,
        ONE_SECOND,
        vk.present_semaphore,
        vk::Fence::null(),
    ) {
        Ok((a, b)) => (a, b),
        Err(err) => {
            if err == vk::Result::ERROR_OUT_OF_DATE_KHR {
                vk.resize_requested = true;
            }

            return;
        }
    };

    assert_eq!(is_suboptimal, false);
    vk.device
        .reset_command_buffer(vk.command_buffer, vk::CommandBufferResetFlags::empty())
        .unwrap();

    vk.device
        .begin_command_buffer(
            vk.command_buffer,
            &vk::CommandBufferBeginInfo::default()
                .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
        )
        .unwrap();

    let flash = f32::abs(f32::sin(*frame_number / 120.0));
    let clear_value = vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0.0, 0.0, flash, 0.0],
        },
    };

    vk.device.cmd_begin_render_pass(
        vk.command_buffer,
        &vk::RenderPassBeginInfo::default()
            .render_pass(vk.render_pass)
            .clear_values(&[clear_value])
            .framebuffer(vk.framebuffers[index as usize])
            .render_area(vk::Rect2D {
                extent: vk.surface_capabilities.current_extent,
                offset: vk::Offset2D { x: 0, y: 0 },
            }),
        vk::SubpassContents::INLINE,
    );

    vk.device.cmd_end_render_pass(vk.command_buffer);
    vk.device.end_command_buffer(vk.command_buffer).unwrap();

    vk.device
        .queue_submit(
            vk.queue,
            &[vk::SubmitInfo::default()
                .command_buffers(&[vk.command_buffer])
                .wait_semaphores(&[vk.present_semaphore])
                .signal_semaphores(&[vk.render_semaphore])
                .wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])],
            vk.render_fence,
        )
        .unwrap();

    let err = vk.swapchain_loader.queue_present(
        vk.queue,
        &vk::PresentInfoKHR::default()
            .swapchains(&[vk.swapchain])
            .wait_semaphores(&[vk.render_semaphore])
            .image_indices(&[index]),
    );

    if let Err(vk::Result::ERROR_OUT_OF_DATE_KHR) = err {
        vk.resize_requested = true;
    }

    *frame_number += 1.0;
}

pub struct Vulkan {
    pub entry: Entry,
    pub instance: Instance,
    pub window: Pin<Box<window::Window>>,
    pub resize_requested: bool,
    pub surface: vk::SurfaceKHR,
    pub surface_capabilities: vk::SurfaceCapabilitiesKHR,
    pub device: Device,
    pub physical_device: vk::PhysicalDevice,
    pub queue: vk::Queue,
    pub queue_index: u32,
    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
    pub render_pass: vk::RenderPass,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub render_fence: vk::Fence,
    pub present_semaphore: vk::Semaphore,
    pub render_semaphore: vk::Semaphore,
    pub debug: Option<vk::DebugUtilsMessengerEXT>,

    pub debug_loader: ext::debug_utils::Instance,
    pub surface_loader: khr::surface::Instance,
    pub swapchain_loader: khr::swapchain::Device,
}

impl Vulkan {
    pub fn new(width: u32, height: u32, debug: bool) -> Self {
        profile!();
        const LAYERS: [*const i8; 1] = [b"VK_LAYER_KHRONOS_validation\0".as_ptr() as *const i8];
        const EXTENSIONS: [*const i8; 2] = [
            khr::surface::NAME.as_ptr(),
            khr::win32_surface::NAME.as_ptr(),
        ];
        const DEBUG_EXTENSIONS: [*const i8; 3] = [
            khr::surface::NAME.as_ptr(),
            khr::win32_surface::NAME.as_ptr(),
            ext::debug_utils::NAME.as_ptr(),
        ];

        unsafe {
            let entry = ash::Entry::linked();
            // let entry = ash::Entry::load().unwrap();
            let instance = entry
                .create_instance(
                    &vk::InstanceCreateInfo::default()
                        .enabled_layer_names(if debug { &LAYERS } else { &[] })
                        .enabled_extension_names(if debug {
                            &DEBUG_EXTENSIONS
                        } else {
                            &EXTENSIONS
                        }),
                    None,
                )
                .unwrap();

            let debug_loader = ext::debug_utils::Instance::new(&entry, &instance);
            let debug = if debug {
                Some(enable_debugging(&debug_loader))
            } else {
                None
            };

            let (window, surface) = create_surface(&entry, &instance, width, height);
            let (physical_device, device, queue, queue_index) = create_device(&instance);
            let (
                swapchain_loader,
                swapchain,
                swapchain_images,
                swapchain_image_views,
                surface_loader,
                surface_capabilities,
            ) = create_swapchain(
                &entry,
                &instance,
                &surface,
                &physical_device,
                &device,
                window.width() as u32,
                window.height() as u32,
            );
            let (command_pool, command_buffer) = create_commands(&device, queue_index);
            let (render_pass, framebuffers) =
                create_render_pass(&device, &swapchain_image_views, surface_capabilities);
            let (fence, present_semaphore, render_semaphore) = create_sync(&device);

            Vulkan {
                entry,
                instance,
                window,
                resize_requested: false,
                surface,
                surface_capabilities,
                physical_device,
                device,
                queue,
                queue_index,
                command_pool,
                command_buffer,
                swapchain,
                render_pass,
                framebuffers,
                swapchain_images,
                swapchain_image_views,
                render_fence: fence,
                present_semaphore,
                render_semaphore,
                debug,
                debug_loader,
                surface_loader,
                swapchain_loader,
            }
        }
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            self.device.queue_wait_idle(self.queue).unwrap(); //Wait for everything to finish.
            self.device.destroy_command_pool(self.command_pool, None); //Free's all command buffers.
            self.device.destroy_fence(self.render_fence, None);
            self.device.destroy_semaphore(self.render_semaphore, None);
            self.device.destroy_semaphore(self.present_semaphore, None);

            for framebuffer in std::mem::take(&mut self.framebuffers) {
                self.device.destroy_framebuffer(framebuffer, None)
            }

            self.device.destroy_render_pass(self.render_pass, None);
            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);

            for image in std::mem::take(&mut self.swapchain_image_views) {
                self.device.destroy_image_view(image, None);
            }

            if let Some(debug) = self.debug {
                self.debug_loader.destroy_debug_utils_messenger(debug, None);
            }

            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);

            self.instance.destroy_instance(None);
        }
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    use std::borrow::Cow;
    use std::ffi::CStr;

    let callback_data = *p_callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    let _message = format!(
        "{message_type:?} [{message_id_name} ({message_id_number})]: {}",
        message.trim_start()
    );

    match message_severity {
        vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => {
            info!("{_message}");
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            info!("{_message}");
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            warn!("{_message}");
        }
        vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            error!("{_message}");
        }
        _ => unreachable!(),
    };

    vk::FALSE
}

pub unsafe fn enable_debugging(
    debug_loader: &ext::debug_utils::Instance,
) -> vk::DebugUtilsMessengerEXT {
    let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
        .message_severity(
            vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        )
        .message_type(
            vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        )
        .pfn_user_callback(Some(vulkan_debug_callback));

    debug_loader
        .create_debug_utils_messenger(&debug_info, None)
        .unwrap()
}
