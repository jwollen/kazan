#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryKHR(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_keys_and_data_info: *const PipelineBinaryKeysAndDataKHR,
    pub pipeline: Pipeline,
    pub p_pipeline_create_info: *const PipelineCreateInfoKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryHandlesInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_binary_count: u32,
    pub p_pipeline_binaries: *mut PipelineBinaryKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryDataKHR {
    pub data_size: usize,
    pub p_data: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryKeysAndDataKHR {
    pub binary_count: u32,
    pub p_pipeline_binary_keys: *const PipelineBinaryKeyKHR,
    pub p_pipeline_binary_data: *const PipelineBinaryDataKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryKeyKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub key_size: u32,
    pub key: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binary_count: u32,
    pub p_pipeline_binaries: *const PipelineBinaryKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReleaseCapturedPipelineDataInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryDataInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binary: PipelineBinaryKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineBinaryFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binaries: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DevicePipelineBinaryInternalCacheControlKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disable_internal_cache: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineBinaryPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binary_internal_cache: Bool32,
    pub pipeline_binary_internal_cache_control: Bool32,
    pub pipeline_binary_prefers_internal_cache: Bool32,
    pub pipeline_binary_precompiled_internal_cache: Bool32,
    pub pipeline_binary_compressed_data: Bool32,
}
pub type PFN_vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineBinaryCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_binaries: *mut PipelineBinaryHandlesInfoKHR,
) -> Result;
pub type PFN_vkDestroyPipelineBinaryKHR = unsafe extern "system" fn(
    device: Device,
    pipeline_binary: PipelineBinaryKHR,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetPipelineKeyKHR = unsafe extern "system" fn(
    device: Device,
    p_pipeline_create_info: *const PipelineCreateInfoKHR,
    p_pipeline_key: *mut PipelineBinaryKeyKHR,
) -> Result;
pub type PFN_vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
    device: Device,
    p_info: *const PipelineBinaryDataInfoKHR,
    p_pipeline_binary_key: *mut PipelineBinaryKeyKHR,
    p_pipeline_binary_data_size: *mut usize,
    p_pipeline_binary_data: *mut c_void,
) -> Result;
pub type PFN_vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn(
    device: Device,
    p_info: *const ReleaseCapturedPipelineDataInfoKHR,
    p_allocator: *const AllocationCallbacks,
) -> Result;
