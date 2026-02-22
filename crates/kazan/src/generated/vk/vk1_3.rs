#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_tool_properties: transmute(
                    load(c"vkGetPhysicalDeviceToolProperties").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: PhysicalDevice,
        tool_properties: impl ExtendUninit<PhysicalDeviceToolProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(tool_properties, |tool_count, tool_properties| {
                (self.get_physical_device_tool_properties)(
                    physical_device,
                    tool_count,
                    tool_properties as _,
                )
            })
        }
    }
}
pub struct DeviceFn {
    create_private_data_slot: PFN_vkCreatePrivateDataSlot,
    destroy_private_data_slot: PFN_vkDestroyPrivateDataSlot,
    set_private_data: PFN_vkSetPrivateData,
    get_private_data: PFN_vkGetPrivateData,
    cmd_pipeline_barrier2: PFN_vkCmdPipelineBarrier2,
    cmd_write_timestamp2: PFN_vkCmdWriteTimestamp2,
    queue_submit2: PFN_vkQueueSubmit2,
    cmd_copy_buffer2: PFN_vkCmdCopyBuffer2,
    cmd_copy_image2: PFN_vkCmdCopyImage2,
    cmd_copy_buffer_to_image2: PFN_vkCmdCopyBufferToImage2,
    cmd_copy_image_to_buffer2: PFN_vkCmdCopyImageToBuffer2,
    get_device_buffer_memory_requirements: PFN_vkGetDeviceBufferMemoryRequirements,
    get_device_image_memory_requirements: PFN_vkGetDeviceImageMemoryRequirements,
    get_device_image_sparse_memory_requirements: PFN_vkGetDeviceImageSparseMemoryRequirements,
    cmd_set_event2: PFN_vkCmdSetEvent2,
    cmd_reset_event2: PFN_vkCmdResetEvent2,
    cmd_wait_events2: PFN_vkCmdWaitEvents2,
    cmd_blit_image2: PFN_vkCmdBlitImage2,
    cmd_resolve_image2: PFN_vkCmdResolveImage2,
    cmd_begin_rendering: PFN_vkCmdBeginRendering,
    cmd_end_rendering: PFN_vkCmdEndRendering,
    cmd_set_cull_mode: PFN_vkCmdSetCullMode,
    cmd_set_front_face: PFN_vkCmdSetFrontFace,
    cmd_set_primitive_topology: PFN_vkCmdSetPrimitiveTopology,
    cmd_set_viewport_with_count: PFN_vkCmdSetViewportWithCount,
    cmd_set_scissor_with_count: PFN_vkCmdSetScissorWithCount,
    cmd_bind_vertex_buffers2: PFN_vkCmdBindVertexBuffers2,
    cmd_set_depth_test_enable: PFN_vkCmdSetDepthTestEnable,
    cmd_set_depth_write_enable: PFN_vkCmdSetDepthWriteEnable,
    cmd_set_depth_compare_op: PFN_vkCmdSetDepthCompareOp,
    cmd_set_depth_bounds_test_enable: PFN_vkCmdSetDepthBoundsTestEnable,
    cmd_set_stencil_test_enable: PFN_vkCmdSetStencilTestEnable,
    cmd_set_stencil_op: PFN_vkCmdSetStencilOp,
    cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
    cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
    cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_private_data_slot: transmute(
                    load(c"vkCreatePrivateDataSlot").ok_or(LoadingError)?,
                ),
                destroy_private_data_slot: transmute(
                    load(c"vkDestroyPrivateDataSlot").ok_or(LoadingError)?,
                ),
                set_private_data: transmute(load(c"vkSetPrivateData").ok_or(LoadingError)?),
                get_private_data: transmute(load(c"vkGetPrivateData").ok_or(LoadingError)?),
                cmd_pipeline_barrier2: transmute(
                    load(c"vkCmdPipelineBarrier2").ok_or(LoadingError)?,
                ),
                cmd_write_timestamp2: transmute(load(c"vkCmdWriteTimestamp2").ok_or(LoadingError)?),
                queue_submit2: transmute(load(c"vkQueueSubmit2").ok_or(LoadingError)?),
                cmd_copy_buffer2: transmute(load(c"vkCmdCopyBuffer2").ok_or(LoadingError)?),
                cmd_copy_image2: transmute(load(c"vkCmdCopyImage2").ok_or(LoadingError)?),
                cmd_copy_buffer_to_image2: transmute(
                    load(c"vkCmdCopyBufferToImage2").ok_or(LoadingError)?,
                ),
                cmd_copy_image_to_buffer2: transmute(
                    load(c"vkCmdCopyImageToBuffer2").ok_or(LoadingError)?,
                ),
                get_device_buffer_memory_requirements: transmute(
                    load(c"vkGetDeviceBufferMemoryRequirements").ok_or(LoadingError)?,
                ),
                get_device_image_memory_requirements: transmute(
                    load(c"vkGetDeviceImageMemoryRequirements").ok_or(LoadingError)?,
                ),
                get_device_image_sparse_memory_requirements: transmute(
                    load(c"vkGetDeviceImageSparseMemoryRequirements").ok_or(LoadingError)?,
                ),
                cmd_set_event2: transmute(load(c"vkCmdSetEvent2").ok_or(LoadingError)?),
                cmd_reset_event2: transmute(load(c"vkCmdResetEvent2").ok_or(LoadingError)?),
                cmd_wait_events2: transmute(load(c"vkCmdWaitEvents2").ok_or(LoadingError)?),
                cmd_blit_image2: transmute(load(c"vkCmdBlitImage2").ok_or(LoadingError)?),
                cmd_resolve_image2: transmute(load(c"vkCmdResolveImage2").ok_or(LoadingError)?),
                cmd_begin_rendering: transmute(load(c"vkCmdBeginRendering").ok_or(LoadingError)?),
                cmd_end_rendering: transmute(load(c"vkCmdEndRendering").ok_or(LoadingError)?),
                cmd_set_cull_mode: transmute(load(c"vkCmdSetCullMode").ok_or(LoadingError)?),
                cmd_set_front_face: transmute(load(c"vkCmdSetFrontFace").ok_or(LoadingError)?),
                cmd_set_primitive_topology: transmute(
                    load(c"vkCmdSetPrimitiveTopology").ok_or(LoadingError)?,
                ),
                cmd_set_viewport_with_count: transmute(
                    load(c"vkCmdSetViewportWithCount").ok_or(LoadingError)?,
                ),
                cmd_set_scissor_with_count: transmute(
                    load(c"vkCmdSetScissorWithCount").ok_or(LoadingError)?,
                ),
                cmd_bind_vertex_buffers2: transmute(
                    load(c"vkCmdBindVertexBuffers2").ok_or(LoadingError)?,
                ),
                cmd_set_depth_test_enable: transmute(
                    load(c"vkCmdSetDepthTestEnable").ok_or(LoadingError)?,
                ),
                cmd_set_depth_write_enable: transmute(
                    load(c"vkCmdSetDepthWriteEnable").ok_or(LoadingError)?,
                ),
                cmd_set_depth_compare_op: transmute(
                    load(c"vkCmdSetDepthCompareOp").ok_or(LoadingError)?,
                ),
                cmd_set_depth_bounds_test_enable: transmute(
                    load(c"vkCmdSetDepthBoundsTestEnable").ok_or(LoadingError)?,
                ),
                cmd_set_stencil_test_enable: transmute(
                    load(c"vkCmdSetStencilTestEnable").ok_or(LoadingError)?,
                ),
                cmd_set_stencil_op: transmute(load(c"vkCmdSetStencilOp").ok_or(LoadingError)?),
                cmd_set_rasterizer_discard_enable: transmute(
                    load(c"vkCmdSetRasterizerDiscardEnable").ok_or(LoadingError)?,
                ),
                cmd_set_depth_bias_enable: transmute(
                    load(c"vkCmdSetDepthBiasEnable").ok_or(LoadingError)?,
                ),
                cmd_set_primitive_restart_enable: transmute(
                    load(c"vkCmdSetPrimitiveRestartEnable").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_private_data_slot(
        &self,
        device: Device,
        create_info: &PrivateDataSlotCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        private_data_slot: &mut PrivateDataSlot,
    ) -> Result {
        unsafe {
            (self.create_private_data_slot)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                private_data_slot,
            )
        }
    }
    pub unsafe fn destroy_private_data_slot(
        &self,
        device: Device,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_private_data_slot)(device, private_data_slot, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn set_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> Result {
        unsafe {
            (self.set_private_data)(device, object_type, object_handle, private_data_slot, data)
        }
    }
    pub unsafe fn get_private_data(
        &self,
        device: Device,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: &mut u64,
    ) {
        unsafe {
            (self.get_private_data)(device, object_type, object_handle, private_data_slot, data)
        }
    }
    pub unsafe fn cmd_pipeline_barrier2(
        &self,
        command_buffer: CommandBuffer,
        dependency_info: &DependencyInfo,
    ) {
        unsafe { (self.cmd_pipeline_barrier2)(command_buffer, dependency_info) }
    }
    pub unsafe fn cmd_write_timestamp2(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe { (self.cmd_write_timestamp2)(command_buffer, stage, query_pool, query) }
    }
    pub unsafe fn queue_submit2(
        &self,
        queue: Queue,
        submits: &[SubmitInfo2],
        fence: Fence,
    ) -> Result {
        unsafe {
            (self.queue_submit2)(
                queue,
                submits.len().try_into().unwrap(),
                submits.as_ptr() as _,
                fence,
            )
        }
    }
    pub unsafe fn cmd_copy_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_info: &CopyBufferInfo2,
    ) {
        unsafe { (self.cmd_copy_buffer2)(command_buffer, copy_buffer_info) }
    }
    pub unsafe fn cmd_copy_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_info: &CopyImageInfo2,
    ) {
        unsafe { (self.cmd_copy_image2)(command_buffer, copy_image_info) }
    }
    pub unsafe fn cmd_copy_buffer_to_image2(
        &self,
        command_buffer: CommandBuffer,
        copy_buffer_to_image_info: &CopyBufferToImageInfo2,
    ) {
        unsafe { (self.cmd_copy_buffer_to_image2)(command_buffer, copy_buffer_to_image_info) }
    }
    pub unsafe fn cmd_copy_image_to_buffer2(
        &self,
        command_buffer: CommandBuffer,
        copy_image_to_buffer_info: &CopyImageToBufferInfo2,
    ) {
        unsafe { (self.cmd_copy_image_to_buffer2)(command_buffer, copy_image_to_buffer_info) }
    }
    pub unsafe fn get_device_buffer_memory_requirements(
        &self,
        device: Device,
        info: &DeviceBufferMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_device_buffer_memory_requirements)(device, info, memory_requirements) }
    }
    pub unsafe fn get_device_image_memory_requirements(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_device_image_memory_requirements)(device, info, memory_requirements) }
    }
    pub unsafe fn get_device_image_sparse_memory_requirements(
        &self,
        device: Device,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirements: impl ExtendUninit<SparseImageMemoryRequirements2>,
    ) {
        unsafe {
            extend_uninit(
                sparse_memory_requirements,
                |sparse_memory_requirement_count, sparse_memory_requirements| {
                    (self.get_device_image_sparse_memory_requirements)(
                        device,
                        info,
                        sparse_memory_requirement_count,
                        sparse_memory_requirements as _,
                    )
                },
            )
        }
    }
    pub unsafe fn cmd_set_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        dependency_info: &DependencyInfo,
    ) {
        unsafe { (self.cmd_set_event2)(command_buffer, event, dependency_info) }
    }
    pub unsafe fn cmd_reset_event2(
        &self,
        command_buffer: CommandBuffer,
        event: Event,
        stage_mask: PipelineStageFlags2,
    ) {
        unsafe { (self.cmd_reset_event2)(command_buffer, event, stage_mask) }
    }
    pub unsafe fn cmd_wait_events2(
        &self,
        command_buffer: CommandBuffer,
        events: &[Event],
        dependency_infos: &[DependencyInfo],
    ) {
        unsafe {
            (self.cmd_wait_events2)(
                command_buffer,
                events.len().try_into().unwrap(),
                events.as_ptr() as _,
                dependency_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_blit_image2(
        &self,
        command_buffer: CommandBuffer,
        blit_image_info: &BlitImageInfo2,
    ) {
        unsafe { (self.cmd_blit_image2)(command_buffer, blit_image_info) }
    }
    pub unsafe fn cmd_resolve_image2(
        &self,
        command_buffer: CommandBuffer,
        resolve_image_info: &ResolveImageInfo2,
    ) {
        unsafe { (self.cmd_resolve_image2)(command_buffer, resolve_image_info) }
    }
    pub unsafe fn cmd_begin_rendering(
        &self,
        command_buffer: CommandBuffer,
        rendering_info: &RenderingInfo,
    ) {
        unsafe { (self.cmd_begin_rendering)(command_buffer, rendering_info) }
    }
    pub unsafe fn cmd_end_rendering(&self, command_buffer: CommandBuffer) {
        unsafe { (self.cmd_end_rendering)(command_buffer) }
    }
    pub unsafe fn cmd_set_cull_mode(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        unsafe { (self.cmd_set_cull_mode)(command_buffer, cull_mode) }
    }
    pub unsafe fn cmd_set_front_face(&self, command_buffer: CommandBuffer, front_face: FrontFace) {
        unsafe { (self.cmd_set_front_face)(command_buffer, front_face) }
    }
    pub unsafe fn cmd_set_primitive_topology(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        unsafe { (self.cmd_set_primitive_topology)(command_buffer, primitive_topology) }
    }
    pub unsafe fn cmd_set_viewport_with_count(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.cmd_set_viewport_with_count)(
                command_buffer,
                viewports.len().try_into().unwrap(),
                viewports.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_scissor_with_count(
        &self,
        command_buffer: CommandBuffer,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_scissor_with_count)(
                command_buffer,
                scissors.len().try_into().unwrap(),
                scissors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_vertex_buffers2(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.cmd_bind_vertex_buffers2)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.to_raw_ptr(),
                strides.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn cmd_set_depth_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_test_enable)(command_buffer, depth_test_enable) }
    }
    pub unsafe fn cmd_set_depth_write_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_write_enable)(command_buffer, depth_write_enable) }
    }
    pub unsafe fn cmd_set_depth_compare_op(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        unsafe { (self.cmd_set_depth_compare_op)(command_buffer, depth_compare_op) }
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_bounds_test_enable)(command_buffer, depth_bounds_test_enable) }
    }
    pub unsafe fn cmd_set_stencil_test_enable(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_stencil_test_enable)(command_buffer, stencil_test_enable) }
    }
    pub unsafe fn cmd_set_stencil_op(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            (self.cmd_set_stencil_op)(
                command_buffer,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        }
    }
    pub unsafe fn cmd_set_rasterizer_discard_enable(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_rasterizer_discard_enable)(command_buffer, rasterizer_discard_enable)
        }
    }
    pub unsafe fn cmd_set_depth_bias_enable(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_bias_enable)(command_buffer, depth_bias_enable) }
    }
    pub unsafe fn cmd_set_primitive_restart_enable(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_primitive_restart_enable)(command_buffer, primitive_restart_enable) }
    }
}
