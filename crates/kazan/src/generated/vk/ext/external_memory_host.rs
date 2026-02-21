#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_memory_host_pointer_properties_ext: PFN_vkGetMemoryHostPointerPropertiesEXT,
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
