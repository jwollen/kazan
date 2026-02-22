#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_draw_cluster_huawei: PFN_vkCmdDrawClusterHUAWEI,
    cmd_draw_cluster_indirect_huawei: PFN_vkCmdDrawClusterIndirectHUAWEI,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_draw_cluster_huawei: transmute(
                    load(c"vkCmdDrawClusterHUAWEI").ok_or(LoadingError)?,
                ),
                cmd_draw_cluster_indirect_huawei: transmute(
                    load(c"vkCmdDrawClusterIndirectHUAWEI").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_draw_cluster_huawei(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_draw_cluster_huawei)(
                command_buffer,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    pub unsafe fn cmd_draw_cluster_indirect_huawei(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
    ) {
        unsafe { (self.cmd_draw_cluster_indirect_huawei)(command_buffer, buffer, offset) }
    }
}
