#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageProcessing2FeaturesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_block_match2: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageProcessing2PropertiesQCOM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_block_match_window: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerBlockMatchWindowCreateInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub window_extent: Extent2D,
    pub window_compare_mode: BlockMatchWindowCompareModeQCOM,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockMatchWindowCompareModeQCOM(i32);
impl BlockMatchWindowCompareModeQCOM {
    pub const MIN_QCOM: Self = Self(0);
    pub const MAX_QCOM: Self = Self(1);
}
