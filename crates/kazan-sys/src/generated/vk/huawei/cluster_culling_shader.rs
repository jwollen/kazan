#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_work_group_count: [u32; 3],
    pub max_work_group_size: [u32; 3],
    pub max_output_cluster_count: u32,
    pub indirect_buffer_offset_alignment: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceClusterCullingShaderPropertiesHUAWEI<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI,
            p_next: core::ptr::null_mut(),
            max_work_group_count: [Default::default(); _],
            max_work_group_size: [Default::default(); _],
            max_output_cluster_count: Default::default(),
            indirect_buffer_offset_alignment: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub clusterculling_shader: Bool32,
    pub multiview_cluster_culling_shader: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceClusterCullingShaderFeaturesHUAWEI<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI,
            p_next: core::ptr::null_mut(),
            clusterculling_shader: Default::default(),
            multiview_cluster_culling_shader: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cluster_shading_rate: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI,
            p_next: core::ptr::null_mut(),
            cluster_shading_rate: Default::default(),
            _marker: PhantomData,
        }
    }
}
pub type PFN_vkCmdDrawClusterHUAWEI = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
pub type PFN_vkCmdDrawClusterIndirectHUAWEI =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
