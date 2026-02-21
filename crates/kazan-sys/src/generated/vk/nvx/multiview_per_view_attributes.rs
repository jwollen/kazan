#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub per_view_position_all_components: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MultiviewPerViewAttributesInfoNVX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub per_view_attributes: Bool32,
    pub per_view_attributes_position_x_only: Bool32,
}
