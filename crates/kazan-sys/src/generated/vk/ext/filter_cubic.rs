#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_view_type: ImageViewType,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub filter_cubic: Bool32,
    pub filter_cubic_minmax: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
