#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_executable_info: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            pipeline_executable_info: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDevicePipelineExecutablePropertiesFeaturesKHR<'a> {
    pub fn pipeline_executable_info(mut self, pipeline_executable_info: Bool32) -> Self {
        self.pipeline_executable_info = pipeline_executable_info;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_INFO_KHR,
            p_next: core::ptr::null(),
            pipeline: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineInfoKHR<'a> {
    pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = pipeline;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineExecutablePropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stages: ShaderStageFlags,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub subgroup_size: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineExecutablePropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_EXECUTABLE_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            stages: Default::default(),
            name: [Default::default(); _],
            description: [Default::default(); _],
            subgroup_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineExecutablePropertiesKHR<'a> {
    pub fn stages(mut self, stages: ShaderStageFlags) -> Self {
        self.stages = stages;
        self
    }
    pub fn subgroup_size(mut self, subgroup_size: u32) -> Self {
        self.subgroup_size = subgroup_size;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineExecutableInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline: Pipeline,
    pub executable_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineExecutableInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_EXECUTABLE_INFO_KHR,
            p_next: core::ptr::null(),
            pipeline: Default::default(),
            executable_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineExecutableInfoKHR<'a> {
    pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
        self.pipeline = pipeline;
        self
    }
    pub fn executable_index(mut self, executable_index: u32) -> Self {
        self.executable_index = executable_index;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineExecutableStatisticKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub format: PipelineExecutableStatisticFormatKHR,
    pub value: PipelineExecutableStatisticValueKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineExecutableStatisticKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_EXECUTABLE_STATISTIC_KHR,
            p_next: core::ptr::null_mut(),
            name: [Default::default(); _],
            description: [Default::default(); _],
            format: Default::default(),
            value: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineExecutableStatisticKHR<'a> {
    pub fn format(mut self, format: PipelineExecutableStatisticFormatKHR) -> Self {
        self.format = format;
        self
    }
    pub fn value(mut self, value: PipelineExecutableStatisticValueKHR) -> Self {
        self.value = value;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for PipelineExecutableInternalRepresentationKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR,
            p_next: core::ptr::null_mut(),
            name: [Default::default(); _],
            description: [Default::default(); _],
            is_text: Default::default(),
            data_size: Default::default(),
            p_data: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineExecutableInternalRepresentationKHR<'a> {
    pub fn is_text(mut self, is_text: Bool32) -> Self {
        self.is_text = is_text;
        self
    }
    pub fn data(mut self, data: &'a mut [u8]) -> Self {
        self.data_size = data.len().try_into().unwrap();
        self.p_data = data.as_mut_ptr() as _;
        self
    }
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
