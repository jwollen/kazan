#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_mask: u64,
    pub shader_core_count: u32,
    pub shader_warps_per_core: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
            shader_core_mask: Default::default(),
            shader_core_count: Default::default(),
            shader_warps_per_core: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_builtins: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM,
            p_next: core::ptr::null_mut(),
            shader_core_builtins: Default::default(),
            _marker: PhantomData,
        }
    }
}
