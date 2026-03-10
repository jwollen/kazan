#![warn(
    clippy::use_self,
    deprecated_in_future,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_qualifications
)]

use std::{
    borrow::Cow, cell::RefCell, default::Default, error::Error, ffi, ops::Drop, os::raw::c_char,
};

pub use kazan::read_spv;
use kazan::{
    Entry, LoadDeviceFn as _, LoadInstanceFn as _,
    vk::{
        self,
        ext::debug_utils,
        khr::{surface, swapchain},
        vk1_0,
    },
};
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    platform::run_on_demand::EventLoopExtRunOnDemand,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::WindowBuilder,
};

// The maximum number of frames we allow to be in flight at any given time
pub const MAX_FRAME_LATENCY: usize = 3;

// Simple offset_of macro akin to C++ offsetof
#[macro_export]
macro_rules! offset_of {
    ($base:path, $field:ident) => {{
        #[allow(unused_unsafe)]
        unsafe {
            let b: $base = mem::zeroed();
            std::ptr::addr_of!(b.$field) as isize - std::ptr::addr_of!(b) as isize
        }
    }};
}

#[allow(clippy::too_many_arguments)]
pub fn record_submit_commandbuffer<F: FnOnce(&vk1_0::DeviceFn, vk::CommandBuffer)>(
    device_fn: &vk1_0::DeviceFn,
    _device: vk::Device,
    command_buffer: vk::CommandBuffer,
    command_buffer_reuse_fence: vk::Fence,
    submit_queue: vk::Queue,
    wait_mask: &[vk::PipelineStageFlags],
    wait_semaphores: &[vk::Semaphore],
    signal_semaphores: &[vk::Semaphore],
    f: F,
) {
    unsafe {
        device_fn
            .reset_command_buffer(
                command_buffer,
                vk::CommandBufferResetFlagBits::RELEASE_RESOURCES.into(),
            )
            .expect("Reset command buffer failed.");

        let command_buffer_begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlagBits::ONE_TIME_SUBMIT.into());

        device_fn
            .begin_command_buffer(command_buffer, &command_buffer_begin_info)
            .expect("Begin commandbuffer");
        f(device_fn, command_buffer);
        device_fn
            .end_command_buffer(command_buffer)
            .expect("End commandbuffer");

        let command_buffers = vec![command_buffer];

        let submit_info = vk::SubmitInfo::default()
            .wait_semaphores(wait_semaphores, wait_mask)
            .command_buffers(&command_buffers)
            .signal_semaphores(signal_semaphores);

        device_fn
            .queue_submit(submit_queue, &[submit_info], command_buffer_reuse_fence)
            .expect("queue submit failed.");
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagBitsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = unsafe { *p_callback_data };
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        unsafe { ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy() }
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        unsafe { ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy() }
    };

    println!(
        "{message_severity:?}:\n{message_type:?} [{message_id_name} ({message_id_number})] : {message}\n",
    );

    vk::FALSE
}

pub fn find_memorytype_index(
    memory_req: &vk::MemoryRequirements,
    memory_prop: &vk::PhysicalDeviceMemoryProperties,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    memory_prop.memory_types[..memory_prop.memory_type_count as _]
        .iter()
        .enumerate()
        .find(|(index, memory_type)| {
            (1 << index) & memory_req.memory_type_bits != 0
                && memory_type.property_flags & flags == flags
        })
        .map(|(index, _memory_type)| index as _)
}

pub struct ExampleBase {
    pub entry: Entry,
    pub instance: vk::Instance,
    pub instance_fn: vk1_0::InstanceFn,
    pub device: vk::Device,
    pub device_fn: vk1_0::DeviceFn,
    pub surface_fn: surface::InstanceFn,
    pub swapchain_fn: swapchain::DeviceFn,
    pub debug_utils_fn: debug_utils::InstanceFn,
    pub window: winit::window::Window,
    pub event_loop: RefCell<EventLoop<()>>,
    pub frame_index: RefCell<usize>,
    pub debug_call_back: vk::DebugUtilsMessengerEXT,

    pub pdevice: vk::PhysicalDevice,
    pub device_memory_properties: vk::PhysicalDeviceMemoryProperties,
    pub queue_family_index: u32,
    pub present_queue: vk::Queue,

    pub surface: vk::SurfaceKHR,
    pub surface_format: vk::SurfaceFormatKHR,
    pub surface_resolution: vk::Extent2D,

