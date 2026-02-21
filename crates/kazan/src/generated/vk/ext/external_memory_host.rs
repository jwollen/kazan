#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_host_pointer_properties_ext: transmute(
                    load(c"vkGetMemoryHostPointerPropertiesEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_host_pointer_properties_ext(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        host_pointer: &c_void,
        memory_host_pointer_properties: &mut MemoryHostPointerPropertiesEXT,
    ) -> Result {
        unsafe {
            (self.get_memory_host_pointer_properties_ext)(
                device,
                handle_type,
                host_pointer,
                memory_host_pointer_properties,
            )
        }
    }
}
