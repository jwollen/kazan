#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct IndirectCommandsLayoutEXT(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct IndirectExecutionSetEXT(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_generated_commands: Bool32,
    pub dynamic_generated_pipeline_layout: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            device_generated_commands: Default::default(),
            dynamic_generated_pipeline_layout: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            max_indirect_pipeline_count: Default::default(),
            max_indirect_shader_object_count: Default::default(),
            max_indirect_sequence_count: Default::default(),
            max_indirect_commands_token_count: Default::default(),
            max_indirect_commands_token_offset: Default::default(),
            max_indirect_commands_indirect_stride: Default::default(),
            supported_indirect_commands_input_modes: Default::default(),
            supported_indirect_commands_shader_stages: Default::default(),
            supported_indirect_commands_shader_stages_pipeline_binding: Default::default(),
            supported_indirect_commands_shader_stages_shader_binding: Default::default(),
            device_generated_commands_transform_feedback: Default::default(),
            device_generated_commands_multi_draw_indirect_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsPipelineInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for GeneratedCommandsPipelineInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GENERATED_COMMANDS_PIPELINE_INFO_EXT,
            p_next: core::ptr::null_mut(),
            pipeline: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsShaderInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_count: u32,
    pub p_shaders: *const ShaderEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for GeneratedCommandsShaderInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GENERATED_COMMANDS_SHADER_INFO_EXT,
            p_next: core::ptr::null_mut(),
            shader_count: Default::default(),
            p_shaders: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsMemoryRequirementsInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub indirect_execution_set: IndirectExecutionSetEXT,
    pub indirect_commands_layout: IndirectCommandsLayoutEXT,
    pub max_sequence_count: u32,
    pub max_draw_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for GeneratedCommandsMemoryRequirementsInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT,
            p_next: core::ptr::null(),
            indirect_execution_set: Default::default(),
            indirect_commands_layout: Default::default(),
            max_sequence_count: Default::default(),
            max_draw_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetPipelineInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub initial_pipeline: Pipeline,
    pub max_pipeline_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectExecutionSetPipelineInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT,
            p_next: core::ptr::null(),
            initial_pipeline: Default::default(),
            max_pipeline_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetShaderLayoutInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectExecutionSetShaderLayoutInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT,
            p_next: core::ptr::null(),
            set_layout_count: Default::default(),
            p_set_layouts: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetShaderInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub shader_count: u32,
    pub p_initial_shaders: *const ShaderEXT,
    pub p_set_layout_infos: *const IndirectExecutionSetShaderLayoutInfoEXT<'a>,
    pub max_shader_count: u32,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectExecutionSetShaderInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_EXECUTION_SET_SHADER_INFO_EXT,
            p_next: core::ptr::null(),
            shader_count: Default::default(),
            p_initial_shaders: core::ptr::null(),
            p_set_layout_infos: core::ptr::null(),
            max_shader_count: Default::default(),
            push_constant_range_count: Default::default(),
            p_push_constant_ranges: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectExecutionSetCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: IndirectExecutionSetInfoTypeEXT,
    pub info: IndirectExecutionSetInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectExecutionSetCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_EXECUTION_SET_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            ty: Default::default(),
            info: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GeneratedCommandsInfoEXT<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for GeneratedCommandsInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::GENERATED_COMMANDS_INFO_EXT,
            p_next: core::ptr::null(),
            shader_stages: Default::default(),
            indirect_execution_set: Default::default(),
            indirect_commands_layout: Default::default(),
            indirect_address: Default::default(),
            indirect_address_size: Default::default(),
            preprocess_address: Default::default(),
            preprocess_size: Default::default(),
            max_sequence_count: Default::default(),
            sequence_count_address: Default::default(),
            max_draw_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteIndirectExecutionSetPipelineEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
    pub pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for WriteIndirectExecutionSetPipelineEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT,
            p_next: core::ptr::null(),
            index: Default::default(),
            pipeline: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteIndirectExecutionSetShaderEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
    pub shader: ShaderEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for WriteIndirectExecutionSetShaderEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT,
            p_next: core::ptr::null(),
            index: Default::default(),
            shader: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IndirectCommandsLayoutUsageFlagsEXT,
    pub shader_stages: ShaderStageFlags,
    pub indirect_stride: u32,
    pub pipeline_layout: PipelineLayout,
    pub token_count: u32,
    pub p_tokens: *const IndirectCommandsLayoutTokenEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectCommandsLayoutCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            shader_stages: Default::default(),
            indirect_stride: Default::default(),
            pipeline_layout: Default::default(),
            token_count: Default::default(),
            p_tokens: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutTokenEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: IndirectCommandsTokenTypeEXT,
    pub data: IndirectCommandsTokenDataEXT<'a>,
    pub offset: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectCommandsLayoutTokenEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT,
            p_next: core::ptr::null(),
            ty: Default::default(),
            data: Default::default(),
            offset: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DrawIndirectCountIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub stride: u32,
    pub command_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct IndirectCommandsVertexBufferTokenEXT {
    pub vertex_binding_unit: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BindVertexBufferIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct IndirectCommandsIndexBufferTokenEXT {
    pub mode: IndirectCommandsInputModeFlagBitsEXT,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BindIndexBufferIndirectCommandEXT {
    pub buffer_address: DeviceAddress,
    pub size: u32,
    pub index_type: IndexType,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct IndirectCommandsPushConstantTokenEXT {
    pub update_range: PushConstantRange,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct IndirectCommandsExecutionSetTokenEXT {
    pub ty: IndirectExecutionSetInfoTypeEXT,
    pub shader_stages: ShaderStageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union IndirectExecutionSetInfoEXT<'a> {
    pub p_pipeline_info: *const IndirectExecutionSetPipelineInfoEXT<'a>,
    pub p_shader_info: *const IndirectExecutionSetShaderInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectExecutionSetInfoEXT<'_> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union IndirectCommandsTokenDataEXT<'a> {
    pub p_push_constant: *const IndirectCommandsPushConstantTokenEXT,
    pub p_vertex_buffer: *const IndirectCommandsVertexBufferTokenEXT,
    pub p_index_buffer: *const IndirectCommandsIndexBufferTokenEXT,
    pub p_execution_set: *const IndirectCommandsExecutionSetTokenEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectCommandsTokenDataEXT<'_> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectExecutionSetInfoTypeEXT(i32);
impl IndirectExecutionSetInfoTypeEXT {
    pub const PIPELINES_EXT: Self = Self(0);
    pub const SHADER_OBJECTS_EXT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const DRAW_MESH_TASKS_COUNT_EXT: Self = Self(1000328001);
    pub const DRAW_MESH_TASKS_COUNT_NV_EXT: Self = Self(1000202003);
    pub const DRAW_MESH_TASKS_EXT: Self = Self(1000328000);
    pub const DRAW_MESH_TASKS_NV_EXT: Self = Self(1000202002);
    pub const PUSH_DATA_EXT: Self = Self(1000135000);
    pub const PUSH_DATA_SEQUENCE_INDEX_EXT: Self = Self(1000135001);
    pub const TRACE_RAYS2_EXT: Self = Self(1000386004);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IndirectCommandsLayoutUsageFlagsEXT: Flags {
        const EXPLICIT_PREPROCESS_EXT = IndirectCommandsLayoutUsageFlagBitsEXT::EXPLICIT_PREPROCESS_EXT.0;
        const UNORDERED_SEQUENCES_EXT = IndirectCommandsLayoutUsageFlagBitsEXT::UNORDERED_SEQUENCES_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsLayoutUsageFlagBitsEXT(u32);
impl IndirectCommandsLayoutUsageFlagBitsEXT {
    pub const EXPLICIT_PREPROCESS_EXT: Self = Self(1 << 0);
    pub const UNORDERED_SEQUENCES_EXT: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IndirectCommandsInputModeFlagsEXT: Flags {
        const VULKAN_INDEX_BUFFER_EXT = IndirectCommandsInputModeFlagBitsEXT::VULKAN_INDEX_BUFFER_EXT.0;
        const DXGI_INDEX_BUFFER_EXT = IndirectCommandsInputModeFlagBitsEXT::DXGI_INDEX_BUFFER_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndirectCommandsInputModeFlagBitsEXT(u32);
impl IndirectCommandsInputModeFlagBitsEXT {
    pub const VULKAN_INDEX_BUFFER_EXT: Self = Self(1 << 0);
    pub const DXGI_INDEX_BUFFER_EXT: Self = Self(1 << 1);
}
pub type PFN_vkCmdExecuteGeneratedCommandsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    is_preprocessed: Bool32,
    p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
);
pub type PFN_vkCmdPreprocessGeneratedCommandsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_generated_commands_info: *const GeneratedCommandsInfoEXT<'_>,
    state_command_buffer: CommandBuffer,
);
pub type PFN_vkGetGeneratedCommandsMemoryRequirementsEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const GeneratedCommandsMemoryRequirementsInfoEXT<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkCreateIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectCommandsLayoutCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
) -> Result;
pub type PFN_vkDestroyIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: Device,
    indirect_commands_layout: IndirectCommandsLayoutEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const IndirectExecutionSetCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_indirect_execution_set: *mut IndirectExecutionSetEXT,
) -> Result;
pub type PFN_vkDestroyIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkUpdateIndirectExecutionSetPipelineEXT = unsafe extern "system" fn(
    device: Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    p_execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT<'_>,
);
pub type PFN_vkUpdateIndirectExecutionSetShaderEXT = unsafe extern "system" fn(
    device: Device,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    p_execution_set_writes: *const WriteIndirectExecutionSetShaderEXT<'_>,
);
