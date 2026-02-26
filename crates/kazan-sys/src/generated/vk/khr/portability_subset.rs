#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
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
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_vertex_input_binding_stride_alignment: u32,
    pub _marker: PhantomData<&'a ()>,
}
