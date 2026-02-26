#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_view_type: ImageViewType,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageViewImageFormatInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT,
            p_next: core::ptr::null_mut(),
            image_view_type: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub filter_cubic: Bool32,
    pub filter_cubic_minmax: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for FilterCubicImageViewImageFormatPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            filter_cubic: Default::default(),
            filter_cubic_minmax: Default::default(),
            _marker: PhantomData,
        }
    }
}
