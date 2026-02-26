#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_cluster_acceleration_structure_build_sizes_nv:
        PFN_vkGetClusterAccelerationStructureBuildSizesNV,
    cmd_build_cluster_acceleration_structure_indirect_nv:
        PFN_vkCmdBuildClusterAccelerationStructureIndirectNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_cluster_acceleration_structure_build_sizes_nv: transmute(
                    load(c"vkGetClusterAccelerationStructureBuildSizesNV").ok_or(LoadingError)?,
                ),
                cmd_build_cluster_acceleration_structure_indirect_nv: transmute(
                    load(c"vkCmdBuildClusterAccelerationStructureIndirectNV")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_cluster_acceleration_structure_build_sizes_nv(
        &self,
        device: Device,
        info: &ClusterAccelerationStructureInputInfoNV<'_>,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_> {
        unsafe {
            let mut size_info = core::mem::MaybeUninit::uninit();
            (self.get_cluster_acceleration_structure_build_sizes_nv)(
                device,
                info,
                size_info.as_mut_ptr(),
            );
            size_info.assume_init()
        }
    }
    pub unsafe fn cmd_build_cluster_acceleration_structure_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_build_cluster_acceleration_structure_indirect_nv)(
                command_buffer,
                command_infos,
            )
        }
    }
}
