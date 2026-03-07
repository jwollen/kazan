//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_draw_indirect_count.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_draw_indirect_count";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub type PFN_vkCmdDrawIndirectCountAMD = PFN_vkCmdDrawIndirectCount;
    pub type PFN_vkCmdDrawIndexedIndirectCountAMD = PFN_vkCmdDrawIndexedIndirectCount;
}

pub struct DeviceFn {
    cmd_draw_indirect_count_amd: PFN_vkCmdDrawIndirectCount,
    cmd_draw_indexed_indirect_count_amd: PFN_vkCmdDrawIndexedIndirectCount,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_draw_indirect_count_amd: transmute(
                    load(c"vkCmdDrawIndirectCountAMD").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_indexed_indirect_count_amd: transmute(
                    load(c"vkCmdDrawIndexedIndirectCountAMD").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirectCountAMD.html>
    #[inline]
    pub unsafe fn cmd_draw_indirect_count_amd(
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
            (self.cmd_draw_indirect_count_amd)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndexedIndirectCountAMD.html>
    #[inline]
    pub unsafe fn cmd_draw_indexed_indirect_count_amd(
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
            (self.cmd_draw_indexed_indirect_count_amd)(
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
