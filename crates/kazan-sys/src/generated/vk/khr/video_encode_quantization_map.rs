#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQuantizationMapCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_quantization_map_extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264QuantizationMapCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_qp_delta: i32,
    pub max_qp_delta: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265QuantizationMapCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_qp_delta: i32,
    pub max_qp_delta: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1QuantizationMapCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_q_index_delta: i32,
    pub max_q_index_delta: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoFormatQuantizationMapPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub quantization_map_texel_size: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoFormatH265QuantizationMapPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compatible_ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoFormatAV1QuantizationMapPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compatible_superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQuantizationMapInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quantization_map: ImageView,
    pub quantization_map_extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQuantizationMapSessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quantization_map_texel_size: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_quantization_map: Bool32,
}
