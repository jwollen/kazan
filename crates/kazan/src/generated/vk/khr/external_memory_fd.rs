#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
    get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
}
impl DeviceFn {
    pub unsafe fn get_memory_fd_khr(
        &self,
        device: Device,
        get_fd_info: &MemoryGetFdInfoKHR,
        fd: &mut c_int,
    ) -> Result {
        unsafe { (self.get_memory_fd_khr)(device, get_fd_info, fd) }
    }
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
        memory_fd_properties: &mut MemoryFdPropertiesKHR,
    ) -> Result {
        unsafe {
            (self.get_memory_fd_properties_khr)(device, handle_type, fd, memory_fd_properties)
        }
    }
}
