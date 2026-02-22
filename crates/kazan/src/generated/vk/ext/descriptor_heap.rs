#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_descriptor_size_ext: PFN_vkGetPhysicalDeviceDescriptorSizeEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_descriptor_size_ext: transmute(
                    load(c"vkGetPhysicalDeviceDescriptorSizeEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
    register_custom_border_color_ext: Option<PFN_vkRegisterCustomBorderColorEXT>,
    unregister_custom_border_color_ext: Option<PFN_vkUnregisterCustomBorderColorEXT>,
    get_tensor_opaque_capture_data_arm: Option<PFN_vkGetTensorOpaqueCaptureDataARM>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                write_sampler_descriptors_ext: transmute(
                    load(c"vkWriteSamplerDescriptorsEXT").ok_or(LoadingError)?,
                ),
                write_resource_descriptors_ext: transmute(
                    load(c"vkWriteResourceDescriptorsEXT").ok_or(LoadingError)?,
                ),
                cmd_bind_sampler_heap_ext: transmute(
                    load(c"vkCmdBindSamplerHeapEXT").ok_or(LoadingError)?,
                ),
                cmd_bind_resource_heap_ext: transmute(
                    load(c"vkCmdBindResourceHeapEXT").ok_or(LoadingError)?,
                ),
                cmd_push_data_ext: transmute(load(c"vkCmdPushDataEXT").ok_or(LoadingError)?),
                get_image_opaque_capture_data_ext: transmute(
                    load(c"vkGetImageOpaqueCaptureDataEXT").ok_or(LoadingError)?,
                ),
                register_custom_border_color_ext: transmute(load(
                    c"vkRegisterCustomBorderColorEXT",
                )),
                unregister_custom_border_color_ext: transmute(load(
                    c"vkUnregisterCustomBorderColorEXT",
                )),
                get_tensor_opaque_capture_data_arm: transmute(load(
                    c"vkGetTensorOpaqueCaptureDataARM",
                )),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn write_sampler_descriptors_ext(
        &self,
        device: Device,
        samplers: &[SamplerCreateInfo],
        descriptors: &[HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            result((self.write_sampler_descriptors_ext)(
                device,
                samplers.len().try_into().unwrap(),
                samplers.as_ptr() as _,
                descriptors.as_ptr() as _,
            ))
        }
    }
    pub unsafe fn write_resource_descriptors_ext(
        &self,
        device: Device,
        resources: &[ResourceDescriptorInfoEXT],
        descriptors: &[HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            result((self.write_resource_descriptors_ext)(
                device,
                resources.len().try_into().unwrap(),
                resources.as_ptr() as _,
                descriptors.as_ptr() as _,
            ))
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
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_image_opaque_capture_data_ext)(
                device,
                images.len().try_into().unwrap(),
                images.as_ptr() as _,
                datas.as_mut_ptr() as _,
            ))
        }
    }
    pub unsafe fn register_custom_border_color_ext(
        &self,
        device: Device,
        border_color: &SamplerCustomBorderColorCreateInfoEXT,
        request_index: Bool32,
        index: &mut u32,
    ) -> crate::Result<()> {
        unsafe {
            result((self.register_custom_border_color_ext.unwrap())(
                device,
                border_color,
                request_index,
                index,
            ))
        }
    }
    pub unsafe fn unregister_custom_border_color_ext(&self, device: Device, index: u32) {
        unsafe { (self.unregister_custom_border_color_ext.unwrap())(device, index) }
    }
    pub unsafe fn get_tensor_opaque_capture_data_arm(
        &self,
        device: Device,
        tensors: &[TensorARM],
        datas: &mut [HostAddressRangeEXT],
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_tensor_opaque_capture_data_arm.unwrap())(
                device,
                tensors.len().try_into().unwrap(),
                tensors.as_ptr() as _,
                datas.as_mut_ptr() as _,
            ))
        }
    }
}
