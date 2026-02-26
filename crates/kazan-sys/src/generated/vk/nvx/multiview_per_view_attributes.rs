#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub per_view_position_all_components: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX,
            p_next: core::ptr::null_mut(),
            per_view_position_all_components: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MultiviewPerViewAttributesInfoNVX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub per_view_attributes: Bool32,
    pub per_view_attributes_position_x_only: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MultiviewPerViewAttributesInfoNVX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX,
            p_next: core::ptr::null(),
            per_view_attributes: Default::default(),
            per_view_attributes_position_x_only: Default::default(),
            _marker: PhantomData,
        }
    }
}
