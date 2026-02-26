#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH264ProfileIdc,
    pub picture_layout: VideoDecodeH264PictureLayoutFlagBitsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH264ProfileInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H264_PROFILE_INFO_KHR,
            p_next: core::ptr::null(),
            std_profile_idc: Default::default(),
            picture_layout: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264CapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_level_idc: StdVideoH264LevelIdc,
    pub field_offset_granularity: Offset2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH264CapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H264_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            max_level_idc: Default::default(),
            field_offset_granularity: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264SessionParametersAddInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const StdVideoH264SequenceParameterSet<'a>,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const StdVideoH264PictureParameterSet<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH264SessionParametersAddInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR,
            p_next: core::ptr::null(),
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
pub struct VideoDecodeH264SessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH264SessionParametersCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            max_std_sps_count: Default::default(),
            max_std_pps_count: Default::default(),
            p_parameters_add_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264PictureInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_picture_info: *const StdVideoDecodeH264PictureInfo,
    pub slice_count: u32,
    pub p_slice_offsets: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH264PictureInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H264_PICTURE_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_picture_info: core::ptr::null(),
            slice_count: Default::default(),
            p_slice_offsets: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeH264DpbSlotInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoDecodeH264ReferenceInfo,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeH264DpbSlotInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_reference_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoDecodeH264PictureLayoutFlagsKHR: Flags {
        const INTERLACED_INTERLEAVED_LINES_KHR = VideoDecodeH264PictureLayoutFlagBitsKHR::INTERLACED_INTERLEAVED_LINES_KHR.0;
        const INTERLACED_SEPARATE_PLANES_KHR = VideoDecodeH264PictureLayoutFlagBitsKHR::INTERLACED_SEPARATE_PLANES_KHR.0;
        const PROGRESSIVE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeH264PictureLayoutFlagBitsKHR(u32);
impl VideoDecodeH264PictureLayoutFlagBitsKHR {
    pub const INTERLACED_INTERLEAVED_LINES_KHR: Self = Self(1 << 0);
    pub const INTERLACED_SEPARATE_PLANES_KHR: Self = Self(1 << 1);
}
