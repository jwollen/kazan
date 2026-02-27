#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_early_and_late_fragment_tests: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'_> {
    fn default() -> Self {
        Self {
            s_type:
                StructureType::PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD,
            p_next: core::ptr::null_mut(),
            shader_early_and_late_fragment_tests: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a> {
    pub fn shader_early_and_late_fragment_tests(
        mut self,
        shader_early_and_late_fragment_tests: Bool32,
    ) -> Self {
        self.shader_early_and_late_fragment_tests = shader_early_and_late_fragment_tests;
        self
    }
}
