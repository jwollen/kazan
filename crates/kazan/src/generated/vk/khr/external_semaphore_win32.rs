#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
        import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.import_semaphore_win32_handle_khr)(
                device,
                import_semaphore_win32_handle_info,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR<'_>,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut handle = core::mem::MaybeUninit::uninit();
            let result = (self.get_semaphore_win32_handle_khr)(
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
}
