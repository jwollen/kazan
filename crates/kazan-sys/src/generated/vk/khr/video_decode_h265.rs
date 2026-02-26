#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH265ProfileIdc,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH265ProfileInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H265_PROFILE_INFO_KHR,
            p_next: core::ptr::null(),
            std_profile_idc: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265CapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level_idc: StdVideoH265LevelIdc,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH265CapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H265_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            max_level_idc: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoDecodeH265SessionParametersAddInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR,
            p_next: core::ptr::null(),
            std_vps_count: Default::default(),
            p_std_vp_ss: core::ptr::null(),
            std_sps_count: Default::default(),
            p_std_sp_ss: core::ptr::null(),
            std_pps_count: Default::default(),
            p_std_pp_ss: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265SessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_vps_count: u32,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoDecodeH265SessionParametersAddInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH265SessionParametersCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            max_std_vps_count: Default::default(),
            max_std_sps_count: Default::default(),
            max_std_pps_count: Default::default(),
            p_parameters_add_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265PictureInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const StdVideoDecodeH265PictureInfo,
    pub slice_segment_count: u32,
    pub p_slice_segment_offsets: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH265PictureInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H265_PICTURE_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_picture_info: core::ptr::null(),
            slice_segment_count: Default::default(),
            p_slice_segment_offsets: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH265DpbSlotInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoDecodeH265ReferenceInfo,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH265DpbSlotInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_reference_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
