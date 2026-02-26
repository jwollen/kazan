#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_priority: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMemoryPriorityFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            memory_priority: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryPriorityAllocateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub priority: f32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MemoryPriorityAllocateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MEMORY_PRIORITY_ALLOCATE_INFO_EXT,
            p_next: core::ptr::null(),
            priority: Default::default(),
            _marker: PhantomData,
        }
    }
}
