#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
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
}
impl DeviceFn {
    pub unsafe fn create_event(
        &self,
        device: Device,
        create_info: &EventCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        event: &mut Event,
    ) -> Result {
        unsafe { (self.create_event)(device, create_info, allocator.to_raw_ptr(), event) }
    }
    pub unsafe fn destroy_event(
        &self,
        device: Device,
        event: Event,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_event)(device, event, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_event_status(&self, device: Device, event: Event) -> Result {
        unsafe { (self.get_event_status)(device, event) }
    }
    pub unsafe fn set_event(&self, device: Device, event: Event) -> Result {
        unsafe { (self.set_event)(device, event) }
    }
    pub unsafe fn reset_event(&self, device: Device, event: Event) -> Result {
        unsafe { (self.reset_event)(device, event) }
    }
    pub unsafe fn create_buffer_view(
        &self,
        device: Device,
        create_info: &BufferViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        view: &mut BufferView,
    ) -> Result {
        unsafe { (self.create_buffer_view)(device, create_info, allocator.to_raw_ptr(), view) }
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
        shader_module: &mut ShaderModule,
    ) -> Result {
        unsafe {
            (self.create_shader_module)(device, create_info, allocator.to_raw_ptr(), shader_module)
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
        pipeline_cache: &mut PipelineCache,
    ) -> Result {
        unsafe {
            (self.create_pipeline_cache)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                pipeline_cache,
            )
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
    ) -> Result {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                (self.get_pipeline_cache_data)(device, pipeline_cache, data_size, data as _)
            })
        }
    }
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: Device,
        dst_cache: PipelineCache,
        src_caches: &[PipelineCache],
    ) -> Result {
        unsafe {
            (self.merge_pipeline_caches)(
                device,
                dst_cache,
                src_caches.len().try_into().unwrap(),
                src_caches.as_ptr() as _,
            )
        }
    }
    pub unsafe fn create_compute_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result {
        unsafe {
            (self.create_compute_pipelines)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            )
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
        pipeline_layout: &mut PipelineLayout,
    ) -> Result {
        unsafe {
            (self.create_pipeline_layout)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                pipeline_layout,
            )
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
        sampler: &mut Sampler,
    ) -> Result {
        unsafe { (self.create_sampler)(device, create_info, allocator.to_raw_ptr(), sampler) }
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
        set_layout: &mut DescriptorSetLayout,
    ) -> Result {
        unsafe {
            (self.create_descriptor_set_layout)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                set_layout,
            )
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
        descriptor_pool: &mut DescriptorPool,
    ) -> Result {
        unsafe {
            (self.create_descriptor_pool)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                descriptor_pool,
            )
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
    ) -> Result {
        unsafe { (self.reset_descriptor_pool)(device, descriptor_pool, flags) }
    }
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: Device,
        allocate_info: &DescriptorSetAllocateInfo,
        descriptor_sets: &mut [DescriptorSet],
    ) -> Result {
        unsafe {
            (self.allocate_descriptor_sets)(
                device,
                allocate_info,
                descriptor_sets.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn free_descriptor_sets(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_sets: &[DescriptorSet],
    ) -> Result {
        unsafe {
            (self.free_descriptor_sets)(
                device,
                descriptor_pool,
                descriptor_sets.len().try_into().unwrap(),
                descriptor_sets.as_ptr() as _,
            )
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
}
