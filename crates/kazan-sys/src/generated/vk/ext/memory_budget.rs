#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub heap_budget: DeviceSize,
    pub heap_usage: DeviceSize,
}
