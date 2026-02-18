#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_set_compute_occupancy_priority_nv: PFN_vkCmdSetComputeOccupancyPriorityNV,
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
