#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_memory_win32_handle_nv: PFN_vkGetMemoryWin32HandleNV,
}
impl DeviceFn {
    pub unsafe fn get_memory_win32_handle_nv(
        &self,
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        handle: &mut HANDLE,
    ) -> Result {
        unsafe { (self.get_memory_win32_handle_nv)(device, memory, handle_type, handle) }
    }
}
