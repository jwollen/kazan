#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCompressionControlEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCompressionFlagsEXT,
    pub compression_control_plane_count: u32,
    pub p_fixed_rate_flags: *mut ImageCompressionFixedRateFlagsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageCompressionControlFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_compression_control: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCompressionPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_compression_flags: ImageCompressionFlagsEXT,
    pub image_compression_fixed_rate_flags: ImageCompressionFixedRateFlagsEXT,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageCompressionFlagsEXT: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageCompressionFixedRateFlagsEXT: Flags {
    }
}
