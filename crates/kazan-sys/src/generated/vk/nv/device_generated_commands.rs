#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutNV(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_generated_commands: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
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
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphicsShaderGroupCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub group_count: u32,
    pub p_groups: *const GraphicsShaderGroupCreateInfoNV,
    pub pipeline_count: u32,
    pub p_pipelines: *const Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindShaderGroupIndirectCommandNV {
    pub group_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindIndexBufferIndirectCommandNV {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub index_type: IndexType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindVertexBufferIndirectCommandNV {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetStateFlagsIndirectCommandNV {
    pub data: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsStreamNV {
    pub buffer: Buffer,
    pub offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutTokenNV {
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
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IndirectCommandsLayoutUsageFlagsNV,
    pub pipeline_bind_point: PipelineBindPoint,
    pub token_count: u32,
    pub p_tokens: *const IndirectCommandsLayoutTokenNV,
    pub stream_count: u32,
    pub p_stream_strides: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsInfoNV {
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
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsMemoryRequirementsInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline: Pipeline,
    pub indirect_commands_layout: IndirectCommandsLayoutNV,
    pub max_sequences_count: u32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct IndirectCommandsLayoutUsageFlagsNV: Flags {
        const EXPLICIT_PREPROCESS_NV = IndirectCommandsLayoutUsageFlagBitsNV::EXPLICIT_PREPROCESS_NV.0;
        const INDEXED_SEQUENCES_NV = IndirectCommandsLayoutUsageFlagBitsNV::INDEXED_SEQUENCES_NV.0;
        const UNORDERED_SEQUENCES_NV = IndirectCommandsLayoutUsageFlagBitsNV::UNORDERED_SEQUENCES_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutUsageFlagBitsNV(u32);
impl IndirectCommandsLayoutUsageFlagBitsNV {
    pub const EXPLICIT_PREPROCESS_NV: Self = Self(1 << 0);
    pub const INDEXED_SEQUENCES_NV: Self = Self(1 << 1);
    pub const UNORDERED_SEQUENCES_NV: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct IndirectStateFlagsNV: Flags {
        const FLAG_FRONTFACE_NV = IndirectStateFlagBitsNV::FLAG_FRONTFACE_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectStateFlagBitsNV(u32);
impl IndirectStateFlagBitsNV {
    pub const FLAG_FRONTFACE_NV: Self = Self(1 << 0);
}
pub type PFN_vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    p_generated_commands_info: *const GeneratedCommandsInfoNV,
);
pub type PFN_vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_generated_commands_info: *const GeneratedCommandsInfoNV,
);
pub type PFN_vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    group_index: u32,
);
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
    device: Device,
    p_info: *const GeneratedCommandsMemoryRequirementsInfoNV,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectCommandsLayoutCreateInfoNV,
    p_allocator: *const AllocationCallbacks,
    p_indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> Result;
pub type PFN_vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: Device,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    p_allocator: *const AllocationCallbacks,
);
