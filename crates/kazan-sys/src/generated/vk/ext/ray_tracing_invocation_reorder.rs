#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            ray_tracing_invocation_reorder: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
    pub max_shader_binding_table_record_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            ray_tracing_invocation_reorder_reordering_hint: Default::default(),
            max_shader_binding_table_record_index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RayTracingInvocationReorderModeEXT(i32);
impl RayTracingInvocationReorderModeEXT {
    pub const NONE_EXT: Self = Self(0);
    pub const REORDER_EXT: Self = Self(1);
    pub const NONE_NV: Self = Self::NONE_EXT;
    pub const REORDER_NV: Self = Self::REORDER_EXT;
}
