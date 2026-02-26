#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometryLinearSweptSpheresDataNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR<'a>,
    pub vertex_stride: DeviceSize,
    pub radius_format: Format,
    pub radius_data: DeviceOrHostAddressConstKHR<'a>,
    pub radius_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR<'a>,
    pub index_stride: DeviceSize,
    pub indexing_mode: RayTracingLssIndexingModeNV,
    pub end_caps_mode: RayTracingLssPrimitiveEndCapsModeNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureGeometryLinearSweptSpheresDataNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV,
            p_next: core::ptr::null(),
            vertex_format: Default::default(),
            vertex_data: Default::default(),
            vertex_stride: Default::default(),
            radius_format: Default::default(),
            radius_data: Default::default(),
            radius_stride: Default::default(),
            index_type: Default::default(),
            index_data: Default::default(),
            index_stride: Default::default(),
            indexing_mode: Default::default(),
            end_caps_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureGeometrySpheresDataNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_format: Format,
    pub vertex_data: DeviceOrHostAddressConstKHR<'a>,
    pub vertex_stride: DeviceSize,
    pub radius_format: Format,
    pub radius_data: DeviceOrHostAddressConstKHR<'a>,
    pub radius_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_data: DeviceOrHostAddressConstKHR<'a>,
    pub index_stride: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureGeometrySpheresDataNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV,
            p_next: core::ptr::null(),
            vertex_format: Default::default(),
            vertex_data: Default::default(),
            vertex_stride: Default::default(),
            radius_format: Default::default(),
            radius_data: Default::default(),
            radius_stride: Default::default(),
            index_type: Default::default(),
            index_data: Default::default(),
            index_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub spheres: Bool32,
    pub linear_swept_spheres: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            spheres: Default::default(),
            linear_swept_spheres: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RayTracingLssIndexingModeNV(i32);
impl RayTracingLssIndexingModeNV {
    pub const LIST_NV: Self = Self(0);
    pub const SUCCESSIVE_NV: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RayTracingLssPrimitiveEndCapsModeNV(i32);
impl RayTracingLssPrimitiveEndCapsModeNV {
    pub const NONE_NV: Self = Self(0);
    pub const CHAINED_NV: Self = Self(1);
}
