#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayNativeHdrSurfaceCapabilitiesAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub local_dimming_support: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayNativeHdrSurfaceCapabilitiesAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD,
            p_next: core::ptr::null_mut(),
            local_dimming_support: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub local_dimming_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainDisplayNativeHdrCreateInfoAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD,
            p_next: core::ptr::null(),
            local_dimming_enable: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: Device,
    swap_chain: SwapchainKHR,
    local_dimming_enable: Bool32,
);
