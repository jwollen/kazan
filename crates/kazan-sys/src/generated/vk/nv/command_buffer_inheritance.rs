#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub command_buffer_inheritance: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCommandBufferInheritanceFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            command_buffer_inheritance: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceCommandBufferInheritanceFeaturesNV<'a> {
    pub fn command_buffer_inheritance(mut self, command_buffer_inheritance: Bool32) -> Self {
        self.command_buffer_inheritance = command_buffer_inheritance;
        self
    }
}
