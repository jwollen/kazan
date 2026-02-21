#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_tool_properties: PFN_vkGetPhysicalDeviceToolProperties,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_tool_properties(
        &self,
        physical_device: PhysicalDevice,
        tool_properties: impl ExtendUninit<PhysicalDeviceToolProperties>,
    ) -> Result {
        unsafe {
            try_extend_uninit(tool_properties, |tool_count, tool_properties| {
                (self.get_physical_device_tool_properties)(
                    physical_device,
                    tool_count,
                    tool_properties as _,
                )
            })
        }
    }
}
