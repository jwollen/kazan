#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    import_fence_win32_handle_khr: PFN_vkImportFenceWin32HandleKHR,
    get_fence_win32_handle_khr: PFN_vkGetFenceWin32HandleKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                import_fence_win32_handle_khr: transmute(
                    load(c"vkImportFenceWin32HandleKHR").ok_or(LoadingError)?,
                ),
                get_fence_win32_handle_khr: transmute(
                    load(c"vkGetFenceWin32HandleKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        device: Device,
        import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.import_fence_win32_handle_khr)(device, import_fence_win32_handle_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_fence_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &FenceGetWin32HandleInfoKHR<'_>,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut handle = core::mem::MaybeUninit::uninit();
            let result = (self.get_fence_win32_handle_khr)(
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