    pub swapchain: vk::SwapchainKHR,
    pub present_images: Vec<vk::Image>,
    pub present_image_views: Vec<vk::ImageView>,

    pub pool: vk::CommandPool,
    pub draw_command_buffers: [vk::CommandBuffer; MAX_FRAME_LATENCY],
    pub setup_command_buffer: vk::CommandBuffer,
    pub app_setup_command_buffer: vk::CommandBuffer,

    pub depth_image: vk::Image,
    pub depth_image_view: vk::ImageView,
    pub depth_image_memory: vk::DeviceMemory,

    pub present_complete_semaphores: [vk::Semaphore; MAX_FRAME_LATENCY],
    pub rendering_complete_semaphores: Vec<vk::Semaphore>,

    pub draw_commands_reuse_fences: [vk::Fence; MAX_FRAME_LATENCY],
}

impl ExampleBase {
    pub fn render_loop<F: Fn(usize)>(&self, f: F) -> Result<(), impl Error> {
        self.event_loop.borrow_mut().run_on_demand(|event, elwp| {
            elwp.set_control_flow(ControlFlow::Poll);
            match event {
                Event::WindowEvent {
                    event:
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    logical_key: Key::Named(NamedKey::Escape),
                                    ..
                                },
                            ..
                        },
                    ..
                } => {
                    elwp.exit();
                }
                Event::AboutToWait => {
                    let mut frame_index = self.frame_index.borrow_mut();

                    // The fence from 3 frames ago, that will also be signaled this frame
                    let draw_commands_reuse_fence =
                        self.draw_commands_reuse_fences[*frame_index % MAX_FRAME_LATENCY];
                    unsafe {
                        self.device_fn.wait_for_fences(
                            self.device,
                            &[draw_commands_reuse_fence],
                            true,
                            u64::MAX,
                        )
                    }
                    .expect("Wait for fence failed.");

                    unsafe {
                        self.device_fn
                            .reset_fences(self.device, &[draw_commands_reuse_fence])
                    }
                    .expect("Reset fences failed.");

                    f(*frame_index);
                    *frame_index += 1;
                }
                _ => (),
            }
        })
    }

    pub fn new(window_width: u32, window_height: u32) -> Result<Self, Box<dyn Error>> {
        unsafe {
            let event_loop = EventLoop::new()?;
            let window = WindowBuilder::new()
                .with_title("Kazan - Example")
                .with_inner_size(winit::dpi::LogicalSize::new(
                    f64::from(window_width),
                    f64::from(window_height),
                ))
                .build(&event_loop)
                .unwrap();
            let entry = Entry::linked()?;

            let app_name = c"VulkanTriangle";

            let layer_names = [c"VK_LAYER_KHRONOS_validation"];
            let layers_names_raw: Vec<*const c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let display_handle = window.display_handle()?.as_raw();
            let mut extension_names: Vec<*const c_char> =
                kazan::window::enumerate_required_extensions(display_handle)?.to_vec();
            extension_names.push(debug_utils::EXTENSION_NAME.as_ptr());

            #[cfg(any(target_os = "macos", target_os = "ios"))]
            {
                extension_names.push(vk::khr::portability_enumeration::EXTENSION_NAME.as_ptr());
                // Enabling this extension is a requirement when using `VK_KHR_portability_subset`
                extension_names
                    .push(vk::khr::get_physical_device_properties2::EXTENSION_NAME.as_ptr());
            }

            let appinfo = vk::ApplicationInfo::default()
                .application_name(app_name)
                .application_version(0)
                .engine_name(app_name)
                .engine_version(0)
                .api_version(vk1_0::API_VERSION);

            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                vk::InstanceCreateFlagBits::ENUMERATE_PORTABILITY_KHR.into()
            } else {
                vk::InstanceCreateFlags::default()
            };

            let create_info = vk::InstanceCreateInfo::default()
                .application_info(&appinfo)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&extension_names)
                .flags(create_flags);

            let instance = entry
                .vk1_0
                .create_instance(&create_info, None)
                .expect("Instance creation error");

            let instance_fn = vk1_0::InstanceFn::load(&entry, instance).unwrap();

            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagBitsEXT::ERROR_EXT
                        | vk::DebugUtilsMessageSeverityFlagBitsEXT::WARNING_EXT
                        | vk::DebugUtilsMessageSeverityFlagBitsEXT::INFO_EXT,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagBitsEXT::GENERAL_EXT
                        | vk::DebugUtilsMessageTypeFlagBitsEXT::VALIDATION_EXT
                        | vk::DebugUtilsMessageTypeFlagBitsEXT::PERFORMANCE_EXT,
                )
                .pfn_user_callback(vulkan_debug_callback);

            let debug_utils_fn = debug_utils::InstanceFn::load(&entry, instance).unwrap();
            let debug_call_back = debug_utils_fn
                .create_debug_utils_messenger_ext(instance, &debug_info, None)
                .unwrap();

            // Create surface
            let surface_fn = surface::InstanceFn::load(&entry, instance).unwrap();

            let surface_factory =
                kazan::window::SurfaceFactory::new(&entry, instance, display_handle)?;

            let window_handle = window.window_handle()?.as_raw();
            let surface = surface_factory.create_surface(instance, window_handle, None)?;

            let mut pdevices = Vec::new();
            instance_fn
                .enumerate_physical_devices(instance, &mut pdevices)
                .expect("Physical device error");
            let (pdevice, queue_family_index) = pdevices
                .iter()
                .find_map(|pdevice| {
                    let mut queue_family_props = Vec::new();
                    instance_fn.get_physical_device_queue_family_properties(
                        *pdevice,
                        &mut queue_family_props,
                    );
                    queue_family_props
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            let supports_graphic_and_surface =
                                info.queue_flags.contains_bit(vk::QueueFlagBits::GRAPHICS)
                                    && surface_fn
                                        .get_physical_device_surface_support_khr(
                                            *pdevice,
                                            index as u32,
                                            surface,
                                        )
                                        .unwrap();
                            if supports_graphic_and_surface {
                                Some((*pdevice, index))
                            } else {
                                None
                            }
                        })
                })
                .expect("Couldn't find suitable device.");
            let queue_family_index = queue_family_index as u32;
            let device_extension_names_raw = [
                swapchain::EXTENSION_NAME.as_ptr(),
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                vk::khr::portability_subset::EXTENSION_NAME.as_ptr(),
            ];
            let features = vk::PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
            };
            let priorities = [1.0];

            let queue_info = vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family_index)
                .queue_priorities(&priorities);

            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(std::slice::from_ref(&queue_info))
                .enabled_extension_names(&device_extension_names_raw)
                .enabled_features(&features);

            let device = instance_fn
                .create_device(pdevice, &device_create_info, None)
                .unwrap();

            let device_fn = vk1_0::DeviceFn::load(&instance_fn, device).unwrap();

            let present_queue = device_fn.get_device_queue(device, queue_family_index, 0);

            let mut surface_formats = Vec::new();
            surface_fn
                .get_physical_device_surface_formats_khr(pdevice, surface, &mut surface_formats)
                .unwrap();
            let surface_format = surface_formats[0];

            let surface_capabilities = surface_fn
                .get_physical_device_surface_capabilities_khr(pdevice, surface)
                .unwrap();
            let mut desired_image_count = surface_capabilities.min_image_count + 1;
            if surface_capabilities.max_image_count > 0
                && desired_image_count > surface_capabilities.max_image_count
            {
                desired_image_count = surface_capabilities.max_image_count;
            }
            let surface_resolution = match surface_capabilities.current_extent.width {
                u32::MAX => vk::Extent2D {
                    width: window_width,
                    height: window_height,
                },
                _ => surface_capabilities.current_extent,
            };
            let pre_transform = if surface_capabilities
                .supported_transforms
                .contains_bit(vk::SurfaceTransformFlagBitsKHR::IDENTITY_KHR)
            {
                vk::SurfaceTransformFlagBitsKHR::IDENTITY_KHR
            } else {
                surface_capabilities.current_transform
            };
            let mut present_modes = Vec::new();
            surface_fn
                .get_physical_device_surface_present_modes_khr(pdevice, surface, &mut present_modes)
                .unwrap();
            let present_mode = present_modes
                .iter()
                .cloned()
                .find(|&mode| mode == vk::PresentModeKHR::MAILBOX_KHR)
                .unwrap_or(vk::PresentModeKHR::FIFO_KHR);
            let swapchain_fn = swapchain::DeviceFn::load(&instance_fn, device).unwrap();

            let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
                .surface(surface)
                .min_image_count(desired_image_count)
                .image_color_space(surface_format.color_space)
                .image_format(surface_format.format)
                .image_extent(surface_resolution)
                .image_usage(vk::ImageUsageFlagBits::COLOR_ATTACHMENT.into())
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .pre_transform(pre_transform)
                .composite_alpha(vk::CompositeAlphaFlagBitsKHR::OPAQUE_KHR)
                .present_mode(present_mode)
                .clipped(true)
                .image_array_layers(1);

            let swapchain = swapchain_fn
                .create_swapchain_khr(device, &swapchain_create_info, None)
                .unwrap();

            let pool_create_info = vk::CommandPoolCreateInfo::default()
                .flags(vk::CommandPoolCreateFlagBits::RESET_COMMAND_BUFFER.into())
                .queue_family_index(queue_family_index);

            let pool = device_fn
                .create_command_pool(device, &pool_create_info, None)
                .unwrap();

            let command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
                .command_buffer_count(2 + MAX_FRAME_LATENCY as u32)
                .command_pool(pool)
                .level(vk::CommandBufferLevel::PRIMARY);

            let mut command_buffers = vec![vk::CommandBuffer::null(); 2 + MAX_FRAME_LATENCY];
            device_fn
                .allocate_command_buffers(
                    device,
                    &command_buffer_allocate_info,
                    &mut command_buffers,
                )
                .unwrap();
            let setup_command_buffer = command_buffers[0];
            let app_setup_command_buffer = command_buffers[1];
            let draw_command_buffers = command_buffers[2..][..MAX_FRAME_LATENCY]
                .try_into()
                .unwrap();

            let mut present_images = Vec::new();
            swapchain_fn
                .get_swapchain_images_khr(device, swapchain, &mut present_images)
                .unwrap();
            let present_image_views: Vec<vk::ImageView> = present_images
                .iter()
                .map(|&image| {
                    let create_view_info = vk::ImageViewCreateInfo::default()
                        .view_type(vk::ImageViewType::_2D)
                        .format(surface_format.format)
                        .components(vk::ComponentMapping {
                            r: vk::ComponentSwizzle::R,
                            g: vk::ComponentSwizzle::G,
                            b: vk::ComponentSwizzle::B,
                            a: vk::ComponentSwizzle::A,
                        })
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlagBits::COLOR.into(),
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        })
                        .image(image);
                    device_fn
                        .create_image_view(device, &create_view_info, None)
                        .unwrap()
                })
                .collect();
            let device_memory_properties =
                instance_fn.get_physical_device_memory_properties(pdevice);
            let depth_image_create_info = vk::ImageCreateInfo::default()
                .image_type(vk::ImageType::_2D)
                .format(vk::Format::D16_UNORM)
                .extent(surface_resolution.into())
                .mip_levels(1)
                .array_layers(1)
                .samples(vk::SampleCountFlagBits::_1)
                .tiling(vk::ImageTiling::OPTIMAL)
                .usage(vk::ImageUsageFlagBits::DEPTH_STENCIL_ATTACHMENT.into())
                .sharing_mode(vk::SharingMode::EXCLUSIVE);

            let depth_image = device_fn
                .create_image(device, &depth_image_create_info, None)
                .unwrap();
            let depth_image_memory_req =
                device_fn.get_image_memory_requirements(device, depth_image);
            let depth_image_memory_index = find_memorytype_index(
                &depth_image_memory_req,
                &device_memory_properties,
                vk::MemoryPropertyFlagBits::DEVICE_LOCAL.into(),
            )
            .expect("Unable to find suitable memory index for depth image.");

            let depth_image_allocate_info = vk::MemoryAllocateInfo::default()
                .allocation_size(depth_image_memory_req.size)
                .memory_type_index(depth_image_memory_index);

            let depth_image_memory = device_fn
                .allocate_memory(device, &depth_image_allocate_info, None)
                .unwrap();

            device_fn
                .bind_image_memory(device, depth_image, depth_image_memory, 0)
                .expect("Unable to bind depth image memory");

            record_submit_commandbuffer(
                &device_fn,
                device,
                setup_command_buffer,
                vk::Fence::null(),
                present_queue,
                &[],
                &[],
                &[],
                |device_fn, setup_command_buffer| {
                    let layout_transition_barriers = vk::ImageMemoryBarrier::default()
                        .image(depth_image)
                        .dst_access_mask(
                            vk::AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_READ
                                | vk::AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_WRITE,
                        )
                        .new_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
                        .old_layout(vk::ImageLayout::UNDEFINED)
                        .subresource_range(
                            vk::ImageSubresourceRange::default()
                                .aspect_mask(vk::ImageAspectFlagBits::DEPTH.into())
                                .layer_count(1)
                                .level_count(1),
                        );

                    device_fn.cmd_pipeline_barrier(
                        setup_command_buffer,
                        vk::PipelineStageFlagBits::BOTTOM_OF_PIPE.into(),
                        vk::PipelineStageFlagBits::LATE_FRAGMENT_TESTS.into(),
                        vk::DependencyFlags::empty(),
                        &[],
                        &[],
                        &[layout_transition_barriers],
                    );
                },
            );

            let depth_image_view_info = vk::ImageViewCreateInfo::default()
                .subresource_range(
                    vk::ImageSubresourceRange::default()
                        .aspect_mask(vk::ImageAspectFlagBits::DEPTH.into())
                        .level_count(1)
                        .layer_count(1),
                )
                .image(depth_image)
                .format(depth_image_create_info.format)
                .view_type(vk::ImageViewType::_2D);

            let depth_image_view = device_fn
                .create_image_view(device, &depth_image_view_info, None)
                .unwrap();

            let semaphore_create_info = vk::SemaphoreCreateInfo::default();

            let present_complete_semaphores = std::array::from_fn(|_| {
                device_fn
                    .create_semaphore(device, &semaphore_create_info, None)
                    .unwrap()
            });
            let rendering_complete_semaphores = (0..present_images.len())
                .map(|_| {
                    device_fn
                        .create_semaphore(device, &semaphore_create_info, None)
                        .unwrap()
                })
                .collect();

            let fence_create_info =
                vk::FenceCreateInfo::default().flags(vk::FenceCreateFlagBits::SIGNALED.into());

            let draw_commands_reuse_fences = std::array::from_fn(|_| {
                device_fn
                    .create_fence(device, &fence_create_info, None)
                    .expect("Create fence failed.")
            });

            Ok(Self {
                event_loop: RefCell::new(event_loop),
                frame_index: RefCell::new(0),
                entry,
                instance,
                instance_fn,
                device,
                device_fn,
                queue_family_index,
                pdevice,
                device_memory_properties,
                window,
                surface_fn,
                swapchain_fn,
                debug_utils_fn,
                surface_format,
                present_queue,
                surface_resolution,
                swapchain,
                present_images,
                present_image_views,
                pool,
                draw_command_buffers,
                setup_command_buffer,
                app_setup_command_buffer,
                depth_image,
                depth_image_view,
                present_complete_semaphores,
                rendering_complete_semaphores,
                draw_commands_reuse_fences,
                surface,
                debug_call_back,
                depth_image_memory,
            })
        }
    }
}

