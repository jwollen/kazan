#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_partitioned_acceleration_structures_build_sizes_nv:
        PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV,
    cmd_build_partitioned_acceleration_structures_nv:
        PFN_vkCmdBuildPartitionedAccelerationStructuresNV,
}
impl DeviceFn {
    pub unsafe fn get_partitioned_acceleration_structures_build_sizes_nv(
        &self,
        device: Device,
        info: &PartitionedAccelerationStructureInstancesInputNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        unsafe {
            (self.get_partitioned_acceleration_structures_build_sizes_nv)(device, info, size_info)
        }
    }
    pub unsafe fn cmd_build_partitioned_acceleration_structures_nv(
        &self,
        command_buffer: CommandBuffer,
        build_info: &BuildPartitionedAccelerationStructureInfoNV,
    ) {
        unsafe {
            (self.cmd_build_partitioned_acceleration_structures_nv)(command_buffer, build_info)
        }
    }
}
