#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25;
pub const COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50;
pub const COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputeOccupancyPriorityParametersNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub occupancy_priority: f32,
    pub occupancy_throttling: f32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ComputeOccupancyPriorityParametersNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV,
            p_next: core::ptr::null(),
            occupancy_priority: Default::default(),
            occupancy_throttling: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceComputeOccupancyPriorityFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_occupancy_priority: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceComputeOccupancyPriorityFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            compute_occupancy_priority: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdSetComputeOccupancyPriorityNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_parameters: *const ComputeOccupancyPriorityParametersNV<'_>,
);
