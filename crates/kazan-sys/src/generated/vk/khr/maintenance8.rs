#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance8FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance8: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMaintenance8FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            maintenance8: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryBarrierAccessFlags3KHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask3: AccessFlags3KHR,
    pub dst_access_mask3: AccessFlags3KHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryBarrierAccessFlags3KHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_BARRIER_ACCESS_FLAGS_3_KHR,
            p_next: core::ptr::null(),
            src_access_mask3: Default::default(),
            dst_access_mask3: Default::default(),
            _marker: PhantomData,
        }
    }
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
