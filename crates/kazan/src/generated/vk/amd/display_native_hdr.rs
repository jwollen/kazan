#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    set_local_dimming_amd: PFN_vkSetLocalDimmingAMD,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                set_local_dimming_amd: transmute(
                    load(c"vkSetLocalDimmingAMD").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn set_local_dimming_amd(
        &self,
        device: Device,
        swap_chain: SwapchainKHR,
        local_dimming_enable: Bool32,
    ) {
        unsafe { (self.set_local_dimming_amd)(device, swap_chain, local_dimming_enable) }
    }
}
