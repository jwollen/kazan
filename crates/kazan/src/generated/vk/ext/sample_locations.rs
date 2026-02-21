#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_multisample_properties_ext: PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: PhysicalDevice,
        samples: SampleCountFlags,
        multisample_properties: &mut MultisamplePropertiesEXT,
    ) {
        unsafe {
            (self.get_physical_device_multisample_properties_ext)(
                physical_device,
                samples,
                multisample_properties,
            )
        }
    }
}
pub struct DeviceFn {
    cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
}
impl DeviceFn {
    pub unsafe fn cmd_set_sample_locations_ext(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_info: &SampleLocationsInfoEXT,
    ) {
        unsafe { (self.cmd_set_sample_locations_ext)(command_buffer, sample_locations_info) }
    }
}
