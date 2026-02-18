#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_set_tessellation_domain_origin_ext: PFN_vkCmdSetTessellationDomainOriginEXT,
    cmd_set_depth_clamp_enable_ext: PFN_vkCmdSetDepthClampEnableEXT,
    cmd_set_polygon_mode_ext: PFN_vkCmdSetPolygonModeEXT,
    cmd_set_rasterization_samples_ext: PFN_vkCmdSetRasterizationSamplesEXT,
    cmd_set_sample_mask_ext: PFN_vkCmdSetSampleMaskEXT,
    cmd_set_alpha_to_coverage_enable_ext: PFN_vkCmdSetAlphaToCoverageEnableEXT,
    cmd_set_alpha_to_one_enable_ext: PFN_vkCmdSetAlphaToOneEnableEXT,
    cmd_set_logic_op_enable_ext: PFN_vkCmdSetLogicOpEnableEXT,
    cmd_set_color_blend_enable_ext: PFN_vkCmdSetColorBlendEnableEXT,
    cmd_set_color_blend_equation_ext: PFN_vkCmdSetColorBlendEquationEXT,
    cmd_set_color_write_mask_ext: PFN_vkCmdSetColorWriteMaskEXT,
    cmd_set_rasterization_stream_ext: PFN_vkCmdSetRasterizationStreamEXT,
    cmd_set_conservative_rasterization_mode_ext: PFN_vkCmdSetConservativeRasterizationModeEXT,
    cmd_set_extra_primitive_overestimation_size_ext:
        PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT,
    cmd_set_depth_clip_enable_ext: PFN_vkCmdSetDepthClipEnableEXT,
    cmd_set_sample_locations_enable_ext: PFN_vkCmdSetSampleLocationsEnableEXT,
    cmd_set_color_blend_advanced_ext: PFN_vkCmdSetColorBlendAdvancedEXT,
    cmd_set_provoking_vertex_mode_ext: PFN_vkCmdSetProvokingVertexModeEXT,
    cmd_set_line_rasterization_mode_ext: PFN_vkCmdSetLineRasterizationModeEXT,
    cmd_set_line_stipple_enable_ext: PFN_vkCmdSetLineStippleEnableEXT,
    cmd_set_depth_clip_negative_one_to_one_ext: PFN_vkCmdSetDepthClipNegativeOneToOneEXT,
    cmd_set_viewport_w_scaling_enable_nv: PFN_vkCmdSetViewportWScalingEnableNV,
    cmd_set_viewport_swizzle_nv: PFN_vkCmdSetViewportSwizzleNV,
    cmd_set_coverage_to_color_enable_nv: PFN_vkCmdSetCoverageToColorEnableNV,
    cmd_set_coverage_to_color_location_nv: PFN_vkCmdSetCoverageToColorLocationNV,
    cmd_set_coverage_modulation_mode_nv: PFN_vkCmdSetCoverageModulationModeNV,
    cmd_set_coverage_modulation_table_enable_nv: PFN_vkCmdSetCoverageModulationTableEnableNV,
    cmd_set_coverage_modulation_table_nv: PFN_vkCmdSetCoverageModulationTableNV,
    cmd_set_shading_rate_image_enable_nv: PFN_vkCmdSetShadingRateImageEnableNV,
    cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
    cmd_set_representative_fragment_test_enable_nv: PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
}
impl DeviceFn {
    pub unsafe fn cmd_set_tessellation_domain_origin_ext(
        &self,
        command_buffer: CommandBuffer,
        domain_origin: TessellationDomainOrigin,
    ) {
        unsafe { (self.cmd_set_tessellation_domain_origin_ext)(command_buffer, domain_origin) }
    }
    pub unsafe fn cmd_set_depth_clamp_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_clamp_enable_ext)(command_buffer, depth_clamp_enable) }
    }
    pub unsafe fn cmd_set_polygon_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        polygon_mode: PolygonMode,
    ) {
        unsafe { (self.cmd_set_polygon_mode_ext)(command_buffer, polygon_mode) }
    }
    pub unsafe fn cmd_set_rasterization_samples_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterization_samples: SampleCountFlags,
    ) {
        unsafe { (self.cmd_set_rasterization_samples_ext)(command_buffer, rasterization_samples) }
    }
    pub unsafe fn cmd_set_sample_mask_ext(
        &self,
        command_buffer: CommandBuffer,
        samples: SampleCountFlags,
        sample_mask: &[SampleMask],
    ) {
        unsafe {
            (self.cmd_set_sample_mask_ext)(command_buffer, samples, sample_mask.as_ptr() as _)
        }
    }
    pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_coverage_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_alpha_to_coverage_enable_ext)(command_buffer, alpha_to_coverage_enable)
        }
    }
    pub unsafe fn cmd_set_alpha_to_one_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_one_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_alpha_to_one_enable_ext)(command_buffer, alpha_to_one_enable) }
    }
    pub unsafe fn cmd_set_logic_op_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        logic_op_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_logic_op_enable_ext)(command_buffer, logic_op_enable) }
    }
    pub unsafe fn cmd_set_color_blend_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_enables: &[Bool32],
    ) {
        unsafe {
            (self.cmd_set_color_blend_enable_ext)(
                command_buffer,
                first_attachment,
                color_blend_enables.len().try_into().unwrap(),
                color_blend_enables.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_color_blend_equation_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_equations: &[ColorBlendEquationEXT],
    ) {
        unsafe {
            (self.cmd_set_color_blend_equation_ext)(
                command_buffer,
                first_attachment,
                color_blend_equations.len().try_into().unwrap(),
                color_blend_equations.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_color_write_mask_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_write_masks: &[ColorComponentFlags],
    ) {
        unsafe {
            (self.cmd_set_color_write_mask_ext)(
                command_buffer,
                first_attachment,
                color_write_masks.len().try_into().unwrap(),
                color_write_masks.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_rasterization_stream_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterization_stream: u32,
    ) {
        unsafe { (self.cmd_set_rasterization_stream_ext)(command_buffer, rasterization_stream) }
    }
    pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        unsafe {
            (self.cmd_set_conservative_rasterization_mode_ext)(
                command_buffer,
                conservative_rasterization_mode,
            )
        }
    }
    pub unsafe fn cmd_set_extra_primitive_overestimation_size_ext(
        &self,
        command_buffer: CommandBuffer,
        extra_primitive_overestimation_size: f32,
    ) {
        unsafe {
            (self.cmd_set_extra_primitive_overestimation_size_ext)(
                command_buffer,
                extra_primitive_overestimation_size,
            )
        }
    }
    pub unsafe fn cmd_set_depth_clip_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clip_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_clip_enable_ext)(command_buffer, depth_clip_enable) }
    }
    pub unsafe fn cmd_set_sample_locations_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_sample_locations_enable_ext)(command_buffer, sample_locations_enable)
        }
    }
    pub unsafe fn cmd_set_color_blend_advanced_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_advanced: &[ColorBlendAdvancedEXT],
    ) {
        unsafe {
            (self.cmd_set_color_blend_advanced_ext)(
                command_buffer,
                first_attachment,
                color_blend_advanced.len().try_into().unwrap(),
                color_blend_advanced.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_provoking_vertex_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) {
        unsafe { (self.cmd_set_provoking_vertex_mode_ext)(command_buffer, provoking_vertex_mode) }
    }
    pub unsafe fn cmd_set_line_rasterization_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        line_rasterization_mode: LineRasterizationModeEXT,
    ) {
        unsafe {
            (self.cmd_set_line_rasterization_mode_ext)(command_buffer, line_rasterization_mode)
        }
    }
    pub unsafe fn cmd_set_line_stipple_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stippled_line_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_line_stipple_enable_ext)(command_buffer, stippled_line_enable) }
    }
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
        &self,
        command_buffer: CommandBuffer,
        negative_one_to_one: Bool32,
    ) {
        unsafe {
            (self.cmd_set_depth_clip_negative_one_to_one_ext)(command_buffer, negative_one_to_one)
        }
    }
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        viewport_w_scaling_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_viewport_w_scaling_enable_nv)(command_buffer, viewport_w_scaling_enable)
        }
    }
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_swizzles: &[ViewportSwizzleNV],
    ) {
        unsafe {
            (self.cmd_set_viewport_swizzle_nv)(
                command_buffer,
                first_viewport,
                viewport_swizzles.len().try_into().unwrap(),
                viewport_swizzles.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_coverage_to_color_enable_nv)(command_buffer, coverage_to_color_enable)
        }
    }
    pub unsafe fn cmd_set_coverage_to_color_location_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        unsafe {
            (self.cmd_set_coverage_to_color_location_nv)(command_buffer, coverage_to_color_location)
        }
    }
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_mode_nv)(command_buffer, coverage_modulation_mode)
        }
    }
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_table_enable_nv)(
                command_buffer,
                coverage_modulation_table_enable,
            )
        }
    }
    pub unsafe fn cmd_set_coverage_modulation_table_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table: &[f32],
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_table_nv)(
                command_buffer,
                coverage_modulation_table.len().try_into().unwrap(),
                coverage_modulation_table.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        shading_rate_image_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_shading_rate_image_enable_nv)(command_buffer, shading_rate_image_enable)
        }
    }
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) {
        unsafe {
            (self.cmd_set_coverage_reduction_mode_nv)(command_buffer, coverage_reduction_mode)
        }
    }
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        representative_fragment_test_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_representative_fragment_test_enable_nv)(
                command_buffer,
                representative_fragment_test_enable,
            )
        }
    }
}
