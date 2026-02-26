#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
#[repr(C)]
pub struct PhysicalDeviceVideoDecodeVP9FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_decode_vp9: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeVP9ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: StdVideoVP9Profile,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeVP9CapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level: StdVideoVP9Level,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
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
