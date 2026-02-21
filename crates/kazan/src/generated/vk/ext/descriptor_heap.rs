#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_descriptor_size_ext: PFN_vkGetPhysicalDeviceDescriptorSizeEXT,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_descriptor_size_ext(
        &self,
        physical_device: PhysicalDevice,
        descriptor_type: DescriptorType,
    ) -> DeviceSize {
        unsafe { (self.get_physical_device_descriptor_size_ext)(physical_device, descriptor_type) }
    }
}
pub struct DeviceFn {
    write_sampler_descriptors_ext: PFN_vkWriteSamplerDescriptorsEXT,
    write_resource_descriptors_ext: PFN_vkWriteResourceDescriptorsEXT,
    cmd_bind_sampler_heap_ext: PFN_vkCmdBindSamplerHeapEXT,
    cmd_bind_resource_heap_ext: PFN_vkCmdBindResourceHeapEXT,
    cmd_push_data_ext: PFN_vkCmdPushDataEXT,
    get_image_opaque_capture_data_ext: PFN_vkGetImageOpaqueCaptureDataEXT,
    register_custom_border_color_ext: PFN_vkRegisterCustomBorderColorEXT,
    unregister_custom_border_color_ext: PFN_vkUnregisterCustomBorderColorEXT,
    get_tensor_opaque_capture_data_arm: PFN_vkGetTensorOpaqueCaptureDataARM,
}
impl DeviceFn {
    pub unsafe fn write_sampler_descriptors_ext(
        &self,
        device: Device,
        samplers: &[SamplerCreateInfo],
        descriptors: &[HostAddressRangeEXT],
    ) -> Result {
        unsafe {
            (self.write_sampler_descriptors_ext)(
                device,
                samplers.len().try_into().unwrap(),
                samplers.as_ptr() as _,
                descriptors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn write_resource_descriptors_ext(
        &self,
        device: Device,
        resources: &[ResourceDescriptorInfoEXT],
        descriptors: &[HostAddressRangeEXT],
    ) -> Result {
        unsafe {
            (self.write_resource_descriptors_ext)(
                device,
                resources.len().try_into().unwrap(),
                resources.as_ptr() as _,
                descriptors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_sampler_heap_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_info: &BindHeapInfoEXT,
    ) {
        unsafe { (self.cmd_bind_sampler_heap_ext)(command_buffer, bind_info) }
    }
    pub unsafe fn cmd_bind_resource_heap_ext(
        &self,
        command_buffer: CommandBuffer,
        bind_info: &BindHeapInfoEXT,
    ) {
        unsafe { (self.cmd_bind_resource_heap_ext)(command_buffer, bind_info) }
    }
    pub unsafe fn cmd_push_data_ext(
        &self,
        command_buffer: CommandBuffer,
        push_data_info: &PushDataInfoEXT,
    ) {
        unsafe { (self.cmd_push_data_ext)(command_buffer, push_data_info) }
    }
    pub unsafe fn get_image_opaque_capture_data_ext(
        &self,
        device: Device,
        images: &[Image],
        datas: &mut [HostAddressRangeEXT],
    ) -> Result {
        unsafe {
            (self.get_image_opaque_capture_data_ext)(
                device,
                images.len().try_into().unwrap(),
                images.as_ptr() as _,
                datas.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn register_custom_border_color_ext(
        &self,
        device: Device,
        border_color: &SamplerCustomBorderColorCreateInfoEXT,
        request_index: Bool32,
        index: &mut u32,
    ) -> Result {
        unsafe {
            (self.register_custom_border_color_ext)(device, border_color, request_index, index)
        }
    }
    pub unsafe fn unregister_custom_border_color_ext(&self, device: Device, index: u32) {
        unsafe { (self.unregister_custom_border_color_ext)(device, index) }
    }
    pub unsafe fn get_tensor_opaque_capture_data_arm(
        &self,
        device: Device,
        tensors: &[TensorARM],
        datas: &mut [HostAddressRangeEXT],
    ) -> Result {
        unsafe {
            (self.get_tensor_opaque_capture_data_arm)(
                device,
                tensors.len().try_into().unwrap(),
                tensors.as_ptr() as _,
                datas.as_mut_ptr() as _,
            )
        }
    }
}
