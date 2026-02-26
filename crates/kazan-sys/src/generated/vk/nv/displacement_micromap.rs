#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDisplacementMicromapFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub displacement_micromap: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDisplacementMicromapFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            displacement_micromap: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDisplacementMicromapPropertiesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_displacement_micromap_subdivision_level: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDisplacementMicromapPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV,
            p_next: core::ptr::null_mut(),
            max_displacement_micromap_subdivision_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureTrianglesDisplacementMicromapNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub displacement_bias_and_scale_format: Format,
    pub displacement_vector_format: Format,
    pub displacement_bias_and_scale_buffer: DeviceOrHostAddressConstKHR<'a>,
    pub displacement_bias_and_scale_stride: DeviceSize,
    pub displacement_vector_buffer: DeviceOrHostAddressConstKHR<'a>,
    pub displacement_vector_stride: DeviceSize,
    pub displaced_micromap_primitive_flags: DeviceOrHostAddressConstKHR<'a>,
    pub displaced_micromap_primitive_flags_stride: DeviceSize,
    pub index_type: IndexType,
    pub index_buffer: DeviceOrHostAddressConstKHR<'a>,
    pub index_stride: DeviceSize,
    pub base_triangle: u32,
    pub usage_counts_count: u32,
    pub p_usage_counts: *const MicromapUsageEXT,
    pub pp_usage_counts: *const *const MicromapUsageEXT,
    pub micromap: MicromapEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureTrianglesDisplacementMicromapNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV,
            p_next: core::ptr::null_mut(),
            displacement_bias_and_scale_format: Default::default(),
            displacement_vector_format: Default::default(),
            displacement_bias_and_scale_buffer: Default::default(),
            displacement_bias_and_scale_stride: Default::default(),
            displacement_vector_buffer: Default::default(),
            displacement_vector_stride: Default::default(),
            displaced_micromap_primitive_flags: Default::default(),
            displaced_micromap_primitive_flags_stride: Default::default(),
            index_type: Default::default(),
            index_buffer: Default::default(),
            index_stride: Default::default(),
            base_triangle: Default::default(),
            usage_counts_count: Default::default(),
            p_usage_counts: core::ptr::null(),
            pp_usage_counts: core::ptr::null(),
            micromap: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplacementMicromapFormatNV(i32);
impl DisplacementMicromapFormatNV {
    pub const _64_TRIANGLES_64_BYTES_NV: Self = Self(1);
    pub const _256_TRIANGLES_128_BYTES_NV: Self = Self(2);
    pub const _1024_TRIANGLES_128_BYTES_NV: Self = Self(3);
}
