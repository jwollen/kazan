#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_external_tensor_properties_arm:
        PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_external_tensor_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
        external_tensor_properties: &mut ExternalTensorPropertiesARM,
    ) {
        unsafe {
            (self.get_physical_device_external_tensor_properties_arm)(
                physical_device,
                external_tensor_info,
                external_tensor_properties,
            )
        }
    }
}
pub struct DeviceFn {
    create_tensor_arm: PFN_vkCreateTensorARM,
    destroy_tensor_arm: PFN_vkDestroyTensorARM,
    create_tensor_view_arm: PFN_vkCreateTensorViewARM,
    destroy_tensor_view_arm: PFN_vkDestroyTensorViewARM,
    get_tensor_memory_requirements_arm: PFN_vkGetTensorMemoryRequirementsARM,
    bind_tensor_memory_arm: PFN_vkBindTensorMemoryARM,
    get_device_tensor_memory_requirements_arm: PFN_vkGetDeviceTensorMemoryRequirementsARM,
    cmd_copy_tensor_arm: PFN_vkCmdCopyTensorARM,
    get_tensor_opaque_capture_descriptor_data_arm:
        Option<PFN_vkGetTensorOpaqueCaptureDescriptorDataARM>,
    get_tensor_view_opaque_capture_descriptor_data_arm:
        Option<PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM>,
}
impl DeviceFn {
    pub unsafe fn create_tensor_arm(
        &self,
        device: Device,
        create_info: &TensorCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
        tensor: &mut TensorARM,
    ) -> Result {
        unsafe { (self.create_tensor_arm)(device, create_info, allocator.to_raw_ptr(), tensor) }
    }
    pub unsafe fn destroy_tensor_arm(
        &self,
        device: Device,
        tensor: TensorARM,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_tensor_arm)(device, tensor, allocator.to_raw_ptr()) }
    }
    pub unsafe fn create_tensor_view_arm(
        &self,
        device: Device,
        create_info: &TensorViewCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
        view: &mut TensorViewARM,
    ) -> Result {
        unsafe { (self.create_tensor_view_arm)(device, create_info, allocator.to_raw_ptr(), view) }
    }
    pub unsafe fn destroy_tensor_view_arm(
        &self,
        device: Device,
        tensor_view: TensorViewARM,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_tensor_view_arm)(device, tensor_view, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_tensor_memory_requirements_arm(
        &self,
        device: Device,
        info: &TensorMemoryRequirementsInfoARM,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe { (self.get_tensor_memory_requirements_arm)(device, info, memory_requirements) }
    }
    pub unsafe fn bind_tensor_memory_arm(
        &self,
        device: Device,
        bind_infos: &[BindTensorMemoryInfoARM],
    ) -> Result {
        unsafe {
            (self.bind_tensor_memory_arm)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn get_device_tensor_memory_requirements_arm(
        &self,
        device: Device,
        info: &DeviceTensorMemoryRequirementsARM,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe {
            (self.get_device_tensor_memory_requirements_arm)(device, info, memory_requirements)
        }
    }
    pub unsafe fn cmd_copy_tensor_arm(
        &self,
        command_buffer: CommandBuffer,
        copy_tensor_info: &CopyTensorInfoARM,
    ) {
        unsafe { (self.cmd_copy_tensor_arm)(command_buffer, copy_tensor_info) }
    }
    pub unsafe fn get_tensor_opaque_capture_descriptor_data_arm(
        &self,
        device: Device,
        info: &TensorCaptureDescriptorDataInfoARM,
        data: &mut c_void,
    ) -> Result {
        unsafe { (self.get_tensor_opaque_capture_descriptor_data_arm.unwrap())(device, info, data) }
    }
    pub unsafe fn get_tensor_view_opaque_capture_descriptor_data_arm(
        &self,
        device: Device,
        info: &TensorViewCaptureDescriptorDataInfoARM,
        data: &mut c_void,
    ) -> Result {
        unsafe {
            (self
                .get_tensor_view_opaque_capture_descriptor_data_arm
                .unwrap())(device, info, data)
        }
    }
}
