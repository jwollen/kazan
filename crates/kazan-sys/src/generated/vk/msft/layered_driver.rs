#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLayeredDriverPropertiesMSFT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub underlying_api: LayeredDriverUnderlyingApiMSFT,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayeredDriverUnderlyingApiMSFT(i32);
impl LayeredDriverUnderlyingApiMSFT {
    pub const NONE_MSFT: Self = Self(0);
    pub const D3D12_MSFT: Self = Self(1);
}
