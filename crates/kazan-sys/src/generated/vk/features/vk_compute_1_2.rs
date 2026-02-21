#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_extended_types: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerReductionModeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reduction_mode: SamplerReductionMode,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFloatControlsProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float16: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float32: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float64: Bool32,
    pub shader_denorm_preserve_float16: Bool32,
    pub shader_denorm_preserve_float32: Bool32,
    pub shader_denorm_preserve_float64: Bool32,
    pub shader_denorm_flush_to_zero_float16: Bool32,
    pub shader_denorm_flush_to_zero_float32: Bool32,
    pub shader_denorm_flush_to_zero_float64: Bool32,
    pub shader_rounding_mode_rte_float16: Bool32,
    pub shader_rounding_mode_rte_float32: Bool32,
    pub shader_rounding_mode_rte_float64: Bool32,
    pub shader_rounding_mode_rtz_float16: Bool32,
    pub shader_rounding_mode_rtz_float32: Bool32,
    pub shader_rounding_mode_rtz_float64: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    pub robust_buffer_access_update_after_bind: Bool32,
    pub quad_divergent_implicit_lod: Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binding_count: u32,
    pub p_binding_flags: *const DescriptorBindingFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_count: u32,
    pub p_descriptor_counts: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_variable_descriptor_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevice8BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer8_bit_access: Bool32,
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    pub storage_push_constant8: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scalar_block_layout: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub uniform_buffer_standard_layout: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerReductionMode(i32);
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderFloatControlsIndependence(i32);
impl ShaderFloatControlsIndependence {
    pub const _32_BIT_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DescriptorBindingFlags: Flags {
        const UPDATE_AFTER_BIND = DescriptorBindingFlagBits::UPDATE_AFTER_BIND.0;
        const UPDATE_UNUSED_WHILE_PENDING = DescriptorBindingFlagBits::UPDATE_UNUSED_WHILE_PENDING.0;
        const PARTIALLY_BOUND = DescriptorBindingFlagBits::PARTIALLY_BOUND.0;
        const VARIABLE_DESCRIPTOR_COUNT = DescriptorBindingFlagBits::VARIABLE_DESCRIPTOR_COUNT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorBindingFlagBits(u32);
impl DescriptorBindingFlagBits {
    pub const UPDATE_AFTER_BIND: Self = Self(1 << 0);
    pub const UPDATE_UNUSED_WHILE_PENDING: Self = Self(1 << 1);
    pub const PARTIALLY_BOUND: Self = Self(1 << 2);
    pub const VARIABLE_DESCRIPTOR_COUNT: Self = Self(1 << 3);
}
