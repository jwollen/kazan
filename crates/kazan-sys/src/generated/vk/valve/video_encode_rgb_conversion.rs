#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
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
        const RGB_IDENTITY_VALVE = 1 << 0;
        const YCBCR_IDENTITY_VALVE = 1 << 1;
        const YCBCR_709_VALVE = 1 << 2;
        const YCBCR_601_VALVE = 1 << 3;
        const YCBCR_2020_VALVE = 1 << 4;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRgbRangeCompressionFlagsVALVE: Flags {
        const FULL_RANGE_VALVE = 1 << 0;
        const NARROW_RANGE_VALVE = 1 << 1;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRgbChromaOffsetFlagsVALVE: Flags {
        const COSITED_EVEN_VALVE = 1 << 0;
        const MIDPOINT_VALVE = 1 << 1;
    }
}
