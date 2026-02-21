#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_memory_metal_handle_ext: PFN_vkGetMemoryMetalHandleEXT,
    get_memory_metal_handle_properties_ext: PFN_vkGetMemoryMetalHandlePropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn get_memory_metal_handle_ext(
        &self,
        device: Device,
        get_metal_handle_info: &MemoryGetMetalHandleInfoEXT,
        handle: &mut *mut c_void,
    ) -> Result {
        unsafe { (self.get_memory_metal_handle_ext)(device, get_metal_handle_info, handle) }
    }
    pub unsafe fn get_memory_metal_handle_properties_ext(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: &c_void,
        memory_metal_handle_properties: &mut MemoryMetalHandlePropertiesEXT,
    ) -> Result {
        unsafe {
            (self.get_memory_metal_handle_properties_ext)(
                device,
                handle_type,
                handle,
                memory_metal_handle_properties,
            )
        }
    }
}
