#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_priority: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub priority: f32,
    pub _marker: PhantomData<&'a ()>,
}
