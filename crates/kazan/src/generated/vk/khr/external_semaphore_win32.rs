#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
    get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
}
impl DeviceFn {
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        device: Device,
        import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result {
        unsafe {
            (self.import_semaphore_win32_handle_khr)(device, import_semaphore_win32_handle_info)
        }
    }
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
        handle: &mut HANDLE,
    ) -> Result {
        unsafe { (self.get_semaphore_win32_handle_khr)(device, get_win32_handle_info, handle) }
    }
}
