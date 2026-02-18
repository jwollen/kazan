#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FragmentShadingRateAttachmentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_fragment_shading_rate_attachment: *const AttachmentReference2,
    pub shading_rate_attachment_texel_size: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub fragment_size: Extent2D,
    pub combiner_ops: FragmentShadingRateCombinerOpKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_fragment_shading_rate: Bool32,
    pub primitive_fragment_shading_rate: Bool32,
    pub attachment_fragment_shading_rate: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_fragment_shading_rate_attachment_texel_size: Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size: Extent2D,
    pub max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    pub primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
    pub layered_shading_rate_attachments: Bool32,
    pub fragment_shading_rate_non_trivial_combiner_ops: Bool32,
    pub max_fragment_size: Extent2D,
    pub max_fragment_size_aspect_ratio: u32,
    pub max_fragment_shading_rate_coverage_samples: u32,
    pub max_fragment_shading_rate_rasterization_samples: SampleCountFlags,
    pub fragment_shading_rate_with_shader_depth_stencil_writes: Bool32,
    pub fragment_shading_rate_with_sample_mask: Bool32,
    pub fragment_shading_rate_with_shader_sample_mask: Bool32,
    pub fragment_shading_rate_with_conservative_rasterization: Bool32,
    pub fragment_shading_rate_with_fragment_shader_interlock: Bool32,
    pub fragment_shading_rate_with_custom_sample_locations: Bool32,
    pub fragment_shading_rate_strict_multiply_combiner: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFragmentShadingRateKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sample_counts: SampleCountFlags,
    pub fragment_size: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub shading_rate_attachment_texel_size: Extent2D,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FragmentShadingRateCombinerOpKHR(i32);
impl FragmentShadingRateCombinerOpKHR {
    pub const KEEP_KHR: Self = Self(0);
    pub const REPLACE_KHR: Self = Self(1);
    pub const MIN_KHR: Self = Self(2);
    pub const MAX_KHR: Self = Self(3);
    pub const MUL_KHR: Self = Self(4);
}
pub type PFN_vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_fragment_size: *const Extent2D,
    combiner_ops: *const FragmentShadingRateCombinerOpKHR,
);
pub type PFN_vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_fragment_shading_rate_count: *mut u32,
    p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
) -> Result;
