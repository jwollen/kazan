#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_rgb_conversion: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeRgbConversionCapabilitiesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rgb_models: VideoEncodeRgbModelConversionFlagsVALVE,
    pub rgb_ranges: VideoEncodeRgbRangeCompressionFlagsVALVE,
    pub x_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
    pub y_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeProfileRgbConversionInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub perform_encode_rgb_conversion: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeSessionRgbConversionCreateInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rgb_model: VideoEncodeRgbModelConversionFlagsVALVE,
    pub rgb_range: VideoEncodeRgbRangeCompressionFlagsVALVE,
    pub x_chroma_offset: VideoEncodeRgbChromaOffsetFlagsVALVE,
    pub y_chroma_offset: VideoEncodeRgbChromaOffsetFlagsVALVE,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRgbModelConversionFlagsVALVE: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRgbRangeCompressionFlagsVALVE: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRgbChromaOffsetFlagsVALVE: Flags {
    }
}
