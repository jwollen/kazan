//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_buffer_marker.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_buffer_marker";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteBufferMarkerAMD.html>
    pub type PFN_vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteBufferMarker2AMD.html>
    pub type PFN_vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    );
}

pub struct DeviceFn {
    cmd_write_buffer_marker_amd: PFN_vkCmdWriteBufferMarkerAMD,
    cmd_write_buffer_marker2_amd: Option<PFN_vkCmdWriteBufferMarker2AMD>,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_write_buffer_marker_amd: transmute(
                    load(c"vkCmdWriteBufferMarkerAMD").ok_or(MissingEntryPointError)?,
                ),
                cmd_write_buffer_marker2_amd: transmute(load(c"vkCmdWriteBufferMarker2AMD")),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteBufferMarkerAMD.html>
    #[inline]
    pub unsafe fn cmd_write_buffer_marker_amd(
        &self,
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ) {
        unsafe {
            (self.cmd_write_buffer_marker_amd)(
                command_buffer,
                pipeline_stage,
                dst_buffer,
                dst_offset,
                marker,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdWriteBufferMarker2AMD.html>
    #[inline]
    pub unsafe fn cmd_write_buffer_marker2_amd(
        &self,
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ) {
        unsafe {
            (self.cmd_write_buffer_marker2_amd.unwrap())(
                command_buffer,
                stage,
                dst_buffer,
                dst_offset,
                marker,
            )
        }
    }
}
