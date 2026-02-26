#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceMemoryOverallocationCreateInfoAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceMemoryOverallocationCreateInfoAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD,
            p_next: core::ptr::null(),
            overallocation_behavior: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryOverallocationBehaviorAMD(i32);
impl MemoryOverallocationBehaviorAMD {
    pub const DEFAULT_AMD: Self = Self(0);
    pub const ALLOWED_AMD: Self = Self(1);
    pub const DISALLOWED_AMD: Self = Self(2);
}
