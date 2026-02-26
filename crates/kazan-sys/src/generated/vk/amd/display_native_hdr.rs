#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub local_dimming_support: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub local_dimming_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: Device,
    swap_chain: SwapchainKHR,
    local_dimming_enable: Bool32,
);
