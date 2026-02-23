#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState3FeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub extended_dynamic_state3_tessellation_domain_origin: Bool32,
    pub extended_dynamic_state3_depth_clamp_enable: Bool32,
    pub extended_dynamic_state3_polygon_mode: Bool32,
    pub extended_dynamic_state3_rasterization_samples: Bool32,
    pub extended_dynamic_state3_sample_mask: Bool32,
    pub extended_dynamic_state3_alpha_to_coverage_enable: Bool32,
    pub extended_dynamic_state3_alpha_to_one_enable: Bool32,
    pub extended_dynamic_state3_logic_op_enable: Bool32,
    pub extended_dynamic_state3_color_blend_enable: Bool32,
    pub extended_dynamic_state3_color_blend_equation: Bool32,
    pub extended_dynamic_state3_color_write_mask: Bool32,
    pub extended_dynamic_state3_rasterization_stream: Bool32,
    pub extended_dynamic_state3_conservative_rasterization_mode: Bool32,
    pub extended_dynamic_state3_extra_primitive_overestimation_size: Bool32,
    pub extended_dynamic_state3_depth_clip_enable: Bool32,
    pub extended_dynamic_state3_sample_locations_enable: Bool32,
    pub extended_dynamic_state3_color_blend_advanced: Bool32,
    pub extended_dynamic_state3_provoking_vertex_mode: Bool32,
    pub extended_dynamic_state3_line_rasterization_mode: Bool32,
    pub extended_dynamic_state3_line_stipple_enable: Bool32,
    pub extended_dynamic_state3_depth_clip_negative_one_to_one: Bool32,
    pub extended_dynamic_state3_viewport_w_scaling_enable: Bool32,
    pub extended_dynamic_state3_viewport_swizzle: Bool32,
    pub extended_dynamic_state3_coverage_to_color_enable: Bool32,
    pub extended_dynamic_state3_coverage_to_color_location: Bool32,
    pub extended_dynamic_state3_coverage_modulation_mode: Bool32,
    pub extended_dynamic_state3_coverage_modulation_table_enable: Bool32,
    pub extended_dynamic_state3_coverage_modulation_table: Bool32,
    pub extended_dynamic_state3_coverage_reduction_mode: Bool32,
    pub extended_dynamic_state3_representative_fragment_test_enable: Bool32,
    pub extended_dynamic_state3_shading_rate_image_enable: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceExtendedDynamicState3PropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_primitive_topology_unrestricted: Bool32,
}
#[repr(C)]
pub struct ColorBlendEquationEXT {
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
}
#[repr(C)]
pub struct ColorBlendAdvancedEXT {
    pub advanced_blend_op: BlendOp,
    pub src_premultiplied: Bool32,
    pub dst_premultiplied: Bool32,
    pub blend_overlap: BlendOverlapEXT,
    pub clamp_results: Bool32,
}
pub type PFN_vkCmdSetTessellationDomainOriginEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    domain_origin: TessellationDomainOrigin,
);
pub type PFN_vkCmdSetDepthClampEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clamp_enable: Bool32);
pub type PFN_vkCmdSetPolygonModeEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, polygon_mode: PolygonMode);
pub type PFN_vkCmdSetRasterizationSamplesEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    rasterization_samples: SampleCountFlagBits,
);
pub type PFN_vkCmdSetSampleMaskEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    samples: SampleCountFlagBits,
    p_sample_mask: *const SampleMask,
);
pub type PFN_vkCmdSetAlphaToCoverageEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_coverage_enable: Bool32);
pub type PFN_vkCmdSetAlphaToOneEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, alpha_to_one_enable: Bool32);
pub type PFN_vkCmdSetLogicOpEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, logic_op_enable: Bool32);
pub type PFN_vkCmdSetColorBlendEnableEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_enables: *const Bool32,
);
pub type PFN_vkCmdSetColorBlendEquationEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_equations: *const ColorBlendEquationEXT,
);
pub type PFN_vkCmdSetColorWriteMaskEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_write_masks: *const ColorComponentFlags,
);
pub type PFN_vkCmdSetRasterizationStreamEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterization_stream: u32);
pub type PFN_vkCmdSetConservativeRasterizationModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
);
pub type PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    extra_primitive_overestimation_size: f32,
);
pub type PFN_vkCmdSetDepthClipEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_clip_enable: Bool32);
pub type PFN_vkCmdSetSampleLocationsEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, sample_locations_enable: Bool32);
pub type PFN_vkCmdSetColorBlendAdvancedEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    p_color_blend_advanced: *const ColorBlendAdvancedEXT,
);
pub type PFN_vkCmdSetProvokingVertexModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    provoking_vertex_mode: ProvokingVertexModeEXT,
);
pub type PFN_vkCmdSetLineRasterizationModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    line_rasterization_mode: LineRasterizationModeEXT,
);
pub type PFN_vkCmdSetLineStippleEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, stippled_line_enable: Bool32);
pub type PFN_vkCmdSetDepthClipNegativeOneToOneEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, negative_one_to_one: Bool32);
pub type PFN_vkCmdSetViewportWScalingEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, viewport_w_scaling_enable: Bool32);
pub type PFN_vkCmdSetViewportSwizzleNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_swizzles: *const ViewportSwizzleNV,
);
pub type PFN_vkCmdSetCoverageToColorEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_enable: Bool32);
pub type PFN_vkCmdSetCoverageToColorLocationNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, coverage_to_color_location: u32);
pub type PFN_vkCmdSetCoverageModulationModeNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_mode: CoverageModulationModeNV,
);
pub type PFN_vkCmdSetCoverageModulationTableEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_table_enable: Bool32,
);
pub type PFN_vkCmdSetCoverageModulationTableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_modulation_table_count: u32,
    p_coverage_modulation_table: *const f32,
);
pub type PFN_vkCmdSetShadingRateImageEnableNV =
    unsafe extern "system" fn(command_buffer: CommandBuffer, shading_rate_image_enable: Bool32);
pub type PFN_vkCmdSetCoverageReductionModeNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    coverage_reduction_mode: CoverageReductionModeNV,
);
pub type PFN_vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    representative_fragment_test_enable: Bool32,
);
