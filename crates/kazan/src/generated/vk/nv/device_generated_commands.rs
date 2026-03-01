#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct IndirectCommandsLayoutNV(u64);
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_generated_commands: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                device_generated_commands: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'a> {
        pub fn device_generated_commands(mut self, device_generated_commands: Bool32) -> Self {
            self.device_generated_commands = device_generated_commands;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
                p_next: core::ptr::null_mut(),
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
        pub fn max_graphics_shader_group_count(
            mut self,
            max_graphics_shader_group_count: u32,
        ) -> Self {
            self.max_graphics_shader_group_count = max_graphics_shader_group_count;
            self
        }
        pub fn max_indirect_sequence_count(mut self, max_indirect_sequence_count: u32) -> Self {
            self.max_indirect_sequence_count = max_indirect_sequence_count;
            self
        }
        pub fn max_indirect_commands_token_count(
            mut self,
            max_indirect_commands_token_count: u32,
        ) -> Self {
            self.max_indirect_commands_token_count = max_indirect_commands_token_count;
            self
        }
        pub fn max_indirect_commands_stream_count(
            mut self,
            max_indirect_commands_stream_count: u32,
        ) -> Self {
            self.max_indirect_commands_stream_count = max_indirect_commands_stream_count;
            self
        }
        pub fn max_indirect_commands_token_offset(
            mut self,
            max_indirect_commands_token_offset: u32,
        ) -> Self {
            self.max_indirect_commands_token_offset = max_indirect_commands_token_offset;
            self
        }
        pub fn max_indirect_commands_stream_stride(
            mut self,
            max_indirect_commands_stream_stride: u32,
        ) -> Self {
            self.max_indirect_commands_stream_stride = max_indirect_commands_stream_stride;
            self
        }
        pub fn min_sequences_count_buffer_offset_alignment(
            mut self,
            min_sequences_count_buffer_offset_alignment: u32,
        ) -> Self {
            self.min_sequences_count_buffer_offset_alignment =
                min_sequences_count_buffer_offset_alignment;
            self
        }
        pub fn min_sequences_index_buffer_offset_alignment(
            mut self,
            min_sequences_index_buffer_offset_alignment: u32,
        ) -> Self {
            self.min_sequences_index_buffer_offset_alignment =
                min_sequences_index_buffer_offset_alignment;
            self
        }
        pub fn min_indirect_commands_buffer_offset_alignment(
            mut self,
            min_indirect_commands_buffer_offset_alignment: u32,
        ) -> Self {
            self.min_indirect_commands_buffer_offset_alignment =
                min_indirect_commands_buffer_offset_alignment;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GraphicsShaderGroupCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stage_count: u32,
        pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
        pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo<'a>,
        pub p_tessellation_state: *const PipelineTessellationStateCreateInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for GraphicsShaderGroupCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV;
    }
    impl Default for GraphicsShaderGroupCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                stage_count: Default::default(),
                p_stages: core::ptr::null(),
                p_vertex_input_state: core::ptr::null(),
                p_tessellation_state: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GraphicsShaderGroupCreateInfoNV<'a> {
        pub fn stages(mut self, stages: &'a [PipelineShaderStageCreateInfo<'a>]) -> Self {
            self.stage_count = stages.len().try_into().unwrap();
            self.p_stages = stages.as_ptr();
            self
        }
        pub fn vertex_input_state(
            mut self,
            vertex_input_state: &'a PipelineVertexInputStateCreateInfo<'a>,
        ) -> Self {
            self.p_vertex_input_state = vertex_input_state;
            self
        }
        pub fn tessellation_state(
            mut self,
            tessellation_state: &'a PipelineTessellationStateCreateInfo<'a>,
        ) -> Self {
            self.p_tessellation_state = tessellation_state;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GraphicsPipelineShaderGroupsCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub group_count: u32,
        pub p_groups: *const GraphicsShaderGroupCreateInfoNV<'a>,
        pub pipeline_count: u32,
        pub p_pipelines: *const Pipeline,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                group_count: Default::default(),
                p_groups: core::ptr::null(),
                pipeline_count: Default::default(),
                p_pipelines: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GraphicsPipelineShaderGroupsCreateInfoNV<'a> {
        pub fn groups(mut self, groups: &'a [GraphicsShaderGroupCreateInfoNV<'a>]) -> Self {
            self.group_count = groups.len().try_into().unwrap();
            self.p_groups = groups.as_ptr();
            self
        }
        pub fn pipelines(mut self, pipelines: &'a [Pipeline]) -> Self {
            self.pipeline_count = pipelines.len().try_into().unwrap();
            self.p_pipelines = pipelines.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct BindShaderGroupIndirectCommandNV {
        pub group_index: u32,
    }
    impl BindShaderGroupIndirectCommandNV {
        pub fn group_index(mut self, group_index: u32) -> Self {
            self.group_index = group_index;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct BindIndexBufferIndirectCommandNV {
        pub buffer_address: DeviceAddress,
        pub size: u32,
        pub index_type: IndexType,
    }
    impl BindIndexBufferIndirectCommandNV {
        pub fn buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
            self.buffer_address = buffer_address;
            self
        }
        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }
        pub fn index_type(mut self, index_type: IndexType) -> Self {
            self.index_type = index_type;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct BindVertexBufferIndirectCommandNV {
        pub buffer_address: DeviceAddress,
        pub size: u32,
        pub stride: u32,
    }
    impl BindVertexBufferIndirectCommandNV {
        pub fn buffer_address(mut self, buffer_address: DeviceAddress) -> Self {
            self.buffer_address = buffer_address;
            self
        }
        pub fn size(mut self, size: u32) -> Self {
            self.size = size;
            self
        }
        pub fn stride(mut self, stride: u32) -> Self {
            self.stride = stride;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct SetStateFlagsIndirectCommandNV {
        pub data: u32,
    }
    impl SetStateFlagsIndirectCommandNV {
        pub fn data(mut self, data: u32) -> Self {
            self.data = data;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct IndirectCommandsStreamNV {
        pub buffer: Buffer,
        pub offset: DeviceSize,
    }
    impl IndirectCommandsStreamNV {
        pub fn buffer(mut self, buffer: Buffer) -> Self {
            self.buffer = buffer;
            self
        }
        pub fn offset(mut self, offset: DeviceSize) -> Self {
            self.offset = offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for IndirectCommandsLayoutTokenNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV;
    }
    impl Default for IndirectCommandsLayoutTokenNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
                p_index_types: core::ptr::null(),
                p_index_type_values: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> IndirectCommandsLayoutTokenNV<'a> {
        pub fn token_type(mut self, token_type: IndirectCommandsTokenTypeNV) -> Self {
            self.token_type = token_type;
            self
        }
        pub fn stream(mut self, stream: u32) -> Self {
            self.stream = stream;
            self
        }
        pub fn offset(mut self, offset: u32) -> Self {
            self.offset = offset;
            self
        }
        pub fn vertex_binding_unit(mut self, vertex_binding_unit: u32) -> Self {
            self.vertex_binding_unit = vertex_binding_unit;
            self
        }
        pub fn vertex_dynamic_stride(mut self, vertex_dynamic_stride: Bool32) -> Self {
            self.vertex_dynamic_stride = vertex_dynamic_stride;
            self
        }
        pub fn pushconstant_pipeline_layout(
            mut self,
            pushconstant_pipeline_layout: PipelineLayout,
        ) -> Self {
            self.pushconstant_pipeline_layout = pushconstant_pipeline_layout;
            self
        }
        pub fn pushconstant_shader_stage_flags(
            mut self,
            pushconstant_shader_stage_flags: ShaderStageFlags,
        ) -> Self {
            self.pushconstant_shader_stage_flags = pushconstant_shader_stage_flags;
            self
        }
        pub fn pushconstant_offset(mut self, pushconstant_offset: u32) -> Self {
            self.pushconstant_offset = pushconstant_offset;
            self
        }
        pub fn pushconstant_size(mut self, pushconstant_size: u32) -> Self {
            self.pushconstant_size = pushconstant_size;
            self
        }
        pub fn indirect_state_flags(mut self, indirect_state_flags: IndirectStateFlagsNV) -> Self {
            self.indirect_state_flags = indirect_state_flags;
            self
        }
        pub fn index_types(mut self, index_types: &'a [IndexType]) -> Self {
            self.index_type_count = index_types.len().try_into().unwrap();
            self.p_index_types = index_types.as_ptr();
            self
        }
        pub fn index_type_values(mut self, index_type_values: &'a [u32]) -> Self {
            self.index_type_count = index_type_values.len().try_into().unwrap();
            self.p_index_type_values = index_type_values.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for IndirectCommandsLayoutCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV;
    }
    impl Default for IndirectCommandsLayoutCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                pipeline_bind_point: Default::default(),
                token_count: Default::default(),
                p_tokens: core::ptr::null(),
                stream_count: Default::default(),
                p_stream_strides: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> IndirectCommandsLayoutCreateInfoNV<'a> {
        pub fn flags(mut self, flags: IndirectCommandsLayoutUsageFlagsNV) -> Self {
            self.flags = flags;
            self
        }
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }
        pub fn tokens(mut self, tokens: &'a [IndirectCommandsLayoutTokenNV<'a>]) -> Self {
            self.token_count = tokens.len().try_into().unwrap();
            self.p_tokens = tokens.as_ptr();
            self
        }
        pub fn stream_strides(mut self, stream_strides: &'a [u32]) -> Self {
            self.stream_count = stream_strides.len().try_into().unwrap();
            self.p_stream_strides = stream_strides.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for GeneratedCommandsInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GENERATED_COMMANDS_INFO_NV;
    }
    impl Default for GeneratedCommandsInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                pipeline_bind_point: Default::default(),
                pipeline: Default::default(),
                indirect_commands_layout: Default::default(),
                stream_count: Default::default(),
                p_streams: core::ptr::null(),
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
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
        pub fn indirect_commands_layout(
            mut self,
            indirect_commands_layout: IndirectCommandsLayoutNV,
        ) -> Self {
            self.indirect_commands_layout = indirect_commands_layout;
            self
        }
        pub fn streams(mut self, streams: &'a [IndirectCommandsStreamNV]) -> Self {
            self.stream_count = streams.len().try_into().unwrap();
            self.p_streams = streams.as_ptr();
            self
        }
        pub fn sequences_count(mut self, sequences_count: u32) -> Self {
            self.sequences_count = sequences_count;
            self
        }
        pub fn preprocess_buffer(mut self, preprocess_buffer: Buffer) -> Self {
            self.preprocess_buffer = preprocess_buffer;
            self
        }
        pub fn preprocess_offset(mut self, preprocess_offset: DeviceSize) -> Self {
            self.preprocess_offset = preprocess_offset;
            self
        }
        pub fn preprocess_size(mut self, preprocess_size: DeviceSize) -> Self {
            self.preprocess_size = preprocess_size;
            self
        }
        pub fn sequences_count_buffer(mut self, sequences_count_buffer: Buffer) -> Self {
            self.sequences_count_buffer = sequences_count_buffer;
            self
        }
        pub fn sequences_count_offset(mut self, sequences_count_offset: DeviceSize) -> Self {
            self.sequences_count_offset = sequences_count_offset;
            self
        }
        pub fn sequences_index_buffer(mut self, sequences_index_buffer: Buffer) -> Self {
            self.sequences_index_buffer = sequences_index_buffer;
            self
        }
        pub fn sequences_index_offset(mut self, sequences_index_offset: DeviceSize) -> Self {
            self.sequences_index_offset = sequences_index_offset;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GeneratedCommandsMemoryRequirementsInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub pipeline_bind_point: PipelineBindPoint,
        pub pipeline: Pipeline,
        pub indirect_commands_layout: IndirectCommandsLayoutNV,
        pub max_sequences_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for GeneratedCommandsMemoryRequirementsInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV;
    }
    impl Default for GeneratedCommandsMemoryRequirementsInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                pipeline_bind_point: Default::default(),
                pipeline: Default::default(),
                indirect_commands_layout: Default::default(),
                max_sequences_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GeneratedCommandsMemoryRequirementsInfoNV<'a> {
        pub fn pipeline_bind_point(mut self, pipeline_bind_point: PipelineBindPoint) -> Self {
            self.pipeline_bind_point = pipeline_bind_point;
            self
        }
        pub fn pipeline(mut self, pipeline: Pipeline) -> Self {
            self.pipeline = pipeline;
            self
        }
        pub fn indirect_commands_layout(
            mut self,
            indirect_commands_layout: IndirectCommandsLayoutNV,
        ) -> Self {
            self.indirect_commands_layout = indirect_commands_layout;
            self
        }
        pub fn max_sequences_count(mut self, max_sequences_count: u32) -> Self {
            self.max_sequences_count = max_sequences_count;
            self
        }
    }
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
        pub const DISPATCH_NV: Self = Self(1000428004);
        pub const DRAW_MESH_TASKS_NV: Self = Self(1000328000);
        pub const PIPELINE_NV: Self = Self(1000428003);
        pub const PUSH_DATA_NV: Self = Self(1000135000);
    }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct IndirectCommandsLayoutUsageFlagBitsNV(u32);
    impl IndirectCommandsLayoutUsageFlagBitsNV {
        pub const EXPLICIT_PREPROCESS_NV: Self = Self(1 << 0);
        pub const INDEXED_SEQUENCES_NV: Self = Self(1 << 1);
        pub const UNORDERED_SEQUENCES_NV: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct IndirectStateFlagsNV(Flags);
    vk_bitflags_wrapped!(IndirectStateFlagsNV, Flags);
    impl IndirectStateFlagsNV {
        pub const FLAG_FRONTFACE_NV: Self = Self(IndirectStateFlagBitsNV::FLAG_FRONTFACE_NV.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct IndirectStateFlagBitsNV(u32);
    impl IndirectStateFlagBitsNV {
        pub const FLAG_FRONTFACE_NV: Self = Self(1 << 0);
    }
    pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
    );
    pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_generated_commands_info: *const GeneratedCommandsInfoNV<'_>,
    );
    pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    );
    pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
        device: Device,
        p_info: *const GeneratedCommandsMemoryRequirementsInfoNV<'_>,
        p_memory_requirements: *mut MemoryRequirements2<'_>,
    );
    pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const IndirectCommandsLayoutCreateInfoNV<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
    ) -> vk::Result;
    pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
        device: Device,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        p_allocator: *const AllocationCallbacks<'_>,
    );
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
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_generated_commands_memory_requirements_nv: transmute(
                    load(c"vkGetGeneratedCommandsMemoryRequirementsNV").ok_or(LoadingError)?,
                ),
                cmd_preprocess_generated_commands_nv: transmute(
                    load(c"vkCmdPreprocessGeneratedCommandsNV").ok_or(LoadingError)?,
                ),
                cmd_execute_generated_commands_nv: transmute(
                    load(c"vkCmdExecuteGeneratedCommandsNV").ok_or(LoadingError)?,
                ),
                cmd_bind_pipeline_shader_group_nv: transmute(
                    load(c"vkCmdBindPipelineShaderGroupNV").ok_or(LoadingError)?,
                ),
                create_indirect_commands_layout_nv: transmute(
                    load(c"vkCreateIndirectCommandsLayoutNV").ok_or(LoadingError)?,
                ),
                destroy_indirect_commands_layout_nv: transmute(
                    load(c"vkDestroyIndirectCommandsLayoutNV").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_generated_commands_memory_requirements_nv(
        &self,
        device: Device,
        info: &GeneratedCommandsMemoryRequirementsInfoNV<'_>,
    ) -> MemoryRequirements2<'_> {
        unsafe {
            let mut memory_requirements = core::mem::MaybeUninit::uninit();
            (self.get_generated_commands_memory_requirements_nv)(
                device,
                info,
                memory_requirements.as_mut_ptr(),
            );
            memory_requirements.assume_init()
        }
    }
    pub unsafe fn cmd_preprocess_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        generated_commands_info: &GeneratedCommandsInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_preprocess_generated_commands_nv)(command_buffer, generated_commands_info)
        }
    }
    pub unsafe fn cmd_execute_generated_commands_nv(
        &self,
        command_buffer: CommandBuffer,
        is_preprocessed: Bool32,
        generated_commands_info: &GeneratedCommandsInfoNV<'_>,
    ) {
        unsafe {
            (self.cmd_execute_generated_commands_nv)(
                command_buffer,
                is_preprocessed,
                generated_commands_info,
            )
        }
    }
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
