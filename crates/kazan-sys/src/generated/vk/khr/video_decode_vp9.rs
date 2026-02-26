#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoDecodeVP9FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_decode_vp9: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVideoDecodeVP9FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            video_decode_vp9: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeVP9ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: StdVideoVP9Profile,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeVP9ProfileInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_VP9_PROFILE_INFO_KHR,
            p_next: core::ptr::null(),
            std_profile: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeVP9CapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level: StdVideoVP9Level,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeVP9CapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_VP9_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            max_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeVP9PictureInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const StdVideoDecodeVP9PictureInfo<'a>,
    pub reference_name_slot_indices: [i32; MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR as usize],
    pub uncompressed_header_offset: u32,
    pub compressed_header_offset: u32,
    pub tiles_offset: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeVP9PictureInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_VP9_PICTURE_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_picture_info: core::ptr::null(),
            reference_name_slot_indices: [Default::default(); _],
            uncompressed_header_offset: Default::default(),
            compressed_header_offset: Default::default(),
            tiles_offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
