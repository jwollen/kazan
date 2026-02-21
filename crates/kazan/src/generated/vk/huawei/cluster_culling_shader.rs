#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_draw_cluster_huawei: PFN_vkCmdDrawClusterHUAWEI,
    cmd_draw_cluster_indirect_huawei: PFN_vkCmdDrawClusterIndirectHUAWEI,
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
