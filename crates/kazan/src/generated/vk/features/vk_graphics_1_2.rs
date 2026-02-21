#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_render_pass2: PFN_vkCreateRenderPass2,
    cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
    cmd_draw_indirect_count: PFN_vkCmdDrawIndirectCount,
    cmd_draw_indexed_indirect_count: PFN_vkCmdDrawIndexedIndirectCount,
}
impl DeviceFn {
    pub unsafe fn create_render_pass2(
        &self,
        device: Device,
        create_info: &RenderPassCreateInfo2,
        allocator: Option<&AllocationCallbacks>,
        render_pass: &mut RenderPass,
    ) -> Result {
        unsafe {
            (self.create_render_pass2)(device, create_info, allocator.to_raw_ptr(), render_pass)
        }
    }
    pub unsafe fn cmd_begin_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) {
        unsafe {
            (self.cmd_begin_render_pass2)(command_buffer, render_pass_begin, subpass_begin_info)
        }
    }
    pub unsafe fn cmd_next_subpass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe { (self.cmd_next_subpass2)(command_buffer, subpass_begin_info, subpass_end_info) }
    }
    pub unsafe fn cmd_end_render_pass2(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe { (self.cmd_end_render_pass2)(command_buffer, subpass_end_info) }
    }
    pub unsafe fn cmd_draw_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indirect_count)(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
    pub unsafe fn cmd_draw_indexed_indirect_count(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indexed_indirect_count)(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
}
