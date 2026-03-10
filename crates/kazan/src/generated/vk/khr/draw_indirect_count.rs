//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_draw_indirect_count.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_draw_indirect_count";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub type PFN_vkCmdDrawIndirectCountKHR = PFN_vkCmdDrawIndirectCount;
    pub type PFN_vkCmdDrawIndexedIndirectCountKHR = PFN_vkCmdDrawIndexedIndirectCount;
}

pub struct DeviceFn {
    cmd_draw_indirect_count_khr: PFN_vkCmdDrawIndirectCount,
    cmd_draw_indexed_indirect_count_khr: PFN_vkCmdDrawIndexedIndirectCount,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_draw_indirect_count_khr: transmute(
                    load(c"vkCmdDrawIndirectCountKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_draw_indexed_indirect_count_khr: transmute(
                    load(c"vkCmdDrawIndexedIndirectCountKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndirectCountKHR.html>
    #[inline]
    pub unsafe fn cmd_draw_indirect_count_khr(
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
            (self.cmd_draw_indirect_count_khr)(
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDrawIndexedIndirectCountKHR.html>
    #[inline]
    pub unsafe fn cmd_draw_indexed_indirect_count_khr(
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
            (self.cmd_draw_indexed_indirect_count_khr)(
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
