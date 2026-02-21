#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_executable_info: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineExecutablePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stages: ShaderStageFlags,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub subgroup_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineExecutableInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
    pub executable_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineExecutableStatisticKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub format: PipelineExecutableStatisticFormatKHR,
    pub value: PipelineExecutableStatisticValueKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineExecutableInternalRepresentationKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub is_text: Bool32,
    pub data_size: usize,
    pub p_data: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PipelineExecutableStatisticValueKHR {
    pub b32: Bool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineExecutableStatisticFormatKHR(i32);
impl PipelineExecutableStatisticFormatKHR {
    pub const BOOL32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT64_KHR: Self = Self(2);
    pub const FLOAT64_KHR: Self = Self(3);
}
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoKHR,
    p_executable_count: *mut u32,
    p_properties: *mut PipelineExecutablePropertiesKHR,
) -> Result;
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
    device: Device,
    p_executable_info: *const PipelineExecutableInfoKHR,
    p_statistic_count: *mut u32,
    p_statistics: *mut PipelineExecutableStatisticKHR,
) -> Result;
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
    unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
    ) -> Result;
