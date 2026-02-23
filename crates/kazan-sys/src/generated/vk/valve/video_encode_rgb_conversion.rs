#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_rgb_conversion: Bool32,
}
#[repr(C)]
pub struct VideoEncodeRgbConversionCapabilitiesVALVE {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rgb_models: VideoEncodeRgbModelConversionFlagsVALVE,
    pub rgb_ranges: VideoEncodeRgbRangeCompressionFlagsVALVE,
    pub x_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
    pub y_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
}
#[repr(C)]
pub struct VideoEncodeProfileRgbConversionInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub perform_encode_rgb_conversion: Bool32,
}
#[repr(C)]
pub struct VideoEncodeSessionRgbConversionCreateInfoVALVE {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rgb_model: VideoEncodeRgbModelConversionFlagBitsVALVE,
    pub rgb_range: VideoEncodeRgbRangeCompressionFlagBitsVALVE,
    pub x_chroma_offset: VideoEncodeRgbChromaOffsetFlagBitsVALVE,
    pub y_chroma_offset: VideoEncodeRgbChromaOffsetFlagBitsVALVE,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRgbModelConversionFlagsVALVE: Flags {
        const RGB_IDENTITY_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::RGB_IDENTITY_VALVE.0;
        const YCBCR_IDENTITY_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_IDENTITY_VALVE.0;
        const YCBCR_709_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_709_VALVE.0;
        const YCBCR_601_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_601_VALVE.0;
        const YCBCR_2020_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_2020_VALVE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRgbModelConversionFlagBitsVALVE(u32);
impl VideoEncodeRgbModelConversionFlagBitsVALVE {
    pub const RGB_IDENTITY_VALVE: Self = Self(1 << 0);
    pub const YCBCR_IDENTITY_VALVE: Self = Self(1 << 1);
    pub const YCBCR_709_VALVE: Self = Self(1 << 2);
    pub const YCBCR_601_VALVE: Self = Self(1 << 3);
    pub const YCBCR_2020_VALVE: Self = Self(1 << 4);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRgbRangeCompressionFlagsVALVE: Flags {
        const FULL_RANGE_VALVE = VideoEncodeRgbRangeCompressionFlagBitsVALVE::FULL_RANGE_VALVE.0;
        const NARROW_RANGE_VALVE = VideoEncodeRgbRangeCompressionFlagBitsVALVE::NARROW_RANGE_VALVE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRgbRangeCompressionFlagBitsVALVE(u32);
impl VideoEncodeRgbRangeCompressionFlagBitsVALVE {
    pub const FULL_RANGE_VALVE: Self = Self(1 << 0);
    pub const NARROW_RANGE_VALVE: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRgbChromaOffsetFlagsVALVE: Flags {
        const COSITED_EVEN_VALVE = VideoEncodeRgbChromaOffsetFlagBitsVALVE::COSITED_EVEN_VALVE.0;
        const MIDPOINT_VALVE = VideoEncodeRgbChromaOffsetFlagBitsVALVE::MIDPOINT_VALVE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRgbChromaOffsetFlagBitsVALVE(u32);
impl VideoEncodeRgbChromaOffsetFlagBitsVALVE {
    pub const COSITED_EVEN_VALVE: Self = Self(1 << 0);
    pub const MIDPOINT_VALVE: Self = Self(1 << 1);
}
