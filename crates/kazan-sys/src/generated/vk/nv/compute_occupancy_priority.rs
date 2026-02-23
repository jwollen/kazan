#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25;
pub const COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50;
pub const COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75;
#[repr(C)]
pub struct ComputeOccupancyPriorityParametersNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub occupancy_priority: f32,
    pub occupancy_throttling: f32,
}
#[repr(C)]
pub struct PhysicalDeviceComputeOccupancyPriorityFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_occupancy_priority: Bool32,
}
pub type PFN_vkCmdSetComputeOccupancyPriorityNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_parameters: *const ComputeOccupancyPriorityParametersNV,
);
