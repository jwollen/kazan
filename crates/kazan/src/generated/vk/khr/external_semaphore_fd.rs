#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_semaphore_fd_khr: PFN_vkGetSemaphoreFdKHR,
    import_semaphore_fd_khr: PFN_vkImportSemaphoreFdKHR,
}
impl DeviceFn {
    pub unsafe fn get_semaphore_fd_khr(
        &self,
        device: Device,
        get_fd_info: &SemaphoreGetFdInfoKHR,
        fd: &mut c_int,
    ) -> Result {
        unsafe { (self.get_semaphore_fd_khr)(device, get_fd_info, fd) }
    }
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        device: Device,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    ) -> Result {
        unsafe { (self.import_semaphore_fd_khr)(device, import_semaphore_fd_info) }
    }
}
