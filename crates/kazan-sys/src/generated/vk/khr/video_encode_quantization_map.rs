#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQuantizationMapCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_quantization_map_extent: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeQuantizationMapCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            max_quantization_map_extent: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264QuantizationMapCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_qp_delta: i32,
    pub max_qp_delta: i32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264QuantizationMapCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            min_qp_delta: Default::default(),
            max_qp_delta: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265QuantizationMapCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_qp_delta: i32,
    pub max_qp_delta: i32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH265QuantizationMapCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            min_qp_delta: Default::default(),
            max_qp_delta: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1QuantizationMapCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_q_index_delta: i32,
    pub max_q_index_delta: i32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1QuantizationMapCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            min_q_index_delta: Default::default(),
            max_q_index_delta: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoFormatQuantizationMapPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub quantization_map_texel_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoFormatQuantizationMapPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            quantization_map_texel_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoFormatH265QuantizationMapPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compatible_ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoFormatH265QuantizationMapPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            compatible_ctb_sizes: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoFormatAV1QuantizationMapPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compatible_superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoFormatAV1QuantizationMapPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            compatible_superblock_sizes: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQuantizationMapInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quantization_map: ImageView,
    pub quantization_map_extent: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeQuantizationMapInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR,
            p_next: core::ptr::null(),
            quantization_map: Default::default(),
            quantization_map_extent: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quantization_map_texel_size: Extent2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeQuantizationMapSessionParametersCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            quantization_map_texel_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_quantization_map: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            video_encode_quantization_map: Default::default(),
            _marker: PhantomData,
        }
    }
}
