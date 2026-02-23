#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
#[repr(C)]
pub struct ConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}
#[repr(C)]
pub struct PhysicalDeviceDriverProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub driver_id: DriverId,
    pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: ConformanceVersion,
}
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_extended_types: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
}
#[repr(C)]
pub struct SamplerReductionModeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub reduction_mode: SamplerReductionMode,
}
#[repr(C)]
pub struct ImageFormatListCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}
#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
}
#[repr(C)]
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
pub struct PhysicalDeviceHostQueryResetFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub host_query_reset: Bool32,
}
#[repr(C)]
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
pub struct DescriptorSetLayoutBindingFlagsCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub binding_count: u32,
    pub p_binding_flags: *const DescriptorBindingFlags,
}
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set_count: u32,
    pub p_descriptor_counts: *const u32,
}
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_variable_descriptor_count: u32,
}
#[repr(C)]
pub struct AttachmentDescription2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}
#[repr(C)]
pub struct AttachmentReference2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment: u32,
    pub layout: ImageLayout,
    pub aspect_mask: ImageAspectFlags,
}
#[repr(C)]
pub struct SubpassDescription2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub view_mask: u32,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference2,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference2,
    pub p_resolve_attachments: *const AttachmentReference2,
    pub p_depth_stencil_attachment: *const AttachmentReference2,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
#[repr(C)]
pub struct SubpassDependency2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
    pub view_offset: i32,
}
#[repr(C)]
pub struct RenderPassCreateInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription2,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription2,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency2,
    pub correlated_view_mask_count: u32,
    pub p_correlated_view_masks: *const u32,
}
#[repr(C)]
pub struct SubpassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub contents: SubpassContents,
}
#[repr(C)]
pub struct SubpassEndInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub timeline_semaphore: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_timeline_semaphore_value_difference: u64,
}
#[repr(C)]
pub struct SemaphoreTypeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore_type: SemaphoreType,
    pub initial_value: u64,
}
#[repr(C)]
pub struct TimelineSemaphoreSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_value_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_value_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
#[repr(C)]
pub struct SemaphoreWaitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreWaitFlags,
    pub semaphore_count: u32,
    pub p_semaphores: *const Semaphore,
    pub p_values: *const u64,
}
#[repr(C)]
pub struct SemaphoreSignalInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
}
#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer8_bit_access: Bool32,
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    pub storage_push_constant8: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: Bool32,
    pub independent_resolve: Bool32,
}
#[repr(C)]
pub struct SubpassDescriptionDepthStencilResolve {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_resolve_mode: ResolveModeFlagBits,
    pub stencil_resolve_mode: ResolveModeFlagBits,
    pub p_depth_stencil_resolve_attachment: *const AttachmentReference2,
}
#[repr(C)]
pub struct ImageStencilUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stencil_usage: ImageUsageFlags,
}
#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub scalar_block_layout: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub uniform_buffer_standard_layout: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
}
#[repr(C)]
pub struct BufferDeviceAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
#[repr(C)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_address: u64,
}
#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub imageless_framebuffer: Bool32,
}
#[repr(C)]
pub struct FramebufferAttachmentsCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_image_info_count: u32,
    pub p_attachment_image_infos: *const FramebufferAttachmentImageInfo,
}
#[repr(C)]
pub struct FramebufferAttachmentImageInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub usage: ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layer_count: u32,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}
#[repr(C)]
pub struct RenderPassAttachmentBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
}
#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub separate_depth_stencil_layouts: Bool32,
}
#[repr(C)]
pub struct AttachmentReferenceStencilLayout {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stencil_layout: ImageLayout,
}
#[repr(C)]
pub struct AttachmentDescriptionStencilLayout {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stencil_initial_layout: ImageLayout,
    pub stencil_final_layout: ImageLayout,
}
#[repr(C)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_address: u64,
}
#[repr(C)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}
#[repr(C)]
pub struct PhysicalDeviceVulkan11Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer16_bit_access: Bool32,
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    pub storage_push_constant16: Bool32,
    pub storage_input_output16: Bool32,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
    pub protected_memory: Bool32,
    pub sampler_ycbcr_conversion: Bool32,
    pub shader_draw_parameters: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceVulkan11Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_uuid: [u8; UUID_SIZE as usize],
    pub driver_uuid: [u8; UUID_SIZE as usize],
    pub device_luid: [u8; LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luid_valid: Bool32,
    pub subgroup_size: u32,
    pub subgroup_supported_stages: ShaderStageFlags,
    pub subgroup_supported_operations: SubgroupFeatureFlags,
    pub subgroup_quad_operations_in_all_stages: Bool32,
    pub point_clipping_behavior: PointClippingBehavior,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
    pub protected_no_fault: Bool32,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: DeviceSize,
}
#[repr(C)]
pub struct PhysicalDeviceVulkan12Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_mirror_clamp_to_edge: Bool32,
    pub draw_indirect_count: Bool32,
    pub storage_buffer8_bit_access: Bool32,
    pub uniform_and_storage_buffer8_bit_access: Bool32,
    pub storage_push_constant8: Bool32,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
    pub descriptor_indexing: Bool32,
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
    pub sampler_filter_minmax: Bool32,
    pub scalar_block_layout: Bool32,
    pub imageless_framebuffer: Bool32,
    pub uniform_buffer_standard_layout: Bool32,
    pub shader_subgroup_extended_types: Bool32,
    pub separate_depth_stencil_layouts: Bool32,
    pub host_query_reset: Bool32,
    pub timeline_semaphore: Bool32,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
    pub shader_output_viewport_index: Bool32,
    pub shader_output_layer: Bool32,
    pub subgroup_broadcast_dynamic_id: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceVulkan12Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub driver_id: DriverId,
    pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: ConformanceVersion,
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
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: Bool32,
    pub independent_resolve: Bool32,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
    pub max_timeline_semaphore_value_difference: u64,
    pub framebuffer_integer_color_sample_counts: SampleCountFlags,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreType(i32);
