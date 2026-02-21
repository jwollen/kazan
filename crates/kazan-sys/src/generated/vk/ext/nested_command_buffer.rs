#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceNestedCommandBufferFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub nested_command_buffer: Bool32,
    pub nested_command_buffer_rendering: Bool32,
    pub nested_command_buffer_simultaneous_use: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceNestedCommandBufferPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_command_buffer_nesting_level: u32,
}
