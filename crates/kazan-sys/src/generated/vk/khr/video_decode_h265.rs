#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct VideoDecodeH265ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH265ProfileIdc,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeH265CapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level_idc: StdVideoH265LevelIdc,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeH265SessionParametersAddInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_vps_count: u32,
    pub p_std_vp_ss: *const StdVideoH265VideoParameterSet<'a>,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const StdVideoH265SequenceParameterSet<'a>,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const StdVideoH265PictureParameterSet<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeH265SessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_vps_count: u32,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoDecodeH265SessionParametersAddInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeH265PictureInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const StdVideoDecodeH265PictureInfo,
    pub slice_segment_count: u32,
    pub p_slice_segment_offsets: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeH265DpbSlotInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoDecodeH265ReferenceInfo,
    pub _marker: PhantomData<&'a ()>,
}
