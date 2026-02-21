#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_cluster_acceleration_structure_build_sizes_nv:
        PFN_vkGetClusterAccelerationStructureBuildSizesNV,
    cmd_build_cluster_acceleration_structure_indirect_nv:
        PFN_vkCmdBuildClusterAccelerationStructureIndirectNV,
}
impl DeviceFn {
    pub unsafe fn get_cluster_acceleration_structure_build_sizes_nv(
        &self,
        device: Device,
        info: &ClusterAccelerationStructureInputInfoNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        unsafe { (self.get_cluster_acceleration_structure_build_sizes_nv)(device, info, size_info) }
    }
    pub unsafe fn cmd_build_cluster_acceleration_structure_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV,
    ) {
        unsafe {
            (self.cmd_build_cluster_acceleration_structure_indirect_nv)(
                command_buffer,
                command_infos,
            )
        }
    }
}
