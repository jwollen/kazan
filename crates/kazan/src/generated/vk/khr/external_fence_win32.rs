#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_fence_win32_handle_khr: PFN_vkGetFenceWin32HandleKHR,
    import_fence_win32_handle_khr: PFN_vkImportFenceWin32HandleKHR,
}
impl DeviceFn {
    pub unsafe fn get_fence_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &FenceGetWin32HandleInfoKHR,
        handle: &mut HANDLE,
    ) -> Result {
        unsafe { (self.get_fence_win32_handle_khr)(device, get_win32_handle_info, handle) }
    }
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        device: Device,
        import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR,
    ) -> Result {
        unsafe { (self.import_fence_win32_handle_khr)(device, import_fence_win32_handle_info) }
    }
}
