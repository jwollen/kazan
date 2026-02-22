#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
    get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                import_semaphore_win32_handle_khr: transmute(
                    load(c"vkImportSemaphoreWin32HandleKHR").ok_or(LoadingError)?,
                ),
                get_semaphore_win32_handle_khr: transmute(
                    load(c"vkGetSemaphoreWin32HandleKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        device: Device,
        import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.import_semaphore_win32_handle_khr)(
                device,
                import_semaphore_win32_handle_info,
            ))
        }
    }
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
        handle: &mut HANDLE,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_semaphore_win32_handle_khr)(
                device,
                get_win32_handle_info,
                handle,
            ))
        }
    }
}
