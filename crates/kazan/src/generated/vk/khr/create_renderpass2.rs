#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    create_render_pass2_khr: PFN_vkCreateRenderPass2,
    cmd_begin_render_pass2_khr: PFN_vkCmdBeginRenderPass2,
    cmd_next_subpass2_khr: PFN_vkCmdNextSubpass2,
    cmd_end_render_pass2_khr: PFN_vkCmdEndRenderPass2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_render_pass2_khr: transmute(
                    load(c"vkCreateRenderPass2KHR").ok_or(LoadingError)?,
                ),
                cmd_begin_render_pass2_khr: transmute(
                    load(c"vkCmdBeginRenderPass2KHR").ok_or(LoadingError)?,
                ),
                cmd_next_subpass2_khr: transmute(
                    load(c"vkCmdNextSubpass2KHR").ok_or(LoadingError)?,
                ),
                cmd_end_render_pass2_khr: transmute(
                    load(c"vkCmdEndRenderPass2KHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_render_pass2_khr(
        &self,
        device: Device,
        create_info: &RenderPassCreateInfo2<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<RenderPass> {
        unsafe {
            let mut render_pass = core::mem::MaybeUninit::uninit();
            let result = (self.create_render_pass2_khr)(
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
    pub unsafe fn cmd_begin_render_pass2_khr(
        &self,
        command_buffer: CommandBuffer,
        render_pass_begin: &RenderPassBeginInfo<'_>,
        subpass_begin_info: &SubpassBeginInfo<'_>,
    ) {
        unsafe {
            (self.cmd_begin_render_pass2_khr)(command_buffer, render_pass_begin, subpass_begin_info)
        }
    }
    pub unsafe fn cmd_next_subpass2_khr(
        &self,
        command_buffer: CommandBuffer,
        subpass_begin_info: &SubpassBeginInfo<'_>,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        unsafe {
            (self.cmd_next_subpass2_khr)(command_buffer, subpass_begin_info, subpass_end_info)
        }
    }
    pub unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        unsafe { (self.cmd_end_render_pass2_khr)(command_buffer, subpass_end_info) }
    }
}
