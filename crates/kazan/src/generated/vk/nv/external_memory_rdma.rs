#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_memory_remote_address_nv: PFN_vkGetMemoryRemoteAddressNV,
}
impl DeviceFn {
    pub unsafe fn get_memory_remote_address_nv(
        &self,
        device: Device,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
        address: &mut RemoteAddressNV,
    ) -> Result {
        unsafe {
            (self.get_memory_remote_address_nv)(device, memory_get_remote_address_info, address)
        }
    }
}
