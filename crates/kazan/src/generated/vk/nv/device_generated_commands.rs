//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_device_generated_commands.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_device_generated_commands";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        IndirectCommandsLayoutNV,
        INDIRECT_COMMANDS_LAYOUT_NV,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutNV.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_generated_commands: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDeviceGeneratedCommandsFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_generated_commands", &self.device_generated_commands)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                device_generated_commands: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {
        #[inline]
        pub fn device_generated_commands(mut self, device_generated_commands: bool) -> Self {
            self.device_generated_commands = device_generated_commands.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_graphics_shader_group_count: u32,
        pub max_indirect_sequence_count: u32,
        pub max_indirect_commands_token_count: u32,
        pub max_indirect_commands_stream_count: u32,
        pub max_indirect_commands_token_offset: u32,
        pub max_indirect_commands_stream_stride: u32,
        pub min_sequences_count_buffer_offset_alignment: u32,
        pub min_sequences_index_buffer_offset_alignment: u32,
        pub min_indirect_commands_buffer_offset_alignment: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDeviceGeneratedCommandsPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_graphics_shader_group_count",
                    &self.max_graphics_shader_group_count,
                )
                .field(
                    "max_indirect_sequence_count",
                    &self.max_indirect_sequence_count,
                )
                .field(
                    "max_indirect_commands_token_count",
                    &self.max_indirect_commands_token_count,
                )
                .field(
                    "max_indirect_commands_stream_count",
                    &self.max_indirect_commands_stream_count,
                )
                .field(
                    "max_indirect_commands_token_offset",
                    &self.max_indirect_commands_token_offset,
                )
                .field(
                    "max_indirect_commands_stream_stride",
                    &self.max_indirect_commands_stream_stride,
                )
                .field(
                    "min_sequences_count_buffer_offset_alignment",
                    &self.min_sequences_count_buffer_offset_alignment,
                )
                .field(
                    "min_sequences_index_buffer_offset_alignment",
                    &self.min_sequences_index_buffer_offset_alignment,
                )
                .field(
                    "min_indirect_commands_buffer_offset_alignment",
                    &self.min_indirect_commands_buffer_offset_alignment,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_graphics_shader_group_count: Default::default(),
                max_indirect_sequence_count: Default::default(),
                max_indirect_commands_token_count: Default::default(),
                max_indirect_commands_stream_count: Default::default(),
                max_indirect_commands_token_offset: Default::default(),
                max_indirect_commands_stream_stride: Default::default(),
                min_sequences_count_buffer_offset_alignment: Default::default(),
                min_sequences_index_buffer_offset_alignment: Default::default(),
                min_indirect_commands_buffer_offset_alignment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'a> {
        #[inline]
        pub fn max_graphics_shader_group_count(
            mut self,
            max_graphics_shader_group_count: u32,
        ) -> Self {
            self.max_graphics_shader_group_count = max_graphics_shader_group_count;
            self
        }

        #[inline]
        pub fn max_indirect_sequence_count(mut self, max_indirect_sequence_count: u32) -> Self {
            self.max_indirect_sequence_count = max_indirect_sequence_count;
            self
        }

        #[inline]
        pub fn max_indirect_commands_token_count(
            mut self,
            max_indirect_commands_token_count: u32,
        ) -> Self {
            self.max_indirect_commands_token_count = max_indirect_commands_token_count;
            self
        }

        #[inline]
        pub fn max_indirect_commands_stream_count(
            mut self,
            max_indirect_commands_stream_count: u32,
        ) -> Self {
            self.max_indirect_commands_stream_count = max_indirect_commands_stream_count;
            self
        }

        #[inline]
        pub fn max_indirect_commands_token_offset(
            mut self,
            max_indirect_commands_token_offset: u32,
        ) -> Self {
            self.max_indirect_commands_token_offset = max_indirect_commands_token_offset;
            self
        }

        #[inline]
        pub fn max_indirect_commands_stream_stride(
            mut self,
            max_indirect_commands_stream_stride: u32,
        ) -> Self {
            self.max_indirect_commands_stream_stride = max_indirect_commands_stream_stride;
            self
        }

        #[inline]
        pub fn min_sequences_count_buffer_offset_alignment(
            mut self,
            min_sequences_count_buffer_offset_alignment: u32,
        ) -> Self {
            self.min_sequences_count_buffer_offset_alignment =
                min_sequences_count_buffer_offset_alignment;
            self
        }

        #[inline]
        pub fn min_sequences_index_buffer_offset_alignment(
            mut self,
            min_sequences_index_buffer_offset_alignment: u32,
        ) -> Self {
            self.min_sequences_index_buffer_offset_alignment =
                min_sequences_index_buffer_offset_alignment;
            self
        }

        #[inline]
        pub fn min_indirect_commands_buffer_offset_alignment(
            mut self,
            min_indirect_commands_buffer_offset_alignment: u32,
        ) -> Self {
            self.min_indirect_commands_buffer_offset_alignment =
                min_indirect_commands_buffer_offset_alignment;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGraphicsShaderGroupCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GraphicsShaderGroupCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stage_count: u32,
        pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
        pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo<'a>,
        pub p_tessellation_state: *const PipelineTessellationStateCreateInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GraphicsShaderGroupCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GraphicsShaderGroupCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stage_count", &self.stage_count)
                .field("p_stages", &self.p_stages)
                .field("p_vertex_input_state", &self.p_vertex_input_state)
                .field("p_tessellation_state", &self.p_tessellation_state)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GraphicsShaderGroupCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV;
    }

    impl Default for GraphicsShaderGroupCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                stage_count: Default::default(),
                p_stages: ptr::null(),
                p_vertex_input_state: ptr::null(),
                p_tessellation_state: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GraphicsShaderGroupCreateInfoNV<'a> {
        #[inline]
        pub fn stages(mut self, stages: &'a [PipelineShaderStageCreateInfo<'a>]) -> Self {
            self.stage_count = stages.len().try_into().unwrap();
            self.p_stages = stages.as_ptr();
            self
        }

        #[inline]
        pub fn vertex_input_state(
            mut self,
            vertex_input_state: &'a PipelineVertexInputStateCreateInfo<'a>,
        ) -> Self {
            self.p_vertex_input_state = vertex_input_state;
            self
        }

        #[inline]
        pub fn tessellation_state(
            mut self,
            tessellation_state: &'a PipelineTessellationStateCreateInfo<'a>,
        ) -> Self {
            self.p_tessellation_state = tessellation_state;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGraphicsPipelineShaderGroupsCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GraphicsPipelineShaderGroupsCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub group_count: u32,
        pub p_groups: *const GraphicsShaderGroupCreateInfoNV<'a>,
        pub pipeline_count: u32,
        pub p_pipelines: *const Pipeline,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GraphicsPipelineShaderGroupsCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GraphicsPipelineShaderGroupsCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("group_count", &self.group_count)
                .field("p_groups", &self.p_groups)
                .field("pipeline_count", &self.pipeline_count)
                .field("p_pipelines", &self.p_pipelines)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GraphicsPipelineShaderGroupsCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>>
        for GraphicsPipelineShaderGroupsCreateInfoNV<'a>
    {
    }

    impl Default for GraphicsPipelineShaderGroupsCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                group_count: Default::default(),
                p_groups: ptr::null(),
                pipeline_count: Default::default(),
                p_pipelines: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GraphicsPipelineShaderGroupsCreateInfoNV<'a> {
        #[inline]
        pub fn groups(mut self, groups: &'a [GraphicsShaderGroupCreateInfoNV<'a>]) -> Self {
            self.group_count = groups.len().try_into().unwrap();
            self.p_groups = groups.as_ptr();
            self
        }

        #[inline]
        pub fn pipelines(mut self, pipelines: &'a [Pipeline]) -> Self {
            self.pipeline_count = pipelines.len().try_into().unwrap();
            self.p_pipelines = pipelines.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindShaderGroupIndirectCommandNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct BindShaderGroupIndirectCommandNV {
        pub group_index: u32,
    }

    impl BindShaderGroupIndirectCommandNV {
        #[inline]
        pub fn group_index(mut self, group_index: u32) -> Self {
            self.group_index = group_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindIndexBufferIndirectCommandNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct BindIndexBufferIndirectCommandNV {
        pub buffer_address: DeviceAddress,
        pub size: u32,
        pub index_type: IndexType,
    }

    impl BindIndexBufferIndirectCommandNV {
        #[inline]
        pub fn buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
            self.buffer_address = buffer_address;
            self
        }

        #[inline]
        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }

        #[inline]
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBindVertexBufferIndirectCommandNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct BindVertexBufferIndirectCommandNV {
        pub buffer_address: DeviceAddress,
        pub size: u32,
        pub stride: u32,
    }

    impl BindVertexBufferIndirectCommandNV {
        #[inline]
        pub fn buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
            self.buffer_address = buffer_address;
            self
        }

        #[inline]
        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }

        #[inline]
        pub fn stride(mut self, stride: u32) -> Self {
            self.stride = stride;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSetStateFlagsIndirectCommandNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct SetStateFlagsIndirectCommandNV {
        pub data: u32,
    }

    impl SetStateFlagsIndirectCommandNV {
        #[inline]
        pub fn data(mut self, data: u32) -> Self {
            self.data = data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsStreamNV.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct IndirectCommandsStreamNV {
        pub buffer: Buffer,
        pub offset: DeviceSize,
    }

    impl IndirectCommandsStreamNV {
        #[inline]
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutTokenNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct IndirectCommandsLayoutTokenNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub token_type: IndirectCommandsTokenTypeNV,
        pub stream: u32,
        pub offset: u32,
        pub vertex_binding_unit: u32,
        pub vertex_dynamic_stride: Bool32,
        pub pushconstant_pipeline_layout: PipelineLayout,
        pub pushconstant_shader_stage_flags: ShaderStageFlags,
        pub pushconstant_offset: u32,
        pub pushconstant_size: u32,
        pub indirect_state_flags: IndirectStateFlagsNV,
        pub index_type_count: u32,
        pub p_index_types: *const IndexType,
        pub p_index_type_values: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for IndirectCommandsLayoutTokenNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("IndirectCommandsLayoutTokenNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("token_type", &self.token_type)
                .field("stream", &self.stream)
                .field("offset", &self.offset)
                .field("vertex_binding_unit", &self.vertex_binding_unit)
                .field("vertex_dynamic_stride", &self.vertex_dynamic_stride)
                .field(
                    "pushconstant_pipeline_layout",
                    &self.pushconstant_pipeline_layout,
                )
                .field(
                    "pushconstant_shader_stage_flags",
                    &self.pushconstant_shader_stage_flags,
                )
                .field("pushconstant_offset", &self.pushconstant_offset)
                .field("pushconstant_size", &self.pushconstant_size)
                .field("indirect_state_flags", &self.indirect_state_flags)
                .field("index_type_count", &self.index_type_count)
                .field("p_index_types", &self.p_index_types)
                .field("p_index_type_values", &self.p_index_type_values)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for IndirectCommandsLayoutTokenNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV;
    }

    impl Default for IndirectCommandsLayoutTokenNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                token_type: Default::default(),
                stream: Default::default(),
                offset: Default::default(),
                vertex_binding_unit: Default::default(),
                vertex_dynamic_stride: Default::default(),
                pushconstant_pipeline_layout: Default::default(),
                pushconstant_shader_stage_flags: Default::default(),
                pushconstant_offset: Default::default(),
                pushconstant_size: Default::default(),
                indirect_state_flags: Default::default(),
                index_type_count: Default::default(),
                p_index_types: ptr::null(),
                p_index_type_values: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> IndirectCommandsLayoutTokenNV<'a> {
        #[inline]
        pub fn token_type(mut self, token_type: IndirectCommandsTokenTypeNV) -> Self {
            self.token_type = token_type;
            self
        }

        #[inline]
        pub fn stream(mut self, stream: u32) -> Self {
            self.stream = stream;
            self
        }

        #[inline]
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }

        #[inline]
        pub fn vertex_binding_unit(mut self, vertex_binding_unit: u32) -> Self {
            self.vertex_binding_unit = vertex_binding_unit;
            self
        }

        #[inline]
        pub fn vertex_dynamic_stride(mut self, vertex_dynamic_stride: bool) -> Self {
            self.vertex_dynamic_stride = vertex_dynamic_stride.into();
            self
        }

        #[inline]
        pub fn pushconstant_pipeline_layout(
            mut self,
            pushconstant_pipeline_layout: PipelineLayout,
        ) -> Self {
            self.pushconstant_pipeline_layout = pushconstant_pipeline_layout;
            self
        }

        #[inline]
        pub fn pushconstant_shader_stage_flags(
            mut self,
            pushconstant_shader_stage_flags: ShaderStageFlags,
        ) -> Self {
            self.pushconstant_shader_stage_flags = pushconstant_shader_stage_flags;
            self
        }

        #[inline]
        pub fn pushconstant_offset(mut self, pushconstant_offset: u32) -> Self {
            self.pushconstant_offset = pushconstant_offset;
            self
        }

        #[inline]
        pub fn pushconstant_size(mut self, pushconstant_size: u32) -> Self {
            self.pushconstant_size = pushconstant_size;
            self
        }

        #[inline]
        pub fn indirect_state_flags(mut self, indirect_state_flags: IndirectStateFlagsNV) -> Self {
            self.indirect_state_flags = indirect_state_flags;
            self
        }

        #[inline]
        pub fn index_types(
            mut self,
            index_types: &'a [IndexType],
            index_type_values: &'a [u32],
        ) -> Self {
            self.index_type_count = index_types.len().try_into().unwrap();
            assert_eq!(index_type_values.len(), self.index_type_count as usize);
            self.p_index_types = index_types.as_ptr();
            self.p_index_type_values = index_type_values.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct IndirectCommandsLayoutCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: IndirectCommandsLayoutUsageFlagsNV,
        pub pipeline_bind_point: PipelineBindPoint,
        pub token_count: u32,
        pub p_tokens: *const IndirectCommandsLayoutTokenNV<'a>,
        pub stream_count: u32,
        pub p_stream_strides: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for IndirectCommandsLayoutCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("IndirectCommandsLayoutCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("pipeline_bind_point", &self.pipeline_bind_point)
                .field("token_count", &self.token_count)
                .field("p_tokens", &self.p_tokens)
                .field("stream_count", &self.stream_count)
                .field("p_stream_strides", &self.p_stream_strides)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for IndirectCommandsLayoutCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV;
    }

    impl Default for IndirectCommandsLayoutCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                pipeline_bind_point: Default::default(),
                token_count: Default::default(),
                p_tokens: ptr::null(),
                stream_count: Default::default(),
                p_stream_strides: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> IndirectCommandsLayoutCreateInfoNV<'a> {
        #[inline]
        pub fn flags(mut self, flags: IndirectCommandsLayoutUsageFlagsNV) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }

        #[inline]
        pub fn tokens(mut self, tokens: &'a [IndirectCommandsLayoutTokenNV<'a>]) -> Self {
            self.token_count = tokens.len().try_into().unwrap();
            self.p_tokens = tokens.as_ptr();
            self
        }

        #[inline]
        pub fn stream_strides(mut self, stream_strides: &'a [u32]) -> Self {
            self.stream_count = stream_strides.len().try_into().unwrap();
            self.p_stream_strides = stream_strides.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeneratedCommandsInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GeneratedCommandsInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline_bind_point: PipelineBindPoint,
        pub pipeline: Pipeline,
        pub indirect_commands_layout: IndirectCommandsLayoutNV,
        pub stream_count: u32,
        pub p_streams: *const IndirectCommandsStreamNV,
        pub sequences_count: u32,
        pub preprocess_buffer: Buffer,
        pub preprocess_offset: DeviceSize,
        pub preprocess_size: DeviceSize,
        pub sequences_count_buffer: Buffer,
        pub sequences_count_offset: DeviceSize,
        pub sequences_index_buffer: Buffer,
        pub sequences_index_offset: DeviceSize,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GeneratedCommandsInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GeneratedCommandsInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_bind_point", &self.pipeline_bind_point)
                .field("pipeline", &self.pipeline)
                .field("indirect_commands_layout", &self.indirect_commands_layout)
                .field("stream_count", &self.stream_count)
                .field("p_streams", &self.p_streams)
                .field("sequences_count", &self.sequences_count)
                .field("preprocess_buffer", &self.preprocess_buffer)
                .field("preprocess_offset", &self.preprocess_offset)
                .field("preprocess_size", &self.preprocess_size)
                .field("sequences_count_buffer", &self.sequences_count_buffer)
                .field("sequences_count_offset", &self.sequences_count_offset)
                .field("sequences_index_buffer", &self.sequences_index_buffer)
                .field("sequences_index_offset", &self.sequences_index_offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GeneratedCommandsInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GENERATED_COMMANDS_INFO_NV;
    }

    impl Default for GeneratedCommandsInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                pipeline_bind_point: Default::default(),
                pipeline: Default::default(),
                indirect_commands_layout: Default::default(),
                stream_count: Default::default(),
                p_streams: ptr::null(),
                sequences_count: Default::default(),
                preprocess_buffer: Default::default(),
                preprocess_offset: Default::default(),
                preprocess_size: Default::default(),
                sequences_count_buffer: Default::default(),
                sequences_count_offset: Default::default(),
                sequences_index_buffer: Default::default(),
                sequences_index_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GeneratedCommandsInfoNV<'a> {
        #[inline]
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }

        #[inline]
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }

        #[inline]
        pub fn indirect_commands_layout(
            mut self,
            indirect_commands_layout: IndirectCommandsLayoutNV,
        ) -> Self {
            self.indirect_commands_layout = indirect_commands_layout;
            self
        }

        #[inline]
        pub fn streams(mut self, streams: &'a [IndirectCommandsStreamNV]) -> Self {
            self.stream_count = streams.len().try_into().unwrap();
            self.p_streams = streams.as_ptr();
            self
        }

        #[inline]
        pub fn sequences_count(mut self, sequences_count: u32) -> Self {
            self.sequences_count = sequences_count;
            self
        }

        #[inline]
        pub fn preprocess_buffer(mut self, preprocess_buffer: Buffer) -> Self {
            self.preprocess_buffer = preprocess_buffer;
            self
        }

        #[inline]
        pub fn preprocess_offset(mut self, preprocess_offset: DeviceSize) -> Self {
            self.preprocess_offset = preprocess_offset;
            self
        }

        #[inline]
        pub fn preprocess_size(mut self, preprocess_size: DeviceSize) -> Self {
            self.preprocess_size = preprocess_size;
            self
        }

        #[inline]
        pub fn sequences_count_buffer(mut self, sequences_count_buffer: Buffer) -> Self {
            self.sequences_count_buffer = sequences_count_buffer;
            self
        }

        #[inline]
        pub fn sequences_count_offset(mut self, sequences_count_offset: DeviceSize) -> Self {
            self.sequences_count_offset = sequences_count_offset;
            self
        }

        #[inline]
        pub fn sequences_index_buffer(mut self, sequences_index_buffer: Buffer) -> Self {
            self.sequences_index_buffer = sequences_index_buffer;
            self
        }

        #[inline]
        pub fn sequences_index_offset(mut self, sequences_index_offset: DeviceSize) -> Self {
            self.sequences_index_offset = sequences_index_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGeneratedCommandsMemoryRequirementsInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GeneratedCommandsMemoryRequirementsInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline_bind_point: PipelineBindPoint,
        pub pipeline: Pipeline,
        pub indirect_commands_layout: IndirectCommandsLayoutNV,
        pub max_sequences_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GeneratedCommandsMemoryRequirementsInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GeneratedCommandsMemoryRequirementsInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("pipeline_bind_point", &self.pipeline_bind_point)
                .field("pipeline", &self.pipeline)
                .field("indirect_commands_layout", &self.indirect_commands_layout)
                .field("max_sequences_count", &self.max_sequences_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GeneratedCommandsMemoryRequirementsInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV;
    }

    impl Default for GeneratedCommandsMemoryRequirementsInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                pipeline_bind_point: Default::default(),
                pipeline: Default::default(),
                indirect_commands_layout: Default::default(),
                max_sequences_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GeneratedCommandsMemoryRequirementsInfoNV<'a> {
        #[inline]
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }

        #[inline]
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }

        #[inline]
        pub fn indirect_commands_layout(
            mut self,
            indirect_commands_layout: IndirectCommandsLayoutNV,
        ) -> Self {
            self.indirect_commands_layout = indirect_commands_layout;
            self
        }

        #[inline]
        pub fn max_sequences_count(mut self, max_sequences_count: u32) -> Self {
            self.max_sequences_count = max_sequences_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsTokenTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IndirectCommandsTokenTypeNV(i32);

    impl IndirectCommandsTokenTypeNV {
        pub const SHADER_GROUP_NV: Self = Self(0);
        pub const STATE_FLAGS_NV: Self = Self(1);
        pub const INDEX_BUFFER_NV: Self = Self(2);
        pub const VERTEX_BUFFER_NV: Self = Self(3);
        pub const PUSH_CONSTANT_NV: Self = Self(4);
        pub const DRAW_INDEXED_NV: Self = Self(5);
        pub const DRAW_NV: Self = Self(6);
        pub const DRAW_TASKS_NV: Self = Self(7);

        // VK_EXT_descriptor_heap
        pub const PUSH_DATA_NV: Self = Self(1000135000);

        // VK_EXT_mesh_shader
        pub const DRAW_MESH_TASKS_NV: Self = Self(1000328000);

        // VK_NV_device_generated_commands_compute
        pub const PIPELINE_NV: Self = Self(1000428003);
        pub const DISPATCH_NV: Self = Self(1000428004);
    }

    impl fmt::Debug for IndirectCommandsTokenTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SHADER_GROUP_NV => Some("SHADER_GROUP_NV"),
                Self::STATE_FLAGS_NV => Some("STATE_FLAGS_NV"),
                Self::INDEX_BUFFER_NV => Some("INDEX_BUFFER_NV"),
                Self::VERTEX_BUFFER_NV => Some("VERTEX_BUFFER_NV"),
                Self::PUSH_CONSTANT_NV => Some("PUSH_CONSTANT_NV"),
                Self::DRAW_INDEXED_NV => Some("DRAW_INDEXED_NV"),
                Self::DRAW_NV => Some("DRAW_NV"),
                Self::DRAW_TASKS_NV => Some("DRAW_TASKS_NV"),
                Self::PUSH_DATA_NV => Some("PUSH_DATA_NV"),
                Self::DRAW_MESH_TASKS_NV => Some("DRAW_MESH_TASKS_NV"),
                Self::PIPELINE_NV => Some("PIPELINE_NV"),
                Self::DISPATCH_NV => Some("DISPATCH_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutUsageFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct IndirectCommandsLayoutUsageFlagsNV(Flags);
    vk_bitflags_wrapped!(IndirectCommandsLayoutUsageFlagsNV, Flags);

    impl IndirectCommandsLayoutUsageFlagsNV {
        pub const EXPLICIT_PREPROCESS_NV: Self =
            Self(IndirectCommandsLayoutUsageFlagBitsNV::EXPLICIT_PREPROCESS_NV.0);
        pub const INDEXED_SEQUENCES_NV: Self =
            Self(IndirectCommandsLayoutUsageFlagBitsNV::INDEXED_SEQUENCES_NV.0);
        pub const UNORDERED_SEQUENCES_NV: Self =
            Self(IndirectCommandsLayoutUsageFlagBitsNV::UNORDERED_SEQUENCES_NV.0);
    }

    impl fmt::Debug for IndirectCommandsLayoutUsageFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    IndirectCommandsLayoutUsageFlagsNV::EXPLICIT_PREPROCESS_NV.0,
                    "EXPLICIT_PREPROCESS_NV",
                ),
                (
                    IndirectCommandsLayoutUsageFlagsNV::INDEXED_SEQUENCES_NV.0,
                    "INDEXED_SEQUENCES_NV",
                ),
                (
                    IndirectCommandsLayoutUsageFlagsNV::UNORDERED_SEQUENCES_NV.0,
                    "UNORDERED_SEQUENCES_NV",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectCommandsLayoutUsageFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct IndirectCommandsLayoutUsageFlagBitsNV(u32);

    impl IndirectCommandsLayoutUsageFlagBitsNV {
        pub const EXPLICIT_PREPROCESS_NV: Self = Self(1 << 0);
        pub const INDEXED_SEQUENCES_NV: Self = Self(1 << 1);
        pub const UNORDERED_SEQUENCES_NV: Self = Self(1 << 2);
    }

    impl fmt::Debug for IndirectCommandsLayoutUsageFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::EXPLICIT_PREPROCESS_NV => Some("EXPLICIT_PREPROCESS_NV"),
                Self::INDEXED_SEQUENCES_NV => Some("INDEXED_SEQUENCES_NV"),
                Self::UNORDERED_SEQUENCES_NV => Some("UNORDERED_SEQUENCES_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectStateFlagsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct IndirectStateFlagsNV(Flags);
    vk_bitflags_wrapped!(IndirectStateFlagsNV, Flags);

    impl IndirectStateFlagsNV {
        pub const FLAG_FRONTFACE_NV: Self = Self(IndirectStateFlagBitsNV::FLAG_FRONTFACE_NV.0);
    }

    impl fmt::Debug for IndirectStateFlagsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[(
                IndirectStateFlagsNV::FLAG_FRONTFACE_NV.0,
                "FLAG_FRONTFACE_NV",
            )];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkIndirectStateFlagBitsNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct IndirectStateFlagBitsNV(u32);

    impl IndirectStateFlagBitsNV {
        pub const FLAG_FRONTFACE_NV: Self = Self(1 << 0);
    }

    impl fmt::Debug for IndirectStateFlagBitsNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FLAG_FRONTFACE_NV => Some("FLAG_FRONTFACE_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdExecuteGeneratedCommandsNV.html>
    pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPreprocessGeneratedCommandsNV.html>
    pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindPipelineShaderGroupNV.html>
    pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html>
    pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
        device: Device,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateIndirectCommandsLayoutNV.html>
    pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyIndirectCommandsLayoutNV.html>
    pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        p_allocator: *const AllocationCallbacks<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkIndirectCommandsLayoutNV = IndirectCommandsLayoutNV;
    pub type VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV =
        PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'static>;
    pub type VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV =
        PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'static>;
    pub type VkGraphicsShaderGroupCreateInfoNV = GraphicsShaderGroupCreateInfoNV<'static>;
    pub type VkGraphicsPipelineShaderGroupsCreateInfoNV =
        GraphicsPipelineShaderGroupsCreateInfoNV<'static>;
    pub type VkBindShaderGroupIndirectCommandNV = BindShaderGroupIndirectCommandNV;
    pub type VkBindIndexBufferIndirectCommandNV = BindIndexBufferIndirectCommandNV;
    pub type VkBindVertexBufferIndirectCommandNV = BindVertexBufferIndirectCommandNV;
    pub type VkSetStateFlagsIndirectCommandNV = SetStateFlagsIndirectCommandNV;
    pub type VkIndirectCommandsStreamNV = IndirectCommandsStreamNV;
    pub type VkIndirectCommandsLayoutTokenNV = IndirectCommandsLayoutTokenNV<'static>;
    pub type VkIndirectCommandsLayoutCreateInfoNV = IndirectCommandsLayoutCreateInfoNV<'static>;
    pub type VkGeneratedCommandsInfoNV = GeneratedCommandsInfoNV<'static>;
    pub type VkGeneratedCommandsMemoryRequirementsInfoNV =
        GeneratedCommandsMemoryRequirementsInfoNV<'static>;
    pub type VkIndirectCommandsTokenTypeNV = IndirectCommandsTokenTypeNV;
    pub type VkIndirectCommandsLayoutUsageFlagsNV = IndirectCommandsLayoutUsageFlagsNV;
    pub type VkIndirectCommandsLayoutUsageFlagBitsNV = IndirectCommandsLayoutUsageFlagBitsNV;
    pub type VkIndirectStateFlagsNV = IndirectStateFlagsNV;
    pub type VkIndirectStateFlagBitsNV = IndirectStateFlagBitsNV;
    impl PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GraphicsShaderGroupCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGraphicsShaderGroupCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GraphicsPipelineShaderGroupsCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGraphicsPipelineShaderGroupsCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl IndirectCommandsLayoutTokenNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkIndirectCommandsLayoutTokenNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl IndirectCommandsLayoutCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkIndirectCommandsLayoutCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GeneratedCommandsInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGeneratedCommandsInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GeneratedCommandsMemoryRequirementsInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGeneratedCommandsMemoryRequirementsInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_generated_commands_memory_requirements_nv: PFN_vkGetGeneratedCommandsMemoryRequirementsNV,
    cmd_preprocess_generated_commands_nv: PFN_vkCmdPreprocessGeneratedCommandsNV,
    cmd_execute_generated_commands_nv: PFN_vkCmdExecuteGeneratedCommandsNV,
    cmd_bind_pipeline_shader_group_nv: PFN_vkCmdBindPipelineShaderGroupNV,
    create_indirect_commands_layout_nv: PFN_vkCreateIndirectCommandsLayoutNV,
    destroy_indirect_commands_layout_nv: PFN_vkDestroyIndirectCommandsLayoutNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_generated_commands_memory_requirements_nv: transmute(
                    load(c"vkGetGeneratedCommandsMemoryRequirementsNV")
                        .ok_or(MissingEntryPointError)?,
                ),
                cmd_preprocess_generated_commands_nv: transmute(
                    load(c"vkCmdPreprocessGeneratedCommandsNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_execute_generated_commands_nv: transmute(
                    load(c"vkCmdExecuteGeneratedCommandsNV").ok_or(MissingEntryPointError)?,
                ),
                cmd_bind_pipeline_shader_group_nv: transmute(
                    load(c"vkCmdBindPipelineShaderGroupNV").ok_or(MissingEntryPointError)?,
                ),
                create_indirect_commands_layout_nv: transmute(
                    load(c"vkCreateIndirectCommandsLayoutNV").ok_or(MissingEntryPointError)?,
                ),
                destroy_indirect_commands_layout_nv: transmute(
                    load(c"vkDestroyIndirectCommandsLayoutNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGeneratedCommandsMemoryRequirementsNV.html>
    #[inline]
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        device: Device,
        info: &GeneratedCommandsMemoryRequirementsInfoNV<'_>,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        unsafe {
            (self.get_generated_commands_memory_requirements_nv)(device, info, memory_requirements)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdPreprocessGeneratedCommandsNV.html>
    #[inline]
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_preprocess_generated_commands_nv)(command_buffer, generated_commands_info)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdExecuteGeneratedCommandsNV.html>
    #[inline]
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_execute_generated_commands_nv)(
                command_buffer,
                is_preprocessed.into(),
                generated_commands_info,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBindPipelineShaderGroupNV.html>
    #[inline]
    pub unsafe fn cmd_bind_pipeline_shader_group_nv(
        &self,
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ) {
        unsafe {
            (self.cmd_bind_pipeline_shader_group_nv)(
                command_buffer,
                pipeline_bind_point,
                pipeline,
                group_index,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateIndirectCommandsLayoutNV.html>
    #[inline]
    pub unsafe fn create_indirect_commands_layout_nv(
        &self,
        device: Device,
        create_info: &IndirectCommandsLayoutCreateInfoNV<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<IndirectCommandsLayoutNV> {
        unsafe {
            let mut indirect_commands_layout = core::mem::MaybeUninit::uninit();
            let result = (self.create_indirect_commands_layout_nv)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                indirect_commands_layout.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(indirect_commands_layout.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyIndirectCommandsLayoutNV.html>
    #[inline]
    pub unsafe fn destroy_indirect_commands_layout_nv(
        &self,
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_indirect_commands_layout_nv)(
                device,
                indirect_commands_layout,
                allocator.to_raw_ptr(),
            )
        }
    }
}
