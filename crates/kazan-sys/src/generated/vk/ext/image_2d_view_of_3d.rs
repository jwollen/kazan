#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImage2DViewOf3DFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image2_d_view_of3_d: Bool32,
    pub sampler2_d_view_of3_d: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImage2DViewOf3DFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            image2_d_view_of3_d: Default::default(),
            sampler2_d_view_of3_d: Default::default(),
            _marker: PhantomData,
        }
    }
}
