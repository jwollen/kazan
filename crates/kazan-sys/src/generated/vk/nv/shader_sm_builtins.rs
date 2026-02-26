#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_sm_count: u32,
    pub shader_warps_per_sm: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderSMBuiltinsPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            shader_sm_count: Default::default(),
            shader_warps_per_sm: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_sm_builtins: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderSMBuiltinsFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            shader_sm_builtins: Default::default(),
            _marker: PhantomData,
        }
    }
}
