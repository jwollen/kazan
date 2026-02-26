#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    create_shaders_ext: PFN_vkCreateShadersEXT,
    destroy_shader_ext: PFN_vkDestroyShaderEXT,
    get_shader_binary_data_ext: PFN_vkGetShaderBinaryDataEXT,
    cmd_bind_shaders_ext: PFN_vkCmdBindShadersEXT,
    cmd_set_cull_mode_ext: PFN_vkCmdSetCullMode,
    cmd_set_front_face_ext: PFN_vkCmdSetFrontFace,
    cmd_set_primitive_topology_ext: PFN_vkCmdSetPrimitiveTopology,
    cmd_set_viewport_with_count_ext: PFN_vkCmdSetViewportWithCount,
    cmd_set_scissor_with_count_ext: PFN_vkCmdSetScissorWithCount,
    cmd_bind_vertex_buffers2_ext: PFN_vkCmdBindVertexBuffers2,
    cmd_set_depth_test_enable_ext: PFN_vkCmdSetDepthTestEnable,
    cmd_set_depth_write_enable_ext: PFN_vkCmdSetDepthWriteEnable,
    cmd_set_depth_compare_op_ext: PFN_vkCmdSetDepthCompareOp,
    cmd_set_depth_bounds_test_enable_ext: PFN_vkCmdSetDepthBoundsTestEnable,
    cmd_set_stencil_test_enable_ext: PFN_vkCmdSetStencilTestEnable,
    cmd_set_stencil_op_ext: PFN_vkCmdSetStencilOp,
    cmd_set_vertex_input_ext: PFN_vkCmdSetVertexInputEXT,
    cmd_set_patch_control_points_ext: PFN_vkCmdSetPatchControlPointsEXT,
    cmd_set_rasterizer_discard_enable_ext: PFN_vkCmdSetRasterizerDiscardEnable,
    cmd_set_depth_bias_enable_ext: PFN_vkCmdSetDepthBiasEnable,
    cmd_set_logic_op_ext: PFN_vkCmdSetLogicOpEXT,
    cmd_set_primitive_restart_enable_ext: PFN_vkCmdSetPrimitiveRestartEnable,
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
    cmd_set_rasterization_stream_ext: Option<PFN_vkCmdSetRasterizationStreamEXT>,
    cmd_set_conservative_rasterization_mode_ext:
        Option<PFN_vkCmdSetConservativeRasterizationModeEXT>,
    cmd_set_extra_primitive_overestimation_size_ext:
        Option<PFN_vkCmdSetExtraPrimitiveOverestimationSizeEXT>,
    cmd_set_depth_clip_enable_ext: Option<PFN_vkCmdSetDepthClipEnableEXT>,
    cmd_set_sample_locations_enable_ext: Option<PFN_vkCmdSetSampleLocationsEnableEXT>,
    cmd_set_color_blend_advanced_ext: Option<PFN_vkCmdSetColorBlendAdvancedEXT>,
    cmd_set_provoking_vertex_mode_ext: Option<PFN_vkCmdSetProvokingVertexModeEXT>,
    cmd_set_line_rasterization_mode_ext: Option<PFN_vkCmdSetLineRasterizationModeEXT>,
    cmd_set_line_stipple_enable_ext: Option<PFN_vkCmdSetLineStippleEnableEXT>,
    cmd_set_depth_clip_negative_one_to_one_ext: Option<PFN_vkCmdSetDepthClipNegativeOneToOneEXT>,
    cmd_set_viewport_w_scaling_enable_nv: Option<PFN_vkCmdSetViewportWScalingEnableNV>,
    cmd_set_viewport_swizzle_nv: Option<PFN_vkCmdSetViewportSwizzleNV>,
    cmd_set_coverage_to_color_enable_nv: Option<PFN_vkCmdSetCoverageToColorEnableNV>,
    cmd_set_coverage_to_color_location_nv: Option<PFN_vkCmdSetCoverageToColorLocationNV>,
    cmd_set_coverage_modulation_mode_nv: Option<PFN_vkCmdSetCoverageModulationModeNV>,
    cmd_set_coverage_modulation_table_enable_nv:
        Option<PFN_vkCmdSetCoverageModulationTableEnableNV>,
    cmd_set_coverage_modulation_table_nv: Option<PFN_vkCmdSetCoverageModulationTableNV>,
    cmd_set_shading_rate_image_enable_nv: Option<PFN_vkCmdSetShadingRateImageEnableNV>,
    cmd_set_representative_fragment_test_enable_nv:
        Option<PFN_vkCmdSetRepresentativeFragmentTestEnableNV>,
    cmd_set_coverage_reduction_mode_nv: Option<PFN_vkCmdSetCoverageReductionModeNV>,
    cmd_set_depth_clamp_range_ext: Option<PFN_vkCmdSetDepthClampRangeEXT>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_shaders_ext: transmute(load(c"vkCreateShadersEXT").ok_or(LoadingError)?),
                destroy_shader_ext: transmute(load(c"vkDestroyShaderEXT").ok_or(LoadingError)?),
                get_shader_binary_data_ext: transmute(
                    load(c"vkGetShaderBinaryDataEXT").ok_or(LoadingError)?,
                ),
                cmd_bind_shaders_ext: transmute(load(c"vkCmdBindShadersEXT").ok_or(LoadingError)?),
                cmd_set_cull_mode_ext: transmute(load(c"vkCmdSetCullModeEXT").ok_or(LoadingError)?),
                cmd_set_front_face_ext: transmute(
                    load(c"vkCmdSetFrontFaceEXT").ok_or(LoadingError)?,
                ),
                cmd_set_primitive_topology_ext: transmute(
                    load(c"vkCmdSetPrimitiveTopologyEXT").ok_or(LoadingError)?,
                ),
                cmd_set_viewport_with_count_ext: transmute(
                    load(c"vkCmdSetViewportWithCountEXT").ok_or(LoadingError)?,
                ),
                cmd_set_scissor_with_count_ext: transmute(
                    load(c"vkCmdSetScissorWithCountEXT").ok_or(LoadingError)?,
                ),
                cmd_bind_vertex_buffers2_ext: transmute(
                    load(c"vkCmdBindVertexBuffers2EXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_test_enable_ext: transmute(
                    load(c"vkCmdSetDepthTestEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_write_enable_ext: transmute(
                    load(c"vkCmdSetDepthWriteEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_compare_op_ext: transmute(
                    load(c"vkCmdSetDepthCompareOpEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_bounds_test_enable_ext: transmute(
                    load(c"vkCmdSetDepthBoundsTestEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_stencil_test_enable_ext: transmute(
                    load(c"vkCmdSetStencilTestEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_stencil_op_ext: transmute(
                    load(c"vkCmdSetStencilOpEXT").ok_or(LoadingError)?,
                ),
                cmd_set_vertex_input_ext: transmute(
                    load(c"vkCmdSetVertexInputEXT").ok_or(LoadingError)?,
                ),
                cmd_set_patch_control_points_ext: transmute(
                    load(c"vkCmdSetPatchControlPointsEXT").ok_or(LoadingError)?,
                ),
                cmd_set_rasterizer_discard_enable_ext: transmute(
                    load(c"vkCmdSetRasterizerDiscardEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_bias_enable_ext: transmute(
                    load(c"vkCmdSetDepthBiasEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_logic_op_ext: transmute(load(c"vkCmdSetLogicOpEXT").ok_or(LoadingError)?),
                cmd_set_primitive_restart_enable_ext: transmute(
                    load(c"vkCmdSetPrimitiveRestartEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_tessellation_domain_origin_ext: transmute(
                    load(c"vkCmdSetTessellationDomainOriginEXT").ok_or(LoadingError)?,
                ),
                cmd_set_depth_clamp_enable_ext: transmute(
                    load(c"vkCmdSetDepthClampEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_polygon_mode_ext: transmute(
                    load(c"vkCmdSetPolygonModeEXT").ok_or(LoadingError)?,
                ),
                cmd_set_rasterization_samples_ext: transmute(
                    load(c"vkCmdSetRasterizationSamplesEXT").ok_or(LoadingError)?,
                ),
                cmd_set_sample_mask_ext: transmute(
                    load(c"vkCmdSetSampleMaskEXT").ok_or(LoadingError)?,
                ),
                cmd_set_alpha_to_coverage_enable_ext: transmute(
                    load(c"vkCmdSetAlphaToCoverageEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_alpha_to_one_enable_ext: transmute(
                    load(c"vkCmdSetAlphaToOneEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_logic_op_enable_ext: transmute(
                    load(c"vkCmdSetLogicOpEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_color_blend_enable_ext: transmute(
                    load(c"vkCmdSetColorBlendEnableEXT").ok_or(LoadingError)?,
                ),
                cmd_set_color_blend_equation_ext: transmute(
                    load(c"vkCmdSetColorBlendEquationEXT").ok_or(LoadingError)?,
                ),
                cmd_set_color_write_mask_ext: transmute(
                    load(c"vkCmdSetColorWriteMaskEXT").ok_or(LoadingError)?,
                ),
                cmd_set_rasterization_stream_ext: transmute(load(
                    c"vkCmdSetRasterizationStreamEXT",
                )),
                cmd_set_conservative_rasterization_mode_ext: transmute(load(
                    c"vkCmdSetConservativeRasterizationModeEXT",
                )),
                cmd_set_extra_primitive_overestimation_size_ext: transmute(load(
                    c"vkCmdSetExtraPrimitiveOverestimationSizeEXT",
                )),
                cmd_set_depth_clip_enable_ext: transmute(load(c"vkCmdSetDepthClipEnableEXT")),
                cmd_set_sample_locations_enable_ext: transmute(load(
                    c"vkCmdSetSampleLocationsEnableEXT",
                )),
                cmd_set_color_blend_advanced_ext: transmute(load(c"vkCmdSetColorBlendAdvancedEXT")),
                cmd_set_provoking_vertex_mode_ext: transmute(load(
                    c"vkCmdSetProvokingVertexModeEXT",
                )),
                cmd_set_line_rasterization_mode_ext: transmute(load(
                    c"vkCmdSetLineRasterizationModeEXT",
                )),
                cmd_set_line_stipple_enable_ext: transmute(load(c"vkCmdSetLineStippleEnableEXT")),
                cmd_set_depth_clip_negative_one_to_one_ext: transmute(load(
                    c"vkCmdSetDepthClipNegativeOneToOneEXT",
                )),
                cmd_set_viewport_w_scaling_enable_nv: transmute(load(
                    c"vkCmdSetViewportWScalingEnableNV",
                )),
                cmd_set_viewport_swizzle_nv: transmute(load(c"vkCmdSetViewportSwizzleNV")),
                cmd_set_coverage_to_color_enable_nv: transmute(load(
                    c"vkCmdSetCoverageToColorEnableNV",
                )),
                cmd_set_coverage_to_color_location_nv: transmute(load(
                    c"vkCmdSetCoverageToColorLocationNV",
                )),
                cmd_set_coverage_modulation_mode_nv: transmute(load(
                    c"vkCmdSetCoverageModulationModeNV",
                )),
                cmd_set_coverage_modulation_table_enable_nv: transmute(load(
                    c"vkCmdSetCoverageModulationTableEnableNV",
                )),
                cmd_set_coverage_modulation_table_nv: transmute(load(
                    c"vkCmdSetCoverageModulationTableNV",
                )),
                cmd_set_shading_rate_image_enable_nv: transmute(load(
                    c"vkCmdSetShadingRateImageEnableNV",
                )),
                cmd_set_representative_fragment_test_enable_nv: transmute(load(
                    c"vkCmdSetRepresentativeFragmentTestEnableNV",
                )),
                cmd_set_coverage_reduction_mode_nv: transmute(load(
                    c"vkCmdSetCoverageReductionModeNV",
                )),
                cmd_set_depth_clamp_range_ext: transmute(load(c"vkCmdSetDepthClampRangeEXT")),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_shaders_ext(
        &self,
        device: Device,
        create_infos: &[ShaderCreateInfoEXT<'_>],
        allocator: Option<&AllocationCallbacks<'_>>,
        shaders: &mut [ShaderEXT],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.create_shaders_ext)(
                device,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                shaders.as_mut_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                VkResult::INCOMPATIBLE_SHADER_BINARY_EXT => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_shader_ext(
        &self,
        device: Device,
        shader: ShaderEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_shader_ext)(device, shader, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_shader_binary_data_ext(
        &self,
        device: Device,
        shader: ShaderEXT,
        data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                let result =
                    (self.get_shader_binary_data_ext)(device, shader, data_size, data as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn cmd_bind_shaders_ext(
        &self,
        command_buffer: CommandBuffer,
        stages: &[ShaderStageFlagBits],
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
        unsafe { (self.cmd_set_cull_mode_ext)(command_buffer, cull_mode) }
    }
    pub unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: CommandBuffer,
        front_face: FrontFace,
    ) {
        unsafe { (self.cmd_set_front_face_ext)(command_buffer, front_face) }
    }
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        unsafe { (self.cmd_set_primitive_topology_ext)(command_buffer, primitive_topology) }
    }
    pub unsafe fn cmd_set_viewport_with_count_ext(
        &self,
        command_buffer: CommandBuffer,
        viewports: &[Viewport],
    ) {
        unsafe {
            (self.cmd_set_viewport_with_count_ext)(
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
            (self.cmd_set_scissor_with_count_ext)(
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
            (self.cmd_bind_vertex_buffers2_ext)(
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
        unsafe { (self.cmd_set_depth_test_enable_ext)(command_buffer, depth_test_enable) }
    }
    pub unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_write_enable_ext)(command_buffer, depth_write_enable) }
    }
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        unsafe { (self.cmd_set_depth_compare_op_ext)(command_buffer, depth_compare_op) }
    }
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_depth_bounds_test_enable_ext)(command_buffer, depth_bounds_test_enable)
        }
    }
    pub unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_stencil_test_enable_ext)(command_buffer, stencil_test_enable) }
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
            (self.cmd_set_stencil_op_ext)(
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
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT<'_>],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT<'_>],
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
            (self.cmd_set_rasterizer_discard_enable_ext)(command_buffer, rasterizer_discard_enable)
        }
    }
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_bias_enable_ext)(command_buffer, depth_bias_enable) }
    }
    pub unsafe fn cmd_set_logic_op_ext(&self, command_buffer: CommandBuffer, logic_op: LogicOp) {
        unsafe { (self.cmd_set_logic_op_ext)(command_buffer, logic_op) }
    }
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_primitive_restart_enable_ext)(command_buffer, primitive_restart_enable)
        }
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
        rasterization_samples: SampleCountFlagBits,
    ) {
        unsafe { (self.cmd_set_rasterization_samples_ext)(command_buffer, rasterization_samples) }
    }
    pub unsafe fn cmd_set_sample_mask_ext(
        &self,
        command_buffer: CommandBuffer,
        samples: SampleCountFlagBits,
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
        unsafe {
            (self.cmd_set_rasterization_stream_ext.unwrap())(command_buffer, rasterization_stream)
        }
    }
    pub unsafe fn cmd_set_conservative_rasterization_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        unsafe {
            (self.cmd_set_conservative_rasterization_mode_ext.unwrap())(
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
            (self
                .cmd_set_extra_primitive_overestimation_size_ext
                .unwrap())(command_buffer, extra_primitive_overestimation_size)
        }
    }
    pub unsafe fn cmd_set_depth_clip_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clip_enable: Bool32,
    ) {
        unsafe { (self.cmd_set_depth_clip_enable_ext.unwrap())(command_buffer, depth_clip_enable) }
    }
    pub unsafe fn cmd_set_sample_locations_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_sample_locations_enable_ext.unwrap())(
                command_buffer,
                sample_locations_enable,
            )
        }
    }
    pub unsafe fn cmd_set_color_blend_advanced_ext(
        &self,
        command_buffer: CommandBuffer,
        first_attachment: u32,
        color_blend_advanced: &[ColorBlendAdvancedEXT],
    ) {
        unsafe {
            (self.cmd_set_color_blend_advanced_ext.unwrap())(
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
        unsafe {
            (self.cmd_set_provoking_vertex_mode_ext.unwrap())(command_buffer, provoking_vertex_mode)
        }
    }
    pub unsafe fn cmd_set_line_rasterization_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        line_rasterization_mode: LineRasterizationModeEXT,
    ) {
        unsafe {
            (self.cmd_set_line_rasterization_mode_ext.unwrap())(
                command_buffer,
                line_rasterization_mode,
            )
        }
    }
    pub unsafe fn cmd_set_line_stipple_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stippled_line_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_line_stipple_enable_ext.unwrap())(command_buffer, stippled_line_enable)
        }
    }
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
        &self,
        command_buffer: CommandBuffer,
        negative_one_to_one: Bool32,
    ) {
        unsafe {
            (self.cmd_set_depth_clip_negative_one_to_one_ext.unwrap())(
                command_buffer,
                negative_one_to_one,
            )
        }
    }
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        viewport_w_scaling_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_viewport_w_scaling_enable_nv.unwrap())(
                command_buffer,
                viewport_w_scaling_enable,
            )
        }
    }
    pub unsafe fn cmd_set_viewport_swizzle_nv(
        &self,
        command_buffer: CommandBuffer,
        first_viewport: u32,
        viewport_swizzles: &[ViewportSwizzleNV],
    ) {
        unsafe {
            (self.cmd_set_viewport_swizzle_nv.unwrap())(
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
            (self.cmd_set_coverage_to_color_enable_nv.unwrap())(
                command_buffer,
                coverage_to_color_enable,
            )
        }
    }
    pub unsafe fn cmd_set_coverage_to_color_location_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_location: u32,
    ) {
        unsafe {
            (self.cmd_set_coverage_to_color_location_nv.unwrap())(
                command_buffer,
                coverage_to_color_location,
            )
        }
    }
    pub unsafe fn cmd_set_coverage_modulation_mode_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_mode: CoverageModulationModeNV,
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_mode_nv.unwrap())(
                command_buffer,
                coverage_modulation_mode,
            )
        }
    }
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_table_enable_nv.unwrap())(
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
            (self.cmd_set_coverage_modulation_table_nv.unwrap())(
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
            (self.cmd_set_shading_rate_image_enable_nv.unwrap())(
                command_buffer,
                shading_rate_image_enable,
            )
        }
    }
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        representative_fragment_test_enable: Bool32,
    ) {
        unsafe {
            (self.cmd_set_representative_fragment_test_enable_nv.unwrap())(
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
            (self.cmd_set_coverage_reduction_mode_nv.unwrap())(
                command_buffer,
                coverage_reduction_mode,
            )
        }
    }
    pub unsafe fn cmd_set_depth_clamp_range_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) {
        unsafe {
            (self.cmd_set_depth_clamp_range_ext.unwrap())(
                command_buffer,
                depth_clamp_mode,
                depth_clamp_range.to_raw_ptr(),
            )
        }
    }
}
