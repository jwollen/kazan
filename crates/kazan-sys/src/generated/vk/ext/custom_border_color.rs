#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCustomBorderColorCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub custom_border_color: ClearColorValue,
    pub format: Format,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerCustomBorderColorCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            custom_border_color: Default::default(),
            format: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_custom_border_color_samplers: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCustomBorderColorPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            max_custom_border_color_samplers: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub custom_border_colors: Bool32,
    pub custom_border_color_without_format: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCustomBorderColorFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            custom_border_colors: Default::default(),
            custom_border_color_without_format: Default::default(),
            _marker: PhantomData,
        }
    }
}
