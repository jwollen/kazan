#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceLayeredDriverPropertiesMSFT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub underlying_api: LayeredDriverUnderlyingApiMSFT,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LayeredDriverUnderlyingApiMSFT(i32);
impl LayeredDriverUnderlyingApiMSFT {
    pub const NONE_MSFT: Self = Self(0);
    pub const D3D12_MSFT: Self = Self(1);
}
