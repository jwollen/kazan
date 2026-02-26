#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceImageProcessing2FeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_block_match2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceImageProcessing2PropertiesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_block_match_window: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SamplerBlockMatchWindowCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub window_extent: Extent2D,
    pub window_compare_mode: BlockMatchWindowCompareModeQCOM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockMatchWindowCompareModeQCOM(i32);
impl BlockMatchWindowCompareModeQCOM {
    pub const MIN_QCOM: Self = Self(0);
    pub const MAX_QCOM: Self = Self(1);
}
