#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize],
    pub heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMemoryBudgetPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            heap_budget: [Default::default(); _],
            heap_usage: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceMemoryBudgetPropertiesEXT<'a> {
    pub fn heap_budget(mut self, heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize]) -> Self {
        self.heap_budget = heap_budget;
        self
    }
    pub fn heap_usage(mut self, heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize]) -> Self {
        self.heap_usage = heap_usage;
        self
    }
}
