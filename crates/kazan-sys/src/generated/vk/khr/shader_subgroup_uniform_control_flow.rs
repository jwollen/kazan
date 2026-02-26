#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_uniform_control_flow: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type:
                StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            shader_subgroup_uniform_control_flow: Default::default(),
            _marker: PhantomData,
        }
    }
}
