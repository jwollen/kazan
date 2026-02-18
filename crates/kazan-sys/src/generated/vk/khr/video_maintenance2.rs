#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoMaintenance2FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_maintenance2: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264InlineSessionParametersInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sps: *const StdVideoH264SequenceParameterSet,
    pub p_std_pps: *const StdVideoH264PictureParameterSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265InlineSessionParametersInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_vps: *const StdVideoH265VideoParameterSet,
    pub p_std_sps: *const StdVideoH265SequenceParameterSet,
    pub p_std_pps: *const StdVideoH265PictureParameterSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeAV1InlineSessionParametersInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const StdVideoAV1SequenceHeader,
}
