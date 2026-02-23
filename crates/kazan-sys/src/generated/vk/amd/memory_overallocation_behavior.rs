#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryOverallocationBehaviorAMD(i32);
impl MemoryOverallocationBehaviorAMD {
    pub const DEFAULT_AMD: Self = Self(0);
    pub const ALLOWED_AMD: Self = Self(1);
    pub const DISALLOWED_AMD: Self = Self(2);
}
