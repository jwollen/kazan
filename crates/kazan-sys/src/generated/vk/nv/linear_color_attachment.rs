#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub linear_color_attachment: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceLinearColorAttachmentFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            linear_color_attachment: Default::default(),
            _marker: PhantomData,
        }
    }
}
