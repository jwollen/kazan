#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub const MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoDecodeVP9FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_decode_vp9: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeVP9ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: StdVideoVP9Profile,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeVP9CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level: StdVideoVP9Level,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeVP9PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const StdVideoDecodeVP9PictureInfo,
    pub reference_name_slot_indices: i32,
    pub uncompressed_header_offset: u32,
    pub compressed_header_offset: u32,
    pub tiles_offset: u32,
}
