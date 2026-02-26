#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;
#[repr(C)]
pub struct VideoDecodeAV1ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: StdVideoAV1Profile,
    pub film_grain_support: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeAV1CapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level: StdVideoAV1Level,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeAV1SessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const StdVideoAV1SequenceHeader<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeAV1PictureInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const StdVideoDecodeAV1PictureInfo<'a>,
    pub reference_name_slot_indices: [i32; MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
    pub frame_header_offset: u32,
    pub tile_count: u32,
    pub p_tile_offsets: *const u32,
    pub p_tile_sizes: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeAV1DpbSlotInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoDecodeAV1ReferenceInfo,
    pub _marker: PhantomData<&'a ()>,
}
