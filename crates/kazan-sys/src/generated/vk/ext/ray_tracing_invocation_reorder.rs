#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
    pub max_shader_binding_table_record_index: u32,
    pub _marker: PhantomData<&'a ()>,
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
