#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    import_fence_fd_khr: PFN_vkImportFenceFdKHR,
    get_fence_fd_khr: PFN_vkGetFenceFdKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                import_fence_fd_khr: transmute(load(c"vkImportFenceFdKHR").ok_or(LoadingError)?),
                get_fence_fd_khr: transmute(load(c"vkGetFenceFdKHR").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn import_fence_fd_khr(
        &self,
        device: Device,
        import_fence_fd_info: &ImportFenceFdInfoKHR,
    ) -> crate::Result<()> {
        unsafe { result((self.import_fence_fd_khr)(device, import_fence_fd_info)) }
    }
    pub unsafe fn get_fence_fd_khr(
        &self,
        device: Device,
        get_fd_info: &FenceGetFdInfoKHR,
        fd: &mut c_int,
    ) -> crate::Result<()> {
        unsafe { result((self.get_fence_fd_khr)(device, get_fd_info, fd)) }
    }
}
