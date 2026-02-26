#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_executable_info: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineExecutablePropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stages: ShaderStageFlags,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub subgroup_size: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineExecutableInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
    pub executable_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineExecutableStatisticKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub format: PipelineExecutableStatisticFormatKHR,
    pub value: PipelineExecutableStatisticValueKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineExecutableInternalRepresentationKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub is_text: Bool32,
    pub data_size: usize,
    pub p_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PipelineExecutableStatisticValueKHR {
    pub b32: Bool32,
    pub i64: i64,
    pub u64: u64,
    pub f64: f64,
}
impl Default for PipelineExecutableStatisticValueKHR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineExecutableStatisticFormatKHR(i32);
impl PipelineExecutableStatisticFormatKHR {
    pub const BOOL32_KHR: Self = Self(0);
    pub const INT64_KHR: Self = Self(1);
    pub const UINT64_KHR: Self = Self(2);
    pub const FLOAT64_KHR: Self = Self(3);
}
pub type PFN_vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const PipelineInfoKHR<'_>,
    p_executable_count: *mut u32,
    p_properties: *mut PipelineExecutablePropertiesKHR<'_>,
) -> Result;
pub type PFN_vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
    device: Device,
    p_executable_info: *const PipelineExecutableInfoKHR<'_>,
    p_statistic_count: *mut u32,
    p_statistics: *mut PipelineExecutableStatisticKHR<'_>,
) -> Result;
pub type PFN_vkGetPipelineExecutableInternalRepresentationsKHR =
    unsafe extern "system" fn(
        device: Device,
        p_executable_info: *const PipelineExecutableInfoKHR<'_>,
        p_internal_representation_count: *mut u32,
        p_internal_representations: *mut PipelineExecutableInternalRepresentationKHR<'_>,
    ) -> Result;
