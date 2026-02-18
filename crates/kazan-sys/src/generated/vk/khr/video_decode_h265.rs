#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH265ProfileIdc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level_idc: StdVideoH265LevelIdc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_vps_count: u32,
    pub p_std_vp_ss: *const StdVideoH265VideoParameterSet,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const StdVideoH265SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const StdVideoH265PictureParameterSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_vps_count: u32,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoDecodeH265SessionParametersAddInfoKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const StdVideoDecodeH265PictureInfo,
    pub slice_segment_count: u32,
    pub p_slice_segment_offsets: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoDecodeH265ReferenceInfo,
}
