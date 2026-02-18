#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
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
