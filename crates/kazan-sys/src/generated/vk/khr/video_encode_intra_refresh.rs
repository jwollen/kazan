#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeIntraRefreshCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub intra_refresh_modes: VideoEncodeIntraRefreshModeFlagsKHR,
    pub max_intra_refresh_cycle_duration: u32,
    pub max_intra_refresh_active_reference_pictures: u32,
    pub partition_independent_intra_refresh_regions: Bool32,
    pub non_rectangular_intra_refresh_regions: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeIntraRefreshCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            intra_refresh_modes: Default::default(),
            max_intra_refresh_cycle_duration: Default::default(),
            max_intra_refresh_active_reference_pictures: Default::default(),
            partition_independent_intra_refresh_regions: Default::default(),
            non_rectangular_intra_refresh_regions: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeSessionIntraRefreshCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub intra_refresh_mode: VideoEncodeIntraRefreshModeFlagBitsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeSessionIntraRefreshCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            intra_refresh_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeIntraRefreshInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub intra_refresh_cycle_duration: u32,
    pub intra_refresh_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeIntraRefreshInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR,
            p_next: core::ptr::null(),
            intra_refresh_cycle_duration: Default::default(),
            intra_refresh_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoReferenceIntraRefreshInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dirty_intra_refresh_regions: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoReferenceIntraRefreshInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR,
            p_next: core::ptr::null(),
            dirty_intra_refresh_regions: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_intra_refresh: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            video_encode_intra_refresh: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeIntraRefreshModeFlagsKHR: Flags {
        const PER_PICTURE_PARTITION_KHR = VideoEncodeIntraRefreshModeFlagBitsKHR::PER_PICTURE_PARTITION_KHR.0;
        const BLOCK_BASED_KHR = VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_BASED_KHR.0;
        const BLOCK_ROW_BASED_KHR = VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_ROW_BASED_KHR.0;
        const BLOCK_COLUMN_BASED_KHR = VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_COLUMN_BASED_KHR.0;
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeIntraRefreshModeFlagBitsKHR(u32);
impl VideoEncodeIntraRefreshModeFlagBitsKHR {
    pub const PER_PICTURE_PARTITION_KHR: Self = Self(1 << 0);
    pub const BLOCK_BASED_KHR: Self = Self(1 << 1);
    pub const BLOCK_ROW_BASED_KHR: Self = Self(1 << 2);
    pub const BLOCK_COLUMN_BASED_KHR: Self = Self(1 << 3);
}
