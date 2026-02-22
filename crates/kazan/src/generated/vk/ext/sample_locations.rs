#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_multisample_properties_ext: PFN_vkGetPhysicalDeviceMultisamplePropertiesEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_multisample_properties_ext: transmute(
                    load(c"vkGetPhysicalDeviceMultisamplePropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_multisample_properties_ext(
        &self,
        physical_device: PhysicalDevice,
        samples: SampleCountFlagBits,
    ) -> MultisamplePropertiesEXT {
        unsafe {
            let mut multisample_properties = core::mem::MaybeUninit::uninit();
            (self.get_physical_device_multisample_properties_ext)(
                physical_device,
                samples,
                multisample_properties.as_mut_ptr(),
            );
            multisample_properties.assume_init()
        }
    }
}
pub struct DeviceFn {
    cmd_set_sample_locations_ext: PFN_vkCmdSetSampleLocationsEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_sample_locations_ext: transmute(
                    load(c"vkCmdSetSampleLocationsEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
