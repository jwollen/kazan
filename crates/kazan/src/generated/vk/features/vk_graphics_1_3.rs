#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
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
    cmd_blit_image2: PFN_vkCmdBlitImage2,
    cmd_resolve_image2: PFN_vkCmdResolveImage2,
    cmd_begin_rendering: PFN_vkCmdBeginRendering,
    cmd_end_rendering: PFN_vkCmdEndRendering,
}
impl DeviceFn {
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
}
