#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct EntryFn {
    create_instance: PFN_vkCreateInstance,
    enumerate_instance_layer_properties: PFN_vkEnumerateInstanceLayerProperties,
    enumerate_instance_extension_properties: PFN_vkEnumerateInstanceExtensionProperties,
}
impl EntryFn {
    pub unsafe fn create_instance(
        &self,
        create_info: &InstanceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        instance: &mut Instance,
    ) -> Result {
        unsafe { (self.create_instance)(create_info, allocator.to_raw_ptr(), instance) }
    }
    pub unsafe fn enumerate_instance_layer_properties(
        &self,
        properties: impl ExtendUninit<LayerProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.enumerate_instance_layer_properties)(property_count, properties as _)
            })
        }
    }
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&CStr>,
        properties: impl ExtendUninit<ExtensionProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.enumerate_instance_extension_properties)(
                    layer_name.to_raw_ptr(),
                    property_count,
                    properties as _,
                )
            })
        }
    }
}
pub struct InstanceFn {
    destroy_instance: PFN_vkDestroyInstance,
    enumerate_physical_devices: PFN_vkEnumeratePhysicalDevices,
    get_instance_proc_addr: PFN_vkGetInstanceProcAddr,
    get_physical_device_properties: PFN_vkGetPhysicalDeviceProperties,
    get_physical_device_queue_family_properties: PFN_vkGetPhysicalDeviceQueueFamilyProperties,
    get_physical_device_memory_properties: PFN_vkGetPhysicalDeviceMemoryProperties,
    get_physical_device_features: PFN_vkGetPhysicalDeviceFeatures,
    get_physical_device_format_properties: PFN_vkGetPhysicalDeviceFormatProperties,
    get_physical_device_image_format_properties: PFN_vkGetPhysicalDeviceImageFormatProperties,
    create_device: PFN_vkCreateDevice,
    enumerate_device_layer_properties: PFN_vkEnumerateDeviceLayerProperties,
    enumerate_device_extension_properties: PFN_vkEnumerateDeviceExtensionProperties,
    get_physical_device_sparse_image_format_properties:
        PFN_vkGetPhysicalDeviceSparseImageFormatProperties,
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
    ) -> Result {
        unsafe {
            try_extend_uninit(
                physical_devices,
                |physical_device_count, physical_devices| {
                    (self.enumerate_physical_devices)(
                        instance,
                        physical_device_count,
                        physical_devices as _,
                    )
                },
            )
        }
    }
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: Instance,
        name: &CStr,
    ) -> PFN_vkVoidFunction {
        unsafe { (self.get_instance_proc_addr)(instance, name.as_ptr() as _) }
    }
    pub unsafe fn get_physical_device_properties(
        &self,
        physical_device: PhysicalDevice,
        properties: &mut PhysicalDeviceProperties,
    ) {
        unsafe { (self.get_physical_device_properties)(physical_device, properties) }
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
        memory_properties: &mut PhysicalDeviceMemoryProperties,
    ) {
        unsafe { (self.get_physical_device_memory_properties)(physical_device, memory_properties) }
    }
    pub unsafe fn get_physical_device_features(
        &self,
        physical_device: PhysicalDevice,
        features: &mut PhysicalDeviceFeatures,
    ) {
        unsafe { (self.get_physical_device_features)(physical_device, features) }
    }
    pub unsafe fn get_physical_device_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        format_properties: &mut FormatProperties,
    ) {
        unsafe {
            (self.get_physical_device_format_properties)(physical_device, format, format_properties)
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
        image_format_properties: &mut ImageFormatProperties,
    ) -> Result {
        unsafe {
            (self.get_physical_device_image_format_properties)(
                physical_device,
                format,
                ty,
                tiling,
                usage,
                flags,
                image_format_properties,
            )
        }
    }
    pub unsafe fn create_device(
        &self,
        physical_device: PhysicalDevice,
        create_info: &DeviceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        device: &mut Device,
    ) -> Result {
        unsafe {
            (self.create_device)(physical_device, create_info, allocator.to_raw_ptr(), device)
        }
    }
    pub unsafe fn enumerate_device_layer_properties(
        &self,
        physical_device: PhysicalDevice,
        properties: impl ExtendUninit<LayerProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.enumerate_device_layer_properties)(
                    physical_device,
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn enumerate_device_extension_properties(
        &self,
        physical_device: PhysicalDevice,
        layer_name: Option<&CStr>,
        properties: impl ExtendUninit<ExtensionProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |property_count, properties| {
                (self.enumerate_device_extension_properties)(
                    physical_device,
                    layer_name.to_raw_ptr(),
                    property_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_physical_device_sparse_image_format_properties(
        &self,
        physical_device: PhysicalDevice,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlags,
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
    get_buffer_memory_requirements: PFN_vkGetBufferMemoryRequirements,
    bind_buffer_memory: PFN_vkBindBufferMemory,
    get_image_memory_requirements: PFN_vkGetImageMemoryRequirements,
    bind_image_memory: PFN_vkBindImageMemory,
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
        queue: &mut Queue,
    ) {
        unsafe { (self.get_device_queue)(device, queue_family_index, queue_index, queue) }
    }
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submits: &[SubmitInfo],
        fence: Fence,
    ) -> Result {
        unsafe {
            (self.queue_submit)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            )
        }
    }
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> Result {
        unsafe { (self.queue_wait_idle)(queue) }
    }
    pub unsafe fn device_wait_idle(&self, device: Device) -> Result {
        unsafe { (self.device_wait_idle)(device) }
    }
    pub unsafe fn allocate_memory(
        &self,
        device: Device,
        allocate_info: &MemoryAllocateInfo,
        allocator: Option<&AllocationCallbacks>,
        memory: &mut DeviceMemory,
    ) -> Result {
        unsafe { (self.allocate_memory)(device, allocate_info, allocator.to_raw_ptr(), memory) }
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
    ) -> Result {
        unsafe { (self.map_memory)(device, memory, offset, size, flags, data) }
    }
    pub unsafe fn unmap_memory(&self, device: Device, memory: DeviceMemory) {
        unsafe { (self.unmap_memory)(device, memory) }
    }
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: Device,
        memory_ranges: &[MappedMemoryRange],
    ) -> Result {
        unsafe {
            (self.flush_mapped_memory_ranges)(
                device,
                memory_ranges.len().try_into().unwrap(),
                memory_ranges.as_ptr() as _,
            )
        }
    }
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: Device,
        memory_ranges: &[MappedMemoryRange],
    ) -> Result {
        unsafe {
            (self.invalidate_mapped_memory_ranges)(
                device,
                memory_ranges.len().try_into().unwrap(),
                memory_ranges.as_ptr() as _,
            )
        }
    }
    pub unsafe fn get_device_memory_commitment(
        &self,
        device: Device,
        memory: DeviceMemory,
        committed_memory_in_bytes: &mut DeviceSize,
    ) {
        unsafe { (self.get_device_memory_commitment)(device, memory, committed_memory_in_bytes) }
    }
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: Device,
        buffer: Buffer,
        memory_requirements: &mut MemoryRequirements,
    ) {
        unsafe { (self.get_buffer_memory_requirements)(device, buffer, memory_requirements) }
    }
    pub unsafe fn bind_buffer_memory(
        &self,
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result {
        unsafe { (self.bind_buffer_memory)(device, buffer, memory, memory_offset) }
    }
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: Device,
        image: Image,
        memory_requirements: &mut MemoryRequirements,
    ) {
        unsafe { (self.get_image_memory_requirements)(device, image, memory_requirements) }
    }
    pub unsafe fn bind_image_memory(
        &self,
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result {
        unsafe { (self.bind_image_memory)(device, image, memory, memory_offset) }
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
    ) -> Result {
        unsafe {
            (self.queue_bind_sparse)(
                queue,
                bind_info.len().try_into().unwrap(),
                bind_info.as_ptr() as _,
                fence,
            )
        }
    }
    pub unsafe fn create_fence(
        &self,
        device: Device,
        create_info: &FenceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        fence: &mut Fence,
    ) -> Result {
        unsafe { (self.create_fence)(device, create_info, allocator.to_raw_ptr(), fence) }
    }
    pub unsafe fn destroy_fence(
        &self,
        device: Device,
        fence: Fence,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_fence)(device, fence, allocator.to_raw_ptr()) }
    }
    pub unsafe fn reset_fences(&self, device: Device, fences: &[Fence]) -> Result {
        unsafe {
            (self.reset_fences)(
                device,
                fences.len().try_into().unwrap(),
                fences.as_ptr() as _,
            )
        }
    }
    pub unsafe fn get_fence_status(&self, device: Device, fence: Fence) -> Result {
        unsafe { (self.get_fence_status)(device, fence) }
    }
    pub unsafe fn wait_for_fences(
        &self,
        device: Device,
        fences: &[Fence],
        wait_all: Bool32,
        timeout: u64,
    ) -> Result {
        unsafe {
            (self.wait_for_fences)(
                device,
                fences.len().try_into().unwrap(),
                fences.as_ptr() as _,
                wait_all,
                timeout,
            )
        }
    }
    pub unsafe fn create_semaphore(
        &self,
        device: Device,
        create_info: &SemaphoreCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        semaphore: &mut Semaphore,
    ) -> Result {
        unsafe { (self.create_semaphore)(device, create_info, allocator.to_raw_ptr(), semaphore) }
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
        query_pool: &mut QueryPool,
    ) -> Result {
        unsafe { (self.create_query_pool)(device, create_info, allocator.to_raw_ptr(), query_pool) }
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
    ) -> Result {
        unsafe {
            (self.get_query_pool_results)(
                device,
                query_pool,
                first_query,
                query_count,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
                stride,
                flags,
            )
        }
    }
    pub unsafe fn create_buffer(
        &self,
        device: Device,
        create_info: &BufferCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        buffer: &mut Buffer,
    ) -> Result {
        unsafe { (self.create_buffer)(device, create_info, allocator.to_raw_ptr(), buffer) }
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
        image: &mut Image,
    ) -> Result {
        unsafe { (self.create_image)(device, create_info, allocator.to_raw_ptr(), image) }
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
        layout: &mut SubresourceLayout,
    ) {
        unsafe { (self.get_image_subresource_layout)(device, image, subresource, layout) }
    }
    pub unsafe fn create_image_view(
        &self,
        device: Device,
        create_info: &ImageViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        view: &mut ImageView,
    ) -> Result {
        unsafe { (self.create_image_view)(device, create_info, allocator.to_raw_ptr(), view) }
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
        command_pool: &mut CommandPool,
    ) -> Result {
        unsafe {
            (self.create_command_pool)(device, create_info, allocator.to_raw_ptr(), command_pool)
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
    ) -> Result {
        unsafe { (self.reset_command_pool)(device, command_pool, flags) }
    }
    pub unsafe fn allocate_command_buffers(
        &self,
        device: Device,
        allocate_info: &CommandBufferAllocateInfo,
        command_buffers: &mut [CommandBuffer],
    ) -> Result {
        unsafe {
            (self.allocate_command_buffers)(
                device,
                allocate_info,
                command_buffers.as_mut_ptr() as _,
            )
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
    ) -> Result {
        unsafe { (self.begin_command_buffer)(command_buffer, begin_info) }
    }
    pub unsafe fn end_command_buffer(&self, command_buffer: CommandBuffer) -> Result {
        unsafe { (self.end_command_buffer)(command_buffer) }
    }
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> Result {
        unsafe { (self.reset_command_buffer)(command_buffer, flags) }
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
        pipeline_stage: PipelineStageFlags,
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
}
