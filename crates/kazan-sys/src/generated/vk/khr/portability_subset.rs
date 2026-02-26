#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub constant_alpha_color_blend_factors: Bool32,
    pub events: Bool32,
    pub image_view_format_reinterpretation: Bool32,
    pub image_view_format_swizzle: Bool32,
    pub image_view2_d_on3_d_image: Bool32,
    pub multisample_array_image: Bool32,
    pub mutable_comparison_samplers: Bool32,
    pub point_polygons: Bool32,
    pub sampler_mip_lod_bias: Bool32,
    pub separate_stencil_mask_ref: Bool32,
    pub shader_sample_rate_interpolation_functions: Bool32,
    pub tessellation_isolines: Bool32,
    pub tessellation_point_mode: Bool32,
    pub triangle_fans: Bool32,
    pub vertex_attribute_access_beyond_stride: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePortabilitySubsetFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            constant_alpha_color_blend_factors: Default::default(),
            events: Default::default(),
            image_view_format_reinterpretation: Default::default(),
            image_view_format_swizzle: Default::default(),
            image_view2_d_on3_d_image: Default::default(),
            multisample_array_image: Default::default(),
            mutable_comparison_samplers: Default::default(),
            point_polygons: Default::default(),
            sampler_mip_lod_bias: Default::default(),
            separate_stencil_mask_ref: Default::default(),
            shader_sample_rate_interpolation_functions: Default::default(),
            tessellation_isolines: Default::default(),
            tessellation_point_mode: Default::default(),
            triangle_fans: Default::default(),
            vertex_attribute_access_beyond_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_vertex_input_binding_stride_alignment: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePortabilitySubsetPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            min_vertex_input_binding_stride_alignment: Default::default(),
            _marker: PhantomData,
        }
    }
}
