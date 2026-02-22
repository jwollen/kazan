#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_memory_win32_handle_nv: PFN_vkGetMemoryWin32HandleNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_win32_handle_nv: transmute(
                    load(c"vkGetMemoryWin32HandleNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_win32_handle_nv(
        &self,
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut handle = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_memory_win32_handle_nv)(device, memory, handle_type, handle.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(handle.assume_init()),
                err => Err(err),
            }
        }
    }
}
