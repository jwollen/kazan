#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize],
    pub heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize],
    pub _marker: PhantomData<&'a ()>,
}
