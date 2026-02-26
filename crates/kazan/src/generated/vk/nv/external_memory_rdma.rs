#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_memory_remote_address_nv: PFN_vkGetMemoryRemoteAddressNV,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_remote_address_nv: transmute(
                    load(c"vkGetMemoryRemoteAddressNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_remote_address_nv(
        &self,
        device: Device,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV<'_>,
    ) -> crate::Result<RemoteAddressNV> {
        unsafe {
            let mut address = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_remote_address_nv)(
                device,
                memory_get_remote_address_info,
                address.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(address.assume_init()),
                err => Err(err),
            }
        }
    }
}
