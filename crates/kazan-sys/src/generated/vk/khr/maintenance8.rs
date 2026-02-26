#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMaintenance8FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance8: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MemoryBarrierAccessFlags3KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask3: AccessFlags3KHR,
    pub dst_access_mask3: AccessFlags3KHR,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccessFlags3KHR: Flags64 {
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlagBits3KHR(u64);
impl AccessFlagBits3KHR {}