impl SemaphoreType {
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
    pub const BINARY_KHR: Self = Self::BINARY;
    pub const TIMELINE_KHR: Self = Self::TIMELINE;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerReductionMode(i32);
impl SamplerReductionMode {
    pub const WEIGHTED_AVERAGE: Self = Self(0);
    pub const MIN: Self = Self(1);
    pub const MAX: Self = Self(2);
    pub const WEIGHTED_AVERAGE_RANGECLAMP_QCOM: Self = Self(1000521000);
    pub const MAX_EXT: Self = Self::MAX;
    pub const MIN_EXT: Self = Self::MIN;
    pub const WEIGHTED_AVERAGE_EXT: Self = Self::WEIGHTED_AVERAGE;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DriverId(i32);
impl DriverId {
    pub const AMD_PROPRIETARY: Self = Self(1);
    pub const AMD_OPEN_SOURCE: Self = Self(2);
    pub const MESA_RADV: Self = Self(3);
    pub const NVIDIA_PROPRIETARY: Self = Self(4);
    pub const INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    pub const INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    pub const IMAGINATION_PROPRIETARY: Self = Self(7);
    pub const QUALCOMM_PROPRIETARY: Self = Self(8);
    pub const ARM_PROPRIETARY: Self = Self(9);
    pub const GOOGLE_SWIFTSHADER: Self = Self(10);
    pub const GGP_PROPRIETARY: Self = Self(11);
    pub const BROADCOM_PROPRIETARY: Self = Self(12);
    pub const MESA_LLVMPIPE: Self = Self(13);
    pub const MOLTENVK: Self = Self(14);
    pub const COREAVI_PROPRIETARY: Self = Self(15);
    pub const JUICE_PROPRIETARY: Self = Self(16);
    pub const VERISILICON_PROPRIETARY: Self = Self(17);
    pub const MESA_TURNIP: Self = Self(18);
    pub const MESA_V3DV: Self = Self(19);
    pub const MESA_PANVK: Self = Self(20);
    pub const SAMSUNG_PROPRIETARY: Self = Self(21);
    pub const MESA_VENUS: Self = Self(22);
    pub const MESA_DOZEN: Self = Self(23);
    pub const MESA_NVK: Self = Self(24);
    pub const IMAGINATION_OPEN_SOURCE_MESA: Self = Self(25);
    pub const MESA_HONEYKRISP: Self = Self(26);
    pub const VULKAN_SC_EMULATION_ON_VULKAN: Self = Self(27);
    pub const MESA_KOSMICKRISP: Self = Self(28);
    pub const AMD_OPEN_SOURCE_KHR: Self = Self::AMD_OPEN_SOURCE;
    pub const AMD_PROPRIETARY_KHR: Self = Self::AMD_PROPRIETARY;
    pub const ARM_PROPRIETARY_KHR: Self = Self::ARM_PROPRIETARY;
    pub const BROADCOM_PROPRIETARY_KHR: Self = Self::BROADCOM_PROPRIETARY;
    pub const GGP_PROPRIETARY_KHR: Self = Self::GGP_PROPRIETARY;
    pub const GOOGLE_SWIFTSHADER_KHR: Self = Self::GOOGLE_SWIFTSHADER;
    pub const IMAGINATION_PROPRIETARY_KHR: Self = Self::IMAGINATION_PROPRIETARY;
    pub const INTEL_OPEN_SOURCE_MESA_KHR: Self = Self::INTEL_OPEN_SOURCE_MESA;
    pub const INTEL_PROPRIETARY_WINDOWS_KHR: Self = Self::INTEL_PROPRIETARY_WINDOWS;
    pub const MESA_RADV_KHR: Self = Self::MESA_RADV;
    pub const NVIDIA_PROPRIETARY_KHR: Self = Self::NVIDIA_PROPRIETARY;
    pub const QUALCOMM_PROPRIETARY_KHR: Self = Self::QUALCOMM_PROPRIETARY;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderFloatControlsIndependence(i32);
impl ShaderFloatControlsIndependence {
    pub const _32_BIT_ONLY: Self = Self(0);
    pub const ALL: Self = Self(1);
    pub const NONE: Self = Self(2);
    pub const _32_BIT_ONLY_KHR: Self = Self::_32_BIT_ONLY;
    pub const ALL_KHR: Self = Self::ALL;
    pub const NONE_KHR: Self = Self::NONE;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SemaphoreWaitFlags: Flags {
        const ANY = SemaphoreWaitFlagBits::ANY.0;
        const ANY_KHR = Self::ANY.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreWaitFlagBits(u32);
impl SemaphoreWaitFlagBits {
    pub const ANY: Self = Self(1 << 0);
    pub const ANY_KHR: Self = Self::ANY;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DescriptorBindingFlags: Flags {
        const UPDATE_AFTER_BIND = DescriptorBindingFlagBits::UPDATE_AFTER_BIND.0;
        const UPDATE_UNUSED_WHILE_PENDING = DescriptorBindingFlagBits::UPDATE_UNUSED_WHILE_PENDING.0;
        const PARTIALLY_BOUND = DescriptorBindingFlagBits::PARTIALLY_BOUND.0;
        const VARIABLE_DESCRIPTOR_COUNT = DescriptorBindingFlagBits::VARIABLE_DESCRIPTOR_COUNT.0;
        const PARTIALLY_BOUND_EXT = Self::PARTIALLY_BOUND.bits();
        const UPDATE_AFTER_BIND_EXT = Self::UPDATE_AFTER_BIND.bits();
        const UPDATE_UNUSED_WHILE_PENDING_EXT = Self::UPDATE_UNUSED_WHILE_PENDING.bits();
        const VARIABLE_DESCRIPTOR_COUNT_EXT = Self::VARIABLE_DESCRIPTOR_COUNT.bits();
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
    pub const PARTIALLY_BOUND_EXT: Self = Self::PARTIALLY_BOUND;
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
    pub const UPDATE_UNUSED_WHILE_PENDING_EXT: Self = Self::UPDATE_UNUSED_WHILE_PENDING;
    pub const VARIABLE_DESCRIPTOR_COUNT_EXT: Self = Self::VARIABLE_DESCRIPTOR_COUNT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ResolveModeFlags: Flags {
        const SAMPLE_ZERO = ResolveModeFlagBits::SAMPLE_ZERO.0;
        const AVERAGE = ResolveModeFlagBits::AVERAGE.0;
        const MIN = ResolveModeFlagBits::MIN.0;
        const MAX = ResolveModeFlagBits::MAX.0;
        const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID = ResolveModeFlagBits::EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID.0;
        const CUSTOM_EXT = ResolveModeFlagBits::CUSTOM_EXT.0;
        const AVERAGE_KHR = Self::AVERAGE.bits();
        const MAX_KHR = Self::MAX.bits();
        const MIN_KHR = Self::MIN.bits();
        const NONE_KHR = Self::NONE.bits();
        const SAMPLE_ZERO_KHR = Self::SAMPLE_ZERO.bits();
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResolveModeFlagBits(u32);
impl ResolveModeFlagBits {
    pub const SAMPLE_ZERO: Self = Self(1 << 0);
    pub const AVERAGE: Self = Self(1 << 1);
    pub const MIN: Self = Self(1 << 2);
    pub const MAX: Self = Self(1 << 3);
    pub const EXTERNAL_FORMAT_DOWNSAMPLE_ANDROID: Self = Self(1 << 4);
    pub const CUSTOM_EXT: Self = Self(1 << 5);
    pub const AVERAGE_KHR: Self = Self::AVERAGE;
    pub const MAX_KHR: Self = Self::MAX;
    pub const MIN_KHR: Self = Self::MIN;
    pub const SAMPLE_ZERO_KHR: Self = Self::SAMPLE_ZERO;
}
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo2,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> Result;
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    p_subpass_begin_info: *const SubpassBeginInfo,
);
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_begin_info: *const SubpassBeginInfo,
    p_subpass_end_info: *const SubpassEndInfo,
);
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_end_info: *const SubpassEndInfo,
);
pub type PFN_vkGetSemaphoreCounterValue =
    unsafe extern "system" fn(device: Device, semaphore: Semaphore, p_value: *mut u64) -> Result;
pub type PFN_vkWaitSemaphores = unsafe extern "system" fn(
    device: Device,
    p_wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> Result;
pub type PFN_vkSignalSemaphore =
    unsafe extern "system" fn(device: Device, p_signal_info: *const SemaphoreSignalInfo) -> Result;
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
pub type PFN_vkGetBufferOpaqueCaptureAddress =
    unsafe extern "system" fn(device: Device, p_info: *const BufferDeviceAddressInfo) -> u64;
pub type PFN_vkGetBufferDeviceAddress = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferDeviceAddressInfo,
) -> DeviceAddress;
pub type PFN_vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;
