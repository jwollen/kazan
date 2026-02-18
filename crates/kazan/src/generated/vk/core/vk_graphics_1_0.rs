#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
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
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo],
        allocator: &AllocationCallbacks,
        pipelines: &mut [Pipeline],
    ) -> Result {
        unsafe {
            (self.create_graphics_pipelines)(
                device,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator,
                pipelines.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn create_framebuffer(
        &self,
        device: Device,
        create_info: &FramebufferCreateInfo,
        allocator: &AllocationCallbacks,
        framebuffer: &mut Framebuffer,
    ) -> Result {
        unsafe { (self.create_framebuffer)(device, create_info, allocator, framebuffer) }
    }
    pub unsafe fn destroy_framebuffer(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_framebuffer)(device, framebuffer, allocator) }
    }
    pub unsafe fn create_render_pass(
        &self,
        device: Device,
        create_info: &RenderPassCreateInfo,
        allocator: &AllocationCallbacks,
        render_pass: &mut RenderPass,
    ) -> Result {
        unsafe { (self.create_render_pass)(device, create_info, allocator, render_pass) }
    }
    pub unsafe fn destroy_render_pass(
        &self,
        device: Device,
        render_pass: RenderPass,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_render_pass)(device, render_pass, allocator) }
    }
    pub unsafe fn get_render_area_granularity(
        &self,
        device: Device,
        render_pass: RenderPass,
        granularity: &mut Extent2D,
    ) {
        unsafe { (self.get_render_area_granularity)(device, render_pass, granularity) }
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
