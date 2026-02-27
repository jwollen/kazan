#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceNestedCommandBufferFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub nested_command_buffer: Bool32,
    pub nested_command_buffer_rendering: Bool32,
    pub nested_command_buffer_simultaneous_use: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceNestedCommandBufferFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            nested_command_buffer: Default::default(),
            nested_command_buffer_rendering: Default::default(),
            nested_command_buffer_simultaneous_use: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceNestedCommandBufferFeaturesEXT<'a> {
    pub fn nested_command_buffer(mut self, nested_command_buffer: Bool32) -> Self {
        self.nested_command_buffer = nested_command_buffer;
        self
    }
    pub fn nested_command_buffer_rendering(
        mut self,
        nested_command_buffer_rendering: Bool32,
    ) -> Self {
        self.nested_command_buffer_rendering = nested_command_buffer_rendering;
        self
    }
    pub fn nested_command_buffer_simultaneous_use(
        mut self,
        nested_command_buffer_simultaneous_use: Bool32,
    ) -> Self {
        self.nested_command_buffer_simultaneous_use = nested_command_buffer_simultaneous_use;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceNestedCommandBufferPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_command_buffer_nesting_level: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceNestedCommandBufferPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            max_command_buffer_nesting_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceNestedCommandBufferPropertiesEXT<'a> {
    pub fn max_command_buffer_nesting_level(
        mut self,
        max_command_buffer_nesting_level: u32,
    ) -> Self {
        self.max_command_buffer_nesting_level = max_command_buffer_nesting_level;
        self
    }
}
