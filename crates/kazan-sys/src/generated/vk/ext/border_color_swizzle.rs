#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub components: ComponentMapping,
    pub srgb: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub border_color_swizzle: Bool32,
    pub border_color_swizzle_from_image: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
