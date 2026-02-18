#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub const FALSE: u32 = 0;
pub const LOD_CLAMP_NONE: f32 = 1000.0;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const REMAINING_ARRAY_LAYERS: u32 = !0;
pub const REMAINING_MIP_LEVELS: u32 = !0;
pub const TRUE: u32 = 1;
pub const WHOLE_SIZE: u64 = !0;
pub const MAX_MEMORY_TYPES: u32 = 32;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const UUID_SIZE: u32 = 16;
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
pub const MAX_MEMORY_HEAPS: u32 = 16;
pub type Bool32 = u32;
pub type Flags = u32;
pub type DeviceSize = u64;
pub type DeviceAddress = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Instance(usize);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevice(usize);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Device(usize);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Queue(usize);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBuffer(usize);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceMemory(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandPool(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Buffer(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Image(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageView(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Fence(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Semaphore(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPool(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseOutStructure {
    pub s_type: StructureType,
    pub p_next: *mut BaseOutStructure,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BaseInStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseInStructure,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [c_char; MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExtensionProperties {
    pub extension_name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub spec_version: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LayerProperties {
    pub layer_name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllocationCallbacks {
    pub p_user_data: *mut c_void,
    pub pfn_allocation: PFN_vkAllocationFunction,
    pub pfn_reallocation: PFN_vkReallocationFunction,
    pub pfn_free: PFN_vkFreeFunction,
    pub pfn_internal_allocation: PFN_vkInternalAllocationNotification,
    pub pfn_internal_free: PFN_vkInternalFreeNotification,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; MAX_MEMORY_TYPES as usize],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: DeviceSize,
    pub image_mip_tail_offset: DeviceSize,
    pub image_mip_tail_stride: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MappedMemoryRange {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryBarrier {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseBufferMemoryBindInfo {
    pub buffer: Buffer,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageOpaqueMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryBindInfo {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseImageMemoryBind,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const SparseBufferMemoryBindInfo,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo,
    pub image_bind_count: u32,
    pub p_image_binds: *const SparseImageMemoryBindInfo,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FenceCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float64: Bool32,
    pub shader_int64: Bool32,
    pub shader_int16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image2_d: Bool32,
    pub sparse_residency_image3_d: Bool32,
    pub sparse_residency2_samples: Bool32,
    pub sparse_residency4_samples: Bool32,
    pub sparse_residency8_samples: Bool32,
    pub sparse_residency16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard2_d_block_shape: Bool32,
    pub residency_standard2_d_multisample_block_shape: Bool32,
    pub residency_standard3_d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension1_d: u32,
    pub max_image_dimension2_d: u32,
    pub max_image_dimension3_d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageLayout(i32);
impl ImageLayout {
    pub const UNDEFINED: Self = Self(0);
    pub const GENERAL: Self = Self(1);
    pub const COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    pub const DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    pub const DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    pub const SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    pub const TRANSFER_SRC_OPTIMAL: Self = Self(6);
    pub const TRANSFER_DST_OPTIMAL: Self = Self(7);
    pub const PREINITIALIZED: Self = Self(8);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageType(i32);
impl ImageType {
    pub const _1D: Self = Self(0);
    pub const _2D: Self = Self(1);
    pub const _3D: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageTiling(i32);
impl ImageTiling {
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageViewType(i32);
impl ImageViewType {
    pub const _1D: Self = Self(0);
    pub const _2D: Self = Self(1);
    pub const _3D: Self = Self(2);
    pub const CUBE: Self = Self(3);
    pub const _1D_ARRAY: Self = Self(4);
    pub const _2D_ARRAY: Self = Self(5);
    pub const CUBE_ARRAY: Self = Self(6);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferLevel(i32);
impl CommandBufferLevel {
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentSwizzle(i32);
impl ComponentSwizzle {
    pub const IDENTITY: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const ONE: Self = Self(2);
    pub const R: Self = Self(3);
    pub const G: Self = Self(4);
    pub const B: Self = Self(5);
    pub const A: Self = Self(6);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryType(i32);
impl QueryType {
    pub const OCCLUSION: Self = Self(0);
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SharingMode(i32);
impl SharingMode {
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexType(i32);
impl IndexType {
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalAllocationType(i32);
impl InternalAllocationType {
    pub const EXECUTABLE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemAllocationScope(i32);
impl SystemAllocationScope {
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceType(i32);
impl PhysicalDeviceType {
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Format(i32);
impl Format {
    pub const UNDEFINED: Self = Self(0);
    pub const R4G4_UNORM_PACK8: Self = Self(1);
    pub const R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const R8_UNORM: Self = Self(9);
    pub const R8_SNORM: Self = Self(10);
    pub const R8_USCALED: Self = Self(11);
    pub const R8_SSCALED: Self = Self(12);
    pub const R8_UINT: Self = Self(13);
    pub const R8_SINT: Self = Self(14);
    pub const R8_SRGB: Self = Self(15);
    pub const R8G8_UNORM: Self = Self(16);
    pub const R8G8_SNORM: Self = Self(17);
    pub const R8G8_USCALED: Self = Self(18);
    pub const R8G8_SSCALED: Self = Self(19);
    pub const R8G8_UINT: Self = Self(20);
    pub const R8G8_SINT: Self = Self(21);
    pub const R8G8_SRGB: Self = Self(22);
    pub const R8G8B8_UNORM: Self = Self(23);
    pub const R8G8B8_SNORM: Self = Self(24);
    pub const R8G8B8_USCALED: Self = Self(25);
    pub const R8G8B8_SSCALED: Self = Self(26);
    pub const R8G8B8_UINT: Self = Self(27);
    pub const R8G8B8_SINT: Self = Self(28);
    pub const R8G8B8_SRGB: Self = Self(29);
    pub const B8G8R8_UNORM: Self = Self(30);
    pub const B8G8R8_SNORM: Self = Self(31);
    pub const B8G8R8_USCALED: Self = Self(32);
    pub const B8G8R8_SSCALED: Self = Self(33);
    pub const B8G8R8_UINT: Self = Self(34);
    pub const B8G8R8_SINT: Self = Self(35);
    pub const B8G8R8_SRGB: Self = Self(36);
    pub const R8G8B8A8_UNORM: Self = Self(37);
    pub const R8G8B8A8_SNORM: Self = Self(38);
    pub const R8G8B8A8_USCALED: Self = Self(39);
    pub const R8G8B8A8_SSCALED: Self = Self(40);
    pub const R8G8B8A8_UINT: Self = Self(41);
    pub const R8G8B8A8_SINT: Self = Self(42);
    pub const R8G8B8A8_SRGB: Self = Self(43);
    pub const B8G8R8A8_UNORM: Self = Self(44);
    pub const B8G8R8A8_SNORM: Self = Self(45);
    pub const B8G8R8A8_USCALED: Self = Self(46);
    pub const B8G8R8A8_SSCALED: Self = Self(47);
    pub const B8G8R8A8_UINT: Self = Self(48);
    pub const B8G8R8A8_SINT: Self = Self(49);
    pub const B8G8R8A8_SRGB: Self = Self(50);
    pub const A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const R16_UNORM: Self = Self(70);
    pub const R16_SNORM: Self = Self(71);
    pub const R16_USCALED: Self = Self(72);
    pub const R16_SSCALED: Self = Self(73);
    pub const R16_UINT: Self = Self(74);
    pub const R16_SINT: Self = Self(75);
    pub const R16_SFLOAT: Self = Self(76);
    pub const R16G16_UNORM: Self = Self(77);
    pub const R16G16_SNORM: Self = Self(78);
    pub const R16G16_USCALED: Self = Self(79);
    pub const R16G16_SSCALED: Self = Self(80);
    pub const R16G16_UINT: Self = Self(81);
    pub const R16G16_SINT: Self = Self(82);
    pub const R16G16_SFLOAT: Self = Self(83);
    pub const R16G16B16_UNORM: Self = Self(84);
    pub const R16G16B16_SNORM: Self = Self(85);
    pub const R16G16B16_USCALED: Self = Self(86);
    pub const R16G16B16_SSCALED: Self = Self(87);
    pub const R16G16B16_UINT: Self = Self(88);
    pub const R16G16B16_SINT: Self = Self(89);
    pub const R16G16B16_SFLOAT: Self = Self(90);
    pub const R16G16B16A16_UNORM: Self = Self(91);
    pub const R16G16B16A16_SNORM: Self = Self(92);
    pub const R16G16B16A16_USCALED: Self = Self(93);
    pub const R16G16B16A16_SSCALED: Self = Self(94);
    pub const R16G16B16A16_UINT: Self = Self(95);
    pub const R16G16B16A16_SINT: Self = Self(96);
    pub const R16G16B16A16_SFLOAT: Self = Self(97);
    pub const R32_UINT: Self = Self(98);
    pub const R32_SINT: Self = Self(99);
    pub const R32_SFLOAT: Self = Self(100);
    pub const R32G32_UINT: Self = Self(101);
    pub const R32G32_SINT: Self = Self(102);
    pub const R32G32_SFLOAT: Self = Self(103);
    pub const R32G32B32_UINT: Self = Self(104);
    pub const R32G32B32_SINT: Self = Self(105);
    pub const R32G32B32_SFLOAT: Self = Self(106);
    pub const R32G32B32A32_UINT: Self = Self(107);
    pub const R32G32B32A32_SINT: Self = Self(108);
    pub const R32G32B32A32_SFLOAT: Self = Self(109);
    pub const R64_UINT: Self = Self(110);
    pub const R64_SINT: Self = Self(111);
    pub const R64_SFLOAT: Self = Self(112);
    pub const R64G64_UINT: Self = Self(113);
    pub const R64G64_SINT: Self = Self(114);
    pub const R64G64_SFLOAT: Self = Self(115);
    pub const R64G64B64_UINT: Self = Self(116);
    pub const R64G64B64_SINT: Self = Self(117);
    pub const R64G64B64_SFLOAT: Self = Self(118);
    pub const R64G64B64A64_UINT: Self = Self(119);
    pub const R64G64B64A64_SINT: Self = Self(120);
    pub const R64G64B64A64_SFLOAT: Self = Self(121);
    pub const B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const D16_UNORM: Self = Self(124);
    pub const X8_D24_UNORM_PACK32: Self = Self(125);
    pub const D32_SFLOAT: Self = Self(126);
    pub const S8_UINT: Self = Self(127);
    pub const D16_UNORM_S8_UINT: Self = Self(128);
    pub const D24_UNORM_S8_UINT: Self = Self(129);
    pub const D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const BC2_UNORM_BLOCK: Self = Self(135);
    pub const BC2_SRGB_BLOCK: Self = Self(136);
    pub const BC3_UNORM_BLOCK: Self = Self(137);
    pub const BC3_SRGB_BLOCK: Self = Self(138);
    pub const BC4_UNORM_BLOCK: Self = Self(139);
    pub const BC4_SNORM_BLOCK: Self = Self(140);
    pub const BC5_UNORM_BLOCK: Self = Self(141);
    pub const BC5_SNORM_BLOCK: Self = Self(142);
    pub const BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const BC7_UNORM_BLOCK: Self = Self(145);
    pub const BC7_SRGB_BLOCK: Self = Self(146);
    pub const ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const ASTC_4X4_UNORM_BLOCK: Self = Self(157);
    pub const ASTC_4X4_SRGB_BLOCK: Self = Self(158);
    pub const ASTC_5X4_UNORM_BLOCK: Self = Self(159);
    pub const ASTC_5X4_SRGB_BLOCK: Self = Self(160);
    pub const ASTC_5X5_UNORM_BLOCK: Self = Self(161);
    pub const ASTC_5X5_SRGB_BLOCK: Self = Self(162);
    pub const ASTC_6X5_UNORM_BLOCK: Self = Self(163);
    pub const ASTC_6X5_SRGB_BLOCK: Self = Self(164);
    pub const ASTC_6X6_UNORM_BLOCK: Self = Self(165);
    pub const ASTC_6X6_SRGB_BLOCK: Self = Self(166);
    pub const ASTC_8X5_UNORM_BLOCK: Self = Self(167);
    pub const ASTC_8X5_SRGB_BLOCK: Self = Self(168);
    pub const ASTC_8X6_UNORM_BLOCK: Self = Self(169);
    pub const ASTC_8X6_SRGB_BLOCK: Self = Self(170);
    pub const ASTC_8X8_UNORM_BLOCK: Self = Self(171);
    pub const ASTC_8X8_SRGB_BLOCK: Self = Self(172);
    pub const ASTC_10X5_UNORM_BLOCK: Self = Self(173);
    pub const ASTC_10X5_SRGB_BLOCK: Self = Self(174);
    pub const ASTC_10X6_UNORM_BLOCK: Self = Self(175);
    pub const ASTC_10X6_SRGB_BLOCK: Self = Self(176);
    pub const ASTC_10X8_UNORM_BLOCK: Self = Self(177);
    pub const ASTC_10X8_SRGB_BLOCK: Self = Self(178);
    pub const ASTC_10X10_UNORM_BLOCK: Self = Self(179);
    pub const ASTC_10X10_SRGB_BLOCK: Self = Self(180);
    pub const ASTC_12X10_UNORM_BLOCK: Self = Self(181);
    pub const ASTC_12X10_SRGB_BLOCK: Self = Self(182);
    pub const ASTC_12X12_UNORM_BLOCK: Self = Self(183);
    pub const ASTC_12X12_SRGB_BLOCK: Self = Self(184);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StructureType(i32);
impl StructureType {
    pub const APPLICATION_INFO: Self = Self(0);
    pub const INSTANCE_CREATE_INFO: Self = Self(1);
    pub const DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const DEVICE_CREATE_INFO: Self = Self(3);
    pub const SUBMIT_INFO: Self = Self(4);
    pub const MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const BIND_SPARSE_INFO: Self = Self(7);
    pub const FENCE_CREATE_INFO: Self = Self(8);
    pub const SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const EVENT_CREATE_INFO: Self = Self(10);
    pub const QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const BUFFER_CREATE_INFO: Self = Self(12);
    pub const BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const IMAGE_CREATE_INFO: Self = Self(14);
    pub const IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const SAMPLER_CREATE_INFO: Self = Self(31);
    pub const DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const MEMORY_BARRIER: Self = Self(46);
    pub const LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    pub const LOADER_DEVICE_CREATE_INFO: Self = Self(48);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Result(i32);
impl Result {
    pub const SUCCESS: Self = Self(0);
    pub const NOT_READY: Self = Self(1);
    pub const TIMEOUT: Self = Self(2);
    pub const EVENT_SET: Self = Self(3);
    pub const EVENT_RESET: Self = Self(4);
    pub const INCOMPLETE: Self = Self(5);
    pub const ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
    pub const ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    pub const ERROR_DEVICE_LOST: Self = Self(-4);
    pub const ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    pub const ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    pub const ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    pub const ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const ERROR_FRAGMENTED_POOL: Self = Self(-12);
    pub const ERROR_UNKNOWN: Self = Self(-13);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectType(i32);
impl ObjectType {
    pub const UNKNOWN: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
    pub const PHYSICAL_DEVICE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const QUEUE: Self = Self(4);
    pub const SEMAPHORE: Self = Self(5);
    pub const COMMAND_BUFFER: Self = Self(6);
    pub const FENCE: Self = Self(7);
    pub const DEVICE_MEMORY: Self = Self(8);
    pub const BUFFER: Self = Self(9);
    pub const IMAGE: Self = Self(10);
    pub const EVENT: Self = Self(11);
    pub const QUERY_POOL: Self = Self(12);
    pub const BUFFER_VIEW: Self = Self(13);
    pub const IMAGE_VIEW: Self = Self(14);
    pub const SHADER_MODULE: Self = Self(15);
    pub const PIPELINE_CACHE: Self = Self(16);
    pub const PIPELINE_LAYOUT: Self = Self(17);
    pub const RENDER_PASS: Self = Self(18);
    pub const PIPELINE: Self = Self(19);
    pub const DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const SAMPLER: Self = Self(21);
    pub const DESCRIPTOR_POOL: Self = Self(22);
    pub const DESCRIPTOR_SET: Self = Self(23);
    pub const FRAMEBUFFER: Self = Self(24);
    pub const COMMAND_POOL: Self = Self(25);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VendorId(i32);
impl VendorId {
    pub const KHRONOS: Self = Self(0x10000);
    pub const VIV: Self = Self(0x10001);
    pub const VSI: Self = Self(0x10002);
    pub const KAZAN: Self = Self(0x10003);
    pub const CODEPLAY: Self = Self(0x10004);
    pub const MESA: Self = Self(0x10005);
    pub const POCL: Self = Self(0x10006);
    pub const MOBILEYE: Self = Self(0x10007);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct QueryPoolCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct InstanceCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DeviceCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DeviceQueueCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct QueueFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MemoryPropertyFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MemoryHeapFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccessFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct BufferUsageFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct BufferCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageUsageFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageViewCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FenceCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SemaphoreCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FormatFeatureFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct QueryControlFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct QueryResultFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct CommandPoolCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct CommandPoolResetFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct CommandBufferResetFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct CommandBufferUsageFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MemoryMapFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ImageAspectFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SparseMemoryBindFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SparseImageFormatFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineStageFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SampleCountFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DependencyFlags: Flags {
    }
}
pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    allocation_scope: SystemAllocationScope,
);
pub type PFN_vkReallocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    p_original: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut c_void;
pub type PFN_vkAllocationFunction = unsafe extern "system" fn(
    p_user_data: *mut c_void,
    size: usize,
    alignment: usize,
    allocation_scope: SystemAllocationScope,
) -> *mut c_void;
pub type PFN_vkFreeFunction =
    unsafe extern "system" fn(p_user_data: *mut c_void, p_memory: *mut c_void);
pub type PFN_vkVoidFunction = unsafe extern "system" fn();
pub type PFN_vkCreateInstance = unsafe extern "system" fn(
    p_create_info: *const InstanceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_instance: *mut Instance,
) -> Result;
pub type PFN_vkDestroyInstance =
    unsafe extern "system" fn(instance: Instance, p_allocator: *const AllocationCallbacks);
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_count: *mut u32,
    p_physical_devices: *mut PhysicalDevice,
) -> Result;
pub type PFN_vkGetDeviceProcAddr =
    unsafe extern "system" fn(device: Device, p_name: *const c_char) -> PFN_vkVoidFunction;
pub type PFN_vkGetInstanceProcAddr =
    unsafe extern "system" fn(instance: Instance, p_name: *const c_char) -> PFN_vkVoidFunction;
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties,
);
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties,
);
pub type PFN_vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures,
);
pub type PFN_vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties,
);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    p_image_format_properties: *mut ImageFormatProperties,
) -> Result;
pub type PFN_vkCreateDevice = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_create_info: *const DeviceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_device: *mut Device,
) -> Result;
pub type PFN_vkDestroyDevice =
    unsafe extern "system" fn(device: Device, p_allocator: *const AllocationCallbacks);
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result;
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result;
pub type PFN_vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_property_count: *mut u32,
    p_properties: *mut LayerProperties,
) -> Result;
pub type PFN_vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_layer_name: *const c_char,
    p_property_count: *mut u32,
    p_properties: *mut ExtensionProperties,
) -> Result;
pub type PFN_vkGetDeviceQueue = unsafe extern "system" fn(
    device: Device,
    queue_family_index: u32,
    queue_index: u32,
    p_queue: *mut Queue,
);
pub type PFN_vkQueueSubmit = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo,
    fence: Fence,
) -> Result;
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: Queue) -> Result;
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: Device) -> Result;
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const MemoryAllocateInfo,
    p_allocator: *const AllocationCallbacks,
    p_memory: *mut DeviceMemory,
) -> Result;
pub type PFN_vkFreeMemory = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkMapMemory = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    pp_data: *mut *mut c_void,
) -> Result;
pub type PFN_vkUnmapMemory = unsafe extern "system" fn(device: Device, memory: DeviceMemory);
pub type PFN_vkFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> Result;
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange,
) -> Result;
pub type PFN_vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    p_committed_memory_in_bytes: *mut DeviceSize,
);
pub type PFN_vkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    buffer: Buffer,
    p_memory_requirements: *mut MemoryRequirements,
);
pub type PFN_vkBindBufferMemory = unsafe extern "system" fn(
    device: Device,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;
pub type PFN_vkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_memory_requirements: *mut MemoryRequirements,
);
pub type PFN_vkBindImageMemory = unsafe extern "system" fn(
    device: Device,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;
pub type PFN_vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements,
);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    ty: ImageType,
    samples: SampleCountFlags,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties,
);
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
    queue: Queue,
    bind_info_count: u32,
    p_bind_info: *const BindSparseInfo,
    fence: Fence,
) -> Result;
pub type PFN_vkCreateFence = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const FenceCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> Result;
pub type PFN_vkDestroyFence = unsafe extern "system" fn(
    device: Device,
    fence: Fence,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkResetFences =
    unsafe extern "system" fn(device: Device, fence_count: u32, p_fences: *const Fence) -> Result;
pub type PFN_vkGetFenceStatus = unsafe extern "system" fn(device: Device, fence: Fence) -> Result;
pub type PFN_vkWaitForFences = unsafe extern "system" fn(
    device: Device,
    fence_count: u32,
    p_fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> Result;
pub type PFN_vkCreateSemaphore = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SemaphoreCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_semaphore: *mut Semaphore,
) -> Result;
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
    device: Device,
    semaphore: Semaphore,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const QueryPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_query_pool: *mut QueryPool,
) -> Result;
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetQueryPoolResults = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    p_data: *mut c_void,
    stride: DeviceSize,
    flags: QueryResultFlags,
) -> Result;
pub type PFN_vkCreateBuffer = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_buffer: *mut Buffer,
) -> Result;
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
    device: Device,
    buffer: Buffer,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateImage = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_image: *mut Image,
) -> Result;
pub type PFN_vkDestroyImage = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource,
    p_layout: *mut SubresourceLayout,
);
pub type PFN_vkCreateImageView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut ImageView,
) -> Result;
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CommandPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_command_pool: *mut CommandPool,
) -> Result;
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> Result;
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const CommandBufferAllocateInfo,
    p_command_buffers: *mut CommandBuffer,
) -> Result;
pub type PFN_vkFreeCommandBuffers = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
pub type PFN_vkBeginCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const CommandBufferBeginInfo,
) -> Result;
pub type PFN_vkEndCommandBuffer =
    unsafe extern "system" fn(command_buffer: CommandBuffer) -> Result;
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    flags: CommandBufferResetFlags,
) -> Result;
pub type PFN_vkCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferCopy,
);
pub type PFN_vkCmdCopyImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageCopy,
);
pub type PFN_vkCmdCopyBufferToImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);
pub type PFN_vkCmdCopyImageToBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    p_regions: *const BufferImageCopy,
);
pub type PFN_vkCmdUpdateBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    p_data: *const c_void,
);
pub type PFN_vkCmdFillBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
);
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
);
pub type PFN_vkCmdBeginQuery = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
);
pub type PFN_vkCmdEndQuery =
    unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32);
pub type PFN_vkCmdResetQueryPool = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);
pub type PFN_vkCmdWriteTimestamp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlags,
    query_pool: QueryPool,
    query: u32,
);
pub type PFN_vkCmdCopyQueryPoolResults = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
);
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
