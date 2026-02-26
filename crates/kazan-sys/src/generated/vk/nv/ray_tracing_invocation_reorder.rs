#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub type RayTracingInvocationReorderModeNV = RayTracingInvocationReorderModeEXT;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            ray_tracing_invocation_reorder: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            ray_tracing_invocation_reorder_reordering_hint: Default::default(),
            _marker: PhantomData,
        }
    }
}
