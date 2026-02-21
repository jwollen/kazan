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
        const FIXED_RATE_DEFAULT_EXT = ImageCompressionFlagBitsEXT::FIXED_RATE_DEFAULT_EXT.0;
        const FIXED_RATE_EXPLICIT_EXT = ImageCompressionFlagBitsEXT::FIXED_RATE_EXPLICIT_EXT.0;
        const DISABLED_EXT = ImageCompressionFlagBitsEXT::DISABLED_EXT.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageCompressionFlagBitsEXT(u32);
impl ImageCompressionFlagBitsEXT {
    pub const FIXED_RATE_DEFAULT_EXT: Self = Self(1 << 0);
    pub const FIXED_RATE_EXPLICIT_EXT: Self = Self(1 << 1);
    pub const DISABLED_EXT: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageCompressionFixedRateFlagsEXT: Flags {
        const _1BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_1BPC_EXT.0;
        const _2BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_2BPC_EXT.0;
        const _3BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_3BPC_EXT.0;
        const _4BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_4BPC_EXT.0;
        const _5BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_5BPC_EXT.0;
        const _6BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_6BPC_EXT.0;
        const _7BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_7BPC_EXT.0;
        const _8BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_8BPC_EXT.0;
        const _9BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_9BPC_EXT.0;
        const _10BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_10BPC_EXT.0;
        const _11BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_11BPC_EXT.0;
        const _12BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_12BPC_EXT.0;
        const _13BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_13BPC_EXT.0;
        const _14BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_14BPC_EXT.0;
        const _15BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_15BPC_EXT.0;
        const _16BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_16BPC_EXT.0;
        const _17BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_17BPC_EXT.0;
        const _18BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_18BPC_EXT.0;
        const _19BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_19BPC_EXT.0;
        const _20BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_20BPC_EXT.0;
        const _21BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_21BPC_EXT.0;
        const _22BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_22BPC_EXT.0;
        const _23BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_23BPC_EXT.0;
        const _24BPC_EXT = ImageCompressionFixedRateFlagBitsEXT::_24BPC_EXT.0;
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageCompressionFixedRateFlagBitsEXT(u32);
impl ImageCompressionFixedRateFlagBitsEXT {
    pub const _1BPC_EXT: Self = Self(1 << 0);
    pub const _2BPC_EXT: Self = Self(1 << 1);
    pub const _3BPC_EXT: Self = Self(1 << 2);
    pub const _4BPC_EXT: Self = Self(1 << 3);
    pub const _5BPC_EXT: Self = Self(1 << 4);
    pub const _6BPC_EXT: Self = Self(1 << 5);
    pub const _7BPC_EXT: Self = Self(1 << 6);
    pub const _8BPC_EXT: Self = Self(1 << 7);
    pub const _9BPC_EXT: Self = Self(1 << 8);
    pub const _10BPC_EXT: Self = Self(1 << 9);
    pub const _11BPC_EXT: Self = Self(1 << 10);
    pub const _12BPC_EXT: Self = Self(1 << 11);
    pub const _13BPC_EXT: Self = Self(1 << 12);
    pub const _14BPC_EXT: Self = Self(1 << 13);
    pub const _15BPC_EXT: Self = Self(1 << 14);
    pub const _16BPC_EXT: Self = Self(1 << 15);
    pub const _17BPC_EXT: Self = Self(1 << 16);
    pub const _18BPC_EXT: Self = Self(1 << 17);
    pub const _19BPC_EXT: Self = Self(1 << 18);
    pub const _20BPC_EXT: Self = Self(1 << 19);
    pub const _21BPC_EXT: Self = Self(1 << 20);
    pub const _22BPC_EXT: Self = Self(1 << 21);
    pub const _23BPC_EXT: Self = Self(1 << 22);
    pub const _24BPC_EXT: Self = Self(1 << 23);
}
