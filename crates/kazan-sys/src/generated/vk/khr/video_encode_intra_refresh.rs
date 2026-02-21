#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeIntraRefreshCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub intra_refresh_modes: VideoEncodeIntraRefreshModeFlagsKHR,
    pub max_intra_refresh_cycle_duration: u32,
    pub max_intra_refresh_active_reference_pictures: u32,
    pub partition_independent_intra_refresh_regions: Bool32,
    pub non_rectangular_intra_refresh_regions: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeSessionIntraRefreshCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub intra_refresh_mode: VideoEncodeIntraRefreshModeFlagBitsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeIntraRefreshInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub intra_refresh_cycle_duration: u32,
    pub intra_refresh_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoReferenceIntraRefreshInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dirty_intra_refresh_regions: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_intra_refresh: Bool32,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeIntraRefreshModeFlagsKHR: Flags {
        const PER_PICTURE_PARTITION_KHR = VideoEncodeIntraRefreshModeFlagBitsKHR::PER_PICTURE_PARTITION_KHR.0;
        const BLOCK_BASED_KHR = VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_BASED_KHR.0;
        const BLOCK_ROW_BASED_KHR = VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_ROW_BASED_KHR.0;
        const BLOCK_COLUMN_BASED_KHR = VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_COLUMN_BASED_KHR.0;
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeIntraRefreshModeFlagBitsKHR(u32);
impl VideoEncodeIntraRefreshModeFlagBitsKHR {
    pub const PER_PICTURE_PARTITION_KHR: Self = Self(1 << 0);
    pub const BLOCK_BASED_KHR: Self = Self(1 << 1);
    pub const BLOCK_ROW_BASED_KHR: Self = Self(1 << 2);
    pub const BLOCK_COLUMN_BASED_KHR: Self = Self(1 << 3);
}
