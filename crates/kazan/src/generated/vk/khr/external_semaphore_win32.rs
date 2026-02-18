#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
    import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
}
impl DeviceFn {
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
        handle: &mut HANDLE,
    ) -> Result {
        unsafe { (self.get_semaphore_win32_handle_khr)(device, get_win32_handle_info, handle) }
    }
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        device: Device,
        import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result {
        unsafe {
            (self.import_semaphore_win32_handle_khr)(device, import_semaphore_win32_handle_info)
        }
    }
}