impl Drop for ExampleBase {
    fn drop(&mut self) {
        unsafe {
            self.device_fn.device_wait_idle(self.device).unwrap();
            for &semaphore in &self.present_complete_semaphores {
                self.device_fn
                    .destroy_semaphore(self.device, semaphore, None);
            }
            for &semaphore in &self.rendering_complete_semaphores {
                self.device_fn
                    .destroy_semaphore(self.device, semaphore, None);
            }
            for &fence in &self.draw_commands_reuse_fences {
                self.device_fn.destroy_fence(self.device, fence, None);
            }
            self.device_fn
                .free_memory(self.device, self.depth_image_memory, None);
            self.device_fn
                .destroy_image_view(self.device, self.depth_image_view, None);
            self.device_fn
                .destroy_image(self.device, self.depth_image, None);
            for &image_view in &self.present_image_views {
                self.device_fn
                    .destroy_image_view(self.device, image_view, None);
            }
            self.device_fn
                .destroy_command_pool(self.device, self.pool, None);
            self.swapchain_fn
                .destroy_swapchain_khr(self.device, self.swapchain, None);
            self.device_fn.destroy_device(self.device, None);
            self.surface_fn
                .destroy_surface_khr(self.instance, self.surface, None);
            self.debug_utils_fn.destroy_debug_utils_messenger_ext(
                self.instance,
                self.debug_call_back,
                None,
            );
            self.instance_fn.destroy_instance(self.instance, None);
        }
    }
}
