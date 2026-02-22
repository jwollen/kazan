#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct EntryFn {
    create_instance: PFN_vkCreateInstance,
    enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
    enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
}
impl EntryFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_instance: transmute(load(c"vkCreateInstance").ok_or(LoadingError)?),
                enumerate_instance_extension_properties: transmute(
                    load(c"vkEnumerateInstanceExtensionProperties").ok_or(LoadingError)?,
                ),
                enumerate_instance_layer_properties: transmute(
                    load(c"vkEnumerateInstanceLayerProperties").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl EntryFn {
    pub unsafe fn create_instance(
        &self,
        create_info: &InstanceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Instance> {
        unsafe {
            let mut instance = core::mem::MaybeUninit::uninit();
            let result =
                (self.create_instance)(create_info, allocator.to_raw_ptr(), instance.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(instance.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&CStr>,
        properties: impl ExtendUninit<ExtensionProperties>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.enumerate_instance_extension_properties)(
                    layer_name.to_raw_ptr(),
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        properties: impl ExtendUninit<LayerProperties>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result =
                    (self.enumerate_instance_layer_properties)(property_count, properties as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
}
pub struct InstanceFn {
    destroy_instance: PFN_vkDestroyInstance,
    enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures,
    get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties,
    get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties,
    get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
    get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
    get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    create_device: PFN_vkCreateDevice,
    enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties,
    enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties,
    get_physical_device_sparse_image_format_properties:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                destroy_instance: transmute(load(c"vkDestroyInstance").ok_or(LoadingError)?),
                enumerate_physical_devices: transmute(
                    load(c"vkEnumeratePhysicalDevices").ok_or(LoadingError)?,
                ),
                get_physical_device_features: transmute(
                    load(c"vkGetPhysicalDeviceFeatures").ok_or(LoadingError)?,
                ),
                get_physical_device_format_properties: transmute(
                    load(c"vkGetPhysicalDeviceFormatProperties").ok_or(LoadingError)?,
                ),
                get_physical_device_image_format_properties: transmute(
                    load(c"vkGetPhysicalDeviceImageFormatProperties").ok_or(LoadingError)?,
                ),
                get_physical_device_properties: transmute(
                    load(c"vkGetPhysicalDeviceProperties").ok_or(LoadingError)?,
                ),
                get_physical_device_queue_family_properties: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyProperties").ok_or(LoadingError)?,
                ),
                get_physical_device_memory_properties: transmute(
                    load(c"vkGetPhysicalDeviceMemoryProperties").ok_or(LoadingError)?,
                ),
                get_instance_proc_addr: transmute(
                    load(c"vkGetInstanceProcAddr").ok_or(LoadingError)?,
                ),
                create_device: transmute(load(c"vkCreateDevice").ok_or(LoadingError)?),
                enumerate_device_extension_properties: transmute(
                    load(c"vkEnumerateDeviceExtensionProperties").ok_or(LoadingError)?,
                ),
                enumerate_device_layer_properties: transmute(
                    load(c"vkEnumerateDeviceLayerProperties").ok_or(LoadingError)?,
                ),
                get_physical_device_sparse_image_format_properties: transmute(
                    load(c"vkGetPhysicalDeviceSparseImageFormatProperties").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn destroy_instance(
        &self,
        instance: Instance,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_instance)(instance, allocator.to_raw_ptr()) }
    }
    pub unsafe fn enumerate_physical_devices(
        &self,
        instance: Instance,
        physical_devices: impl ExtendUninit<PhysicalDevice>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                physical_devices,
                |physical_device_count, physical_devices| {
                    let result = (self.enumerate_physical_devices)(
                        instance,
                        physical_device_count,
                        physical_devices as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
                },
            )
        }
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceFeatures {
        unsafe {
            let mut features = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_features)(physical_device, features.as_mut_ptr());
            features.assume_init()
        }
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
    ) -> FormatProperties {
        unsafe {
            let mut format_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_format_properties)(
                physical_device,
                format,
                format_properties.as_mut_ptr(),
            );
            format_properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> crate::Result<ImageFormatProperties> {
        unsafe {
            let mut image_format_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_image_format_properties)(
                physical_device,
                format,
                ty,
                tiling,
                usage,
                flags,
                image_format_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(image_format_properties.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceProperties {
        unsafe {
            let mut properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_properties)(physical_device, properties.as_mut_ptr());
            properties.assume_init()
        }
    }
    pub unsafe fn get_physical_device_queue_family_properties(
        &self,
        physical_device: PhysicalDevice,
        queue_family_properties: impl ExtendUninit<QueueFamilyProperties>,
    ) {
        unsafe {
            extend_uninit(
                queue_family_properties,
                |queue_family_property_count, queue_family_properties| {
                    (self.get_physical_device_queue_family_properties)(
                        physical_device,
                        queue_family_property_count,
                        queue_family_properties as _,
                    )
                },
            )
        }
    }
    pub unsafe fn get_physical_device_memory_properties(
        &self,
        physical_device: PhysicalDevice,
    ) -> PhysicalDeviceMemoryProperties {
        unsafe {
            let mut memory_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_memory_properties)(
                physical_device,
                memory_properties.as_mut_ptr(),
            );
            memory_properties.assume_init()
        }
    }
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Instance,
        name: &CStr,
    ) -> PFN_vkVoidFunction {
        unsafe { (self.get_instance_proc_addr)(instance, name.as_ptr() as _) }
    }
    pub unsafe fn create_device(
        &self,
        physical_device: PhysicalDevice,
        create_info: &DeviceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Device> {
        unsafe {
            let mut device = core::mem::MaybeUninit::uninit();
            let result = (self.create_device)(
                physical_device,
                create_info,
                allocator.to_raw_ptr(),
                device.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(device.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        layer_name: Option<&CStr>,
        properties: impl ExtendUninit<ExtensionProperties>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.enumerate_device_extension_properties)(
                    physical_device,
                    layer_name.to_raw_ptr(),
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<LayerProperties>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                let result = (self.enumerate_device_layer_properties)(
                    physical_device,
                    property_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlagBits,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
        properties: impl ExtendUninit<SparseImageFormatProperties>,
    ) {
        unsafe {
            extend_uninit(properties, |property_count, properties| {
                (self.get_physical_device_sparse_image_format_properties)(
                    physical_device,
                    format,
                    ty,
                    samples,
                    usage,
                    tiling,
                    property_count,
                    properties as _,
                )
            })
        }
    }
}
pub struct DeviceFn {
    get_device_proc_addr: PFN_vkGetDeviceProcAddr,
    destroy_device: PFN_vkDestroyDevice,
    get_device_queue: PFN_vkGetDeviceQueue,
    queue_submit: PFN_vkQueueSubmit,
    queue_wait_idle: PFN_vkQueueWaitIdle,
    device_wait_idle: PFN_vkDeviceWaitIdle,
    allocate_memory: PFN_vkAllocateMemory,
    free_memory: PFN_vkFreeMemory,
    map_memory: PFN_vkMapMemory,
    unmap_memory: PFN_vkUnmapMemory,
    flush_mapped_memory_ranges: PFN_vkFlushMappedMemoryRanges,
    invalidate_mapped_memory_ranges: PFN_vkInvalidateMappedMemoryRanges,
    get_device_memory_commitment: PFN_vkGetDeviceMemoryCommitment,
    bind_buffer_memory: PFN_vkBindBufferMemory,
    bind_image_memory: PFN_vkBindImageMemory,
    get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements,
    get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
    get_image_sparse_memory_requirements: PFN_vkGetImageSparseMemoryRequirements,
    queue_bind_sparse: PFN_vkQueueBindSparse,
    create_fence: PFN_vkCreateFence,
    destroy_fence: PFN_vkDestroyFence,
    reset_fences: PFN_vkResetFences,
    get_fence_status: PFN_vkGetFenceStatus,
    wait_for_fences: PFN_vkWaitForFences,
    create_semaphore: PFN_vkCreateSemaphore,
    destroy_semaphore: PFN_vkDestroySemaphore,
    create_query_pool: PFN_vkCreateQueryPool,
    destroy_query_pool: PFN_vkDestroyQueryPool,
    get_query_pool_results: PFN_vkGetQueryPoolResults,
    create_buffer: PFN_vkCreateBuffer,
    destroy_buffer: PFN_vkDestroyBuffer,
    create_image: PFN_vkCreateImage,
    destroy_image: PFN_vkDestroyImage,
    get_image_subresource_layout: PFN_vkGetImageSubresourceLayout,
    create_image_view: PFN_vkCreateImageView,
    destroy_image_view: PFN_vkDestroyImageView,
    create_command_pool: PFN_vkCreateCommandPool,
    destroy_command_pool: PFN_vkDestroyCommandPool,
    reset_command_pool: PFN_vkResetCommandPool,
    allocate_command_buffers: PFN_vkAllocateCommandBuffers,
    free_command_buffers: PFN_vkFreeCommandBuffers,
    begin_command_buffer: PFN_vkBeginCommandBuffer,
    end_command_buffer: PFN_vkEndCommandBuffer,
    reset_command_buffer: PFN_vkResetCommandBuffer,
    cmd_copy_buffer: PFN_vkCmdCopyBuffer,
    cmd_copy_image: PFN_vkCmdCopyImage,
    cmd_copy_buffer_to_image: PFN_vkCmdCopyBufferToImage,
    cmd_copy_image_to_buffer: PFN_vkCmdCopyImageToBuffer,
    cmd_update_buffer: PFN_vkCmdUpdateBuffer,
    cmd_fill_buffer: PFN_vkCmdFillBuffer,
    cmd_pipeline_barrier: PFN_vkCmdPipelineBarrier,
    cmd_begin_query: PFN_vkCmdBeginQuery,
    cmd_end_query: PFN_vkCmdEndQuery,
    cmd_reset_query_pool: PFN_vkCmdResetQueryPool,
    cmd_write_timestamp: PFN_vkCmdWriteTimestamp,
    cmd_copy_query_pool_results: PFN_vkCmdCopyQueryPoolResults,
    cmd_execute_commands: PFN_vkCmdExecuteCommands,
    create_event: PFN_vkCreateEvent,
    destroy_event: PFN_vkDestroyEvent,
    get_event_status: PFN_vkGetEventStatus,
    set_event: PFN_vkSetEvent,
    reset_event: PFN_vkResetEvent,
    create_buffer_view: PFN_vkCreateBufferView,
    destroy_buffer_view: PFN_vkDestroyBufferView,
    create_shader_module: PFN_vkCreateShaderModule,
    destroy_shader_module: PFN_vkDestroyShaderModule,
    create_pipeline_cache: PFN_vkCreatePipelineCache,
    destroy_pipeline_cache: PFN_vkDestroyPipelineCache,
    get_pipeline_cache_data: PFN_vkGetPipelineCacheData,
    merge_pipeline_caches: PFN_vkMergePipelineCaches,
    create_compute_pipelines: PFN_vkCreateComputePipelines,
    destroy_pipeline: PFN_vkDestroyPipeline,
    create_pipeline_layout: PFN_vkCreatePipelineLayout,
    destroy_pipeline_layout: PFN_vkDestroyPipelineLayout,
    create_sampler: PFN_vkCreateSampler,
    destroy_sampler: PFN_vkDestroySampler,
    create_descriptor_set_layout: PFN_vkCreateDescriptorSetLayout,
    destroy_descriptor_set_layout: PFN_vkDestroyDescriptorSetLayout,
    create_descriptor_pool: PFN_vkCreateDescriptorPool,
    destroy_descriptor_pool: PFN_vkDestroyDescriptorPool,
    reset_descriptor_pool: PFN_vkResetDescriptorPool,
    allocate_descriptor_sets: PFN_vkAllocateDescriptorSets,
    free_descriptor_sets: PFN_vkFreeDescriptorSets,
    update_descriptor_sets: PFN_vkUpdateDescriptorSets,
    cmd_bind_pipeline: PFN_vkCmdBindPipeline,
    cmd_bind_descriptor_sets: PFN_vkCmdBindDescriptorSets,
    cmd_clear_color_image: PFN_vkCmdClearColorImage,
    cmd_dispatch: PFN_vkCmdDispatch,
    cmd_dispatch_indirect: PFN_vkCmdDispatchIndirect,
    cmd_set_event: PFN_vkCmdSetEvent,
    cmd_reset_event: PFN_vkCmdResetEvent,
    cmd_wait_events: PFN_vkCmdWaitEvents,
    cmd_push_constants: PFN_vkCmdPushConstants,
    create_graphics_pipelines: PFN_vkCreateGraphicsPipelines,
    create_framebuffer: PFN_vkCreateFramebuffer,
    destroy_framebuffer: PFN_vkDestroyFramebuffer,
    create_render_pass: PFN_vkCreateRenderPass,
    destroy_render_pass: PFN_vkDestroyRenderPass,
    get_render_area_granularity: PFN_vkGetRenderAreaGranularity,
    cmd_set_viewport: PFN_vkCmdSetViewport,
    cmd_set_scissor: PFN_vkCmdSetScissor,
    cmd_set_line_width: PFN_vkCmdSetLineWidth,
    cmd_set_depth_bias: PFN_vkCmdSetDepthBias,
    cmd_set_blend_constants: PFN_vkCmdSetBlendConstants,
    cmd_set_depth_bounds: PFN_vkCmdSetDepthBounds,
    cmd_set_stencil_compare_mask: PFN_vkCmdSetStencilCompareMask,
    cmd_set_stencil_write_mask: PFN_vkCmdSetStencilWriteMask,
    cmd_set_stencil_reference: PFN_vkCmdSetStencilReference,
    cmd_bind_index_buffer: PFN_vkCmdBindIndexBuffer,
    cmd_bind_vertex_buffers: PFN_vkCmdBindVertexBuffers,
    cmd_draw: PFN_vkCmdDraw,
    cmd_draw_indexed: PFN_vkCmdDrawIndexed,
    cmd_draw_indirect: PFN_vkCmdDrawIndirect,
    cmd_draw_indexed_indirect: PFN_vkCmdDrawIndexedIndirect,
    cmd_blit_image: PFN_vkCmdBlitImage,
    cmd_clear_depth_stencil_image: PFN_vkCmdClearDepthStencilImage,
    cmd_clear_attachments: PFN_vkCmdClearAttachments,
    cmd_resolve_image: PFN_vkCmdResolveImage,
    cmd_begin_render_pass: PFN_vkCmdBeginRenderPass,
    cmd_next_subpass: PFN_vkCmdNextSubpass,
    cmd_end_render_pass: PFN_vkCmdEndRenderPass,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_device_proc_addr: transmute(load(c"vkGetDeviceProcAddr").ok_or(LoadingError)?),
                destroy_device: transmute(load(c"vkDestroyDevice").ok_or(LoadingError)?),
                get_device_queue: transmute(load(c"vkGetDeviceQueue").ok_or(LoadingError)?),
                queue_submit: transmute(load(c"vkQueueSubmit").ok_or(LoadingError)?),
                queue_wait_idle: transmute(load(c"vkQueueWaitIdle").ok_or(LoadingError)?),
                device_wait_idle: transmute(load(c"vkDeviceWaitIdle").ok_or(LoadingError)?),
                allocate_memory: transmute(load(c"vkAllocateMemory").ok_or(LoadingError)?),
                free_memory: transmute(load(c"vkFreeMemory").ok_or(LoadingError)?),
                map_memory: transmute(load(c"vkMapMemory").ok_or(LoadingError)?),
                unmap_memory: transmute(load(c"vkUnmapMemory").ok_or(LoadingError)?),
                flush_mapped_memory_ranges: transmute(
                    load(c"vkFlushMappedMemoryRanges").ok_or(LoadingError)?,
                ),
                invalidate_mapped_memory_ranges: transmute(
                    load(c"vkInvalidateMappedMemoryRanges").ok_or(LoadingError)?,
                ),
                get_device_memory_commitment: transmute(
                    load(c"vkGetDeviceMemoryCommitment").ok_or(LoadingError)?,
                ),
                bind_buffer_memory: transmute(load(c"vkBindBufferMemory").ok_or(LoadingError)?),
                bind_image_memory: transmute(load(c"vkBindImageMemory").ok_or(LoadingError)?),
                get_buffer_memory_requirements: transmute(
                    load(c"vkGetBufferMemoryRequirements").ok_or(LoadingError)?,
                ),
                get_image_memory_requirements: transmute(
                    load(c"vkGetImageMemoryRequirements").ok_or(LoadingError)?,
                ),
                get_image_sparse_memory_requirements: transmute(
                    load(c"vkGetImageSparseMemoryRequirements").ok_or(LoadingError)?,
                ),
                queue_bind_sparse: transmute(load(c"vkQueueBindSparse").ok_or(LoadingError)?),
                create_fence: transmute(load(c"vkCreateFence").ok_or(LoadingError)?),
                destroy_fence: transmute(load(c"vkDestroyFence").ok_or(LoadingError)?),
                reset_fences: transmute(load(c"vkResetFences").ok_or(LoadingError)?),
                get_fence_status: transmute(load(c"vkGetFenceStatus").ok_or(LoadingError)?),
                wait_for_fences: transmute(load(c"vkWaitForFences").ok_or(LoadingError)?),
                create_semaphore: transmute(load(c"vkCreateSemaphore").ok_or(LoadingError)?),
                destroy_semaphore: transmute(load(c"vkDestroySemaphore").ok_or(LoadingError)?),
                create_query_pool: transmute(load(c"vkCreateQueryPool").ok_or(LoadingError)?),
                destroy_query_pool: transmute(load(c"vkDestroyQueryPool").ok_or(LoadingError)?),
                get_query_pool_results: transmute(
                    load(c"vkGetQueryPoolResults").ok_or(LoadingError)?,
                ),
                create_buffer: transmute(load(c"vkCreateBuffer").ok_or(LoadingError)?),
                destroy_buffer: transmute(load(c"vkDestroyBuffer").ok_or(LoadingError)?),
                create_image: transmute(load(c"vkCreateImage").ok_or(LoadingError)?),
                destroy_image: transmute(load(c"vkDestroyImage").ok_or(LoadingError)?),
                get_image_subresource_layout: transmute(
                    load(c"vkGetImageSubresourceLayout").ok_or(LoadingError)?,
                ),
                create_image_view: transmute(load(c"vkCreateImageView").ok_or(LoadingError)?),
                destroy_image_view: transmute(load(c"vkDestroyImageView").ok_or(LoadingError)?),
                create_command_pool: transmute(load(c"vkCreateCommandPool").ok_or(LoadingError)?),
                destroy_command_pool: transmute(load(c"vkDestroyCommandPool").ok_or(LoadingError)?),
                reset_command_pool: transmute(load(c"vkResetCommandPool").ok_or(LoadingError)?),
                allocate_command_buffers: transmute(
                    load(c"vkAllocateCommandBuffers").ok_or(LoadingError)?,
                ),
                free_command_buffers: transmute(load(c"vkFreeCommandBuffers").ok_or(LoadingError)?),
                begin_command_buffer: transmute(load(c"vkBeginCommandBuffer").ok_or(LoadingError)?),
                end_command_buffer: transmute(load(c"vkEndCommandBuffer").ok_or(LoadingError)?),
                reset_command_buffer: transmute(load(c"vkResetCommandBuffer").ok_or(LoadingError)?),
                cmd_copy_buffer: transmute(load(c"vkCmdCopyBuffer").ok_or(LoadingError)?),
                cmd_copy_image: transmute(load(c"vkCmdCopyImage").ok_or(LoadingError)?),
                cmd_copy_buffer_to_image: transmute(
                    load(c"vkCmdCopyBufferToImage").ok_or(LoadingError)?,
                ),
                cmd_copy_image_to_buffer: transmute(
                    load(c"vkCmdCopyImageToBuffer").ok_or(LoadingError)?,
                ),
                cmd_update_buffer: transmute(load(c"vkCmdUpdateBuffer").ok_or(LoadingError)?),
                cmd_fill_buffer: transmute(load(c"vkCmdFillBuffer").ok_or(LoadingError)?),
                cmd_pipeline_barrier: transmute(load(c"vkCmdPipelineBarrier").ok_or(LoadingError)?),
                cmd_begin_query: transmute(load(c"vkCmdBeginQuery").ok_or(LoadingError)?),
                cmd_end_query: transmute(load(c"vkCmdEndQuery").ok_or(LoadingError)?),
                cmd_reset_query_pool: transmute(load(c"vkCmdResetQueryPool").ok_or(LoadingError)?),
                cmd_write_timestamp: transmute(load(c"vkCmdWriteTimestamp").ok_or(LoadingError)?),
                cmd_copy_query_pool_results: transmute(
                    load(c"vkCmdCopyQueryPoolResults").ok_or(LoadingError)?,
                ),
                cmd_execute_commands: transmute(load(c"vkCmdExecuteCommands").ok_or(LoadingError)?),
                create_event: transmute(load(c"vkCreateEvent").ok_or(LoadingError)?),
                destroy_event: transmute(load(c"vkDestroyEvent").ok_or(LoadingError)?),
                get_event_status: transmute(load(c"vkGetEventStatus").ok_or(LoadingError)?),
                set_event: transmute(load(c"vkSetEvent").ok_or(LoadingError)?),
                reset_event: transmute(load(c"vkResetEvent").ok_or(LoadingError)?),
                create_buffer_view: transmute(load(c"vkCreateBufferView").ok_or(LoadingError)?),
                destroy_buffer_view: transmute(load(c"vkDestroyBufferView").ok_or(LoadingError)?),
                create_shader_module: transmute(load(c"vkCreateShaderModule").ok_or(LoadingError)?),
                destroy_shader_module: transmute(
                    load(c"vkDestroyShaderModule").ok_or(LoadingError)?,
                ),
                create_pipeline_cache: transmute(
                    load(c"vkCreatePipelineCache").ok_or(LoadingError)?,
                ),
                destroy_pipeline_cache: transmute(
                    load(c"vkDestroyPipelineCache").ok_or(LoadingError)?,
                ),
                get_pipeline_cache_data: transmute(
                    load(c"vkGetPipelineCacheData").ok_or(LoadingError)?,
                ),
                merge_pipeline_caches: transmute(
                    load(c"vkMergePipelineCaches").ok_or(LoadingError)?,
                ),
                create_compute_pipelines: transmute(
                    load(c"vkCreateComputePipelines").ok_or(LoadingError)?,
                ),
                destroy_pipeline: transmute(load(c"vkDestroyPipeline").ok_or(LoadingError)?),
                create_pipeline_layout: transmute(
                    load(c"vkCreatePipelineLayout").ok_or(LoadingError)?,
                ),
                destroy_pipeline_layout: transmute(
                    load(c"vkDestroyPipelineLayout").ok_or(LoadingError)?,
                ),
                create_sampler: transmute(load(c"vkCreateSampler").ok_or(LoadingError)?),
                destroy_sampler: transmute(load(c"vkDestroySampler").ok_or(LoadingError)?),
                create_descriptor_set_layout: transmute(
                    load(c"vkCreateDescriptorSetLayout").ok_or(LoadingError)?,
                ),
                destroy_descriptor_set_layout: transmute(
                    load(c"vkDestroyDescriptorSetLayout").ok_or(LoadingError)?,
                ),
                create_descriptor_pool: transmute(
                    load(c"vkCreateDescriptorPool").ok_or(LoadingError)?,
                ),
                destroy_descriptor_pool: transmute(
                    load(c"vkDestroyDescriptorPool").ok_or(LoadingError)?,
                ),
                reset_descriptor_pool: transmute(
                    load(c"vkResetDescriptorPool").ok_or(LoadingError)?,
                ),
                allocate_descriptor_sets: transmute(
                    load(c"vkAllocateDescriptorSets").ok_or(LoadingError)?,
                ),
                free_descriptor_sets: transmute(load(c"vkFreeDescriptorSets").ok_or(LoadingError)?),
                update_descriptor_sets: transmute(
                    load(c"vkUpdateDescriptorSets").ok_or(LoadingError)?,
                ),
                cmd_bind_pipeline: transmute(load(c"vkCmdBindPipeline").ok_or(LoadingError)?),
                cmd_bind_descriptor_sets: transmute(
                    load(c"vkCmdBindDescriptorSets").ok_or(LoadingError)?,
                ),
                cmd_clear_color_image: transmute(
                    load(c"vkCmdClearColorImage").ok_or(LoadingError)?,
                ),
                cmd_dispatch: transmute(load(c"vkCmdDispatch").ok_or(LoadingError)?),
                cmd_dispatch_indirect: transmute(
                    load(c"vkCmdDispatchIndirect").ok_or(LoadingError)?,
                ),
                cmd_set_event: transmute(load(c"vkCmdSetEvent").ok_or(LoadingError)?),
                cmd_reset_event: transmute(load(c"vkCmdResetEvent").ok_or(LoadingError)?),
                cmd_wait_events: transmute(load(c"vkCmdWaitEvents").ok_or(LoadingError)?),
                cmd_push_constants: transmute(load(c"vkCmdPushConstants").ok_or(LoadingError)?),
                create_graphics_pipelines: transmute(
                    load(c"vkCreateGraphicsPipelines").ok_or(LoadingError)?,
                ),
                create_framebuffer: transmute(load(c"vkCreateFramebuffer").ok_or(LoadingError)?),
                destroy_framebuffer: transmute(load(c"vkDestroyFramebuffer").ok_or(LoadingError)?),
                create_render_pass: transmute(load(c"vkCreateRenderPass").ok_or(LoadingError)?),
                destroy_render_pass: transmute(load(c"vkDestroyRenderPass").ok_or(LoadingError)?),
                get_render_area_granularity: transmute(
                    load(c"vkGetRenderAreaGranularity").ok_or(LoadingError)?,
                ),
                cmd_set_viewport: transmute(load(c"vkCmdSetViewport").ok_or(LoadingError)?),
                cmd_set_scissor: transmute(load(c"vkCmdSetScissor").ok_or(LoadingError)?),
                cmd_set_line_width: transmute(load(c"vkCmdSetLineWidth").ok_or(LoadingError)?),
                cmd_set_depth_bias: transmute(load(c"vkCmdSetDepthBias").ok_or(LoadingError)?),
                cmd_set_blend_constants: transmute(
                    load(c"vkCmdSetBlendConstants").ok_or(LoadingError)?,
                ),
                cmd_set_depth_bounds: transmute(load(c"vkCmdSetDepthBounds").ok_or(LoadingError)?),
                cmd_set_stencil_compare_mask: transmute(
                    load(c"vkCmdSetStencilCompareMask").ok_or(LoadingError)?,
                ),
                cmd_set_stencil_write_mask: transmute(
                    load(c"vkCmdSetStencilWriteMask").ok_or(LoadingError)?,
                ),
                cmd_set_stencil_reference: transmute(
                    load(c"vkCmdSetStencilReference").ok_or(LoadingError)?,
                ),
                cmd_bind_index_buffer: transmute(
                    load(c"vkCmdBindIndexBuffer").ok_or(LoadingError)?,
                ),
                cmd_bind_vertex_buffers: transmute(
                    load(c"vkCmdBindVertexBuffers").ok_or(LoadingError)?,
                ),
                cmd_draw: transmute(load(c"vkCmdDraw").ok_or(LoadingError)?),
                cmd_draw_indexed: transmute(load(c"vkCmdDrawIndexed").ok_or(LoadingError)?),
                cmd_draw_indirect: transmute(load(c"vkCmdDrawIndirect").ok_or(LoadingError)?),
                cmd_draw_indexed_indirect: transmute(
                    load(c"vkCmdDrawIndexedIndirect").ok_or(LoadingError)?,
                ),
                cmd_blit_image: transmute(load(c"vkCmdBlitImage").ok_or(LoadingError)?),
                cmd_clear_depth_stencil_image: transmute(
                    load(c"vkCmdClearDepthStencilImage").ok_or(LoadingError)?,
                ),
                cmd_clear_attachments: transmute(
                    load(c"vkCmdClearAttachments").ok_or(LoadingError)?,
                ),
                cmd_resolve_image: transmute(load(c"vkCmdResolveImage").ok_or(LoadingError)?),
                cmd_begin_render_pass: transmute(
                    load(c"vkCmdBeginRenderPass").ok_or(LoadingError)?,
                ),
                cmd_next_subpass: transmute(load(c"vkCmdNextSubpass").ok_or(LoadingError)?),
                cmd_end_render_pass: transmute(load(c"vkCmdEndRenderPass").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_device_proc_addr(&self, device: Device, name: &CStr) -> PFN_vkVoidFunction {
        unsafe { (self.get_device_proc_addr)(device, name.as_ptr() as _) }
    }
    pub unsafe fn destroy_device(&self, device: Device, allocator: Option<&AllocationCallbacks>) {
        unsafe { (self.destroy_device)(device, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_device_queue(
        &self,
        device: Device,
        queue_family_index: u32,
        queue_index: u32,
    ) -> Queue {
        unsafe {
            let mut queue = core::mem::MaybeUninit::uninit();
            (self.get_device_queue)(device, queue_family_index, queue_index, queue.as_mut_ptr());
            queue.assume_init()
        }
    }
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submits: &[SubmitInfo],
        fence: Fence,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_submit)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_wait_idle)(queue);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn device_wait_idle(&self, device: Device) -> crate::Result<()> {
        unsafe {
            let result = (self.device_wait_idle)(device);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn allocate_memory(
        &self,
        device: Device,
        allocate_info: &MemoryAllocateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<DeviceMemory> {
        unsafe {
            let mut memory = core::mem::MaybeUninit::uninit();
            let result = (self.allocate_memory)(
                device,
                allocate_info,
                allocator.to_raw_ptr(),
                memory.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn free_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.free_memory)(device, memory, allocator.to_raw_ptr()) }
    }
    pub unsafe fn map_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
        data: &mut *mut c_void,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.map_memory)(device, memory, offset, size, flags, data);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn unmap_memory(&self, device: Device, memory: DeviceMemory) {
        unsafe { (self.unmap_memory)(device, memory) }
    }
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: Device,
        memory_ranges: &[MappedMemoryRange],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.flush_mapped_memory_ranges)(
                device,
                memory_ranges.len().try_into().unwrap(),
                memory_ranges.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: Device,
        memory_ranges: &[MappedMemoryRange],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.invalidate_mapped_memory_ranges)(
                device,
                memory_ranges.len().try_into().unwrap(),
                memory_ranges.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: Device,
        memory: DeviceMemory,
    ) -> DeviceSize {
        unsafe {
            let mut committed_memory_in_bytes = core::mem::MaybeUninit::uninit();
            (self.get_device_memory_commitment)(
                device,
                memory,
                committed_memory_in_bytes.as_mut_ptr(),
            );
            committed_memory_in_bytes.assume_init()
        }
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_buffer_memory)(device, buffer, memory, memory_offset);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn bind_image_memory(
        &self,
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_image_memory)(device, image, memory, memory_offset);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: Device,
        buffer: Buffer,
    ) -> MemoryRequirements {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_buffer_memory_requirements)(device, buffer, memory_requirements.as_mut_ptr());
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: Device,
        image: Image,
    ) -> MemoryRequirements {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_image_memory_requirements)(device, image, memory_requirements.as_mut_ptr());
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn get_image_sparse_memory_requirements(
        &self,
        device: Device,
        image: Image,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_image_sparse_memory_requirements)(
                        device,
                        image,
                        sparse_memory_requirement_count,
                        sparse_memory_requirements as _,
                    )
                },
            )
        }
    }
    pub unsafe fn queue_bind_sparse(
        &self,
        queue: Queue,
        bind_info: &[BindSparseInfo],
        fence: Fence,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_bind_sparse)(
                queue,
                bind_info.len().try_into().unwrap(),
                bind_info.as_ptr() as _,
                fence,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_fence(
        &self,
        device: Device,
        create_info: &FenceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Fence> {
        unsafe {
            let mut fence = core::mem::MaybeUninit::uninit();
            let result = (self.create_fence)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                fence.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(fence.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_fence(
        &self,
        device: Device,
        fence: Fence,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_fence)(device, fence, allocator.to_raw_ptr()) }
    }
    pub unsafe fn reset_fences(&self, device: Device, fences: &[Fence]) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_fences)(
                device,
                fences.len().try_into().unwrap(),
                fences.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_fence_status(&self, device: Device, fence: Fence) -> crate::Result<()> {
        unsafe {
            let result = (self.get_fence_status)(device, fence);

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::NOT_READY => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn wait_for_fences(
        &self,
        device: Device,
        fences: &[Fence],
        wait_all: Bool32,
        timeout: u64,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.wait_for_fences)(
                device,
                fences.len().try_into().unwrap(),
                fences.as_ptr() as _,
                wait_all,
                timeout,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::TIMEOUT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_semaphore(
        &self,
        device: Device,
        create_info: &SemaphoreCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Semaphore> {
        unsafe {
            let mut semaphore = core::mem::MaybeUninit::uninit();
            let result = (self.create_semaphore)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                semaphore.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(semaphore.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_semaphore(
        &self,
        device: Device,
        semaphore: Semaphore,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_semaphore)(device, semaphore, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_query_pool(
        &self,
        device: Device,
        create_info: &QueryPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<QueryPool> {
        unsafe {
            let mut query_pool = core::mem::MaybeUninit::uninit();
            let result = (self.create_query_pool)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                query_pool.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(query_pool.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_query_pool)(device, query_pool, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_query_pool_results(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data: &mut [u8],
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_query_pool_results)(
                device,
                query_pool,
                first_query,
                query_count,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
                stride,
                flags,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::NOT_READY => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_buffer(
        &self,
        device: Device,
        create_info: &BufferCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Buffer> {
        unsafe {
            let mut buffer = core::mem::MaybeUninit::uninit();
            let result = (self.create_buffer)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                buffer.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(buffer.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_buffer(
        &self,
        device: Device,
        buffer: Buffer,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_buffer)(device, buffer, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_image(
        &self,
        device: Device,
        create_info: &ImageCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Image> {
        unsafe {
            let mut image = core::mem::MaybeUninit::uninit();
            let result = (self.create_image)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                image.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(image.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_image(
        &self,
        device: Device,
        image: Image,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_image)(device, image, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_image_subresource_layout(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource,
    ) -> SubresourceLayout {
        unsafe {
            let mut layout = core::mem::MaybeUninit::uninit();
            (self.get_image_subresource_layout)(device, image, subresource, layout.as_mut_ptr());
            layout.assume_init()
        }
    }
    pub unsafe fn create_image_view(
        &self,
        device: Device,
        create_info: &ImageViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<ImageView> {
        unsafe {
            let mut view = core::mem::MaybeUninit::uninit();
            let result = (self.create_image_view)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                view.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(view.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_image_view(
        &self,
        device: Device,
        image_view: ImageView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_image_view)(device, image_view, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_command_pool(
        &self,
        device: Device,
        create_info: &CommandPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<CommandPool> {
        unsafe {
            let mut command_pool = core::mem::MaybeUninit::uninit();
            let result = (self.create_command_pool)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                command_pool.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(command_pool.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_command_pool)(device, command_pool, allocator.to_raw_ptr()) }
    }
    pub unsafe fn reset_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_command_pool)(device, command_pool, flags);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn allocate_command_buffers(
        &self,
        device: Device,
        allocate_info: &CommandBufferAllocateInfo,
        command_buffers: &mut [CommandBuffer],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.allocate_command_buffers)(
                device,
                allocate_info,
                command_buffers.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn free_command_buffers(
        &self,
        device: Device,
        command_pool: CommandPool,
        command_buffers: &[CommandBuffer],
    ) {
        unsafe {
            (self.free_command_buffers)(
                device,
                command_pool,
                command_buffers.len().try_into().unwrap(),
                command_buffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn begin_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &CommandBufferBeginInfo,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.begin_command_buffer)(command_buffer, begin_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn end_command_buffer(&self, command_buffer: CommandBuffer) -> crate::Result<()> {
        unsafe {
            let result = (self.end_command_buffer)(command_buffer);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_command_buffer)(command_buffer, flags);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_copy_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_buffer: Buffer,
        regions: &[BufferCopy],
    ) {
        unsafe {
            (self.cmd_copy_buffer)(
                command_buffer,
                src_buffer,
                dst_buffer,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_copy_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy],
    ) {
        unsafe {
            (self.cmd_copy_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_copy_buffer_to_image(
        &self,
        command_buffer: CommandBuffer,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            (self.cmd_copy_buffer_to_image)(
                command_buffer,
                src_buffer,
                dst_image,
                dst_image_layout,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_copy_image_to_buffer(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            (self.cmd_copy_image_to_buffer)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_buffer,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_update_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        data: &[u8],
    ) {
        unsafe {
            (self.cmd_update_buffer)(
                command_buffer,
                dst_buffer,
                dst_offset,
                data.len().try_into().unwrap(),
                data.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_fill_buffer(
        &self,
        command_buffer: CommandBuffer,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ) {
        unsafe { (self.cmd_fill_buffer)(command_buffer, dst_buffer, dst_offset, size, data) }
    }
    pub unsafe fn cmd_pipeline_barrier(
        &self,
        command_buffer: CommandBuffer,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) {
        unsafe {
            (self.cmd_pipeline_barrier)(
                command_buffer,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                memory_barriers.len().try_into().unwrap(),
                memory_barriers.as_ptr() as _,
                buffer_memory_barriers.len().try_into().unwrap(),
                buffer_memory_barriers.as_ptr() as _,
                image_memory_barriers.len().try_into().unwrap(),
                image_memory_barriers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_begin_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
    ) {
        unsafe { (self.cmd_begin_query)(command_buffer, query_pool, query, flags) }
    }
    pub unsafe fn cmd_end_query(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_end_query)(command_buffer, query_pool, query) }
    }
    pub unsafe fn cmd_reset_query_pool(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
    ) {
        unsafe { (self.cmd_reset_query_pool)(command_buffer, query_pool, first_query, query_count) }
    }
    pub unsafe fn cmd_write_timestamp(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp)(command_buffer, pipeline_stage, query_pool, query) }
    }
    pub unsafe fn cmd_copy_query_pool_results(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) {
        unsafe {
            (self.cmd_copy_query_pool_results)(
                command_buffer,
                query_pool,
                first_query,
                query_count,
                dst_buffer,
                dst_offset,
                stride,
                flags,
            )
        }
    }
    pub unsafe fn cmd_execute_commands(
        &self,
        command_buffer: CommandBuffer,
        command_buffers: &[CommandBuffer],
    ) {
        unsafe {
            (self.cmd_execute_commands)(
                command_buffer,
                command_buffers.len().try_into().unwrap(),
                command_buffers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn create_event(
        &self,
        device: Device,
        create_info: &EventCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Event> {
        unsafe {
            let mut event = core::mem::MaybeUninit::uninit();
            let result = (self.create_event)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                event.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(event.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_event(
        &self,
        device: Device,
        event: Event,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_event)(device, event, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_event_status(&self, device: Device, event: Event) -> crate::Result<()> {
        unsafe {
            let result = (self.get_event_status)(device, event);

            match result {
                VkResult::EVENT_SET => Ok(()),
                VkResult::EVENT_RESET => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn set_event(&self, device: Device, event: Event) -> crate::Result<()> {
        unsafe {
            let result = (self.set_event)(device, event);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn reset_event(&self, device: Device, event: Event) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_event)(device, event);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_buffer_view(
        &self,
        device: Device,
        create_info: &BufferViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<BufferView> {
        unsafe {
            let mut view = core::mem::MaybeUninit::uninit();
            let result = (self.create_buffer_view)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                view.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(view.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_buffer_view(
        &self,
        device: Device,
        buffer_view: BufferView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_buffer_view)(device, buffer_view, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_shader_module(
        &self,
        device: Device,
        create_info: &ShaderModuleCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<ShaderModule> {
        unsafe {
            let mut shader_module = core::mem::MaybeUninit::uninit();
            let result = (self.create_shader_module)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                shader_module.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(shader_module.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_shader_module(
        &self,
        device: Device,
        shader_module: ShaderModule,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_shader_module)(device, shader_module, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_pipeline_cache(
        &self,
        device: Device,
        create_info: &PipelineCacheCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<PipelineCache> {
        unsafe {
            let mut pipeline_cache = core::mem::MaybeUninit::uninit();
            let result = (self.create_pipeline_cache)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                pipeline_cache.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(pipeline_cache.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_pipeline_cache(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_pipeline_cache)(device, pipeline_cache, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_pipeline_cache_data(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                let result =
                    (self.get_pipeline_cache_data)(device, pipeline_cache, data_size, data as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: Device,
        dst_cache: PipelineCache,
        src_caches: &[PipelineCache],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.merge_pipeline_caches)(
                device,
                dst_cache,
                src_caches.len().try_into().unwrap(),
                src_caches.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_compute_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_compute_pipelines)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::PIPELINE_COMPILE_REQUIRED_EXT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_pipeline(
        &self,
        device: Device,
        pipeline: Pipeline,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_pipeline)(device, pipeline, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_pipeline_layout(
        &self,
        device: Device,
        create_info: &PipelineLayoutCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<PipelineLayout> {
        unsafe {
            let mut pipeline_layout = core::mem::MaybeUninit::uninit();
            let result = (self.create_pipeline_layout)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                pipeline_layout.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(pipeline_layout.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_pipeline_layout(
        &self,
        device: Device,
        pipeline_layout: PipelineLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_pipeline_layout)(device, pipeline_layout, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_sampler(
        &self,
        device: Device,
        create_info: &SamplerCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Sampler> {
        unsafe {
            let mut sampler = core::mem::MaybeUninit::uninit();
            let result = (self.create_sampler)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                sampler.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(sampler.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_sampler(
        &self,
        device: Device,
        sampler: Sampler,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_sampler)(device, sampler, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: Device,
        create_info: &DescriptorSetLayoutCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<DescriptorSetLayout> {
        unsafe {
            let mut set_layout = core::mem::MaybeUninit::uninit();
            let result = (self.create_descriptor_set_layout)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                set_layout.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(set_layout.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        device: Device,
        descriptor_set_layout: DescriptorSetLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_descriptor_set_layout)(
                device,
                descriptor_set_layout,
                allocator.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn create_descriptor_pool(
        &self,
        device: Device,
        create_info: &DescriptorPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<DescriptorPool> {
        unsafe {
            let mut descriptor_pool = core::mem::MaybeUninit::uninit();
            let result = (self.create_descriptor_pool)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                descriptor_pool.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(descriptor_pool.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_descriptor_pool)(device, descriptor_pool, allocator.to_raw_ptr()) }
    }
    pub unsafe fn reset_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_descriptor_pool)(device, descriptor_pool, flags);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: Device,
        allocate_info: &DescriptorSetAllocateInfo,
        descriptor_sets: &mut [DescriptorSet],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.allocate_descriptor_sets)(
                device,
                allocate_info,
                descriptor_sets.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn free_descriptor_sets(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_sets: &[DescriptorSet],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.free_descriptor_sets)(
                device,
                descriptor_pool,
                descriptor_sets.len().try_into().unwrap(),
                descriptor_sets.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn update_descriptor_sets(
        &self,
        device: Device,
        descriptor_writes: &[WriteDescriptorSet],
        descriptor_copies: &[CopyDescriptorSet],
    ) {
        unsafe {
            (self.update_descriptor_sets)(
                device,
                descriptor_writes.len().try_into().unwrap(),
                descriptor_writes.as_ptr() as _,
                descriptor_copies.len().try_into().unwrap(),
                descriptor_copies.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_pipeline(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        unsafe { (self.cmd_bind_pipeline)(command_buffer, pipeline_bind_point, pipeline) }
    }
    pub unsafe fn cmd_bind_descriptor_sets(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            (self.cmd_bind_descriptor_sets)(
                command_buffer,
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_sets.len().try_into().unwrap(),
                descriptor_sets.as_ptr() as _,
                dynamic_offsets.len().try_into().unwrap(),
                dynamic_offsets.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_clear_color_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            (self.cmd_clear_color_image)(
                command_buffer,
                image,
                image_layout,
                color,
                ranges.len().try_into().unwrap(),
                ranges.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_dispatch(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe { (self.cmd_dispatch)(command_buffer, group_count_x, group_count_y, group_count_z) }
    }
    pub unsafe fn cmd_dispatch_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        unsafe { (self.cmd_dispatch_indirect)(command_buffer, buffer, offset) }
    }
    pub unsafe fn cmd_set_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        unsafe { (self.cmd_set_event)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_reset_event(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags,
    ) {
        unsafe { (self.cmd_reset_event)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_wait_events(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) {
        unsafe {
            (self.cmd_wait_events)(
                command_buffer,
                events.len().try_into().unwrap(),
                events.as_ptr() as _,
                src_stage_mask,
                dst_stage_mask,
                memory_barriers.len().try_into().unwrap(),
                memory_barriers.as_ptr() as _,
                buffer_memory_barriers.len().try_into().unwrap(),
                buffer_memory_barriers.as_ptr() as _,
                image_memory_barriers.len().try_into().unwrap(),
                image_memory_barriers.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_push_constants(
        &self,
        command_buffer: CommandBuffer,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        values: &[u8],
    ) {
        unsafe {
            (self.cmd_push_constants)(
                command_buffer,
                layout,
                stage_flags,
                offset,
                values.len().try_into().unwrap(),
                values.as_ptr() as _,
            )
        }
    }
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_graphics_pipelines)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::PIPELINE_COMPILE_REQUIRED_EXT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_framebuffer(
        &self,
        device: Device,
        create_info: &FramebufferCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Framebuffer> {
        unsafe {
            let mut framebuffer = core::mem::MaybeUninit::uninit();
            let result = (self.create_framebuffer)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                framebuffer.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(framebuffer.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_framebuffer(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_framebuffer)(device, framebuffer, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_render_pass(
        &self,
        device: Device,
        create_info: &RenderPassCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<RenderPass> {
        unsafe {
            let mut render_pass = core::mem::MaybeUninit::uninit();
            let result = (self.create_render_pass)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                render_pass.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(render_pass.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_render_pass(
        &self,
        device: Device,
        render_pass: RenderPass,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_render_pass)(device, render_pass, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_render_area_granularity(
        &self,
        device: Device,
        render_pass: RenderPass,
    ) -> Extent2D {
        unsafe {
            let mut granularity = core::mem::MaybeUninit::uninit();
            (self.get_render_area_granularity)(device, render_pass, granularity.as_mut_ptr());
            granularity.assume_init()
        }
    }
    pub unsafe fn cmd_set_viewport(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.cmd_set_viewport)(
                command_buffer,
                first_viewport,
                viewports.len().try_into().unwrap(),
                viewports.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_scissor(
        &self,
        command_buffer: CommandBuffer,
        first_scissor: u32,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_scissor)(
                command_buffer,
                first_scissor,
                scissors.len().try_into().unwrap(),
                scissors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_line_width(&self, command_buffer: CommandBuffer, line_width: f32) {
        unsafe { (self.cmd_set_line_width)(command_buffer, line_width) }
    }
    pub unsafe fn cmd_set_depth_bias(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        unsafe {
            (self.cmd_set_depth_bias)(
                command_buffer,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
            )
        }
    }
    pub unsafe fn cmd_set_blend_constants(
        &self,
        command_buffer: CommandBuffer,
        blend_constants: &[f32; 4],
    ) {
        unsafe { (self.cmd_set_blend_constants)(command_buffer, blend_constants) }
    }
    pub unsafe fn cmd_set_depth_bounds(
        &self,
        command_buffer: CommandBuffer,
        min_depth_bounds: f32,
        max_depth_bounds: f32,
    ) {
        unsafe { (self.cmd_set_depth_bounds)(command_buffer, min_depth_bounds, max_depth_bounds) }
    }
    pub unsafe fn cmd_set_stencil_compare_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        compare_mask: u32,
    ) {
        unsafe { (self.cmd_set_stencil_compare_mask)(command_buffer, face_mask, compare_mask) }
    }
    pub unsafe fn cmd_set_stencil_write_mask(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        write_mask: u32,
    ) {
        unsafe { (self.cmd_set_stencil_write_mask)(command_buffer, face_mask, write_mask) }
    }
    pub unsafe fn cmd_set_stencil_reference(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        reference: u32,
    ) {
        unsafe { (self.cmd_set_stencil_reference)(command_buffer, face_mask, reference) }
    }
    pub unsafe fn cmd_bind_index_buffer(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe { (self.cmd_bind_index_buffer)(command_buffer, buffer, offset, index_type) }
    }
    pub unsafe fn cmd_bind_vertex_buffers(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
    ) {
        unsafe {
            (self.cmd_bind_vertex_buffers)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_draw(
        &self,
        command_buffer: CommandBuffer,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            (self.cmd_draw)(
                command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        }
    }
    pub unsafe fn cmd_draw_indexed(
        &self,
        command_buffer: CommandBuffer,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            (self.cmd_draw_indexed)(
                command_buffer,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        }
    }
    pub unsafe fn cmd_draw_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe { (self.cmd_draw_indirect)(command_buffer, buffer, offset, draw_count, stride) }
    }
    pub unsafe fn cmd_draw_indexed_indirect(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indexed_indirect)(command_buffer, buffer, offset, draw_count, stride)
        }
    }
    pub unsafe fn cmd_blit_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter,
    ) {
        unsafe {
            (self.cmd_blit_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
                filter,
            )
        }
    }
    pub unsafe fn cmd_clear_depth_stencil_image(
        &self,
        command_buffer: CommandBuffer,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            (self.cmd_clear_depth_stencil_image)(
                command_buffer,
                image,
                image_layout,
                depth_stencil,
                ranges.len().try_into().unwrap(),
                ranges.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_clear_attachments(
        &self,
        command_buffer: CommandBuffer,
        attachments: &[ClearAttachment],
        rects: &[ClearRect],
    ) {
        unsafe {
            (self.cmd_clear_attachments)(
                command_buffer,
                attachments.len().try_into().unwrap(),
                attachments.as_ptr() as _,
                rects.len().try_into().unwrap(),
                rects.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_resolve_image(
        &self,
        command_buffer: CommandBuffer,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve],
    ) {
        unsafe {
            (self.cmd_resolve_image)(
                command_buffer,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len().try_into().unwrap(),
                regions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_begin_render_pass(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        unsafe { (self.cmd_begin_render_pass)(command_buffer, render_pass_begin, contents) }
    }
    pub unsafe fn cmd_next_subpass(
        &self,
        command_buffer: CommandBuffer,
        contents: SubpassContents,
    ) {
        unsafe { (self.cmd_next_subpass)(command_buffer, contents) }
    }
    pub unsafe fn cmd_end_render_pass(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_render_pass)(command_buffer) }
    }
}
