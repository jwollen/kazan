#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type RayTracingInvocationReorderModeNV = RayTracingInvocationReorderModeEXT;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
}
