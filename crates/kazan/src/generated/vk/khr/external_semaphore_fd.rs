#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    import_semaphore_fd_khr: PFN_vkImportSemaphoreFdKHR,
    get_semaphore_fd_khr: PFN_vkGetSemaphoreFdKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                import_semaphore_fd_khr: transmute(
                    load(c"vkImportSemaphoreFdKHR").ok_or(LoadingError)?,
                ),
                get_semaphore_fd_khr: transmute(load(c"vkGetSemaphoreFdKHR").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        device: Device,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.import_semaphore_fd_khr)(
                device,
                import_semaphore_fd_info,
            ))
        }
    }
    pub unsafe fn get_semaphore_fd_khr(
        &self,
        device: Device,
        get_fd_info: &SemaphoreGetFdInfoKHR,
        fd: &mut c_int,
    ) -> crate::Result<()> {
        unsafe { result((self.get_semaphore_fd_khr)(device, get_fd_info, fd)) }
    }
}
