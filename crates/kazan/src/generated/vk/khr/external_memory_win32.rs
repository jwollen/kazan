#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
    get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
}
impl DeviceFn {
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &MemoryGetWin32HandleInfoKHR,
        handle: &mut HANDLE,
    ) -> Result {
        unsafe { (self.get_memory_win32_handle_khr)(device, get_win32_handle_info, handle) }
    }
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        memory_win32_handle_properties: &mut MemoryWin32HandlePropertiesKHR,
    ) -> Result {
        unsafe {
            (self.get_memory_win32_handle_properties_khr)(
                device,
                handle_type,
                handle,
                memory_win32_handle_properties,
            )
        }
    }
}
