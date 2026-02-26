#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PipelineBinaryKHR(u64);
#[repr(C)]
pub struct PipelineBinaryCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_keys_and_data_info: *const PipelineBinaryKeysAndDataKHR<'a>,
    pub pipeline: Pipeline,
    pub p_pipeline_create_info: *const PipelineCreateInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineBinaryHandlesInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_binary_count: u32,
    pub p_pipeline_binaries: *mut PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineBinaryDataKHR<'a> {
    pub data_size: usize,
    pub p_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineBinaryKeysAndDataKHR<'a> {
    pub binary_count: u32,
    pub p_pipeline_binary_keys: *const PipelineBinaryKeyKHR<'a>,
    pub p_pipeline_binary_data: *const PipelineBinaryDataKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineBinaryKeyKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub key_size: u32,
    pub key: [u8; MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize],
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineBinaryInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binary_count: u32,
    pub p_pipeline_binaries: *const PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ReleaseCapturedPipelineDataInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineBinaryDataInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binary: PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDevicePipelineBinaryFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binaries: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DevicePipelineBinaryInternalCacheControlKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disable_internal_cache: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDevicePipelineBinaryPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binary_internal_cache: Bool32,
    pub pipeline_binary_internal_cache_control: Bool32,
    pub pipeline_binary_prefers_internal_cache: Bool32,
    pub pipeline_binary_precompiled_internal_cache: Bool32,
    pub pipeline_binary_compressed_data: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineBinaryCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_binaries: *mut PipelineBinaryHandlesInfoKHR<'_>,
) -> Result;
pub type PFN_vkDestroyPipelineBinaryKHR = unsafe extern "system" fn(
    device: Device,
    pipeline_binary: PipelineBinaryKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetPipelineKeyKHR = unsafe extern "system" fn(
    device: Device,
    p_pipeline_create_info: *const PipelineCreateInfoKHR<'_>,
    p_pipeline_key: *mut PipelineBinaryKeyKHR<'_>,
) -> Result;
pub type PFN_vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
    device: Device,
    p_info: *const PipelineBinaryDataInfoKHR<'_>,
    p_pipeline_binary_key: *mut PipelineBinaryKeyKHR<'_>,
    p_pipeline_binary_data_size: *mut usize,
    p_pipeline_binary_data: *mut c_void,
) -> Result;
pub type PFN_vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn(
    device: Device,
    p_info: *const ReleaseCapturedPipelineDataInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
) -> Result;
