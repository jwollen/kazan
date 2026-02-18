#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputeOccupancyPriorityParametersNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub occupancy_priority: f32,
    pub occupancy_throttling: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceComputeOccupancyPriorityFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub compute_occupancy_priority: Bool32,
}
pub type PFN_vkCmdSetComputeOccupancyPriorityNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_parameters: *const ComputeOccupancyPriorityParametersNV,
);
