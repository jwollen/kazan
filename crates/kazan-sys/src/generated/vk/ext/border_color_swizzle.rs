#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub components: ComponentMapping,
    pub srgb: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerBorderColorComponentMappingCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            components: Default::default(),
            srgb: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub border_color_swizzle: Bool32,
    pub border_color_swizzle_from_image: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            border_color_swizzle: Default::default(),
            border_color_swizzle_from_image: Default::default(),
            _marker: PhantomData,
        }
    }
}
