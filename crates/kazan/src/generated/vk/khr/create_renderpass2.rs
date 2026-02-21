#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_render_pass2: PFN_vkCreateRenderPass2,
    cmd_begin_render_pass2: PFN_vkCmdBeginRenderPass2,
    cmd_next_subpass2: PFN_vkCmdNextSubpass2,
    cmd_end_render_pass2: PFN_vkCmdEndRenderPass2,
}
impl DeviceFn {
    pub unsafe fn create_render_pass2_khr(
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
    pub unsafe fn cmd_begin_render_pass2_khr(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) {
        unsafe {
            (self.cmd_begin_render_pass2)(command_buffer, render_pass_begin, subpass_begin_info)
        }
    }
    pub unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe { (self.cmd_next_subpass2)(command_buffer, subpass_begin_info, subpass_end_info) }
    }
    pub unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe { (self.cmd_end_render_pass2)(command_buffer, subpass_end_info) }
    }
}
