#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_tool_properties_ext: PFN_vkGetPhysicalDeviceToolProperties,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_tool_properties_ext: transmute(
                    load(c"vkGetPhysicalDeviceToolPropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_tool_properties_ext(
        &self,
        physical_device: PhysicalDevice,
        tool_properties: impl ExtendUninit<PhysicalDeviceToolProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(tool_properties, |tool_count, tool_properties| {
                (self.get_physical_device_tool_properties_ext)(
                    physical_device,
                    tool_count,
                    tool_properties as _,
                )
            })
        }
    }
}
