#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_shader_object";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    handle_nondispatchable!(
        ShaderEXT,
        SHADER_EXT,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderEXT.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderRequiredSubgroupSizeCreateInfoEXT.html>
    pub type ShaderRequiredSubgroupSizeCreateInfoEXT<'a> =
        PipelineShaderStageRequiredSubgroupSizeCreateInfo<'a>;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderObjectFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderObjectFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_object: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderObjectFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderObjectFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_object", &self.shader_object)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderObjectFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceShaderObjectFeaturesEXT<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderObjectFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceShaderObjectFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_object: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderObjectFeaturesEXT<'a> {
        pub fn shader_object(mut self, shader_object: bool) -> Self {
            self.shader_object = shader_object.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderObjectPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderObjectPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_binary_uuid: [u8; UUID_SIZE as usize],
        pub shader_binary_version: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderObjectPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderObjectPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_binary_uuid", &self.shader_binary_uuid)
                .field("shader_binary_version", &self.shader_binary_version)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderObjectPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderObjectPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceShaderObjectPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_binary_uuid: [Default::default(); _],
                shader_binary_version: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderObjectPropertiesEXT<'a> {
        pub fn shader_binary_uuid(mut self, shader_binary_uuid: [u8; UUID_SIZE as usize]) -> Self {
            self.shader_binary_uuid = shader_binary_uuid;
            self
        }

        pub fn shader_binary_version(mut self, shader_binary_version: u32) -> Self {
            self.shader_binary_version = shader_binary_version;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ShaderCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: ShaderCreateFlagsEXT,
        pub stage: ShaderStageFlagBits,
        pub next_stage: ShaderStageFlags,
        pub code_type: ShaderCodeTypeEXT,
        pub code_size: usize,
        pub p_code: *const c_void,
        pub p_name: *const c_char,
        pub set_layout_count: u32,
        pub p_set_layouts: *const DescriptorSetLayout,
        pub push_constant_range_count: u32,
        pub p_push_constant_ranges: *const PushConstantRange,
        pub p_specialization_info: *const SpecializationInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ShaderCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ShaderCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("stage", &self.stage)
                .field("next_stage", &self.next_stage)
                .field("code_type", &self.code_type)
                .field("code_size", &self.code_size)
                .field("p_code", &self.p_code)
                .field("p_name", &unsafe { as_c_str(self.p_name) })
                .field("set_layout_count", &self.set_layout_count)
                .field("p_set_layouts", &self.p_set_layouts)
                .field("push_constant_range_count", &self.push_constant_range_count)
                .field("p_push_constant_ranges", &self.p_push_constant_ranges)
                .field("p_specialization_info", &self.p_specialization_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ShaderCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SHADER_CREATE_INFO_EXT;
    }

    impl Default for ShaderCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                stage: Default::default(),
                next_stage: Default::default(),
                code_type: Default::default(),
                code_size: Default::default(),
                p_code: core::ptr::null(),
                p_name: core::ptr::null(),
                set_layout_count: Default::default(),
                p_set_layouts: core::ptr::null(),
                push_constant_range_count: Default::default(),
                p_push_constant_ranges: core::ptr::null(),
                p_specialization_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ShaderCreateInfoEXT<'a> {
        pub fn flags(mut self, flags: ShaderCreateFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        pub fn stage(mut self, stage: ShaderStageFlagBits) -> Self {
            self.stage = stage;
            self
        }

        pub fn next_stage(mut self, next_stage: ShaderStageFlags) -> Self {
            self.next_stage = next_stage;
            self
        }

        pub fn code_type(mut self, code_type: ShaderCodeTypeEXT) -> Self {
            self.code_type = code_type;
            self
        }

        pub fn code(mut self, code: &'a [u8]) -> Self {
            self.code_size = code.len().try_into().unwrap();
            self.p_code = code.as_ptr() as _;
            self
        }

        pub fn name(mut self, name: &'a CStr) -> Self {
            self.p_name = name.as_ptr();
            self
        }

        pub fn set_layouts(mut self, set_layouts: &'a [DescriptorSetLayout]) -> Self {
            self.set_layout_count = set_layouts.len().try_into().unwrap();
            self.p_set_layouts = set_layouts.as_ptr();
            self
        }

        pub fn push_constant_ranges(
            mut self,
            push_constant_ranges: &'a [PushConstantRange],
        ) -> Self {
            self.push_constant_range_count = push_constant_ranges.len().try_into().unwrap();
            self.p_push_constant_ranges = push_constant_ranges.as_ptr();
            self
        }

        pub fn specialization_info(
            mut self,
            specialization_info: &'a SpecializationInfo<'a>,
        ) -> Self {
            self.p_specialization_info = specialization_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderCodeTypeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderCodeTypeEXT(i32);

    impl ShaderCodeTypeEXT {
        pub const BINARY_EXT: Self = Self(0);
        pub const SPIRV_EXT: Self = Self(1);
    }

    impl fmt::Debug for ShaderCodeTypeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BINARY_EXT => Some("BINARY_EXT"),
                Self::SPIRV_EXT => Some("SPIRV_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderCreateFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ShaderCreateFlagsEXT(Flags);
    vk_bitflags_wrapped!(ShaderCreateFlagsEXT, Flags);

    impl ShaderCreateFlagsEXT {
        pub const LINK_STAGE_EXT: Self = Self(ShaderCreateFlagBitsEXT::LINK_STAGE_EXT.0);
        // VK_EXT_descriptor_heap
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(ShaderCreateFlagBitsEXT::DESCRIPTOR_HEAP_EXT.0);

        // VK_EXT_device_generated_commands
        pub const INDIRECT_BINDABLE_EXT: Self =
            Self(ShaderCreateFlagBitsEXT::INDIRECT_BINDABLE_EXT.0);

        // VK_EXT_shader_64bit_indexing
        pub const _64_INDEXING_EXT: Self = Self(ShaderCreateFlagBitsEXT::_64_INDEXING_EXT.0);

        // VK_EXT_shader_object
        pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self =
            Self(ShaderCreateFlagBitsEXT::ALLOW_VARYING_SUBGROUP_SIZE_EXT.0);
        pub const REQUIRE_FULL_SUBGROUPS_EXT: Self =
            Self(ShaderCreateFlagBitsEXT::REQUIRE_FULL_SUBGROUPS_EXT.0);
        pub const NO_TASK_SHADER_EXT: Self = Self(ShaderCreateFlagBitsEXT::NO_TASK_SHADER_EXT.0);
        pub const DISPATCH_BASE_EXT: Self = Self(ShaderCreateFlagBitsEXT::DISPATCH_BASE_EXT.0);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT: Self =
            Self(ShaderCreateFlagBitsEXT::FRAGMENT_SHADING_RATE_ATTACHMENT_EXT.0);
        pub const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
            Self(ShaderCreateFlagBitsEXT::FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0);
    }

    impl fmt::Debug for ShaderCreateFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (ShaderCreateFlagsEXT::LINK_STAGE_EXT.0, "LINK_STAGE_EXT"),
                (
                    ShaderCreateFlagsEXT::DESCRIPTOR_HEAP_EXT.0,
                    "DESCRIPTOR_HEAP_EXT",
                ),
                (
                    ShaderCreateFlagsEXT::INDIRECT_BINDABLE_EXT.0,
                    "INDIRECT_BINDABLE_EXT",
                ),
                (ShaderCreateFlagsEXT::_64_INDEXING_EXT.0, "_64_INDEXING_EXT"),
                (
                    ShaderCreateFlagsEXT::ALLOW_VARYING_SUBGROUP_SIZE_EXT.0,
                    "ALLOW_VARYING_SUBGROUP_SIZE_EXT",
                ),
                (
                    ShaderCreateFlagsEXT::REQUIRE_FULL_SUBGROUPS_EXT.0,
                    "REQUIRE_FULL_SUBGROUPS_EXT",
                ),
                (
                    ShaderCreateFlagsEXT::NO_TASK_SHADER_EXT.0,
                    "NO_TASK_SHADER_EXT",
                ),
                (
                    ShaderCreateFlagsEXT::DISPATCH_BASE_EXT.0,
                    "DISPATCH_BASE_EXT",
                ),
                (
                    ShaderCreateFlagsEXT::FRAGMENT_SHADING_RATE_ATTACHMENT_EXT.0,
                    "FRAGMENT_SHADING_RATE_ATTACHMENT_EXT",
                ),
                (
                    ShaderCreateFlagsEXT::FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0,
                    "FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderCreateFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ShaderCreateFlagBitsEXT(u32);

    impl ShaderCreateFlagBitsEXT {
        pub const LINK_STAGE_EXT: Self = Self(1 << 0);
        // VK_EXT_descriptor_heap
        pub const DESCRIPTOR_HEAP_EXT: Self = Self(1 << 10);

        // VK_EXT_device_generated_commands
        pub const INDIRECT_BINDABLE_EXT: Self = Self(1 << 7);

        // VK_EXT_shader_64bit_indexing
        pub const _64_INDEXING_EXT: Self = Self(1 << 15);

        // VK_EXT_shader_object
        pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self(1 << 1);
        pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self(1 << 2);
        pub const NO_TASK_SHADER_EXT: Self = Self(1 << 3);
        pub const DISPATCH_BASE_EXT: Self = Self(1 << 4);
        pub const FRAGMENT_SHADING_RATE_ATTACHMENT_EXT: Self = Self(1 << 5);
        pub const FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(1 << 6);
    }

    impl fmt::Debug for ShaderCreateFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::LINK_STAGE_EXT => Some("LINK_STAGE_EXT"),
                Self::DESCRIPTOR_HEAP_EXT => Some("DESCRIPTOR_HEAP_EXT"),
                Self::INDIRECT_BINDABLE_EXT => Some("INDIRECT_BINDABLE_EXT"),
                Self::_64_INDEXING_EXT => Some("_64_INDEXING_EXT"),
                Self::ALLOW_VARYING_SUBGROUP_SIZE_EXT => Some("ALLOW_VARYING_SUBGROUP_SIZE_EXT"),
                Self::REQUIRE_FULL_SUBGROUPS_EXT => Some("REQUIRE_FULL_SUBGROUPS_EXT"),
                Self::NO_TASK_SHADER_EXT => Some("NO_TASK_SHADER_EXT"),
                Self::DISPATCH_BASE_EXT => Some("DISPATCH_BASE_EXT"),
                Self::FRAGMENT_SHADING_RATE_ATTACHMENT_EXT => {
                    Some("FRAGMENT_SHADING_RATE_ATTACHMENT_EXT")
                }
                Self::FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT => {
                    Some("FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateShadersEXT.html>
    pub type PFN_vkCreateShadersEXT = unsafe extern "system" fn(
        device: Device,
        create_info_count: u32,
        p_create_infos: *const ShaderCreateInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_shaders: *mut ShaderEXT,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyShaderEXT.html>
    pub type PFN_vkDestroyShaderEXT = unsafe extern "system" fn(
        device: Device,
        shader: ShaderEXT,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetShaderBinaryDataEXT.html>
    pub type PFN_vkGetShaderBinaryDataEXT = unsafe extern "system" fn(
        device: Device,
        shader: ShaderEXT,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindShadersEXT.html>
    pub type PFN_vkCmdBindShadersEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage_count: u32,
        p_stages: *const ShaderStageFlagBits,
        p_shaders: *const ShaderEXT,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClampRangeEXT.html>
    pub type PFN_vkCmdSetDepthClampRangeEXT = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        depth_clamp_mode: DepthClampModeEXT,
        p_depth_clamp_range: *const DepthClampRangeEXT,
    );
}

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
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_shaders_ext: transmute(
                    load(c"vkCreateShadersEXT").ok_or(MissingEntryPointError)?,
                ),
                destroy_shader_ext: transmute(
                    load(c"vkDestroyShaderEXT").ok_or(MissingEntryPointError)?,
                ),
                get_shader_binary_data_ext: transmute(
                    load(c"vkGetShaderBinaryDataEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_shaders_ext: transmute(
                    load(c"vkCmdBindShadersEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_cull_mode_ext: transmute(
                    load(c"vkCmdSetCullModeEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_front_face_ext: transmute(
                    load(c"vkCmdSetFrontFaceEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_primitive_topology_ext: transmute(
                    load(c"vkCmdSetPrimitiveTopologyEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_viewport_with_count_ext: transmute(
                    load(c"vkCmdSetViewportWithCountEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_scissor_with_count_ext: transmute(
                    load(c"vkCmdSetScissorWithCountEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_vertex_buffers2_ext: transmute(
                    load(c"vkCmdBindVertexBuffers2EXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_test_enable_ext: transmute(
                    load(c"vkCmdSetDepthTestEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_write_enable_ext: transmute(
                    load(c"vkCmdSetDepthWriteEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_compare_op_ext: transmute(
                    load(c"vkCmdSetDepthCompareOpEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bounds_test_enable_ext: transmute(
                    load(c"vkCmdSetDepthBoundsTestEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_test_enable_ext: transmute(
                    load(c"vkCmdSetStencilTestEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_stencil_op_ext: transmute(
                    load(c"vkCmdSetStencilOpEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_vertex_input_ext: transmute(
                    load(c"vkCmdSetVertexInputEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_patch_control_points_ext: transmute(
                    load(c"vkCmdSetPatchControlPointsEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rasterizer_discard_enable_ext: transmute(
                    load(c"vkCmdSetRasterizerDiscardEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_bias_enable_ext: transmute(
                    load(c"vkCmdSetDepthBiasEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_logic_op_ext: transmute(
                    load(c"vkCmdSetLogicOpEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_primitive_restart_enable_ext: transmute(
                    load(c"vkCmdSetPrimitiveRestartEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_tessellation_domain_origin_ext: transmute(
                    load(c"vkCmdSetTessellationDomainOriginEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_depth_clamp_enable_ext: transmute(
                    load(c"vkCmdSetDepthClampEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_polygon_mode_ext: transmute(
                    load(c"vkCmdSetPolygonModeEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_rasterization_samples_ext: transmute(
                    load(c"vkCmdSetRasterizationSamplesEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_sample_mask_ext: transmute(
                    load(c"vkCmdSetSampleMaskEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_alpha_to_coverage_enable_ext: transmute(
                    load(c"vkCmdSetAlphaToCoverageEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_alpha_to_one_enable_ext: transmute(
                    load(c"vkCmdSetAlphaToOneEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_logic_op_enable_ext: transmute(
                    load(c"vkCmdSetLogicOpEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_color_blend_enable_ext: transmute(
                    load(c"vkCmdSetColorBlendEnableEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_color_blend_equation_ext: transmute(
                    load(c"vkCmdSetColorBlendEquationEXT").ok_or(MissingEntryPointError)?,
                ),
                cmd_set_color_write_mask_ext: transmute(
                    load(c"vkCmdSetColorWriteMaskEXT").ok_or(MissingEntryPointError)?,
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateShadersEXT.html>
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
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyShaderEXT.html>
    pub unsafe fn destroy_shader_ext(
        &self,
        device: Device,
        shader: ShaderEXT,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_shader_ext)(device, shader, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetShaderBinaryDataEXT.html>
    pub unsafe fn get_shader_binary_data_ext(
        &self,
        device: Device,
        shader: ShaderEXT,
        mut data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |data_size, data| {
                let result =
                    (self.get_shader_binary_data_ext)(device, shader, data_size, data as _);

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let data_buf = data.reserve(capacity);
            len = data_buf.len().try_into().unwrap();
            let result = call(&mut len, data_buf.as_mut_ptr() as *mut _)?;
            data.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindShadersEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCullModeEXT.html>
    pub unsafe fn cmd_set_cull_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        cull_mode: CullModeFlags,
    ) {
        unsafe { (self.cmd_set_cull_mode_ext)(command_buffer, cull_mode) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetFrontFaceEXT.html>
    pub unsafe fn cmd_set_front_face_ext(
        &self,
        command_buffer: CommandBuffer,
        front_face: FrontFace,
    ) {
        unsafe { (self.cmd_set_front_face_ext)(command_buffer, front_face) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveTopologyEXT.html>
    pub unsafe fn cmd_set_primitive_topology_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_topology: PrimitiveTopology,
    ) {
        unsafe { (self.cmd_set_primitive_topology_ext)(command_buffer, primitive_topology) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportWithCountEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetScissorWithCountEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindVertexBuffers2EXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthTestEnableEXT.html>
    pub unsafe fn cmd_set_depth_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_test_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_test_enable_ext)(command_buffer, depth_test_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthWriteEnableEXT.html>
    pub unsafe fn cmd_set_depth_write_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_write_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_write_enable_ext)(command_buffer, depth_write_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthCompareOpEXT.html>
    pub unsafe fn cmd_set_depth_compare_op_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_compare_op: CompareOp,
    ) {
        unsafe { (self.cmd_set_depth_compare_op_ext)(command_buffer, depth_compare_op) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBoundsTestEnableEXT.html>
    pub unsafe fn cmd_set_depth_bounds_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bounds_test_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_depth_bounds_test_enable_ext)(
                command_buffer,
                depth_bounds_test_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetStencilTestEnableEXT.html>
    pub unsafe fn cmd_set_stencil_test_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stencil_test_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_stencil_test_enable_ext)(command_buffer, stencil_test_enable.into())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetStencilOpEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetVertexInputEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPatchControlPointsEXT.html>
    pub unsafe fn cmd_set_patch_control_points_ext(
        &self,
        command_buffer: CommandBuffer,
        patch_control_points: u32,
    ) {
        unsafe { (self.cmd_set_patch_control_points_ext)(command_buffer, patch_control_points) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizerDiscardEnableEXT.html>
    pub unsafe fn cmd_set_rasterizer_discard_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterizer_discard_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_rasterizer_discard_enable_ext)(
                command_buffer,
                rasterizer_discard_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthBiasEnableEXT.html>
    pub unsafe fn cmd_set_depth_bias_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_bias_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_bias_enable_ext)(command_buffer, depth_bias_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLogicOpEXT.html>
    pub unsafe fn cmd_set_logic_op_ext(&self, command_buffer: CommandBuffer, logic_op: LogicOp) {
        unsafe { (self.cmd_set_logic_op_ext)(command_buffer, logic_op) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPrimitiveRestartEnableEXT.html>
    pub unsafe fn cmd_set_primitive_restart_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        primitive_restart_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_primitive_restart_enable_ext)(
                command_buffer,
                primitive_restart_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetTessellationDomainOriginEXT.html>
    pub unsafe fn cmd_set_tessellation_domain_origin_ext(
        &self,
        command_buffer: CommandBuffer,
        domain_origin: TessellationDomainOrigin,
    ) {
        unsafe { (self.cmd_set_tessellation_domain_origin_ext)(command_buffer, domain_origin) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClampEnableEXT.html>
    pub unsafe fn cmd_set_depth_clamp_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clamp_enable: bool,
    ) {
        unsafe { (self.cmd_set_depth_clamp_enable_ext)(command_buffer, depth_clamp_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetPolygonModeEXT.html>
    pub unsafe fn cmd_set_polygon_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        polygon_mode: PolygonMode,
    ) {
        unsafe { (self.cmd_set_polygon_mode_ext)(command_buffer, polygon_mode) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizationSamplesEXT.html>
    pub unsafe fn cmd_set_rasterization_samples_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterization_samples: SampleCountFlagBits,
    ) {
        unsafe { (self.cmd_set_rasterization_samples_ext)(command_buffer, rasterization_samples) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetSampleMaskEXT.html>
    pub unsafe fn cmd_set_sample_mask_ext(
        &self,
        command_buffer: CommandBuffer,
        samples: SampleCountFlagBits,
        sample_mask: Option<&[SampleMask]>,
    ) {
        unsafe { (self.cmd_set_sample_mask_ext)(command_buffer, samples, sample_mask.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetAlphaToCoverageEnableEXT.html>
    pub unsafe fn cmd_set_alpha_to_coverage_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_coverage_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_alpha_to_coverage_enable_ext)(
                command_buffer,
                alpha_to_coverage_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetAlphaToOneEnableEXT.html>
    pub unsafe fn cmd_set_alpha_to_one_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        alpha_to_one_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_alpha_to_one_enable_ext)(command_buffer, alpha_to_one_enable.into())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLogicOpEnableEXT.html>
    pub unsafe fn cmd_set_logic_op_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        logic_op_enable: bool,
    ) {
        unsafe { (self.cmd_set_logic_op_enable_ext)(command_buffer, logic_op_enable.into()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendEnableEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendEquationEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorWriteMaskEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRasterizationStreamEXT.html>
    pub unsafe fn cmd_set_rasterization_stream_ext(
        &self,
        command_buffer: CommandBuffer,
        rasterization_stream: u32,
    ) {
        unsafe {
            (self.cmd_set_rasterization_stream_ext.unwrap())(command_buffer, rasterization_stream)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetConservativeRasterizationModeEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClipEnableEXT.html>
    pub unsafe fn cmd_set_depth_clip_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        depth_clip_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_depth_clip_enable_ext.unwrap())(command_buffer, depth_clip_enable.into())
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetSampleLocationsEnableEXT.html>
    pub unsafe fn cmd_set_sample_locations_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        sample_locations_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_sample_locations_enable_ext.unwrap())(
                command_buffer,
                sample_locations_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetColorBlendAdvancedEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetProvokingVertexModeEXT.html>
    pub unsafe fn cmd_set_provoking_vertex_mode_ext(
        &self,
        command_buffer: CommandBuffer,
        provoking_vertex_mode: ProvokingVertexModeEXT,
    ) {
        unsafe {
            (self.cmd_set_provoking_vertex_mode_ext.unwrap())(command_buffer, provoking_vertex_mode)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineRasterizationModeEXT.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetLineStippleEnableEXT.html>
    pub unsafe fn cmd_set_line_stipple_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        stippled_line_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_line_stipple_enable_ext.unwrap())(
                command_buffer,
                stippled_line_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClipNegativeOneToOneEXT.html>
    pub unsafe fn cmd_set_depth_clip_negative_one_to_one_ext(
        &self,
        command_buffer: CommandBuffer,
        negative_one_to_one: bool,
    ) {
        unsafe {
            (self.cmd_set_depth_clip_negative_one_to_one_ext.unwrap())(
                command_buffer,
                negative_one_to_one.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportWScalingEnableNV.html>
    pub unsafe fn cmd_set_viewport_w_scaling_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        viewport_w_scaling_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_viewport_w_scaling_enable_nv.unwrap())(
                command_buffer,
                viewport_w_scaling_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetViewportSwizzleNV.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageToColorEnableNV.html>
    pub unsafe fn cmd_set_coverage_to_color_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_to_color_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_coverage_to_color_enable_nv.unwrap())(
                command_buffer,
                coverage_to_color_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageToColorLocationNV.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationModeNV.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationTableEnableNV.html>
    pub unsafe fn cmd_set_coverage_modulation_table_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        coverage_modulation_table_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_coverage_modulation_table_enable_nv.unwrap())(
                command_buffer,
                coverage_modulation_table_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageModulationTableNV.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetShadingRateImageEnableNV.html>
    pub unsafe fn cmd_set_shading_rate_image_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        shading_rate_image_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_shading_rate_image_enable_nv.unwrap())(
                command_buffer,
                shading_rate_image_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetRepresentativeFragmentTestEnableNV.html>
    pub unsafe fn cmd_set_representative_fragment_test_enable_nv(
        &self,
        command_buffer: CommandBuffer,
        representative_fragment_test_enable: bool,
    ) {
        unsafe {
            (self.cmd_set_representative_fragment_test_enable_nv.unwrap())(
                command_buffer,
                representative_fragment_test_enable.into(),
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetCoverageReductionModeNV.html>
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetDepthClampRangeEXT.html>
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
