#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceMaintenance8FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance8: Bool32,
}
#[repr(C)]
pub struct MemoryBarrierAccessFlags3KHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask3: AccessFlags3KHR,
    pub dst_access_mask3: AccessFlags3KHR,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccessFlags3KHR: Flags64 {
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlagBits3KHR(u64);
impl AccessFlagBits3KHR {}
