#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    import_fence_fd_khr: PFN_vkImportFenceFdKHR,
    get_fence_fd_khr: PFN_vkGetFenceFdKHR,
}
impl DeviceFn {
    pub unsafe fn import_fence_fd_khr(
        &self,
        device: Device,
        import_fence_fd_info: &ImportFenceFdInfoKHR,
    ) -> Result {
        unsafe { (self.import_fence_fd_khr)(device, import_fence_fd_info) }
    }
    pub unsafe fn get_fence_fd_khr(
        &self,
        device: Device,
        get_fd_info: &FenceGetFdInfoKHR,
        fd: &mut c_int,
    ) -> Result {
        unsafe { (self.get_fence_fd_khr)(device, get_fd_info, fd) }
    }
}
