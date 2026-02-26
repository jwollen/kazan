#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_rgb_conversion: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE,
            p_next: core::ptr::null_mut(),
            video_encode_rgb_conversion: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeRgbConversionCapabilitiesVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rgb_models: VideoEncodeRgbModelConversionFlagsVALVE,
    pub rgb_ranges: VideoEncodeRgbRangeCompressionFlagsVALVE,
    pub x_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
    pub y_chroma_offsets: VideoEncodeRgbChromaOffsetFlagsVALVE,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeRgbConversionCapabilitiesVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE,
            p_next: core::ptr::null_mut(),
            rgb_models: Default::default(),
            rgb_ranges: Default::default(),
            x_chroma_offsets: Default::default(),
            y_chroma_offsets: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeProfileRgbConversionInfoVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub perform_encode_rgb_conversion: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeProfileRgbConversionInfoVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE,
            p_next: core::ptr::null(),
            perform_encode_rgb_conversion: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeSessionRgbConversionCreateInfoVALVE<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub rgb_model: VideoEncodeRgbModelConversionFlagBitsVALVE,
    pub rgb_range: VideoEncodeRgbRangeCompressionFlagBitsVALVE,
    pub x_chroma_offset: VideoEncodeRgbChromaOffsetFlagBitsVALVE,
    pub y_chroma_offset: VideoEncodeRgbChromaOffsetFlagBitsVALVE,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeSessionRgbConversionCreateInfoVALVE<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE,
            p_next: core::ptr::null(),
            rgb_model: Default::default(),
            rgb_range: Default::default(),
            x_chroma_offset: Default::default(),
            y_chroma_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeRgbModelConversionFlagsVALVE: Flags {
        const RGB_IDENTITY_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::RGB_IDENTITY_VALVE.0;
        const YCBCR_IDENTITY_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_IDENTITY_VALVE.0;
        const YCBCR_709_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_709_VALVE.0;
        const YCBCR_601_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_601_VALVE.0;
        const YCBCR_2020_VALVE = VideoEncodeRgbModelConversionFlagBitsVALVE::YCBCR_2020_VALVE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeRgbRangeCompressionFlagsVALVE: Flags {
        const FULL_RANGE_VALVE = VideoEncodeRgbRangeCompressionFlagBitsVALVE::FULL_RANGE_VALVE.0;
        const NARROW_RANGE_VALVE = VideoEncodeRgbRangeCompressionFlagBitsVALVE::NARROW_RANGE_VALVE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRgbRangeCompressionFlagBitsVALVE(u32);
impl VideoEncodeRgbRangeCompressionFlagBitsVALVE {
    pub const FULL_RANGE_VALVE: Self = Self(1 << 0);
    pub const NARROW_RANGE_VALVE: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeRgbChromaOffsetFlagsVALVE: Flags {
        const COSITED_EVEN_VALVE = VideoEncodeRgbChromaOffsetFlagBitsVALVE::COSITED_EVEN_VALVE.0;
        const MIDPOINT_VALVE = VideoEncodeRgbChromaOffsetFlagBitsVALVE::MIDPOINT_VALVE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRgbChromaOffsetFlagBitsVALVE(u32);
impl VideoEncodeRgbChromaOffsetFlagBitsVALVE {
    pub const COSITED_EVEN_VALVE: Self = Self(1 << 0);
    pub const MIDPOINT_VALVE: Self = Self(1 << 1);
}
