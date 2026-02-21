#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_micromap_ext: PFN_vkCreateMicromapEXT,
    destroy_micromap_ext: PFN_vkDestroyMicromapEXT,
    cmd_build_micromaps_ext: PFN_vkCmdBuildMicromapsEXT,
    build_micromaps_ext: PFN_vkBuildMicromapsEXT,
    copy_micromap_ext: PFN_vkCopyMicromapEXT,
    copy_micromap_to_memory_ext: PFN_vkCopyMicromapToMemoryEXT,
    copy_memory_to_micromap_ext: PFN_vkCopyMemoryToMicromapEXT,
    write_micromaps_properties_ext: PFN_vkWriteMicromapsPropertiesEXT,
    cmd_copy_micromap_ext: PFN_vkCmdCopyMicromapEXT,
    cmd_copy_micromap_to_memory_ext: PFN_vkCmdCopyMicromapToMemoryEXT,
    cmd_copy_memory_to_micromap_ext: PFN_vkCmdCopyMemoryToMicromapEXT,
    cmd_write_micromaps_properties_ext: PFN_vkCmdWriteMicromapsPropertiesEXT,
    get_device_micromap_compatibility_ext: PFN_vkGetDeviceMicromapCompatibilityEXT,
    get_micromap_build_sizes_ext: PFN_vkGetMicromapBuildSizesEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_micromap_ext: transmute(load(c"vkCreateMicromapEXT").ok_or(LoadingError)?),
                destroy_micromap_ext: transmute(load(c"vkDestroyMicromapEXT").ok_or(LoadingError)?),
                cmd_build_micromaps_ext: transmute(
                    load(c"vkCmdBuildMicromapsEXT").ok_or(LoadingError)?,
                ),
                build_micromaps_ext: transmute(load(c"vkBuildMicromapsEXT").ok_or(LoadingError)?),
                copy_micromap_ext: transmute(load(c"vkCopyMicromapEXT").ok_or(LoadingError)?),
                copy_micromap_to_memory_ext: transmute(
                    load(c"vkCopyMicromapToMemoryEXT").ok_or(LoadingError)?,
                ),
                copy_memory_to_micromap_ext: transmute(
                    load(c"vkCopyMemoryToMicromapEXT").ok_or(LoadingError)?,
                ),
                write_micromaps_properties_ext: transmute(
                    load(c"vkWriteMicromapsPropertiesEXT").ok_or(LoadingError)?,
                ),
                cmd_copy_micromap_ext: transmute(
                    load(c"vkCmdCopyMicromapEXT").ok_or(LoadingError)?,
                ),
                cmd_copy_micromap_to_memory_ext: transmute(
                    load(c"vkCmdCopyMicromapToMemoryEXT").ok_or(LoadingError)?,
                ),
                cmd_copy_memory_to_micromap_ext: transmute(
                    load(c"vkCmdCopyMemoryToMicromapEXT").ok_or(LoadingError)?,
                ),
                cmd_write_micromaps_properties_ext: transmute(
                    load(c"vkCmdWriteMicromapsPropertiesEXT").ok_or(LoadingError)?,
                ),
                get_device_micromap_compatibility_ext: transmute(
                    load(c"vkGetDeviceMicromapCompatibilityEXT").ok_or(LoadingError)?,
                ),
                get_micromap_build_sizes_ext: transmute(
                    load(c"vkGetMicromapBuildSizesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_micromap_ext(
        &self,
        device: Device,
        create_info: &MicromapCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        micromap: &mut MicromapEXT,
    ) -> Result {
        unsafe { (self.create_micromap_ext)(device, create_info, allocator.to_raw_ptr(), micromap) }
    }
    pub unsafe fn destroy_micromap_ext(
        &self,
        device: Device,
        micromap: MicromapEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_micromap_ext)(device, micromap, allocator.to_raw_ptr()) }
    }
    pub unsafe fn cmd_build_micromaps_ext(
        &self,
        command_buffer: CommandBuffer,
        infos: &[MicromapBuildInfoEXT],
    ) {
        unsafe {
            (self.cmd_build_micromaps_ext)(
                command_buffer,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn build_micromaps_ext(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        infos: &[MicromapBuildInfoEXT],
    ) -> Result {
        unsafe {
            (self.build_micromaps_ext)(
                device,
                deferred_operation,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn copy_micromap_ext(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapInfoEXT,
    ) -> Result {
        unsafe { (self.copy_micromap_ext)(device, deferred_operation, info) }
    }
    pub unsafe fn copy_micromap_to_memory_ext(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapToMemoryInfoEXT,
    ) -> Result {
        unsafe { (self.copy_micromap_to_memory_ext)(device, deferred_operation, info) }
    }
    pub unsafe fn copy_memory_to_micromap_ext(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToMicromapInfoEXT,
    ) -> Result {
        unsafe { (self.copy_memory_to_micromap_ext)(device, deferred_operation, info) }
    }
    pub unsafe fn write_micromaps_properties_ext(
        &self,
        device: Device,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> Result {
        unsafe {
            (self.write_micromaps_properties_ext)(
                device,
                micromaps.len().try_into().unwrap(),
                micromaps.as_ptr() as _,
                query_type,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
                stride,
            )
        }
    }
    pub unsafe fn cmd_copy_micromap_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMicromapInfoEXT,
    ) {
        unsafe { (self.cmd_copy_micromap_ext)(command_buffer, info) }
    }
    pub unsafe fn cmd_copy_micromap_to_memory_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMicromapToMemoryInfoEXT,
    ) {
        unsafe { (self.cmd_copy_micromap_to_memory_ext)(command_buffer, info) }
    }
    pub unsafe fn cmd_copy_memory_to_micromap_ext(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMemoryToMicromapInfoEXT,
    ) {
        unsafe { (self.cmd_copy_memory_to_micromap_ext)(command_buffer, info) }
    }
    pub unsafe fn cmd_write_micromaps_properties_ext(
        &self,
        command_buffer: CommandBuffer,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        unsafe {
            (self.cmd_write_micromaps_properties_ext)(
                command_buffer,
                micromaps.len().try_into().unwrap(),
                micromaps.as_ptr() as _,
                query_type,
                query_pool,
                first_query,
            )
        }
    }
    pub unsafe fn get_device_micromap_compatibility_ext(
        &self,
        device: Device,
        version_info: &MicromapVersionInfoEXT,
        compatibility: &mut AccelerationStructureCompatibilityKHR,
    ) {
        unsafe { (self.get_device_micromap_compatibility_ext)(device, version_info, compatibility) }
    }
    pub unsafe fn get_micromap_build_sizes_ext(
        &self,
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &MicromapBuildInfoEXT,
        size_info: &mut MicromapBuildSizesInfoEXT,
    ) {
        unsafe { (self.get_micromap_build_sizes_ext)(device, build_type, build_info, size_info) }
    }
}
