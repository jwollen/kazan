#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCreateFlags2CreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePushDescriptorProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_push_descriptors: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineProtectedAccessFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_protected_access: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineRobustnessFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_robustness: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRobustnessCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub storage_buffers: PipelineRobustnessBufferBehavior,
    pub uniform_buffers: PipelineRobustnessBufferBehavior,
    pub vertex_inputs: PipelineRobustnessBufferBehavior,
    pub images: PipelineRobustnessImageBehavior,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineRobustnessProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
    pub default_robustness_images: PipelineRobustnessImageBehavior,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindDescriptorSetsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub first_set: u32,
    pub descriptor_set_count: u32,
    pub p_descriptor_sets: *const DescriptorSet,
    pub dynamic_offset_count: u32,
    pub p_dynamic_offsets: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstantsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub layout: PipelineLayout,
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
    pub p_values: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushDescriptorSetInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stage_flags: ShaderStageFlags,
    pub layout: PipelineLayout,
    pub set: u32,
    pub descriptor_write_count: u32,
    pub p_descriptor_writes: *const WriteDescriptorSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushDescriptorSetWithTemplateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_update_template: DescriptorUpdateTemplate,
    pub layout: PipelineLayout,
    pub set: u32,
    pub p_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderSubgroupRotateFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_rotate: Bool32,
    pub shader_subgroup_rotate_clustered: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderExpectAssumeFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_expect_assume: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderFloatControls2Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float_controls2: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRobustnessBufferBehavior(i32);
impl PipelineRobustnessBufferBehavior {
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_BUFFER_ACCESS: Self = Self(2);
    pub const ROBUST_BUFFER_ACCESS_2: Self = Self(3);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineRobustnessImageBehavior(i32);
impl PipelineRobustnessImageBehavior {
    pub const DEVICE_DEFAULT: Self = Self(0);
    pub const DISABLED: Self = Self(1);
    pub const ROBUST_IMAGE_ACCESS: Self = Self(2);
    pub const ROBUST_IMAGE_ACCESS_2: Self = Self(3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineCreateFlags2: Flags64 {
        const DISABLE_OPTIMIZATION = PipelineCreateFlagBits2::DISABLE_OPTIMIZATION.0;
        const ALLOW_DERIVATIVES = PipelineCreateFlagBits2::ALLOW_DERIVATIVES.0;
        const DERIVATIVE = PipelineCreateFlagBits2::DERIVATIVE.0;
        const VIEW_INDEX_FROM_DEVICE_INDEX = PipelineCreateFlagBits2::VIEW_INDEX_FROM_DEVICE_INDEX.0;
        const DISPATCH_BASE = PipelineCreateFlagBits2::DISPATCH_BASE.0;
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED = PipelineCreateFlagBits2::FAIL_ON_PIPELINE_COMPILE_REQUIRED.0;
        const EARLY_RETURN_ON_FAILURE = PipelineCreateFlagBits2::EARLY_RETURN_ON_FAILURE.0;
        const NO_PROTECTED_ACCESS = PipelineCreateFlagBits2::NO_PROTECTED_ACCESS.0;
        const PROTECTED_ACCESS_ONLY = PipelineCreateFlagBits2::PROTECTED_ACCESS_ONLY.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreateFlagBits2(u64);
impl PipelineCreateFlagBits2 {
    pub const DISABLE_OPTIMIZATION: Self = Self(1 << 0);
    pub const ALLOW_DERIVATIVES: Self = Self(1 << 1);
    pub const DERIVATIVE: Self = Self(1 << 2);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(1 << 3);
    pub const DISPATCH_BASE: Self = Self(1 << 4);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(1 << 8);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(1 << 9);
    pub const NO_PROTECTED_ACCESS: Self = Self(1 << 27);
    pub const PROTECTED_ACCESS_ONLY: Self = Self(1 << 30);
}
pub type PFN_vkCmdPushDescriptorSet = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
);
pub type PFN_vkCmdPushDescriptorSetWithTemplate = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    p_data: *const c_void,
);
pub type PFN_vkCmdBindDescriptorSets2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
);
pub type PFN_vkCmdPushConstants2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_constants_info: *const PushConstantsInfo,
);
pub type PFN_vkCmdPushDescriptorSet2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_descriptor_set_info: *const PushDescriptorSetInfo,
);
pub type PFN_vkCmdPushDescriptorSetWithTemplate2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
);
