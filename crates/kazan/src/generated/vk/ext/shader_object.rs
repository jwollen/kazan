#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    create_shaders_ext: PFN_vkCreateShadersEXT,
    destroy_shader_ext: PFN_vkDestroyShaderEXT,
    get_shader_binary_data_ext: PFN_vkGetShaderBinaryDataEXT,
    cmd_bind_shaders_ext: PFN_vkCmdBindShadersEXT,
    cmd_set_cull_mode: PFN_vkCmdSetCullMode,
    cmd_set_front_face: PFN_vkCmdSetFrontFace,
    cmd_set_primitive_topology: PFN_vkCmdSetPrimitiveTopology,
    cmd_set_viewport_with_count: PFN_vkCmdSetViewportWithCount,
    cmd_set_scissor_with_count: PFN_vkCmdSetScissorWithCount,
    cmd_bind_vertex_buffers2: PFN_vkCmdBindVertexBuffers2,
    cmd_set_depth_test_enable: PFN_vkCmdSetDepthTestEnable,
    cmd_set_depth_write_enable: PFN_vkCmdSetDepthWriteEnable,
    cmd_set_depth_compare_op: PFN_vkCmdSetDepthCompareOp,
    cmd_set_depth_bounds_test_enable: PFN_vkCmdSetDepthBoundsTestEnable,
    cmd_set_stencil_test_enable: PFN_vkCmdSetStencilTestEnable,
    cmd_set_stencil_op: PFN_vkCmdSetStencilOp,
    cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
    cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
    cmd_set_rasterizer_discard_enable: PFN_vkCmdSetRasterizerDiscardEnable,
    cmd_set_depth_bias_enable: PFN_vkCmdSetDepthBiasEnable,
    cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
    cmd_set_primitive_restart_enable: PFN_vkCmdSetPrimitiveRestartEnable,
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
    cmd_set_representative_fragment_test_enable_nv: PFN_vkCmdSetRepresentativeFragmentTestEnableNV,
    cmd_set_coverage_reduction_mode_nv: PFN_vkCmdSetCoverageReductionModeNV,
    cmd_set_depth_clamp_range_ext: PFN_vkCmdSetDepthClampRangeEXT,
}
impl DeviceFn {
    pub unsafe fn create_shaders_ext(
        &self,
        device: Device,
        create_infos: &[ShaderCreateInfoEXT],
        allocator: Option<&AllocationCallbacks>,
        shaders: &mut [ShaderEXT],
    ) -> Result {
        unsafe {
            (self.create_shaders_ext)(
                device,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                shaders.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn destroy_shader_ext(
        &self,
        device: Device,
        shader: ShaderEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe { (self.destroy_shader_ext)(device, shader, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_shader_binary_data_ext(
        &self,
        device: Device,
        shader: ShaderEXT,
        data: impl ExtendUninit<u8>,
    ) -> Result {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                (self.get_shader_binary_data_ext)(device, shader, data_size, data as _)
            })
        }
    }
    pub unsafe fn cmd_bind_shaders_ext(
        &self,
        command_buffer: CommandBuffer,
        stages: &[ShaderStageFlags],
        shaders: Option<&[ShaderEXT]>,
    ) {
        unsafe {
            (self.cmd_bind_shaders_ext)(
                command_buffer,
                stages.len().try_into().unwrap(),
                stages.as_ptr() as _,
                shaders.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        unsafe { (self.cmd_set_cull_mode)(command_buffer, cull_mode) }
    }
    pub unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: CommandBuffer,
        front_face: FrontFace,
    ) {
        unsafe { (self.cmd_set_front_face)(command_buffer, front_face) }
    }
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        unsafe { (self.cmd_set_primitive_topology)(command_buffer, primitive_topology) }
    }
    pub unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.cmd_set_viewport_with_count)(
                command_buffer,
                viewports.len().try_into().unwrap(),
                viewports.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_scissor_with_count_ext(
        &self,
        command_buffer: CommandBuffer,
        scissors: &[Rect2D],
    ) {
        unsafe {
            (self.cmd_set_scissor_with_count)(
                command_buffer,
                scissors.len().try_into().unwrap(),
                scissors.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_bind_vertex_buffers2_ext(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: Option<&[DeviceSize]>,
        strides: Option<&[DeviceSize]>,
    ) {
        unsafe {
            (self.cmd_bind_vertex_buffers2)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.to_raw_ptr(),
                strides.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_test_enable)(command_buffer, depth_test_enable) }
    }
    pub unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_write_enable)(command_buffer, depth_write_enable) }
    }
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        unsafe { (self.cmd_set_depth_compare_op)(command_buffer, depth_compare_op) }
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_bounds_test_enable)(command_buffer, depth_bounds_test_enable) }
    }
    pub unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_stencil_test_enable)(command_buffer, stencil_test_enable) }
    }
    pub unsafe fn cmd_set_stencil_op_ext(
        &self,
        command_buffer: CommandBuffer,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            (self.cmd_set_stencil_op)(
                command_buffer,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        }
    }
    pub unsafe fn cmd_set_vertex_input_ext(
        &self,
        command_buffer: CommandBuffer,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT],
    ) {
        unsafe {
            (self.cmd_set_vertex_input_ext)(
                command_buffer,
                vertex_binding_descriptions.len().try_into().unwrap(),
                vertex_binding_descriptions.as_ptr() as _,
                vertex_attribute_descriptions.len().try_into().unwrap(),
                vertex_attribute_descriptions.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: CommandBuffer,
        patch_control_points: u32,
    ) {
        unsafe { (self.cmd_set_patch_control_points_ext)(command_buffer, patch_control_points) }
    }
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_rasterizer_discard_enable)(command_buffer, rasterizer_discard_enable)
        }
    }
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_bias_enable)(command_buffer, depth_bias_enable) }
    }
    pub unsafe fn cmd_set_logic_op_ext(&self, command_buffer: CommandBuffer, logic_op: LogicOp) {
        unsafe { (self.cmd_set_logic_op_ext)(command_buffer, logic_op) }
    }
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_primitive_restart_enable)(command_buffer, primitive_restart_enable) }
    }
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
        sample_mask: Option<&[SampleMask]>,
    ) {
        unsafe { (self.cmd_set_sample_mask_ext)(command_buffer, samples, sample_mask.to_raw_ptr()) }
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
    pub unsafe fn cmd_set_coverage_reduction_mode_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_reduction_mode: CoverageReductionModeNV,
    ) {
        unsafe {
            (self.cmd_set_coverage_reduction_mode_nv)(command_buffer, coverage_reduction_mode)
        }
    }
    pub unsafe fn cmd_set_depth_clamp_range_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) {
        unsafe {
            (self.cmd_set_depth_clamp_range_ext)(
                command_buffer,
                depth_clamp_mode,
                depth_clamp_range.to_raw_ptr(),
            )
        }
    }
}
