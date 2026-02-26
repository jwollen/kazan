#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
    get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_fd_khr: transmute(load(c"vkGetMemoryFdKHR").ok_or(LoadingError)?),
                get_memory_fd_properties_khr: transmute(
                    load(c"vkGetMemoryFdPropertiesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_fd_khr(
        &self,
        device: Device,
        get_fd_info: &MemoryGetFdInfoKHR<'_>,
    ) -> crate::Result<c_int> {
        unsafe {
            let mut fd = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_fd_khr)(device, get_fd_info, fd.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(fd.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: c_int,
    ) -> crate::Result<MemoryFdPropertiesKHR<'_>> {
        unsafe {
            let mut memory_fd_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_fd_properties_khr)(
                device,
                handle_type,
                fd,
                memory_fd_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory_fd_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
