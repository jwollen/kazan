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
#[derive(Copy, Clone)]
pub struct PipelineBinaryCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_keys_and_data_info: *const PipelineBinaryKeysAndDataKHR<'a>,
    pub pipeline: Pipeline,
    pub p_pipeline_create_info: *const PipelineCreateInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineBinaryCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_BINARY_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            p_keys_and_data_info: core::ptr::null(),
            pipeline: Default::default(),
            p_pipeline_create_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryHandlesInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_binary_count: u32,
    pub p_pipeline_binaries: *mut PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineBinaryHandlesInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_BINARY_HANDLES_INFO_KHR,
            p_next: core::ptr::null(),
            pipeline_binary_count: Default::default(),
            p_pipeline_binaries: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryDataKHR<'a> {
    pub data_size: usize,
    pub p_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineBinaryDataKHR<'_> {
    fn default() -> Self {
        Self {
            data_size: Default::default(),
            p_data: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryKeysAndDataKHR<'a> {
    pub binary_count: u32,
    pub p_pipeline_binary_keys: *const PipelineBinaryKeyKHR<'a>,
    pub p_pipeline_binary_data: *const PipelineBinaryDataKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineBinaryKeysAndDataKHR<'_> {
    fn default() -> Self {
        Self {
            binary_count: Default::default(),
            p_pipeline_binary_keys: core::ptr::null(),
            p_pipeline_binary_data: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryKeyKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub key_size: u32,
    pub key: [u8; MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineBinaryKeyKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_BINARY_KEY_KHR,
            p_next: core::ptr::null_mut(),
            key_size: Default::default(),
            key: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binary_count: u32,
    pub p_pipeline_binaries: *const PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineBinaryInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_BINARY_INFO_KHR,
            p_next: core::ptr::null(),
            binary_count: Default::default(),
            p_pipeline_binaries: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReleaseCapturedPipelineDataInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ReleaseCapturedPipelineDataInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR,
            p_next: core::ptr::null_mut(),
            pipeline: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineBinaryDataInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binary: PipelineBinaryKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineBinaryDataInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_BINARY_DATA_INFO_KHR,
            p_next: core::ptr::null_mut(),
            pipeline_binary: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_CREATE_INFO_KHR,
            p_next: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineBinaryFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_binaries: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePipelineBinaryFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            pipeline_binaries: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DevicePipelineBinaryInternalCacheControlKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disable_internal_cache: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DevicePipelineBinaryInternalCacheControlKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR,
            p_next: core::ptr::null(),
            disable_internal_cache: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PhysicalDevicePipelineBinaryPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            pipeline_binary_internal_cache: Default::default(),
            pipeline_binary_internal_cache_control: Default::default(),
            pipeline_binary_prefers_internal_cache: Default::default(),
            pipeline_binary_precompiled_internal_cache: Default::default(),
            pipeline_binary_compressed_data: Default::default(),
            _marker: PhantomData,
        }
    }
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
