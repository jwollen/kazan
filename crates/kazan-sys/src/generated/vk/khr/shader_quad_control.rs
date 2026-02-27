#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderQuadControlFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_quad_control: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderQuadControlFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            shader_quad_control: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShaderQuadControlFeaturesKHR<'a> {
    pub fn shader_quad_control(mut self, shader_quad_control: Bool32) -> Self {
        self.shader_quad_control = shader_quad_control;
        self
    }
}
