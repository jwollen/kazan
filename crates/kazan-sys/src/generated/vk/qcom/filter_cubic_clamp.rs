#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceCubicClampFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub cubic_range_clamp: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceCubicClampFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            cubic_range_clamp: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceCubicClampFeaturesQCOM<'a> {
    pub fn cubic_range_clamp(mut self, cubic_range_clamp: Bool32) -> Self {
        self.cubic_range_clamp = cubic_range_clamp;
        self
    }
}
