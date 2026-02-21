#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
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
        const FIXED_RATE_DEFAULT_EXT = 1 << 0;
        const FIXED_RATE_EXPLICIT_EXT = 1 << 1;
        const DISABLED_EXT = 1 << 2;
        const DEFAULT = 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageCompressionFixedRateFlagsEXT: Flags {
        const _1BPC_EXT = 1 << 0;
        const _2BPC_EXT = 1 << 1;
        const _3BPC_EXT = 1 << 2;
        const _4BPC_EXT = 1 << 3;
        const _5BPC_EXT = 1 << 4;
        const _6BPC_EXT = 1 << 5;
        const _7BPC_EXT = 1 << 6;
        const _8BPC_EXT = 1 << 7;
        const _9BPC_EXT = 1 << 8;
        const _10BPC_EXT = 1 << 9;
        const _11BPC_EXT = 1 << 10;
        const _12BPC_EXT = 1 << 11;
        const _13BPC_EXT = 1 << 12;
        const _14BPC_EXT = 1 << 13;
        const _15BPC_EXT = 1 << 14;
        const _16BPC_EXT = 1 << 15;
        const _17BPC_EXT = 1 << 16;
        const _18BPC_EXT = 1 << 17;
        const _19BPC_EXT = 1 << 18;
        const _20BPC_EXT = 1 << 19;
        const _21BPC_EXT = 1 << 20;
        const _22BPC_EXT = 1 << 21;
        const _23BPC_EXT = 1 << 22;
        const _24BPC_EXT = 1 << 23;
        const NONE = 0;
    }
}
