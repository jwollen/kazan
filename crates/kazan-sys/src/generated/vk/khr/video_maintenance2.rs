#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceVideoMaintenance2FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_maintenance2: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeH264InlineSessionParametersInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sps: *const StdVideoH264SequenceParameterSet<'a>,
    pub p_std_pps: *const StdVideoH264PictureParameterSet<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeH265InlineSessionParametersInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_vps: *const StdVideoH265VideoParameterSet<'a>,
    pub p_std_sps: *const StdVideoH265SequenceParameterSet<'a>,
    pub p_std_pps: *const StdVideoH265PictureParameterSet<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoDecodeAV1InlineSessionParametersInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const StdVideoAV1SequenceHeader<'a>,
    pub _marker: PhantomData<&'a ()>,
}
