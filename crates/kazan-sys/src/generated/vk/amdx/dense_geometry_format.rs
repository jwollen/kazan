#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dense_geometry_format: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDenseGeometryFormatFeaturesAMDX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX,
            p_next: core::ptr::null_mut(),
            dense_geometry_format: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compressed_data: DeviceOrHostAddressConstKHR<'a>,
    pub data_size: DeviceSize,
    pub num_triangles: u32,
    pub num_vertices: u32,
    pub max_primitive_index: u32,
    pub max_geometry_index: u32,
    pub format: CompressedTriangleFormatAMDX,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureDenseGeometryFormatTrianglesDataAMDX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX,
            p_next: core::ptr::null(),
            compressed_data: Default::default(),
            data_size: Default::default(),
            num_triangles: Default::default(),
            num_vertices: Default::default(),
            max_primitive_index: Default::default(),
            max_geometry_index: Default::default(),
            format: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompressedTriangleFormatAMDX(i32);
impl CompressedTriangleFormatAMDX {
    pub const DGF1_AMDX: Self = Self(0);
}
