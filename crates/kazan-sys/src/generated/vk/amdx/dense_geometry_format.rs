#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDenseGeometryFormatFeaturesAMDX {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dense_geometry_format: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureDenseGeometryFormatTrianglesDataAMDX {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compressed_data: DeviceOrHostAddressConstKHR,
    pub data_size: DeviceSize,
    pub num_triangles: u32,
    pub num_vertices: u32,
    pub max_primitive_index: u32,
    pub max_geometry_index: u32,
    pub format: CompressedTriangleFormatAMDX,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompressedTriangleFormatAMDX(i32);
impl CompressedTriangleFormatAMDX {
    pub const DGF1_AMDX: Self = Self(0);
}
