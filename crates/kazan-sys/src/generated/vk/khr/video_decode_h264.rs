#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH264ProfileIdc,
    pub picture_layout: VideoDecodeH264PictureLayoutFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level_idc: StdVideoH264LevelIdc,
    pub field_offset_granularity: Offset2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const StdVideoH264SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const StdVideoH264PictureParameterSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const StdVideoDecodeH264PictureInfo,
    pub slice_count: u32,
    pub p_slice_offsets: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoDecodeH264ReferenceInfo,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoDecodeH264PictureLayoutFlagsKHR: Flags {
        const INTERLACED_INTERLEAVED_LINES_KHR = 1 << 0;
        const INTERLACED_SEPARATE_PLANES_KHR = 1 << 1;
        const PROGRESSIVE = 0;
    }
}
