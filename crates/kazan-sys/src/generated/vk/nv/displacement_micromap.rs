#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceDisplacementMicromapFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub displacement_micromap: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceDisplacementMicromapPropertiesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_displacement_micromap_subdivision_level: u32,
}
#[repr(C)]
pub struct AccelerationStructureTrianglesDisplacementMicromapNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub displacement_bias_and_scale_format: Format,
    pub displacement_vector_format: Format,
    pub displacement_bias_and_scale_buffer: DeviceOrHostAddressConstKHR,
    pub displacement_bias_and_scale_stride: DeviceSize,
    pub displacement_vector_buffer: DeviceOrHostAddressConstKHR,
    pub displacement_vector_stride: DeviceSize,
    pub displaced_micromap_primitive_flags: DeviceOrHostAddressConstKHR,
    pub displaced_micromap_primitive_flags_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_buffer: DeviceOrHostAddressConstKHR,
    pub index_stride: DeviceSize,
    pub base_triangle: u32,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub micromap: MicromapEXT,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplacementMicromapFormatNV(i32);
impl DisplacementMicromapFormatNV {
    pub const _64_TRIANGLES_64_BYTES_NV: Self = Self(1);
    pub const _256_TRIANGLES_128_BYTES_NV: Self = Self(2);
    pub const _1024_TRIANGLES_128_BYTES_NV: Self = Self(3);
}
