#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    destroy_acceleration_structure_khr: PFN_vkDestroyAccelerationStructureKHR,
    cmd_copy_acceleration_structure_khr: PFN_vkCmdCopyAccelerationStructureKHR,
    copy_acceleration_structure_khr: PFN_vkCopyAccelerationStructureKHR,
    cmd_copy_acceleration_structure_to_memory_khr: PFN_vkCmdCopyAccelerationStructureToMemoryKHR,
    copy_acceleration_structure_to_memory_khr: PFN_vkCopyAccelerationStructureToMemoryKHR,
    cmd_copy_memory_to_acceleration_structure_khr: PFN_vkCmdCopyMemoryToAccelerationStructureKHR,
    copy_memory_to_acceleration_structure_khr: PFN_vkCopyMemoryToAccelerationStructureKHR,
    cmd_write_acceleration_structures_properties_khr:
        PFN_vkCmdWriteAccelerationStructuresPropertiesKHR,
    write_acceleration_structures_properties_khr: PFN_vkWriteAccelerationStructuresPropertiesKHR,
    get_device_acceleration_structure_compatibility_khr:
        PFN_vkGetDeviceAccelerationStructureCompatibilityKHR,
    create_acceleration_structure_khr: PFN_vkCreateAccelerationStructureKHR,
    cmd_build_acceleration_structures_khr: PFN_vkCmdBuildAccelerationStructuresKHR,
    cmd_build_acceleration_structures_indirect_khr: PFN_vkCmdBuildAccelerationStructuresIndirectKHR,
    build_acceleration_structures_khr: PFN_vkBuildAccelerationStructuresKHR,
    get_acceleration_structure_device_address_khr: PFN_vkGetAccelerationStructureDeviceAddressKHR,
    get_acceleration_structure_build_sizes_khr: PFN_vkGetAccelerationStructureBuildSizesKHR,
}
impl DeviceFn {
    pub unsafe fn destroy_acceleration_structure_khr(
        &self,
        device: Device,
        acceleration_structure: AccelerationStructureKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_acceleration_structure_khr)(
                device,
                acceleration_structure,
                allocator.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn cmd_copy_acceleration_structure_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyAccelerationStructureInfoKHR,
    ) {
        unsafe { (self.cmd_copy_acceleration_structure_khr)(command_buffer, info) }
    }
    pub unsafe fn copy_acceleration_structure_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureInfoKHR,
    ) -> Result {
        unsafe { (self.copy_acceleration_structure_khr)(device, deferred_operation, info) }
    }
    pub unsafe fn cmd_copy_acceleration_structure_to_memory_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        unsafe { (self.cmd_copy_acceleration_structure_to_memory_khr)(command_buffer, info) }
    }
    pub unsafe fn copy_acceleration_structure_to_memory_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> Result {
        unsafe {
            (self.copy_acceleration_structure_to_memory_khr)(device, deferred_operation, info)
        }
    }
    pub unsafe fn cmd_copy_memory_to_acceleration_structure_khr(
        &self,
        command_buffer: CommandBuffer,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        unsafe { (self.cmd_copy_memory_to_acceleration_structure_khr)(command_buffer, info) }
    }
    pub unsafe fn copy_memory_to_acceleration_structure_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> Result {
        unsafe {
            (self.copy_memory_to_acceleration_structure_khr)(device, deferred_operation, info)
        }
    }
    pub unsafe fn cmd_write_acceleration_structures_properties_khr(
        &self,
        command_buffer: CommandBuffer,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        unsafe {
            (self.cmd_write_acceleration_structures_properties_khr)(
                command_buffer,
                acceleration_structures.len().try_into().unwrap(),
                acceleration_structures.as_ptr() as _,
                query_type,
                query_pool,
                first_query,
            )
        }
    }
    pub unsafe fn write_acceleration_structures_properties_khr(
        &self,
        device: Device,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> Result {
        unsafe {
            (self.write_acceleration_structures_properties_khr)(
                device,
                acceleration_structures.len().try_into().unwrap(),
                acceleration_structures.as_ptr() as _,
                query_type,
                data.len().try_into().unwrap(),
                data.as_mut_ptr() as _,
                stride,
            )
        }
    }
    pub unsafe fn get_device_acceleration_structure_compatibility_khr(
        &self,
        device: Device,
        version_info: &AccelerationStructureVersionInfoKHR,
        compatibility: &mut AccelerationStructureCompatibilityKHR,
    ) {
        unsafe {
            (self.get_device_acceleration_structure_compatibility_khr)(
                device,
                version_info,
                compatibility,
            )
        }
    }
    pub unsafe fn create_acceleration_structure_khr(
        &self,
        device: Device,
        create_info: &AccelerationStructureCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        acceleration_structure: &mut AccelerationStructureKHR,
    ) -> Result {
        unsafe {
            (self.create_acceleration_structure_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                acceleration_structure,
            )
        }
    }
    pub unsafe fn cmd_build_acceleration_structures_khr(
        &self,
        command_buffer: CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) {
        unsafe {
            (self.cmd_build_acceleration_structures_khr)(
                command_buffer,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
                build_range_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_build_acceleration_structures_indirect_khr(
        &self,
        command_buffer: CommandBuffer,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        indirect_device_addresses: &[DeviceAddress],
        indirect_strides: &[u32],
        max_primitive_counts: &[*const u32],
    ) {
        unsafe {
            (self.cmd_build_acceleration_structures_indirect_khr)(
                command_buffer,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
                indirect_device_addresses.as_ptr() as _,
                indirect_strides.as_ptr() as _,
                max_primitive_counts.as_ptr() as _,
            )
        }
    }
    pub unsafe fn build_acceleration_structures_khr(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> Result {
        unsafe {
            (self.build_acceleration_structures_khr)(
                device,
                deferred_operation,
                infos.len().try_into().unwrap(),
                infos.as_ptr() as _,
                build_range_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn get_acceleration_structure_device_address_khr(
        &self,
        device: Device,
        info: &AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress {
        unsafe { (self.get_acceleration_structure_device_address_khr)(device, info) }
    }
    pub unsafe fn get_acceleration_structure_build_sizes_khr(
        &self,
        device: Device,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR,
        max_primitive_counts: Option<&[u32]>,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR,
    ) {
        unsafe {
            (self.get_acceleration_structure_build_sizes_khr)(
                device,
                build_type,
                build_info,
                max_primitive_counts.to_raw_ptr(),
                size_info,
            )
        }
    }
}
