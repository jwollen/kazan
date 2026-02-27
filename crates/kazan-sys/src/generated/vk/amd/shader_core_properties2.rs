#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderCoreProperties2AMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_core_features: ShaderCorePropertiesFlagsAMD,
    pub active_compute_unit_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderCoreProperties2AMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD,
            p_next: core::ptr::null_mut(),
            shader_core_features: Default::default(),
            active_compute_unit_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShaderCoreProperties2AMD<'a> {
    pub fn shader_core_features(
        mut self,
        shader_core_features: ShaderCorePropertiesFlagsAMD,
    ) -> Self {
        self.shader_core_features = shader_core_features;
        self
    }
    pub fn active_compute_unit_count(mut self, active_compute_unit_count: u32) -> Self {
        self.active_compute_unit_count = active_compute_unit_count;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderCorePropertiesFlagsAMD: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderCorePropertiesFlagBitsAMD(u32);
impl ShaderCorePropertiesFlagBitsAMD {}
