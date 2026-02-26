#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct VideoEncodeQuantizationMapCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_quantization_map_extent: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoEncodeH264QuantizationMapCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_qp_delta: i32,
    pub max_qp_delta: i32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoEncodeH265QuantizationMapCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_qp_delta: i32,
    pub max_qp_delta: i32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoEncodeAV1QuantizationMapCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_q_index_delta: i32,
    pub max_q_index_delta: i32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoFormatQuantizationMapPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub quantization_map_texel_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoFormatH265QuantizationMapPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compatible_ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoFormatAV1QuantizationMapPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compatible_superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoEncodeQuantizationMapInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quantization_map: ImageView,
    pub quantization_map_extent: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quantization_map_texel_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_quantization_map: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
