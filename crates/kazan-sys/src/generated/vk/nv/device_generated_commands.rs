#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
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
impl Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            device_generated_commands: Default::default(),
            _marker: PhantomData,
        }
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
impl Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV,
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
impl Default for GraphicsShaderGroupCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GRAPHICS_SHADER_GROUP_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            stage_count: Default::default(),
            p_stages: core::ptr::null(),
            p_vertex_input_state: core::ptr::null(),
            p_tessellation_state: core::ptr::null(),
            _marker: PhantomData,
        }
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
impl Default for GraphicsPipelineShaderGroupsCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            group_count: Default::default(),
            p_groups: core::ptr::null(),
            pipeline_count: Default::default(),
            p_pipelines: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BindShaderGroupIndirectCommandNV {
    pub group_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BindIndexBufferIndirectCommandNV {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub index_type: IndexType,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BindVertexBufferIndirectCommandNV {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct SetStateFlagsIndirectCommandNV {
    pub data: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct IndirectCommandsStreamNV {
    pub buffer: Buffer,
    pub offset: DeviceSize,
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
impl Default for IndirectCommandsLayoutTokenNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_NV,
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
impl Default for IndirectCommandsLayoutCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV,
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
impl Default for GeneratedCommandsInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GENERATED_COMMANDS_INFO_NV,
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
impl Default for GeneratedCommandsMemoryRequirementsInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV,
            p_next: core::ptr::null(),
            pipeline_bind_point: Default::default(),
            pipeline: Default::default(),
            indirect_commands_layout: Default::default(),
            max_sequences_count: Default::default(),
            _marker: PhantomData,
        }
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IndirectCommandsLayoutUsageFlagsNV: Flags {
        const EXPLICIT_PREPROCESS_NV = IndirectCommandsLayoutUsageFlagBitsNV::EXPLICIT_PREPROCESS_NV.0;
        const INDEXED_SEQUENCES_NV = IndirectCommandsLayoutUsageFlagBitsNV::INDEXED_SEQUENCES_NV.0;
        const UNORDERED_SEQUENCES_NV = IndirectCommandsLayoutUsageFlagBitsNV::UNORDERED_SEQUENCES_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutUsageFlagBitsNV(u32);
impl IndirectCommandsLayoutUsageFlagBitsNV {
    pub const EXPLICIT_PREPROCESS_NV: Self = Self(1 << 0);
    pub const INDEXED_SEQUENCES_NV: Self = Self(1 << 1);
    pub const UNORDERED_SEQUENCES_NV: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IndirectStateFlagsNV: Flags {
        const FLAG_FRONTFACE_NV = IndirectStateFlagBitsNV::FLAG_FRONTFACE_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
) -> Result;
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: Device,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    p_allocator: *const AllocationCallbacks<'_>,
);
