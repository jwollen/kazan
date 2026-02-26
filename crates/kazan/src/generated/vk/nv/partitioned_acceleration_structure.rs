#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_partitioned_acceleration_structures_build_sizes_nv:
        PFN_vkGetPartitionedAccelerationStructuresBuildSizesNV,
    cmd_build_partitioned_acceleration_structures_nv:
        PFN_vkCmdBuildPartitionedAccelerationStructuresNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_partitioned_acceleration_structures_build_sizes_nv: transmute(
                    load(c"vkGetPartitionedAccelerationStructuresBuildSizesNV")
                        .ok_or(LoadingError)?,
                ),
                cmd_build_partitioned_acceleration_structures_nv: transmute(
                    load(c"vkCmdBuildPartitionedAccelerationStructuresNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_partitioned_acceleration_structures_build_sizes_nv(
        &self,
        device: Device,
        info: &PartitionedAccelerationStructureInstancesInputNV<'_>,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_> {
        unsafe {
            let mut size_info = core::mem::MaybeUninit::uninit();
            (self.get_partitioned_acceleration_structures_build_sizes_nv)(
                device,
                info,
                size_info.as_mut_ptr(),
            );
            size_info.assume_init()
        }
    }
    pub unsafe fn cmd_build_partitioned_acceleration_structures_nv(
        &self,
        command_buffer: CommandBuffer,
        build_info: &BuildPartitionedAccelerationStructureInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_build_partitioned_acceleration_structures_nv)(command_buffer, build_info)
        }
    }
}
