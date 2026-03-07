#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_create_renderpass2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAttachmentDescription2KHR.html>
    pub type AttachmentDescription2KHR<'a> = AttachmentDescription2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAttachmentReference2KHR.html>
    pub type AttachmentReference2KHR<'a> = AttachmentReference2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassDescription2KHR.html>
    pub type SubpassDescription2KHR<'a> = SubpassDescription2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassDependency2KHR.html>
    pub type SubpassDependency2KHR<'a> = SubpassDependency2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassCreateInfo2KHR.html>
    pub type RenderPassCreateInfo2KHR<'a> = RenderPassCreateInfo2<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassBeginInfoKHR.html>
    pub type SubpassBeginInfoKHR<'a> = SubpassBeginInfo<'a>;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSubpassEndInfoKHR.html>
    pub type SubpassEndInfoKHR<'a> = SubpassEndInfo<'a>;
    pub type PFN_vkCreateRenderPass2KHR = PFN_vkCreateRenderPass2;
    pub type PFN_vkCmdBeginRenderPass2KHR = PFN_vkCmdBeginRenderPass2;
    pub type PFN_vkCmdNextSubpass2KHR = PFN_vkCmdNextSubpass2;
    pub type PFN_vkCmdEndRenderPass2KHR = PFN_vkCmdEndRenderPass2;
}

pub struct DeviceFn {
    create_render_pass2_khr: PFN_vkCreateRenderPass2,
    cmd_begin_render_pass2_khr: PFN_vkCmdBeginRenderPass2,
    cmd_next_subpass2_khr: PFN_vkCmdNextSubpass2,
    cmd_end_render_pass2_khr: PFN_vkCmdEndRenderPass2,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_render_pass2_khr: transmute(
                    load(c"vkCreateRenderPass2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_render_pass2_khr: transmute(
                    load(c"vkCmdBeginRenderPass2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_next_subpass2_khr: transmute(
                    load(c"vkCmdNextSubpass2KHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_render_pass2_khr: transmute(
                    load(c"vkCmdEndRenderPass2KHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateRenderPass2KHR.html>
    #[inline]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginRenderPass2KHR.html>
    #[inline]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdNextSubpass2KHR.html>
    #[inline]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndRenderPass2KHR.html>
    #[inline]
    pub unsafe fn cmd_end_render_pass2_khr(
        &self,
        command_buffer: CommandBuffer,
        subpass_end_info: &SubpassEndInfo<'_>,
    ) {
        unsafe { (self.cmd_end_render_pass2_khr)(command_buffer, subpass_end_info) }
    }
}
