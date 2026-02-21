#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConformanceVersion {
    pub major: u8,
    pub minor: u8,
    pub subminor: u8,
    pub patch: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDriverProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub driver_id: DriverId,
    pub driver_name: [c_char; MAX_DRIVER_NAME_SIZE as usize],
    pub driver_info: [c_char; MAX_DRIVER_INFO_SIZE as usize],
    pub conformance_version: ConformanceVersion,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatListCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceHostQueryResetFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub host_query_reset: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub timeline_semaphore: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_timeline_semaphore_value_difference: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreTypeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore_type: SemaphoreType,
    pub initial_value: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TimelineSemaphoreSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_value_count: u32,
    pub p_wait_semaphore_values: *const u64,
    pub signal_semaphore_value_count: u32,
    pub p_signal_semaphore_values: *const u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreWaitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreWaitFlags,
    pub semaphore_count: u32,
    pub p_semaphores: *const Semaphore,
    pub p_values: *const u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreSignalInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferDeviceAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferOpaqueCaptureAddressCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_address: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryOpaqueCaptureAddressAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_address: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceMemoryOpaqueCaptureAddressInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreType(i32);
impl SemaphoreType {
    pub const BINARY: Self = Self(0);
    pub const TIMELINE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SemaphoreWaitFlags: Flags {
        const ANY = SemaphoreWaitFlagBits::ANY.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreWaitFlagBits(u32);
impl SemaphoreWaitFlagBits {
    pub const ANY: Self = Self(1 << 0);
}
pub type PFN_vkResetQueryPool = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
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
