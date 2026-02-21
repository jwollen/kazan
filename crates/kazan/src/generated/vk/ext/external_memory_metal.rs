#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_memory_metal_handle_ext: PFN_vkGetMemoryMetalHandleEXT,
    get_memory_metal_handle_properties_ext: PFN_vkGetMemoryMetalHandlePropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_metal_handle_ext: transmute(
                    load(c"vkGetMemoryMetalHandleEXT").ok_or(LoadingError)?,
                ),
                get_memory_metal_handle_properties_ext: transmute(
                    load(c"vkGetMemoryMetalHandlePropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
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
