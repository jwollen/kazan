#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryLinearSweptSpheresDataNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR,
    pub vertex_stride: DeviceSize,
    pub radius_format: Format,
    pub radius_data: DeviceOrHostAddressConstKHR,
    pub radius_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR,
    pub index_stride: DeviceSize,
    pub indexing_mode: RayTracingLssIndexingModeNV,
    pub end_caps_mode: RayTracingLssPrimitiveEndCapsModeNV,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometrySpheresDataNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR,
    pub vertex_stride: DeviceSize,
    pub radius_format: Format,
    pub radius_data: DeviceOrHostAddressConstKHR,
    pub radius_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR,
    pub index_stride: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub spheres: Bool32,
    pub linear_swept_spheres: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RayTracingLssIndexingModeNV(i32);
impl RayTracingLssIndexingModeNV {
    pub const LIST_NV: Self = Self(0);
    pub const SUCCESSIVE_NV: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RayTracingLssPrimitiveEndCapsModeNV(i32);
impl RayTracingLssPrimitiveEndCapsModeNV {
    pub const NONE_NV: Self = Self(0);
    pub const CHAINED_NV: Self = Self(1);
}
