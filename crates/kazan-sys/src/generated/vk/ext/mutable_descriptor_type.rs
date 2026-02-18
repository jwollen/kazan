#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub mutable_descriptor_type: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MutableDescriptorTypeListEXT {
    pub descriptor_type_count: u32,
    pub p_descriptor_types: *const DescriptorType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MutableDescriptorTypeCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mutable_descriptor_type_list_count: u32,
    pub p_mutable_descriptor_type_lists: *const MutableDescriptorTypeListEXT,
}
