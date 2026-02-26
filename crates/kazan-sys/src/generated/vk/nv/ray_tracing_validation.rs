#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingValidationFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_validation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayTracingValidationFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            ray_tracing_validation: Default::default(),
            _marker: PhantomData,
        }
    }
}
