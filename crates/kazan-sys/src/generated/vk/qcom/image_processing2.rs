#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageProcessing2FeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_block_match2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageProcessing2FeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            texture_block_match2: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageProcessing2PropertiesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_block_match_window: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceImageProcessing2PropertiesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM,
            p_next: core::ptr::null_mut(),
            max_block_match_window: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerBlockMatchWindowCreateInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub window_extent: Extent2D,
    pub window_compare_mode: BlockMatchWindowCompareModeQCOM,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerBlockMatchWindowCreateInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM,
            p_next: core::ptr::null(),
            window_extent: Default::default(),
            window_compare_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlockMatchWindowCompareModeQCOM(i32);
impl BlockMatchWindowCompareModeQCOM {
    pub const MIN_QCOM: Self = Self(0);
    pub const MAX_QCOM: Self = Self(1);
}
