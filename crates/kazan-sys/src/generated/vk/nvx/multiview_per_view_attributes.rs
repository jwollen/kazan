#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub per_view_position_all_components: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MultiviewPerViewAttributesInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub per_view_attributes: Bool32,
    pub per_view_attributes_position_x_only: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
