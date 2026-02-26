#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
    get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_win32_handle_khr: transmute(
                    load(c"vkGetMemoryWin32HandleKHR").ok_or(LoadingError)?,
                ),
                get_memory_win32_handle_properties_khr: transmute(
                    load(c"vkGetMemoryWin32HandlePropertiesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &MemoryGetWin32HandleInfoKHR<'_>,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut handle = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_win32_handle_khr)(
                device,
                get_win32_handle_info,
                handle.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(handle.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
    ) -> crate::Result<MemoryWin32HandlePropertiesKHR<'_>> {
        unsafe {
            let mut memory_win32_handle_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_win32_handle_properties_khr)(
                device,
                handle_type,
                handle,
                memory_win32_handle_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory_win32_handle_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
