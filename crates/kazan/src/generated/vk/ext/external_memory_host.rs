#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
        handle_type: ExternalMemoryHandleTypeFlagBits,
        host_pointer: &c_void,
    ) -> crate::Result<MemoryHostPointerPropertiesEXT> {
        unsafe {
            let mut memory_host_pointer_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_host_pointer_properties_ext)(
                device,
                handle_type,
                host_pointer,
                memory_host_pointer_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory_host_pointer_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
