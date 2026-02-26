#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMutableDescriptorTypeFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub mutable_descriptor_type: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MutableDescriptorTypeListEXT<'a> {
    pub descriptor_type_count: u32,
    pub p_descriptor_types: *const DescriptorType,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MutableDescriptorTypeCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mutable_descriptor_type_list_count: u32,
    pub p_mutable_descriptor_type_lists: *const MutableDescriptorTypeListEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
