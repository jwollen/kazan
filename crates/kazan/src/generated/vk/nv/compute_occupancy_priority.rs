#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    cmd_set_compute_occupancy_priority_nv: PFN_vkCmdSetComputeOccupancyPriorityNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_compute_occupancy_priority_nv: transmute(
                    load(c"vkCmdSetComputeOccupancyPriorityNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_compute_occupancy_priority_nv(
        &self,
        command_buffer: CommandBuffer,
        parameters: &ComputeOccupancyPriorityParametersNV,
    ) {
        unsafe { (self.cmd_set_compute_occupancy_priority_nv)(command_buffer, parameters) }
    }
}
