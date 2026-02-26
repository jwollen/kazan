#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const REMAINING_3D_SLICES_EXT: u32 = !0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewSlicedCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub slice_offset: u32,
    pub slice_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageViewSlicedCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_VIEW_SLICED_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            slice_offset: Default::default(),
            slice_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_sliced_view_of3_d: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageSlicedViewOf3DFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            image_sliced_view_of3_d: Default::default(),
            _marker: PhantomData,
        }
    }
}
