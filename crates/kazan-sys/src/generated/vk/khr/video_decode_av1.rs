#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeAV1ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: StdVideoAV1Profile,
    pub film_grain_support: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeAV1ProfileInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_AV1_PROFILE_INFO_KHR,
            p_next: core::ptr::null(),
            std_profile: Default::default(),
            film_grain_support: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeAV1CapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level: StdVideoAV1Level,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeAV1CapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_AV1_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            max_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeAV1SessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const StdVideoAV1SequenceHeader<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeAV1SessionParametersCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_sequence_header: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoDecodeAV1PictureInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_AV1_PICTURE_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_picture_info: core::ptr::null(),
            reference_name_slot_indices: [Default::default(); _],
            frame_header_offset: Default::default(),
            tile_count: Default::default(),
            p_tile_offsets: core::ptr::null(),
            p_tile_sizes: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeAV1DpbSlotInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoDecodeAV1ReferenceInfo,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeAV1DpbSlotInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_reference_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
