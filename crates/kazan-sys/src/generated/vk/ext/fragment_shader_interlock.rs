#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub fragment_shader_sample_interlock: Bool32,
    pub fragment_shader_pixel_interlock: Bool32,
    pub fragment_shader_shading_rate_interlock: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            fragment_shader_sample_interlock: Default::default(),
            fragment_shader_pixel_interlock: Default::default(),
            fragment_shader_shading_rate_interlock: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceFragmentShaderInterlockFeaturesEXT<'a> {
    pub fn fragment_shader_sample_interlock(
        mut self,
        fragment_shader_sample_interlock: Bool32,
    ) -> Self {
        self.fragment_shader_sample_interlock = fragment_shader_sample_interlock;
        self
    }
    pub fn fragment_shader_pixel_interlock(
        mut self,
        fragment_shader_pixel_interlock: Bool32,
    ) -> Self {
        self.fragment_shader_pixel_interlock = fragment_shader_pixel_interlock;
        self
    }
    pub fn fragment_shader_shading_rate_interlock(
        mut self,
        fragment_shader_shading_rate_interlock: Bool32,
    ) -> Self {
        self.fragment_shader_shading_rate_interlock = fragment_shader_shading_rate_interlock;
        self
    }
}
