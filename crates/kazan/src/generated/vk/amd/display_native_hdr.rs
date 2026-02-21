#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    set_local_dimming_amd: PFN_vkSetLocalDimmingAMD,
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
