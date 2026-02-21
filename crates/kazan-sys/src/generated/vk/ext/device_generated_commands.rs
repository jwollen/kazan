#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutEXT(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetEXT(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_generated_commands: Bool32,
    pub dynamic_generated_pipeline_layout: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_indirect_pipeline_count: u32,
    pub max_indirect_shader_object_count: u32,
    pub max_indirect_sequence_count: u32,
    pub max_indirect_commands_token_count: u32,
    pub max_indirect_commands_token_offset: u32,
    pub max_indirect_commands_indirect_stride: u32,
    pub supported_indirect_commands_input_modes: IndirectCommandsInputModeFlagsEXT,
    pub supported_indirect_commands_shader_stages: ShaderStageFlags,
    pub supported_indirect_commands_shader_stages_pipeline_binding: ShaderStageFlags,
    pub supported_indirect_commands_shader_stages_shader_binding: ShaderStageFlags,
    pub device_generated_commands_transform_feedback: Bool32,
    pub device_generated_commands_multi_draw_indirect_count: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsPipelineInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsShaderInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_count: u32,
    pub p_shaders: *const ShaderEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsMemoryRequirementsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub indirect_execution_set: IndirectExecutionSetEXT,
    pub indirect_commands_layout: IndirectCommandsLayoutEXT,
    pub max_sequence_count: u32,
    pub max_draw_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetPipelineInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub initial_pipeline: Pipeline,
    pub max_pipeline_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetShaderLayoutInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetShaderInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_count: u32,
    pub p_initial_shaders: *const ShaderEXT,
    pub p_set_layout_infos: *const IndirectExecutionSetShaderLayoutInfoEXT,
    pub max_shader_count: u32,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: IndirectExecutionSetInfoTypeEXT,
    pub info: IndirectExecutionSetInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_stages: ShaderStageFlags,
    pub indirect_execution_set: IndirectExecutionSetEXT,
    pub indirect_commands_layout: IndirectCommandsLayoutEXT,
    pub indirect_address: DeviceAddress,
    pub indirect_address_size: DeviceSize,
    pub preprocess_address: DeviceAddress,
    pub preprocess_size: DeviceSize,
    pub max_sequence_count: u32,
    pub sequence_count_address: DeviceAddress,
    pub max_draw_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteIndirectExecutionSetPipelineEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
    pub pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteIndirectExecutionSetShaderEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
    pub shader: ShaderEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IndirectCommandsLayoutUsageFlagsEXT,
    pub shader_stages: ShaderStageFlags,
    pub indirect_stride: u32,
    pub pipeline_layout: PipelineLayout,
    pub token_count: u32,
    pub p_tokens: *const IndirectCommandsLayoutTokenEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutTokenEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: IndirectCommandsTokenTypeEXT,
    pub data: IndirectCommandsTokenDataEXT,
    pub offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DrawIndirectCountIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub stride: u32,
    pub command_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsVertexBufferTokenEXT {
    pub vertex_binding_unit: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindVertexBufferIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsIndexBufferTokenEXT {
    pub mode: IndirectCommandsInputModeFlagBitsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindIndexBufferIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub index_type: IndexType,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsPushConstantTokenEXT {
    pub update_range: PushConstantRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsExecutionSetTokenEXT {
    pub ty: IndirectExecutionSetInfoTypeEXT,
    pub shader_stages: ShaderStageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union IndirectExecutionSetInfoEXT {
    pub p_pipeline_info: *const IndirectExecutionSetPipelineInfoEXT,
    pub p_shader_info: *const IndirectExecutionSetShaderInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union IndirectCommandsTokenDataEXT {
    pub p_push_constant: *const IndirectCommandsPushConstantTokenEXT,
    pub p_vertex_buffer: *const IndirectCommandsVertexBufferTokenEXT,
    pub p_index_buffer: *const IndirectCommandsIndexBufferTokenEXT,
    pub p_execution_set: *const IndirectCommandsExecutionSetTokenEXT,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectExecutionSetInfoTypeEXT(i32);
impl IndirectExecutionSetInfoTypeEXT {
    pub const PIPELINES_EXT: Self = Self(0);
    pub const SHADER_OBJECTS_EXT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsTokenTypeEXT(i32);
impl IndirectCommandsTokenTypeEXT {
    pub const EXECUTION_SET_EXT: Self = Self(0);
    pub const PUSH_CONSTANT_EXT: Self = Self(1);
    pub const SEQUENCE_INDEX_EXT: Self = Self(2);
    pub const INDEX_BUFFER_EXT: Self = Self(3);
    pub const VERTEX_BUFFER_EXT: Self = Self(4);
    pub const DRAW_INDEXED_EXT: Self = Self(5);
    pub const DRAW_EXT: Self = Self(6);
    pub const DRAW_INDEXED_COUNT_EXT: Self = Self(7);
    pub const DRAW_COUNT_EXT: Self = Self(8);
    pub const DISPATCH_EXT: Self = Self(9);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct IndirectCommandsLayoutUsageFlagsEXT: Flags {
        const EXPLICIT_PREPROCESS_EXT = IndirectCommandsLayoutUsageFlagBitsEXT::EXPLICIT_PREPROCESS_EXT.0;
        const UNORDERED_SEQUENCES_EXT = IndirectCommandsLayoutUsageFlagBitsEXT::UNORDERED_SEQUENCES_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutUsageFlagBitsEXT(u32);
impl IndirectCommandsLayoutUsageFlagBitsEXT {
    pub const EXPLICIT_PREPROCESS_EXT: Self = Self(1 << 0);
    pub const UNORDERED_SEQUENCES_EXT: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct IndirectCommandsInputModeFlagsEXT: Flags {
        const VULKAN_INDEX_BUFFER_EXT = IndirectCommandsInputModeFlagBitsEXT::VULKAN_INDEX_BUFFER_EXT.0;
        const DXGI_INDEX_BUFFER_EXT = IndirectCommandsInputModeFlagBitsEXT::DXGI_INDEX_BUFFER_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsInputModeFlagBitsEXT(u32);
impl IndirectCommandsInputModeFlagBitsEXT {
    pub const VULKAN_INDEX_BUFFER_EXT: Self = Self(1 << 0);
    pub const DXGI_INDEX_BUFFER_EXT: Self = Self(1 << 1);
}
pub type PFN_vkCmdExecuteGeneratedCommandsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    p_generated_commands_info: *const GeneratedCommandsInfoEXT,
);
pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_generated_commands_info: *const GeneratedCommandsInfoEXT,
    state_command_buffer: CommandBuffer,
);
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const GeneratedCommandsMemoryRequirementsInfoEXT,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkCreateIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectCommandsLayoutCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
) -> Result;
pub type PFN_vkDestroyIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: Device,
    indirect_commands_layout: IndirectCommandsLayoutEXT,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectExecutionSetCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_indirect_execution_set: *mut IndirectExecutionSetEXT,
) -> Result;
pub type PFN_vkDestroyIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = unsafe extern "system" fn(
    device: Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    p_execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT,
);
pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = unsafe extern "system" fn(
    device: Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    p_execution_set_writes: *const WriteIndirectExecutionSetShaderEXT,
);
