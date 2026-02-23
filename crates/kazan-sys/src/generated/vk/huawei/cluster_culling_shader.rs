#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_work_group_count: [u32; 3],
    pub max_work_group_size: [u32; 3],
    pub max_output_cluster_count: u32,
    pub indirect_buffer_offset_alignment: DeviceSize,
}
#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub clusterculling_shader: Bool32,
    pub multiview_cluster_culling_shader: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cluster_shading_rate: Bool32,
}
pub type PFN_vkCmdDrawClusterHUAWEI = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
pub type PFN_vkCmdDrawClusterIndirectHUAWEI =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
