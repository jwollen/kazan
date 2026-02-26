#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
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
pub const ATTACHMENT_UNUSED: u32 = !0;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub type SampleMask = u32;
pub type Bool32 = u32;
pub type Flags = u32;
pub type DeviceSize = u64;
pub type DeviceAddress = u64;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Instance(usize);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PhysicalDevice(usize);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Device(usize);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Queue(usize);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CommandBuffer(usize);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DeviceMemory(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct CommandPool(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Buffer(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct BufferView(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Image(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ImageView(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ShaderModule(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Pipeline(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PipelineLayout(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Sampler(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DescriptorSet(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DescriptorSetLayout(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DescriptorPool(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Fence(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Semaphore(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Event(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct QueryPool(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Framebuffer(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct RenderPass(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PipelineCache(u64);
#[repr(C)]
pub struct BaseOutStructure {
    pub s_type: StructureType,
    pub p_next: *mut BaseOutStructure,
}
#[repr(C)]
pub struct BaseInStructure {
    pub s_type: StructureType,
    pub p_next: *const BaseInStructure,
}
#[repr(C)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
#[repr(C)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}
#[repr(C)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}
#[repr(C)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
#[repr(C)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}
#[repr(C)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}
#[repr(C)]
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
pub struct ExtensionProperties {
    pub extension_name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub spec_version: u32,
}
#[repr(C)]
pub struct LayerProperties {
    pub layer_name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
}
#[repr(C)]
pub struct ApplicationInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_application_name: *const c_char,
    pub application_version: u32,
    pub p_engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AllocationCallbacks<'a> {
    pub p_user_data: *mut c_void,
    pub pfn_allocation: Option<PFN_vkAllocationFunction>,
    pub pfn_reallocation: Option<PFN_vkReallocationFunction>,
    pub pfn_free: Option<PFN_vkFreeFunction>,
    pub pfn_internal_allocation: Option<PFN_vkInternalAllocationNotification>,
    pub pfn_internal_free: Option<PFN_vkInternalFreeNotification>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceQueueCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub p_queue_priorities: *const f32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub p_queue_create_infos: *const DeviceQueueCreateInfo<'a>,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
    pub p_enabled_features: *const PhysicalDeviceFeatures,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct InstanceCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub p_application_info: *const ApplicationInfo<'a>,
    pub enabled_layer_count: u32,
    pub pp_enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub pp_enabled_extension_names: *const *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; MAX_MEMORY_TYPES as usize],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS as usize],
}
#[repr(C)]
pub struct MemoryAllocateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}
#[repr(C)]
pub struct SparseImageFormatProperties {
    pub aspect_mask: ImageAspectFlags,
    pub image_granularity: Extent3D,
    pub flags: SparseImageFormatFlags,
}
#[repr(C)]
pub struct SparseImageMemoryRequirements {
    pub format_properties: SparseImageFormatProperties,
    pub image_mip_tail_first_lod: u32,
    pub image_mip_tail_size: DeviceSize,
    pub image_mip_tail_offset: DeviceSize,
    pub image_mip_tail_stride: DeviceSize,
}
#[repr(C)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}
#[repr(C)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}
#[repr(C)]
pub struct MappedMemoryRange<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct FormatProperties {
    pub linear_tiling_features: FormatFeatureFlags,
    pub optimal_tiling_features: FormatFeatureFlags,
    pub buffer_features: FormatFeatureFlags,
}
#[repr(C)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}
#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}
#[repr(C)]
pub struct WriteDescriptorSet<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub p_image_info: *const DescriptorImageInfo,
    pub p_buffer_info: *const DescriptorBufferInfo,
    pub p_texel_buffer_view: *const BufferView,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CopyDescriptorSet<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferViewCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}
#[repr(C)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}
#[repr(C)]
pub struct MemoryBarrier<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferMemoryBarrier<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageMemoryBarrier<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}
#[repr(C)]
pub struct ImageViewCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
pub struct SparseMemoryBind {
    pub resource_offset: DeviceSize,
    pub size: DeviceSize,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
pub struct SparseImageMemoryBind {
    pub subresource: ImageSubresource,
    pub offset: Offset3D,
    pub extent: Extent3D,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub flags: SparseMemoryBindFlags,
}
#[repr(C)]
pub struct SparseBufferMemoryBindInfo<'a> {
    pub buffer: Buffer,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SparseImageOpaqueMemoryBindInfo<'a> {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseMemoryBind,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SparseImageMemoryBindInfo<'a> {
    pub image: Image,
    pub bind_count: u32,
    pub p_binds: *const SparseImageMemoryBind,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BindSparseInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub buffer_bind_count: u32,
    pub p_buffer_binds: *const SparseBufferMemoryBindInfo<'a>,
    pub image_opaque_bind_count: u32,
    pub p_image_opaque_binds: *const SparseImageOpaqueMemoryBindInfo<'a>,
    pub image_bind_count: u32,
    pub p_image_binds: *const SparseImageMemoryBindInfo<'a>,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
}
#[repr(C)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
pub struct ImageResolve {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
pub struct ShaderModuleCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DescriptorSetLayoutBinding<'a> {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const DescriptorSetLayoutBinding<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DescriptorPoolSize {
    pub ty: DescriptorType,
    pub descriptor_count: u32,
}
#[repr(C)]
pub struct DescriptorPoolCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const DescriptorPoolSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DescriptorSetAllocateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}
#[repr(C)]
pub struct SpecializationInfo<'a> {
    pub map_entry_count: u32,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineShaderStageCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlagBits,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ComputePipelineCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo<'a>,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}
#[repr(C)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}
#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub p_vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub p_vertex_attribute_descriptions: *const VertexInputAttributeDescription,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineTessellationStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineViewportStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub p_viewports: *const Viewport,
    pub scissor_count: u32,
    pub p_scissors: *const Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: Bool32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlagBits,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: f32,
    pub p_sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}
#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: u32,
    pub p_attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineDynamicStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub p_dynamic_states: *const DynamicState,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}
#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: Bool32,
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct GraphicsPipelineCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub p_stages: *const PipelineShaderStageCreateInfo<'a>,
    pub p_vertex_input_state: *const PipelineVertexInputStateCreateInfo<'a>,
    pub p_input_assembly_state: *const PipelineInputAssemblyStateCreateInfo<'a>,
    pub p_tessellation_state: *const PipelineTessellationStateCreateInfo<'a>,
    pub p_viewport_state: *const PipelineViewportStateCreateInfo<'a>,
    pub p_rasterization_state: *const PipelineRasterizationStateCreateInfo<'a>,
    pub p_multisample_state: *const PipelineMultisampleStateCreateInfo<'a>,
    pub p_depth_stencil_state: *const PipelineDepthStencilStateCreateInfo<'a>,
    pub p_color_blend_state: *const PipelineColorBlendStateCreateInfo<'a>,
    pub p_dynamic_state: *const PipelineDynamicStateCreateInfo<'a>,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineCacheCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineCacheHeaderVersionOne {
    pub header_size: u32,
    pub header_version: PipelineCacheHeaderVersion,
    pub vendor_id: u32,
    pub device_id: u32,
    pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
}
#[repr(C)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}
#[repr(C)]
pub struct PipelineLayoutCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SamplerCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CommandPoolCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CommandBufferAllocateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CommandBufferInheritanceInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CommandBufferBeginInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub p_inheritance_info: *const CommandBufferInheritanceInfo<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct RenderPassBeginInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: u32,
    pub p_clear_values: *const ClearValue,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}
#[repr(C)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}
#[repr(C)]
pub struct AttachmentDescription {
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
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}
#[repr(C)]
pub struct SubpassDescription<'a> {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference,
    pub p_resolve_attachments: *const AttachmentReference,
    pub p_depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}
#[repr(C)]
pub struct RenderPassCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription<'a>,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct EventCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: EventCreateFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct FenceCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FenceCreateFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
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
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard2_d_block_shape: Bool32,
    pub residency_standard2_d_multisample_block_shape: Bool32,
    pub residency_standard3_d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}
#[repr(C)]
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
pub struct SemaphoreCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SemaphoreCreateFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct QueryPoolCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct FramebufferCreateInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}
#[repr(C)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}
#[repr(C)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
#[repr(C)]
pub struct SubmitInfo<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphores: *const Semaphore,
    pub p_wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub p_command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphores: *const Semaphore,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}
impl Default for ClearColorValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}
impl Default for ClearValue {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT: Self = Self(1000339000);
    pub const ATTACHMENT_OPTIMAL: Self = Self(1000314001);
    pub const DEPTH_ATTACHMENT_OPTIMAL: Self = Self(1000241000);
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000117001);
    pub const DEPTH_READ_ONLY_OPTIMAL: Self = Self(1000241001);
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000117000);
    pub const FRAGMENT_DENSITY_MAP_OPTIMAL_EXT: Self = Self(1000218000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: Self = Self(1000164003);
    pub const PRESENT_SRC_KHR: Self = Self(1000001002);
    pub const READ_ONLY_OPTIMAL: Self = Self(1000314000);
    pub const RENDERING_LOCAL_READ: Self = Self(1000232000);
    pub const SHARED_PRESENT_KHR: Self = Self(1000111000);
    pub const STENCIL_ATTACHMENT_OPTIMAL: Self = Self(1000241002);
    pub const STENCIL_READ_ONLY_OPTIMAL: Self = Self(1000241003);
    pub const TENSOR_ALIASING_ARM: Self = Self(1000460000);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1000024002);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1000024001);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1000299002);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_KHR: Self = Self(1000553000);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1000299001);
    pub const ZERO_INITIALIZED_EXT: Self = Self(1000620000);
    pub const ATTACHMENT_OPTIMAL_KHR: Self = Self::ATTACHMENT_OPTIMAL;
    pub const DEPTH_ATTACHMENT_OPTIMAL_KHR: Self = Self::DEPTH_ATTACHMENT_OPTIMAL;
    pub const DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR: Self =
        Self::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL;
    pub const DEPTH_READ_ONLY_OPTIMAL_KHR: Self = Self::DEPTH_READ_ONLY_OPTIMAL;
    pub const DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR: Self =
        Self::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL;
    pub const READ_ONLY_OPTIMAL_KHR: Self = Self::READ_ONLY_OPTIMAL;
    pub const RENDERING_LOCAL_READ_KHR: Self = Self::RENDERING_LOCAL_READ;
    pub const SHADING_RATE_OPTIMAL_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR;
    pub const STENCIL_ATTACHMENT_OPTIMAL_KHR: Self = Self::STENCIL_ATTACHMENT_OPTIMAL;
    pub const STENCIL_READ_ONLY_OPTIMAL_KHR: Self = Self::STENCIL_READ_ONLY_OPTIMAL;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentLoadOp(i32);
impl AttachmentLoadOp {
    pub const LOAD: Self = Self(0);
    pub const CLEAR: Self = Self(1);
    pub const DONT_CARE: Self = Self(2);
    pub const NONE: Self = Self(1000400000);
    pub const NONE_EXT: Self = Self::NONE;
    pub const NONE_KHR: Self = Self::NONE;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentStoreOp(i32);
impl AttachmentStoreOp {
    pub const STORE: Self = Self(0);
    pub const DONT_CARE: Self = Self(1);
    pub const NONE: Self = Self(1000301000);
    pub const NONE_EXT: Self = Self::NONE;
    pub const NONE_KHR: Self = Self::NONE;
    pub const NONE_QCOM: Self = Self::NONE;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageType(i32);
impl ImageType {
    pub const _1D: Self = Self(0);
    pub const _2D: Self = Self(1);
    pub const _3D: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageTiling(i32);
impl ImageTiling {
    pub const OPTIMAL: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const DRM_FORMAT_MODIFIER_EXT: Self = Self(1000158000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferLevel(i32);
impl CommandBufferLevel {
    pub const PRIMARY: Self = Self(0);
    pub const SECONDARY: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorType(i32);
impl DescriptorType {
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const BLOCK_MATCH_IMAGE_QCOM: Self = Self(1000440001);
    pub const INLINE_UNIFORM_BLOCK: Self = Self(1000138000);
    pub const MUTABLE_EXT: Self = Self(1000351000);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570000);
    pub const SAMPLE_WEIGHT_IMAGE_QCOM: Self = Self(1000440000);
    pub const TENSOR_ARM: Self = Self(1000460000);
    pub const INLINE_UNIFORM_BLOCK_EXT: Self = Self::INLINE_UNIFORM_BLOCK;
    pub const MUTABLE_VALVE: Self = Self::MUTABLE_EXT;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryType(i32);
impl QueryType {
    pub const OCCLUSION: Self = Self(0);
    pub const PIPELINE_STATISTICS: Self = Self(1);
    pub const TIMESTAMP: Self = Self(2);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV: Self = Self(1000165000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR: Self =
        Self(1000386000);
    pub const ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR: Self = Self(1000150001);
    pub const ACCELERATION_STRUCTURE_SIZE_KHR: Self = Self(1000386001);
    pub const MESH_PRIMITIVES_GENERATED_EXT: Self = Self(1000328000);
    pub const MICROMAP_COMPACTED_SIZE_EXT: Self = Self(1000396001);
    pub const MICROMAP_SERIALIZATION_SIZE_EXT: Self = Self(1000396000);
    pub const PERFORMANCE_QUERY_INTEL: Self = Self(1000210000);
    pub const PERFORMANCE_QUERY_KHR: Self = Self(1000116000);
    pub const PRIMITIVES_GENERATED_EXT: Self = Self(1000382000);
    pub const RESULT_STATUS_ONLY_KHR: Self = Self(1000023000);
    pub const TRANSFORM_FEEDBACK_STREAM_EXT: Self = Self(1000028004);
    pub const VIDEO_ENCODE_FEEDBACK_KHR: Self = Self(1000299000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BorderColor(i32);
impl BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
    pub const FLOAT_CUSTOM_EXT: Self = Self(1000287003);
    pub const INT_CUSTOM_EXT: Self = Self(1000287004);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineBindPoint(i32);
impl PipelineBindPoint {
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
    pub const DATA_GRAPH_ARM: Self = Self(1000507000);
    pub const EXECUTION_GRAPH_AMDX: Self = Self(1000134000);
    pub const RAY_TRACING_KHR: Self = Self(1000165000);
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1000369003);
    pub const RAY_TRACING_NV: Self = Self::RAY_TRACING_KHR;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCacheHeaderVersion(i32);
impl PipelineCacheHeaderVersion {
    pub const ONE: Self = Self(1);
    pub const DATA_GRAPH_QCOM: Self = Self(1000629000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimitiveTopology(i32);
impl PrimitiveTopology {
    pub const POINT_LIST: Self = Self(0);
    pub const LINE_LIST: Self = Self(1);
    pub const LINE_STRIP: Self = Self(2);
    pub const TRIANGLE_LIST: Self = Self(3);
    pub const TRIANGLE_STRIP: Self = Self(4);
    pub const TRIANGLE_FAN: Self = Self(5);
    pub const LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const PATCH_LIST: Self = Self(10);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SharingMode(i32);
impl SharingMode {
    pub const EXCLUSIVE: Self = Self(0);
    pub const CONCURRENT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IndexType(i32);
impl IndexType {
    pub const UINT16: Self = Self(0);
    pub const UINT32: Self = Self(1);
    pub const NONE_KHR: Self = Self(1000165000);
    pub const UINT8: Self = Self(1000265000);
    pub const NONE_NV: Self = Self::NONE_KHR;
    pub const UINT8_EXT: Self = Self::UINT8;
    pub const UINT8_KHR: Self = Self::UINT8;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Filter(i32);
impl Filter {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
    pub const CUBIC_EXT: Self = Self(1000015000);
    pub const CUBIC_IMG: Self = Self::CUBIC_EXT;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerMipmapMode(i32);
impl SamplerMipmapMode {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerAddressMode(i32);
impl SamplerAddressMode {
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
    pub const MIRROR_CLAMP_TO_EDGE: Self = Self(4);
    pub const MIRROR_CLAMP_TO_EDGE_KHR: Self = Self::MIRROR_CLAMP_TO_EDGE;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CompareOp(i32);
impl CompareOp {
    pub const NEVER: Self = Self(0);
    pub const LESS: Self = Self(1);
    pub const EQUAL: Self = Self(2);
    pub const LESS_OR_EQUAL: Self = Self(3);
    pub const GREATER: Self = Self(4);
    pub const NOT_EQUAL: Self = Self(5);
    pub const GREATER_OR_EQUAL: Self = Self(6);
    pub const ALWAYS: Self = Self(7);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolygonMode(i32);
impl PolygonMode {
    pub const FILL: Self = Self(0);
    pub const LINE: Self = Self(1);
    pub const POINT: Self = Self(2);
    pub const FILL_RECTANGLE_NV: Self = Self(1000153000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrontFace(i32);
impl FrontFace {
    pub const COUNTER_CLOCKWISE: Self = Self(0);
    pub const CLOCKWISE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendFactor(i32);
impl BlendFactor {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(1);
    pub const SRC_COLOR: Self = Self(2);
    pub const ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const DST_COLOR: Self = Self(4);
    pub const ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const SRC_ALPHA: Self = Self(6);
    pub const ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const DST_ALPHA: Self = Self(8);
    pub const ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const CONSTANT_COLOR: Self = Self(10);
    pub const ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const CONSTANT_ALPHA: Self = Self(12);
    pub const ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const SRC_ALPHA_SATURATE: Self = Self(14);
    pub const SRC1_COLOR: Self = Self(15);
    pub const ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const SRC1_ALPHA: Self = Self(17);
    pub const ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlendOp(i32);
impl BlendOp {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const REVERSE_SUBTRACT: Self = Self(2);
    pub const MIN: Self = Self(3);
    pub const MAX: Self = Self(4);
    pub const BLUE_EXT: Self = Self(1000148045);
    pub const COLORBURN_EXT: Self = Self(1000148018);
    pub const COLORDODGE_EXT: Self = Self(1000148017);
    pub const CONTRAST_EXT: Self = Self(1000148041);
    pub const DARKEN_EXT: Self = Self(1000148015);
    pub const DIFFERENCE_EXT: Self = Self(1000148021);
    pub const DST_ATOP_EXT: Self = Self(1000148010);
    pub const DST_EXT: Self = Self(1000148002);
    pub const DST_IN_EXT: Self = Self(1000148006);
    pub const DST_OUT_EXT: Self = Self(1000148008);
    pub const DST_OVER_EXT: Self = Self(1000148004);
    pub const EXCLUSION_EXT: Self = Self(1000148022);
    pub const GREEN_EXT: Self = Self(1000148044);
    pub const HARDLIGHT_EXT: Self = Self(1000148019);
    pub const HARDMIX_EXT: Self = Self(1000148030);
    pub const HSL_COLOR_EXT: Self = Self(1000148033);
    pub const HSL_HUE_EXT: Self = Self(1000148031);
    pub const HSL_LUMINOSITY_EXT: Self = Self(1000148034);
    pub const HSL_SATURATION_EXT: Self = Self(1000148032);
    pub const INVERT_EXT: Self = Self(1000148023);
    pub const INVERT_OVG_EXT: Self = Self(1000148042);
    pub const INVERT_RGB_EXT: Self = Self(1000148024);
    pub const LIGHTEN_EXT: Self = Self(1000148016);
    pub const LINEARBURN_EXT: Self = Self(1000148026);
    pub const LINEARDODGE_EXT: Self = Self(1000148025);
    pub const LINEARLIGHT_EXT: Self = Self(1000148028);
    pub const MINUS_CLAMPED_EXT: Self = Self(1000148040);
    pub const MINUS_EXT: Self = Self(1000148039);
    pub const MULTIPLY_EXT: Self = Self(1000148012);
    pub const OVERLAY_EXT: Self = Self(1000148014);
    pub const PINLIGHT_EXT: Self = Self(1000148029);
    pub const PLUS_CLAMPED_ALPHA_EXT: Self = Self(1000148037);
    pub const PLUS_CLAMPED_EXT: Self = Self(1000148036);
    pub const PLUS_DARKER_EXT: Self = Self(1000148038);
    pub const PLUS_EXT: Self = Self(1000148035);
    pub const RED_EXT: Self = Self(1000148043);
    pub const SCREEN_EXT: Self = Self(1000148013);
    pub const SOFTLIGHT_EXT: Self = Self(1000148020);
    pub const SRC_ATOP_EXT: Self = Self(1000148009);
    pub const SRC_EXT: Self = Self(1000148001);
    pub const SRC_IN_EXT: Self = Self(1000148005);
    pub const SRC_OUT_EXT: Self = Self(1000148007);
    pub const SRC_OVER_EXT: Self = Self(1000148003);
    pub const VIVIDLIGHT_EXT: Self = Self(1000148027);
    pub const XOR_EXT: Self = Self(1000148011);
    pub const ZERO_EXT: Self = Self(1000148000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilOp(i32);
impl StencilOp {
    pub const KEEP: Self = Self(0);
    pub const ZERO: Self = Self(1);
    pub const REPLACE: Self = Self(2);
    pub const INCREMENT_AND_CLAMP: Self = Self(3);
    pub const DECREMENT_AND_CLAMP: Self = Self(4);
    pub const INVERT: Self = Self(5);
    pub const INCREMENT_AND_WRAP: Self = Self(6);
    pub const DECREMENT_AND_WRAP: Self = Self(7);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LogicOp(i32);
impl LogicOp {
    pub const CLEAR: Self = Self(0);
    pub const AND: Self = Self(1);
    pub const AND_REVERSE: Self = Self(2);
    pub const COPY: Self = Self(3);
    pub const AND_INVERTED: Self = Self(4);
    pub const NO_OP: Self = Self(5);
    pub const XOR: Self = Self(6);
    pub const OR: Self = Self(7);
    pub const NOR: Self = Self(8);
    pub const EQUIVALENT: Self = Self(9);
    pub const INVERT: Self = Self(10);
    pub const OR_REVERSE: Self = Self(11);
    pub const COPY_INVERTED: Self = Self(12);
    pub const OR_INVERTED: Self = Self(13);
    pub const NAND: Self = Self(14);
    pub const SET: Self = Self(15);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InternalAllocationType(i32);
impl InternalAllocationType {
    pub const EXECUTABLE: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SystemAllocationScope(i32);
impl SystemAllocationScope {
    pub const COMMAND: Self = Self(0);
    pub const OBJECT: Self = Self(1);
    pub const CACHE: Self = Self(2);
    pub const DEVICE: Self = Self(3);
    pub const INSTANCE: Self = Self(4);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceType(i32);
impl PhysicalDeviceType {
    pub const OTHER: Self = Self(0);
    pub const INTEGRATED_GPU: Self = Self(1);
    pub const DISCRETE_GPU: Self = Self(2);
    pub const VIRTUAL_GPU: Self = Self(3);
    pub const CPU: Self = Self(4);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VertexInputRate(i32);
impl VertexInputRate {
    pub const VERTEX: Self = Self(0);
    pub const INSTANCE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const A1B5G5R5_UNORM_PACK16: Self = Self(1000470000);
    pub const A4B4G4R4_UNORM_PACK16: Self = Self(1000340001);
    pub const A4R4G4B4_UNORM_PACK16: Self = Self(1000340000);
    pub const A8_UNORM: Self = Self(1000470001);
    pub const ASTC_10X10_SFLOAT_BLOCK: Self = Self(1000066011);
    pub const ASTC_10X5_SFLOAT_BLOCK: Self = Self(1000066008);
    pub const ASTC_10X6_SFLOAT_BLOCK: Self = Self(1000066009);
    pub const ASTC_10X8_SFLOAT_BLOCK: Self = Self(1000066010);
    pub const ASTC_12X10_SFLOAT_BLOCK: Self = Self(1000066012);
    pub const ASTC_12X12_SFLOAT_BLOCK: Self = Self(1000066013);
    pub const ASTC_3X3X3_SFLOAT_BLOCK_EXT: Self = Self(1000288002);
    pub const ASTC_3X3X3_SRGB_BLOCK_EXT: Self = Self(1000288001);
    pub const ASTC_3X3X3_UNORM_BLOCK_EXT: Self = Self(1000288000);
    pub const ASTC_4X3X3_SFLOAT_BLOCK_EXT: Self = Self(1000288005);
    pub const ASTC_4X3X3_SRGB_BLOCK_EXT: Self = Self(1000288004);
    pub const ASTC_4X3X3_UNORM_BLOCK_EXT: Self = Self(1000288003);
    pub const ASTC_4X4_SFLOAT_BLOCK: Self = Self(1000066000);
    pub const ASTC_4X4X3_SFLOAT_BLOCK_EXT: Self = Self(1000288008);
    pub const ASTC_4X4X3_SRGB_BLOCK_EXT: Self = Self(1000288007);
    pub const ASTC_4X4X3_UNORM_BLOCK_EXT: Self = Self(1000288006);
    pub const ASTC_4X4X4_SFLOAT_BLOCK_EXT: Self = Self(1000288011);
    pub const ASTC_4X4X4_SRGB_BLOCK_EXT: Self = Self(1000288010);
    pub const ASTC_4X4X4_UNORM_BLOCK_EXT: Self = Self(1000288009);
    pub const ASTC_5X4_SFLOAT_BLOCK: Self = Self(1000066001);
    pub const ASTC_5X4X4_SFLOAT_BLOCK_EXT: Self = Self(1000288014);
    pub const ASTC_5X4X4_SRGB_BLOCK_EXT: Self = Self(1000288013);
    pub const ASTC_5X4X4_UNORM_BLOCK_EXT: Self = Self(1000288012);
    pub const ASTC_5X5_SFLOAT_BLOCK: Self = Self(1000066002);
    pub const ASTC_5X5X4_SFLOAT_BLOCK_EXT: Self = Self(1000288017);
    pub const ASTC_5X5X4_SRGB_BLOCK_EXT: Self = Self(1000288016);
    pub const ASTC_5X5X4_UNORM_BLOCK_EXT: Self = Self(1000288015);
    pub const ASTC_5X5X5_SFLOAT_BLOCK_EXT: Self = Self(1000288020);
    pub const ASTC_5X5X5_SRGB_BLOCK_EXT: Self = Self(1000288019);
    pub const ASTC_5X5X5_UNORM_BLOCK_EXT: Self = Self(1000288018);
    pub const ASTC_6X5_SFLOAT_BLOCK: Self = Self(1000066003);
    pub const ASTC_6X5X5_SFLOAT_BLOCK_EXT: Self = Self(1000288023);
    pub const ASTC_6X5X5_SRGB_BLOCK_EXT: Self = Self(1000288022);
    pub const ASTC_6X5X5_UNORM_BLOCK_EXT: Self = Self(1000288021);
    pub const ASTC_6X6_SFLOAT_BLOCK: Self = Self(1000066004);
    pub const ASTC_6X6X5_SFLOAT_BLOCK_EXT: Self = Self(1000288026);
    pub const ASTC_6X6X5_SRGB_BLOCK_EXT: Self = Self(1000288025);
    pub const ASTC_6X6X5_UNORM_BLOCK_EXT: Self = Self(1000288024);
    pub const ASTC_6X6X6_SFLOAT_BLOCK_EXT: Self = Self(1000288029);
    pub const ASTC_6X6X6_SRGB_BLOCK_EXT: Self = Self(1000288028);
    pub const ASTC_6X6X6_UNORM_BLOCK_EXT: Self = Self(1000288027);
    pub const ASTC_8X5_SFLOAT_BLOCK: Self = Self(1000066005);
    pub const ASTC_8X6_SFLOAT_BLOCK: Self = Self(1000066006);
    pub const ASTC_8X8_SFLOAT_BLOCK: Self = Self(1000066007);
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: Self = Self(1000156011);
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: Self = Self(1000156021);
    pub const B16G16R16G16_422_UNORM: Self = Self(1000156028);
    pub const B8G8R8G8_422_UNORM: Self = Self(1000156001);
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: Self = Self(1000156010);
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: Self = Self(1000156013);
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: Self = Self(1000156015);
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: Self = Self(1000330001);
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: Self = Self(1000156012);
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: Self = Self(1000156014);
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: Self = Self(1000156016);
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: Self = Self(1000156020);
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: Self = Self(1000156023);
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: Self = Self(1000156025);
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: Self = Self(1000330002);
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: Self = Self(1000156022);
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: Self = Self(1000156024);
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: Self = Self(1000156026);
    pub const G14X2_B14X2R14X2_2PLANE_420_UNORM_3PACK16_ARM: Self = Self(1000609012);
    pub const G14X2_B14X2R14X2_2PLANE_422_UNORM_3PACK16_ARM: Self = Self(1000609013);
    pub const G16B16G16R16_422_UNORM: Self = Self(1000156027);
    pub const G16_B16R16_2PLANE_420_UNORM: Self = Self(1000156030);
    pub const G16_B16R16_2PLANE_422_UNORM: Self = Self(1000156032);
    pub const G16_B16R16_2PLANE_444_UNORM: Self = Self(1000330003);
    pub const G16_B16_R16_3PLANE_420_UNORM: Self = Self(1000156029);
    pub const G16_B16_R16_3PLANE_422_UNORM: Self = Self(1000156031);
    pub const G16_B16_R16_3PLANE_444_UNORM: Self = Self(1000156033);
    pub const G8B8G8R8_422_UNORM: Self = Self(1000156000);
    pub const G8_B8R8_2PLANE_420_UNORM: Self = Self(1000156003);
    pub const G8_B8R8_2PLANE_422_UNORM: Self = Self(1000156005);
    pub const G8_B8R8_2PLANE_444_UNORM: Self = Self(1000330000);
    pub const G8_B8_R8_3PLANE_420_UNORM: Self = Self(1000156002);
    pub const G8_B8_R8_3PLANE_422_UNORM: Self = Self(1000156004);
    pub const G8_B8_R8_3PLANE_444_UNORM: Self = Self(1000156006);
    pub const PVRTC1_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054004);
    pub const PVRTC1_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054000);
    pub const PVRTC1_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054005);
    pub const PVRTC1_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054001);
    pub const PVRTC2_2BPP_SRGB_BLOCK_IMG: Self = Self(1000054006);
    pub const PVRTC2_2BPP_UNORM_BLOCK_IMG: Self = Self(1000054002);
    pub const PVRTC2_4BPP_SRGB_BLOCK_IMG: Self = Self(1000054007);
    pub const PVRTC2_4BPP_UNORM_BLOCK_IMG: Self = Self(1000054003);
    pub const R10X6G10X6B10X6A10X6_UINT_4PACK16_ARM: Self = Self(1000609002);
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16: Self = Self(1000156009);
    pub const R10X6G10X6_UINT_2PACK16_ARM: Self = Self(1000609001);
    pub const R10X6G10X6_UNORM_2PACK16: Self = Self(1000156008);
    pub const R10X6_UINT_PACK16_ARM: Self = Self(1000609000);
    pub const R10X6_UNORM_PACK16: Self = Self(1000156007);
    pub const R12X4G12X4B12X4A12X4_UINT_4PACK16_ARM: Self = Self(1000609005);
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16: Self = Self(1000156019);
    pub const R12X4G12X4_UINT_2PACK16_ARM: Self = Self(1000609004);
    pub const R12X4G12X4_UNORM_2PACK16: Self = Self(1000156018);
    pub const R12X4_UINT_PACK16_ARM: Self = Self(1000609003);
    pub const R12X4_UNORM_PACK16: Self = Self(1000156017);
    pub const R14X2G14X2B14X2A14X2_UINT_4PACK16_ARM: Self = Self(1000609008);
    pub const R14X2G14X2B14X2A14X2_UNORM_4PACK16_ARM: Self = Self(1000609011);
    pub const R14X2G14X2_UINT_2PACK16_ARM: Self = Self(1000609007);
    pub const R14X2G14X2_UNORM_2PACK16_ARM: Self = Self(1000609010);
    pub const R14X2_UINT_PACK16_ARM: Self = Self(1000609006);
    pub const R14X2_UNORM_PACK16_ARM: Self = Self(1000609009);
    pub const R16G16_SFIXED5_NV: Self = Self(1000464000);
    pub const R8_BOOL_ARM: Self = Self(1000460000);
    pub const A1B5G5R5_UNORM_PACK16_KHR: Self = Self::A1B5G5R5_UNORM_PACK16;
    pub const A4B4G4R4_UNORM_PACK16_EXT: Self = Self::A4B4G4R4_UNORM_PACK16;
    pub const A4R4G4B4_UNORM_PACK16_EXT: Self = Self::A4R4G4B4_UNORM_PACK16;
    pub const A8_UNORM_KHR: Self = Self::A8_UNORM;
    pub const ASTC_10X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X10_SFLOAT_BLOCK;
    pub const ASTC_10X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X5_SFLOAT_BLOCK;
    pub const ASTC_10X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X6_SFLOAT_BLOCK;
    pub const ASTC_10X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_10X8_SFLOAT_BLOCK;
    pub const ASTC_12X10_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X10_SFLOAT_BLOCK;
    pub const ASTC_12X12_SFLOAT_BLOCK_EXT: Self = Self::ASTC_12X12_SFLOAT_BLOCK;
    pub const ASTC_4X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_4X4_SFLOAT_BLOCK;
    pub const ASTC_5X4_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X4_SFLOAT_BLOCK;
    pub const ASTC_5X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_5X5_SFLOAT_BLOCK;
    pub const ASTC_6X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X5_SFLOAT_BLOCK;
    pub const ASTC_6X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_6X6_SFLOAT_BLOCK;
    pub const ASTC_8X5_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X5_SFLOAT_BLOCK;
    pub const ASTC_8X6_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X6_SFLOAT_BLOCK;
    pub const ASTC_8X8_SFLOAT_BLOCK_EXT: Self = Self::ASTC_8X8_SFLOAT_BLOCK;
    pub const B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR: Self =
        Self::B10X6G10X6R10X6G10X6_422_UNORM_4PACK16;
    pub const B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR: Self =
        Self::B12X4G12X4R12X4G12X4_422_UNORM_4PACK16;
    pub const B16G16R16G16_422_UNORM_KHR: Self = Self::B16G16R16G16_422_UNORM;
    pub const B8G8R8G8_422_UNORM_KHR: Self = Self::B8G8R8G8_422_UNORM;
    pub const G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR: Self =
        Self::G10X6B10X6G10X6R10X6_422_UNORM_4PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16_EXT: Self =
        Self::G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16;
    pub const G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16;
    pub const G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR: Self =
        Self::G12X4B12X4G12X4R12X4_422_UNORM_4PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16_EXT: Self =
        Self::G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16;
    pub const G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR: Self =
        Self::G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16;
    pub const G16B16G16R16_422_UNORM_KHR: Self = Self::G16B16G16R16_422_UNORM;
    pub const G16_B16R16_2PLANE_420_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_420_UNORM;
    pub const G16_B16R16_2PLANE_422_UNORM_KHR: Self = Self::G16_B16R16_2PLANE_422_UNORM;
    pub const G16_B16R16_2PLANE_444_UNORM_EXT: Self = Self::G16_B16R16_2PLANE_444_UNORM;
    pub const G16_B16_R16_3PLANE_420_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_420_UNORM;
    pub const G16_B16_R16_3PLANE_422_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_422_UNORM;
    pub const G16_B16_R16_3PLANE_444_UNORM_KHR: Self = Self::G16_B16_R16_3PLANE_444_UNORM;
    pub const G8B8G8R8_422_UNORM_KHR: Self = Self::G8B8G8R8_422_UNORM;
    pub const G8_B8R8_2PLANE_420_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_420_UNORM;
    pub const G8_B8R8_2PLANE_422_UNORM_KHR: Self = Self::G8_B8R8_2PLANE_422_UNORM;
    pub const G8_B8R8_2PLANE_444_UNORM_EXT: Self = Self::G8_B8R8_2PLANE_444_UNORM;
    pub const G8_B8_R8_3PLANE_420_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_420_UNORM;
    pub const G8_B8_R8_3PLANE_422_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_422_UNORM;
    pub const G8_B8_R8_3PLANE_444_UNORM_KHR: Self = Self::G8_B8_R8_3PLANE_444_UNORM;
    pub const R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR: Self =
        Self::R10X6G10X6B10X6A10X6_UNORM_4PACK16;
    pub const R10X6G10X6_UNORM_2PACK16_KHR: Self = Self::R10X6G10X6_UNORM_2PACK16;
    pub const R10X6_UNORM_PACK16_KHR: Self = Self::R10X6_UNORM_PACK16;
    pub const R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR: Self =
        Self::R12X4G12X4B12X4A12X4_UNORM_4PACK16;
    pub const R12X4G12X4_UNORM_2PACK16_KHR: Self = Self::R12X4G12X4_UNORM_2PACK16;
    pub const R12X4_UNORM_PACK16_KHR: Self = Self::R12X4_UNORM_PACK16;
    pub const R16G16_S10_5_NV: Self = Self::R16G16_SFIXED5_NV;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR: Self = Self(1000150020);
    pub const ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316009);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_KHR: Self = Self(1000150017);
    pub const ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self = Self(1000165001);
    pub const ACCELERATION_STRUCTURE_DENSE_GEOMETRY_FORMAT_TRIANGLES_DATA_AMDX: Self =
        Self(1000478001);
    pub const ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR: Self = Self(1000150002);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR: Self = Self(1000150003);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR: Self = Self(1000150004);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_KHR: Self = Self(1000150006);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_LINEAR_SWEPT_SPHERES_DATA_NV: Self = Self(1000429009);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV: Self = Self(1000327000);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_SPHERES_DATA_NV: Self = Self(1000429010);
    pub const ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR: Self = Self(1000150005);
    pub const ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000165012);
    pub const ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000165008);
    pub const ACCELERATION_STRUCTURE_MOTION_INFO_NV: Self = Self(1000327002);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_DISPLACEMENT_MICROMAP_NV: Self = Self(1000397002);
    pub const ACCELERATION_STRUCTURE_TRIANGLES_OPACITY_MICROMAP_EXT: Self = Self(1000396009);
    pub const ACCELERATION_STRUCTURE_VERSION_INFO_KHR: Self = Self(1000150009);
    pub const ACQUIRE_NEXT_IMAGE_INFO_KHR: Self = Self(1000060010);
    pub const ACQUIRE_PROFILING_LOCK_INFO_KHR: Self = Self(1000116004);
    pub const AMIGO_PROFILING_SUBMIT_INFO_SEC: Self = Self(1000485001);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID: Self = Self(1000129006);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID: Self = Self(1000129002);
    pub const ANDROID_HARDWARE_BUFFER_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468002);
    pub const ANDROID_HARDWARE_BUFFER_PROPERTIES_ANDROID: Self = Self(1000129001);
    pub const ANDROID_HARDWARE_BUFFER_USAGE_ANDROID: Self = Self(1000129000);
    pub const ANDROID_SURFACE_CREATE_INFO_KHR: Self = Self(1000008000);
    pub const ANTI_LAG_DATA_AMD: Self = Self(1000476001);
    pub const ANTI_LAG_PRESENTATION_INFO_AMD: Self = Self(1000476002);
    pub const ATTACHMENT_DESCRIPTION_2: Self = Self(1000109000);
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: Self = Self(1000241002);
    pub const ATTACHMENT_FEEDBACK_LOOP_INFO_EXT: Self = Self(1000527001);
    pub const ATTACHMENT_REFERENCE_2: Self = Self(1000109001);
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT: Self = Self(1000241001);
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_AMD: Self = Self(1000044008);
    pub const BEGIN_CUSTOM_RESOLVE_INFO_EXT: Self = Self(1000628001);
    pub const BIND_ACCELERATION_STRUCTURE_MEMORY_INFO_NV: Self = Self(1000165006);
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060013);
    pub const BIND_BUFFER_MEMORY_INFO: Self = Self(1000157000);
    pub const BIND_DATA_GRAPH_PIPELINE_SESSION_MEMORY_INFO_ARM: Self = Self(1000507005);
    pub const BIND_DESCRIPTOR_BUFFER_EMBEDDED_SAMPLERS_INFO_EXT: Self = Self(1000545008);
    pub const BIND_DESCRIPTOR_SETS_INFO: Self = Self(1000545003);
    pub const BIND_HEAP_INFO_EXT: Self = Self(1000135003);
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: Self = Self(1000060014);
    pub const BIND_IMAGE_MEMORY_INFO: Self = Self(1000157001);
    pub const BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: Self = Self(1000060009);
    pub const BIND_IMAGE_PLANE_MEMORY_INFO: Self = Self(1000156002);
    pub const BIND_MEMORY_STATUS: Self = Self(1000545002);
    pub const BIND_TENSOR_MEMORY_INFO_ARM: Self = Self(1000460002);
    pub const BIND_VIDEO_SESSION_MEMORY_INFO_KHR: Self = Self(1000023004);
    pub const BLIT_IMAGE_CUBIC_WEIGHTS_INFO_QCOM: Self = Self(1000519002);
    pub const BLIT_IMAGE_INFO_2: Self = Self(1000337004);
    pub const BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316005);
    pub const BUFFER_COLLECTION_BUFFER_CREATE_INFO_FUCHSIA: Self = Self(1000366005);
    pub const BUFFER_COLLECTION_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366009);
    pub const BUFFER_COLLECTION_CREATE_INFO_FUCHSIA: Self = Self(1000366000);
    pub const BUFFER_COLLECTION_IMAGE_CREATE_INFO_FUCHSIA: Self = Self(1000366002);
    pub const BUFFER_COLLECTION_PROPERTIES_FUCHSIA: Self = Self(1000366003);
    pub const BUFFER_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366004);
    pub const BUFFER_COPY_2: Self = Self(1000337006);
    pub const BUFFER_DEVICE_ADDRESS_CREATE_INFO_EXT: Self = Self(1000244002);
    pub const BUFFER_DEVICE_ADDRESS_INFO: Self = Self(1000244001);
    pub const BUFFER_IMAGE_COPY_2: Self = Self(1000337009);
    pub const BUFFER_MEMORY_BARRIER_2: Self = Self(1000314001);
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146000);
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: Self = Self(1000257002);
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO: Self = Self(1000470006);
    pub const BUILD_PARTITIONED_ACCELERATION_STRUCTURE_INFO_NV: Self = Self(1000570004);
    pub const CALIBRATED_TIMESTAMP_INFO_KHR: Self = Self(1000184000);
    pub const CHECKPOINT_DATA_2_NV: Self = Self(1000314009);
    pub const CHECKPOINT_DATA_NV: Self = Self(1000206000);
    pub const CLUSTER_ACCELERATION_STRUCTURE_CLUSTERS_BOTTOM_LEVEL_INPUT_NV: Self =
        Self(1000569002);
    pub const CLUSTER_ACCELERATION_STRUCTURE_COMMANDS_INFO_NV: Self = Self(1000569006);
    pub const CLUSTER_ACCELERATION_STRUCTURE_INPUT_INFO_NV: Self = Self(1000569005);
    pub const CLUSTER_ACCELERATION_STRUCTURE_MOVE_OBJECTS_INPUT_NV: Self = Self(1000569004);
    pub const CLUSTER_ACCELERATION_STRUCTURE_TRIANGLE_CLUSTER_INPUT_NV: Self = Self(1000569003);
    pub const COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT: Self = Self(1000081000);
    pub const COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT: Self = Self(1000135010);
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: Self = Self(1000044004);
    pub const COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM: Self = Self(1000282000);
    pub const COMMAND_BUFFER_INHERITANCE_VIEWPORT_SCISSOR_INFO_NV: Self = Self(1000278001);
    pub const COMMAND_BUFFER_SUBMIT_INFO: Self = Self(1000314006);
    pub const COMPUTE_OCCUPANCY_PRIORITY_PARAMETERS_NV: Self = Self(1000645000);
    pub const COMPUTE_PIPELINE_INDIRECT_BUFFER_INFO_NV: Self = Self(1000428001);
    pub const CONDITIONAL_RENDERING_BEGIN_INFO_EXT: Self = Self(1000081002);
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_INFO_NV: Self = Self(1000491004);
    pub const COOPERATIVE_MATRIX_FLEXIBLE_DIMENSIONS_PROPERTIES_NV: Self = Self(1000593001);
    pub const COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506001);
    pub const COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249001);
    pub const COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491002);
    pub const COPY_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150010);
    pub const COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR: Self = Self(1000150011);
    pub const COPY_BUFFER_INFO_2: Self = Self(1000337000);
    pub const COPY_BUFFER_TO_IMAGE_INFO_2: Self = Self(1000337002);
    pub const COPY_COMMAND_TRANSFORM_INFO_QCOM: Self = Self(1000333000);
    pub const COPY_IMAGE_INFO_2: Self = Self(1000337001);
    pub const COPY_IMAGE_TO_BUFFER_INFO_2: Self = Self(1000337003);
    pub const COPY_IMAGE_TO_IMAGE_INFO: Self = Self(1000270007);
    pub const COPY_IMAGE_TO_MEMORY_INFO: Self = Self(1000270004);
    pub const COPY_MEMORY_INDIRECT_INFO_KHR: Self = Self(1000549002);
    pub const COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR: Self = Self(1000150012);
    pub const COPY_MEMORY_TO_IMAGE_INDIRECT_INFO_KHR: Self = Self(1000549003);
    pub const COPY_MEMORY_TO_IMAGE_INFO: Self = Self(1000270005);
    pub const COPY_MEMORY_TO_MICROMAP_INFO_EXT: Self = Self(1000396004);
    pub const COPY_MICROMAP_INFO_EXT: Self = Self(1000396002);
    pub const COPY_MICROMAP_TO_MEMORY_INFO_EXT: Self = Self(1000396003);
    pub const COPY_TENSOR_INFO_ARM: Self = Self(1000460011);
    pub const CUDA_FUNCTION_CREATE_INFO_NV: Self = Self(1000307001);
    pub const CUDA_LAUNCH_INFO_NV: Self = Self(1000307002);
    pub const CUDA_MODULE_CREATE_INFO_NV: Self = Self(1000307000);
    pub const CUSTOM_RESOLVE_CREATE_INFO_EXT: Self = Self(1000628002);
    pub const CU_FUNCTION_CREATE_INFO_NVX: Self = Self(1000029001);
    pub const CU_LAUNCH_INFO_NVX: Self = Self(1000029002);
    pub const CU_MODULE_CREATE_INFO_NVX: Self = Self(1000029000);
    pub const CU_MODULE_TEXTURING_MODE_CREATE_INFO_NVX: Self = Self(1000029004);
    pub const D3D12_FENCE_SUBMIT_INFO_KHR: Self = Self(1000078002);
    pub const DATA_GRAPH_PIPELINE_BUILTIN_MODEL_CREATE_INFO_QCOM: Self = Self(1000629001);
    pub const DATA_GRAPH_PIPELINE_COMPILER_CONTROL_CREATE_INFO_ARM: Self = Self(1000507010);
    pub const DATA_GRAPH_PIPELINE_CONSTANT_ARM: Self = Self(1000507003);
    pub const DATA_GRAPH_PIPELINE_CONSTANT_TENSOR_SEMI_STRUCTURED_SPARSITY_INFO_ARM: Self =
        Self(1000507015);
    pub const DATA_GRAPH_PIPELINE_CREATE_INFO_ARM: Self = Self(1000507000);
    pub const DATA_GRAPH_PIPELINE_DISPATCH_INFO_ARM: Self = Self(1000507014);
    pub const DATA_GRAPH_PIPELINE_IDENTIFIER_CREATE_INFO_ARM: Self = Self(1000507013);
    pub const DATA_GRAPH_PIPELINE_INFO_ARM: Self = Self(1000507009);
    pub const DATA_GRAPH_PIPELINE_PROPERTY_QUERY_RESULT_ARM: Self = Self(1000507008);
    pub const DATA_GRAPH_PIPELINE_RESOURCE_INFO_ARM: Self = Self(1000507002);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENTS_INFO_ARM: Self = Self(1000507011);
    pub const DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_REQUIREMENT_ARM: Self = Self(1000507012);
    pub const DATA_GRAPH_PIPELINE_SESSION_CREATE_INFO_ARM: Self = Self(1000507001);
    pub const DATA_GRAPH_PIPELINE_SESSION_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000507004);
    pub const DATA_GRAPH_PIPELINE_SHADER_MODULE_CREATE_INFO_ARM: Self = Self(1000507007);
    pub const DATA_GRAPH_PROCESSING_ENGINE_CREATE_INFO_ARM: Self = Self(1000507016);
    pub const DEBUG_MARKER_MARKER_INFO_EXT: Self = Self(1000022002);
    pub const DEBUG_MARKER_OBJECT_NAME_INFO_EXT: Self = Self(1000022000);
    pub const DEBUG_MARKER_OBJECT_TAG_INFO_EXT: Self = Self(1000022001);
    pub const DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT: Self = Self(1000011000);
    pub const DEBUG_UTILS_LABEL_EXT: Self = Self(1000128002);
    pub const DEBUG_UTILS_MESSENGER_CALLBACK_DATA_EXT: Self = Self(1000128003);
    pub const DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1000128004);
    pub const DEBUG_UTILS_OBJECT_NAME_INFO_EXT: Self = Self(1000128000);
    pub const DEBUG_UTILS_OBJECT_TAG_INFO_EXT: Self = Self(1000128001);
    pub const DECOMPRESS_MEMORY_INFO_EXT: Self = Self(1000550002);
    pub const DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV: Self = Self(1000026001);
    pub const DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV: Self = Self(1000026000);
    pub const DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000026002);
    pub const DEPENDENCY_INFO: Self = Self(1000314003);
    pub const DEPTH_BIAS_INFO_EXT: Self = Self(1000283001);
    pub const DEPTH_BIAS_REPRESENTATION_INFO_EXT: Self = Self(1000283002);
    pub const DESCRIPTOR_ADDRESS_INFO_EXT: Self = Self(1000316003);
    pub const DESCRIPTOR_BUFFER_BINDING_INFO_EXT: Self = Self(1000316011);
    pub const DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT: Self = Self(1000316012);
    pub const DESCRIPTOR_GET_INFO_EXT: Self = Self(1000316004);
    pub const DESCRIPTOR_GET_TENSOR_INFO_ARM: Self = Self(1000460020);
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: Self = Self(1000138003);
    pub const DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT: Self = Self(1000135005);
    pub const DESCRIPTOR_SET_BINDING_REFERENCE_VALVE: Self = Self(1000420001);
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: Self = Self(1000161000);
    pub const DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE: Self = Self(1000420002);
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT: Self = Self(1000168001);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: Self = Self(1000161003);
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: Self = Self(1000161004);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: Self = Self(1000085000);
    pub const DEVICE_ADDRESS_BINDING_CALLBACK_DATA_EXT: Self = Self(1000354001);
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS: Self = Self(1000413002);
    pub const DEVICE_DEVICE_MEMORY_REPORT_CREATE_INFO_EXT: Self = Self(1000284001);
    pub const DEVICE_DIAGNOSTICS_CONFIG_CREATE_INFO_NV: Self = Self(1000300001);
    pub const DEVICE_EVENT_INFO_EXT: Self = Self(1000091001);
    pub const DEVICE_FAULT_COUNTS_EXT: Self = Self(1000341001);
    pub const DEVICE_FAULT_INFO_EXT: Self = Self(1000341002);
    pub const DEVICE_GROUP_BIND_SPARSE_INFO: Self = Self(1000060006);
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: Self = Self(1000060004);
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO: Self = Self(1000070001);
    pub const DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: Self = Self(1000060007);
    pub const DEVICE_GROUP_PRESENT_INFO_KHR: Self = Self(1000060011);
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: Self = Self(1000060003);
    pub const DEVICE_GROUP_SUBMIT_INFO: Self = Self(1000060005);
    pub const DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060012);
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS: Self = Self(1000413003);
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO: Self = Self(1000470004);
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: Self = Self(1000257004);
    pub const DEVICE_MEMORY_OVERALLOCATION_CREATE_INFO_AMD: Self = Self(1000189000);
    pub const DEVICE_MEMORY_REPORT_CALLBACK_DATA_EXT: Self = Self(1000284002);
    pub const DEVICE_PIPELINE_BINARY_INTERNAL_CACHE_CONTROL_KHR: Self = Self(1000483008);
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO: Self = Self(1000295001);
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: Self = Self(1000174000);
    pub const DEVICE_QUEUE_INFO_2: Self = Self(1000145003);
    pub const DEVICE_QUEUE_SHADER_CORE_CONTROL_CREATE_INFO_ARM: Self = Self(1000417000);
    pub const DEVICE_TENSOR_MEMORY_REQUIREMENTS_ARM: Self = Self(1000460010);
    pub const DIRECTFB_SURFACE_CREATE_INFO_EXT: Self = Self(1000346000);
    pub const DIRECT_DRIVER_LOADING_INFO_LUNARG: Self = Self(1000459000);
    pub const DIRECT_DRIVER_LOADING_LIST_LUNARG: Self = Self(1000459001);
    pub const DISPATCH_TILE_INFO_QCOM: Self = Self(1000309005);
    pub const DISPLAY_EVENT_INFO_EXT: Self = Self(1000091002);
    pub const DISPLAY_MODE_CREATE_INFO_KHR: Self = Self(1000002000);
    pub const DISPLAY_MODE_PROPERTIES_2_KHR: Self = Self(1000121002);
    pub const DISPLAY_MODE_STEREO_PROPERTIES_NV: Self = Self(1000551001);
    pub const DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD: Self = Self(1000213000);
    pub const DISPLAY_PLANE_CAPABILITIES_2_KHR: Self = Self(1000121004);
    pub const DISPLAY_PLANE_INFO_2_KHR: Self = Self(1000121003);
    pub const DISPLAY_PLANE_PROPERTIES_2_KHR: Self = Self(1000121001);
    pub const DISPLAY_POWER_INFO_EXT: Self = Self(1000091000);
    pub const DISPLAY_PRESENT_INFO_KHR: Self = Self(1000003000);
    pub const DISPLAY_PROPERTIES_2_KHR: Self = Self(1000121000);
    pub const DISPLAY_SURFACE_CREATE_INFO_KHR: Self = Self(1000002001);
    pub const DISPLAY_SURFACE_STEREO_CREATE_INFO_NV: Self = Self(1000551000);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_2_EXT: Self = Self(1000158006);
    pub const DRM_FORMAT_MODIFIER_PROPERTIES_LIST_EXT: Self = Self(1000158000);
    pub const EXECUTION_GRAPH_PIPELINE_CREATE_INFO_AMDX: Self = Self(1000134003);
    pub const EXECUTION_GRAPH_PIPELINE_SCRATCH_SIZE_AMDX: Self = Self(1000134002);
    pub const EXPORT_FENCE_CREATE_INFO: Self = Self(1000113000);
    pub const EXPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114001);
    pub const EXPORT_MEMORY_ALLOCATE_INFO: Self = Self(1000072002);
    pub const EXPORT_MEMORY_ALLOCATE_INFO_NV: Self = Self(1000056001);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073001);
    pub const EXPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057001);
    pub const EXPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311004);
    pub const EXPORT_METAL_COMMAND_QUEUE_INFO_EXT: Self = Self(1000311003);
    pub const EXPORT_METAL_DEVICE_INFO_EXT: Self = Self(1000311002);
    pub const EXPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311008);
    pub const EXPORT_METAL_OBJECTS_INFO_EXT: Self = Self(1000311001);
    pub const EXPORT_METAL_OBJECT_CREATE_INFO_EXT: Self = Self(1000311000);
    pub const EXPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311010);
    pub const EXPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311006);
    pub const EXPORT_SEMAPHORE_CREATE_INFO: Self = Self(1000077000);
    pub const EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078001);
    pub const EXTERNAL_BUFFER_PROPERTIES: Self = Self(1000071003);
    pub const EXTERNAL_COMPUTE_QUEUE_CREATE_INFO_NV: Self = Self(1000556001);
    pub const EXTERNAL_COMPUTE_QUEUE_DATA_PARAMS_NV: Self = Self(1000556002);
    pub const EXTERNAL_COMPUTE_QUEUE_DEVICE_CREATE_INFO_NV: Self = Self(1000556000);
    pub const EXTERNAL_FENCE_PROPERTIES: Self = Self(1000112001);
    pub const EXTERNAL_FORMAT_ANDROID: Self = Self(1000129005);
    pub const EXTERNAL_FORMAT_OHOS: Self = Self(1000452005);
    pub const EXTERNAL_FORMAT_QNX: Self = Self(1000529003);
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES: Self = Self(1000071001);
    pub const EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT: Self = Self(1000453000);
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO: Self = Self(1000072000);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO: Self = Self(1000072001);
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_NV: Self = Self(1000056000);
    pub const EXTERNAL_MEMORY_TENSOR_CREATE_INFO_ARM: Self = Self(1000460017);
    pub const EXTERNAL_SEMAPHORE_PROPERTIES: Self = Self(1000076001);
    pub const EXTERNAL_TENSOR_PROPERTIES_ARM: Self = Self(1000460016);
    pub const FENCE_GET_FD_INFO_KHR: Self = Self(1000115001);
    pub const FENCE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000114002);
    pub const FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000170001);
    pub const FORMAT_PROPERTIES_2: Self = Self(1000059002);
    pub const FORMAT_PROPERTIES_3: Self = Self(1000360000);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000226000);
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: Self = Self(1000108001);
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: Self = Self(1000108002);
    pub const FRAMEBUFFER_MIXED_SAMPLES_COMBINATION_NV: Self = Self(1000250002);
    pub const FRAME_BOUNDARY_EXT: Self = Self(1000375001);
    pub const FRAME_BOUNDARY_TENSORS_ARM: Self = Self(1000460023);
    pub const GENERATED_COMMANDS_INFO_EXT: Self = Self(1000572004);
    pub const GENERATED_COMMANDS_INFO_NV: Self = Self(1000277005);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_EXT: Self = Self(1000572002);
    pub const GENERATED_COMMANDS_MEMORY_REQUIREMENTS_INFO_NV: Self = Self(1000277006);
    pub const GENERATED_COMMANDS_PIPELINE_INFO_EXT: Self = Self(1000572013);
    pub const GENERATED_COMMANDS_SHADER_INFO_EXT: Self = Self(1000572014);
    pub const GEOMETRY_AABB_NV: Self = Self(1000165005);
    pub const GEOMETRY_NV: Self = Self(1000165003);
    pub const GEOMETRY_TRIANGLES_NV: Self = Self(1000165004);
    pub const GET_LATENCY_MARKER_INFO_NV: Self = Self(1000505003);
    pub const GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT: Self = Self(1000320002);
    pub const GRAPHICS_PIPELINE_SHADER_GROUPS_CREATE_INFO_NV: Self = Self(1000277002);
    pub const GRAPHICS_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000277001);
    pub const HDR_METADATA_EXT: Self = Self(1000105000);
    pub const HDR_VIVID_DYNAMIC_METADATA_HUAWEI: Self = Self(1000590001);
    pub const HEADLESS_SURFACE_CREATE_INFO_EXT: Self = Self(1000256000);
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY: Self = Self(1000270009);
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO: Self = Self(1000270006);
    pub const IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA: Self = Self(1000214000);
    pub const IMAGE_ALIGNMENT_CONTROL_CREATE_INFO_MESA: Self = Self(1000575002);
    pub const IMAGE_BLIT_2: Self = Self(1000337008);
    pub const IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316006);
    pub const IMAGE_COMPRESSION_CONTROL_EXT: Self = Self(1000338001);
    pub const IMAGE_COMPRESSION_PROPERTIES_EXT: Self = Self(1000338004);
    pub const IMAGE_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366006);
    pub const IMAGE_COPY_2: Self = Self(1000337007);
    pub const IMAGE_DESCRIPTOR_INFO_EXT: Self = Self(1000135001);
    pub const IMAGE_DRM_FORMAT_MODIFIER_EXPLICIT_CREATE_INFO_EXT: Self = Self(1000158004);
    pub const IMAGE_DRM_FORMAT_MODIFIER_LIST_CREATE_INFO_EXT: Self = Self(1000158003);
    pub const IMAGE_DRM_FORMAT_MODIFIER_PROPERTIES_EXT: Self = Self(1000158005);
    pub const IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA: Self = Self(1000366007);
    pub const IMAGE_FORMAT_LIST_CREATE_INFO: Self = Self(1000147000);
    pub const IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059003);
    pub const IMAGE_MEMORY_BARRIER_2: Self = Self(1000314002);
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146001);
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: Self = Self(1000156003);
    pub const IMAGE_RESOLVE_2: Self = Self(1000337010);
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: Self = Self(1000146002);
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO: Self = Self(1000246000);
    pub const IMAGE_SUBRESOURCE_2: Self = Self(1000338003);
    pub const IMAGE_SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000060008);
    pub const IMAGE_TO_MEMORY_COPY: Self = Self(1000270003);
    pub const IMAGE_VIEW_ADDRESS_PROPERTIES_NVX: Self = Self(1000030001);
    pub const IMAGE_VIEW_ASTC_DECODE_MODE_EXT: Self = Self(1000067000);
    pub const IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316007);
    pub const IMAGE_VIEW_HANDLE_INFO_NVX: Self = Self(1000030000);
    pub const IMAGE_VIEW_MIN_LOD_CREATE_INFO_EXT: Self = Self(1000391001);
    pub const IMAGE_VIEW_SAMPLE_WEIGHT_CREATE_INFO_QCOM: Self = Self(1000440002);
    pub const IMAGE_VIEW_SLICED_CREATE_INFO_EXT: Self = Self(1000418001);
    pub const IMAGE_VIEW_USAGE_CREATE_INFO: Self = Self(1000117002);
    pub const IMPORT_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129003);
    pub const IMPORT_FENCE_FD_INFO_KHR: Self = Self(1000115000);
    pub const IMPORT_FENCE_WIN32_HANDLE_INFO_KHR: Self = Self(1000114000);
    pub const IMPORT_MEMORY_BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366001);
    pub const IMPORT_MEMORY_FD_INFO_KHR: Self = Self(1000074000);
    pub const IMPORT_MEMORY_HOST_POINTER_INFO_EXT: Self = Self(1000178000);
    pub const IMPORT_MEMORY_METAL_HANDLE_INFO_EXT: Self = Self(1000602000);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR: Self = Self(1000073000);
    pub const IMPORT_MEMORY_WIN32_HANDLE_INFO_NV: Self = Self(1000057000);
    pub const IMPORT_MEMORY_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364000);
    pub const IMPORT_METAL_BUFFER_INFO_EXT: Self = Self(1000311005);
    pub const IMPORT_METAL_IO_SURFACE_INFO_EXT: Self = Self(1000311009);
    pub const IMPORT_METAL_SHARED_EVENT_INFO_EXT: Self = Self(1000311011);
    pub const IMPORT_METAL_TEXTURE_INFO_EXT: Self = Self(1000311007);
    pub const IMPORT_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452003);
    pub const IMPORT_SCREEN_BUFFER_INFO_QNX: Self = Self(1000529002);
    pub const IMPORT_SEMAPHORE_FD_INFO_KHR: Self = Self(1000079000);
    pub const IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR: Self = Self(1000078000);
    pub const IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365000);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_EXT: Self = Self(1000572006);
    pub const INDIRECT_COMMANDS_LAYOUT_CREATE_INFO_NV: Self = Self(1000277004);
    pub const INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV: Self = Self(1000135012);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_EXT: Self = Self(1000572007);
    pub const INDIRECT_COMMANDS_LAYOUT_TOKEN_NV: Self = Self(1000277003);
    pub const INDIRECT_EXECUTION_SET_CREATE_INFO_EXT: Self = Self(1000572003);
    pub const INDIRECT_EXECUTION_SET_PIPELINE_INFO_EXT: Self = Self(1000572010);
    pub const INDIRECT_EXECUTION_SET_SHADER_INFO_EXT: Self = Self(1000572011);
    pub const INDIRECT_EXECUTION_SET_SHADER_LAYOUT_INFO_EXT: Self = Self(1000572012);
    pub const INITIALIZE_PERFORMANCE_API_INFO_INTEL: Self = Self(1000210001);
    pub const IOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000122000);
    pub const LATENCY_SLEEP_INFO_NV: Self = Self(1000505001);
    pub const LATENCY_SLEEP_MODE_INFO_NV: Self = Self(1000505000);
    pub const LATENCY_SUBMISSION_PRESENT_ID_NV: Self = Self(1000505005);
    pub const LATENCY_SURFACE_CAPABILITIES_NV: Self = Self(1000505008);
    pub const LATENCY_TIMINGS_FRAME_REPORT_NV: Self = Self(1000505004);
    pub const LAYER_SETTINGS_CREATE_INFO_EXT: Self = Self(1000496000);
    pub const MACOS_SURFACE_CREATE_INFO_MVK: Self = Self(1000123000);
    pub const MEMORY_ALLOCATE_FLAGS_INFO: Self = Self(1000060000);
    pub const MEMORY_BARRIER_2: Self = Self(1000314000);
    pub const MEMORY_BARRIER_ACCESS_FLAGS_3_KHR: Self = Self(1000574002);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO: Self = Self(1000127001);
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_TENSOR_ARM: Self = Self(1000460014);
    pub const MEMORY_DEDICATED_REQUIREMENTS: Self = Self(1000127000);
    pub const MEMORY_FD_PROPERTIES_KHR: Self = Self(1000074001);
    pub const MEMORY_GET_ANDROID_HARDWARE_BUFFER_INFO_ANDROID: Self = Self(1000129004);
    pub const MEMORY_GET_FD_INFO_KHR: Self = Self(1000074002);
    pub const MEMORY_GET_METAL_HANDLE_INFO_EXT: Self = Self(1000602002);
    pub const MEMORY_GET_NATIVE_BUFFER_INFO_OHOS: Self = Self(1000452004);
    pub const MEMORY_GET_REMOTE_ADDRESS_INFO_NV: Self = Self(1000371000);
    pub const MEMORY_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000073003);
    pub const MEMORY_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000364002);
    pub const MEMORY_HOST_POINTER_PROPERTIES_EXT: Self = Self(1000178001);
    pub const MEMORY_MAP_INFO: Self = Self(1000271000);
    pub const MEMORY_MAP_PLACED_INFO_EXT: Self = Self(1000272002);
    pub const MEMORY_METAL_HANDLE_PROPERTIES_EXT: Self = Self(1000602001);
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: Self = Self(1000257003);
    pub const MEMORY_PRIORITY_ALLOCATE_INFO_EXT: Self = Self(1000238001);
    pub const MEMORY_REQUIREMENTS_2: Self = Self(1000146003);
    pub const MEMORY_TO_IMAGE_COPY: Self = Self(1000270002);
    pub const MEMORY_UNMAP_INFO: Self = Self(1000271001);
    pub const MEMORY_WIN32_HANDLE_PROPERTIES_KHR: Self = Self(1000073002);
    pub const MEMORY_ZIRCON_HANDLE_PROPERTIES_FUCHSIA: Self = Self(1000364001);
    pub const METAL_SURFACE_CREATE_INFO_EXT: Self = Self(1000217000);
    pub const MICROMAP_BUILD_INFO_EXT: Self = Self(1000396000);
    pub const MICROMAP_BUILD_SIZES_INFO_EXT: Self = Self(1000396008);
    pub const MICROMAP_CREATE_INFO_EXT: Self = Self(1000396007);
    pub const MICROMAP_VERSION_INFO_EXT: Self = Self(1000396001);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_INFO_EXT: Self = Self(1000376002);
    pub const MULTISAMPLE_PROPERTIES_EXT: Self = Self(1000143004);
    pub const MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: Self = Self(1000044009);
    pub const MULTIVIEW_PER_VIEW_RENDER_AREAS_RENDER_PASS_BEGIN_INFO_QCOM: Self = Self(1000510001);
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: Self = Self(1000351002);
    pub const NATIVE_BUFFER_FORMAT_PROPERTIES_OHOS: Self = Self(1000452002);
    pub const NATIVE_BUFFER_PROPERTIES_OHOS: Self = Self(1000452001);
    pub const NATIVE_BUFFER_USAGE_OHOS: Self = Self(1000452000);
    pub const OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT: Self = Self(1000135007);
    pub const OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT: Self = Self(1000316010);
    pub const OPTICAL_FLOW_EXECUTE_INFO_NV: Self = Self(1000464005);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_INFO_NV: Self = Self(1000464002);
    pub const OPTICAL_FLOW_IMAGE_FORMAT_PROPERTIES_NV: Self = Self(1000464003);
    pub const OPTICAL_FLOW_SESSION_CREATE_INFO_NV: Self = Self(1000464004);
    pub const OPTICAL_FLOW_SESSION_CREATE_PRIVATE_DATA_INFO_NV: Self = Self(1000464010);
    pub const OUT_OF_BAND_QUEUE_TYPE_INFO_NV: Self = Self(1000505006);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_FLAGS_NV: Self = Self(1000570005);
    pub const PARTITIONED_ACCELERATION_STRUCTURE_INSTANCES_INPUT_NV: Self = Self(1000570003);
    pub const PAST_PRESENTATION_TIMING_EXT: Self = Self(1000208007);
    pub const PAST_PRESENTATION_TIMING_INFO_EXT: Self = Self(1000208005);
    pub const PAST_PRESENTATION_TIMING_PROPERTIES_EXT: Self = Self(1000208006);
    pub const PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL: Self = Self(1000210005);
    pub const PERFORMANCE_COUNTER_ARM: Self = Self(1000605002);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_ARM: Self = Self(1000605003);
    pub const PERFORMANCE_COUNTER_DESCRIPTION_KHR: Self = Self(1000116006);
    pub const PERFORMANCE_COUNTER_KHR: Self = Self(1000116005);
    pub const PERFORMANCE_MARKER_INFO_INTEL: Self = Self(1000210002);
    pub const PERFORMANCE_OVERRIDE_INFO_INTEL: Self = Self(1000210004);
    pub const PERFORMANCE_QUERY_SUBMIT_INFO_KHR: Self = Self(1000116003);
    pub const PERFORMANCE_STREAM_MARKER_INFO_INTEL: Self = Self(1000210003);
    pub const PER_TILE_BEGIN_INFO_QCOM: Self = Self(1000309003);
    pub const PER_TILE_END_INFO_QCOM: Self = Self(1000309004);
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: Self = Self(1000083000);
    pub const PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT: Self = Self(1000340000);
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: Self = Self(1000177000);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR: Self = Self(1000150013);
    pub const PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR: Self = Self(1000150014);
    pub const PHYSICAL_DEVICE_ADDRESS_BINDING_REPORT_FEATURES_EXT: Self = Self(1000354000);
    pub const PHYSICAL_DEVICE_AMIGO_PROFILING_FEATURES_SEC: Self = Self(1000485000);
    pub const PHYSICAL_DEVICE_ANTI_LAG_FEATURES_AMD: Self = Self(1000476000);
    pub const PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT: Self = Self(1000067001);
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT: Self =
        Self(1000524000);
    pub const PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT: Self = Self(1000339000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT: Self = Self(1000148000);
    pub const PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT: Self = Self(1000148001);
    pub const PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT: Self = Self(1000411000);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: Self = Self(1000257000);
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT: Self = Self(1000244000);
    pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_FEATURES_NV: Self = Self(1000569000);
    pub const PHYSICAL_DEVICE_CLUSTER_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self = Self(1000569001);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_FEATURES_HUAWEI: Self = Self(1000404000);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_PROPERTIES_HUAWEI: Self = Self(1000404001);
    pub const PHYSICAL_DEVICE_CLUSTER_CULLING_SHADER_VRS_FEATURES_HUAWEI: Self = Self(1000404002);
    pub const PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD: Self = Self(1000229000);
    pub const PHYSICAL_DEVICE_COLOR_WRITE_ENABLE_FEATURES_EXT: Self = Self(1000381000);
    pub const PHYSICAL_DEVICE_COMMAND_BUFFER_INHERITANCE_FEATURES_NV: Self = Self(1000559000);
    pub const PHYSICAL_DEVICE_COMPUTE_OCCUPANCY_PRIORITY_FEATURES_NV: Self = Self(1000645001);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR: Self = Self(1000201000);
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_PROPERTIES_KHR: Self = Self(1000511000);
    pub const PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT: Self = Self(1000081001);
    pub const PHYSICAL_DEVICE_CONSERVATIVE_RASTERIZATION_PROPERTIES_EXT: Self = Self(1000101000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_FEATURES_NV: Self = Self(1000593000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_2_PROPERTIES_NV: Self = Self(1000593002);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_KHR: Self = Self(1000506000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV: Self = Self(1000249000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_KHR: Self = Self(1000506002);
    pub const PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV: Self = Self(1000249002);
    pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_FEATURES_NV: Self = Self(1000491000);
    pub const PHYSICAL_DEVICE_COOPERATIVE_VECTOR_PROPERTIES_NV: Self = Self(1000491001);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_KHR: Self = Self(1000549000);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_FEATURES_NV: Self = Self(1000426000);
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR: Self = Self(1000426001);
    pub const PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV: Self = Self(1000050000);
    pub const PHYSICAL_DEVICE_COVERAGE_REDUCTION_MODE_FEATURES_NV: Self = Self(1000250000);
    pub const PHYSICAL_DEVICE_CUBIC_CLAMP_FEATURES_QCOM: Self = Self(1000521000);
    pub const PHYSICAL_DEVICE_CUBIC_WEIGHTS_FEATURES_QCOM: Self = Self(1000519001);
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_FEATURES_NV: Self = Self(1000307003);
    pub const PHYSICAL_DEVICE_CUDA_KERNEL_LAUNCH_PROPERTIES_NV: Self = Self(1000307004);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT: Self = Self(1000287002);
    pub const PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT: Self = Self(1000287001);
    pub const PHYSICAL_DEVICE_CUSTOM_RESOLVE_FEATURES_EXT: Self = Self(1000628000);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_FEATURES_ARM: Self = Self(1000507006);
    pub const PHYSICAL_DEVICE_DATA_GRAPH_MODEL_FEATURES_QCOM: Self = Self(1000629000);
    pub const PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV: Self =
        Self(1000240000);
    pub const PHYSICAL_DEVICE_DENSE_GEOMETRY_FORMAT_FEATURES_AMDX: Self = Self(1000478000);
    pub const PHYSICAL_DEVICE_DEPTH_BIAS_CONTROL_FEATURES_EXT: Self = Self(1000283000);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_CONTROL_FEATURES_EXT: Self = Self(1000582000);
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR: Self = Self(1000421000);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT: Self = Self(1000355000);
    pub const PHYSICAL_DEVICE_DEPTH_CLIP_ENABLE_FEATURES_EXT: Self = Self(1000102000);
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: Self = Self(1000199000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000316001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT: Self = Self(1000316002);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT: Self = Self(1000316000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_FEATURES_ARM: Self = Self(1000460018);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_TENSOR_PROPERTIES_ARM: Self = Self(1000460019);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT: Self = Self(1000135009);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT: Self = Self(1000135008);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM: Self = Self(1000135014);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: Self = Self(1000161001);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: Self = Self(1000161002);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV: Self = Self(1000546000);
    pub const PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE: Self = Self(1000420000);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_COMPUTE_FEATURES_NV: Self =
        Self(1000428000);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_EXT: Self = Self(1000572000);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_FEATURES_NV: Self = Self(1000277007);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_EXT: Self = Self(1000572001);
    pub const PHYSICAL_DEVICE_DEVICE_GENERATED_COMMANDS_PROPERTIES_NV: Self = Self(1000277000);
    pub const PHYSICAL_DEVICE_DEVICE_MEMORY_REPORT_FEATURES_EXT: Self = Self(1000284000);
    pub const PHYSICAL_DEVICE_DIAGNOSTICS_CONFIG_FEATURES_NV: Self = Self(1000300000);
    pub const PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT: Self = Self(1000099000);
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_FEATURES_NV: Self = Self(1000397000);
    pub const PHYSICAL_DEVICE_DISPLACEMENT_MICROMAP_PROPERTIES_NV: Self = Self(1000397001);
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES: Self = Self(1000196000);
    pub const PHYSICAL_DEVICE_DRM_PROPERTIES_EXT: Self = Self(1000353000);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: Self = Self(1000044003);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES: Self = Self(1000232000);
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT: Self =
        Self(1000499000);
    pub const PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV: Self = Self(1000205002);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_2_FEATURES_EXT: Self = Self(1000377000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_FEATURES_EXT: Self = Self(1000455000);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_3_PROPERTIES_EXT: Self = Self(1000455001);
    pub const PHYSICAL_DEVICE_EXTENDED_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000267000);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_FEATURES_NV: Self = Self(1000492000);
    pub const PHYSICAL_DEVICE_EXTENDED_SPARSE_ADDRESS_SPACE_PROPERTIES_NV: Self = Self(1000492001);
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: Self = Self(1000071002);
    pub const PHYSICAL_DEVICE_EXTERNAL_COMPUTE_QUEUE_PROPERTIES_NV: Self = Self(1000556003);
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: Self = Self(1000112000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_FEATURES_ANDROID: Self = Self(1000468000);
    pub const PHYSICAL_DEVICE_EXTERNAL_FORMAT_RESOLVE_PROPERTIES_ANDROID: Self = Self(1000468001);
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: Self = Self(1000071000);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_HOST_PROPERTIES_EXT: Self = Self(1000178002);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_RDMA_FEATURES_NV: Self = Self(1000371001);
    pub const PHYSICAL_DEVICE_EXTERNAL_MEMORY_SCREEN_BUFFER_FEATURES_QNX: Self = Self(1000529004);
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: Self = Self(1000076000);
    pub const PHYSICAL_DEVICE_EXTERNAL_TENSOR_INFO_ARM: Self = Self(1000460015);
    pub const PHYSICAL_DEVICE_FAULT_FEATURES_EXT: Self = Self(1000341000);
    pub const PHYSICAL_DEVICE_FEATURES_2: Self = Self(1000059000);
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: Self = Self(1000197000);
    pub const PHYSICAL_DEVICE_FORMAT_PACK_FEATURES_ARM: Self = Self(1000609000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_FEATURES_EXT: Self = Self(1000332000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_2_PROPERTIES_EXT: Self = Self(1000332001);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_FEATURES_EXT: Self = Self(1000218000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_FEATURES_VALVE: Self = Self(1000611000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_LAYERED_PROPERTIES_VALVE: Self =
        Self(1000611001);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT: Self = Self(1000425000);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT: Self = Self(1000425001);
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_PROPERTIES_EXT: Self = Self(1000218001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: Self = Self(1000203000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR: Self = Self(1000322000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_INTERLOCK_FEATURES_EXT: Self = Self(1000251000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_FEATURES_NV: Self = Self(1000326001);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_ENUMS_PROPERTIES_NV: Self = Self(1000326000);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_FEATURES_KHR: Self = Self(1000226003);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226004);
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADING_RATE_PROPERTIES_KHR: Self = Self(1000226002);
    pub const PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT: Self = Self(1000375000);
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: Self = Self(1000388000);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT: Self = Self(1000320000);
    pub const PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT: Self = Self(1000320001);
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES: Self = Self(1000070000);
    pub const PHYSICAL_DEVICE_HDR_VIVID_FEATURES_HUAWEI: Self = Self(1000590000);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES: Self = Self(1000270000);
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES: Self = Self(1000270001);
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: Self = Self(1000261000);
    pub const PHYSICAL_DEVICE_ID_PROPERTIES: Self = Self(1000071004);
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: Self = Self(1000108000);
    pub const PHYSICAL_DEVICE_IMAGE_2D_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000393000);
    pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_FEATURES_MESA: Self = Self(1000575000);
    pub const PHYSICAL_DEVICE_IMAGE_ALIGNMENT_CONTROL_PROPERTIES_MESA: Self = Self(1000575001);
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_FEATURES_EXT: Self = Self(1000338000);
    pub const PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT: Self =
        Self(1000437000);
    pub const PHYSICAL_DEVICE_IMAGE_DRM_FORMAT_MODIFIER_INFO_EXT: Self = Self(1000158002);
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: Self = Self(1000059004);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_FEATURES_QCOM: Self = Self(1000518000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_2_PROPERTIES_QCOM: Self = Self(1000518001);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_FEATURES_QCOM: Self = Self(1000440000);
    pub const PHYSICAL_DEVICE_IMAGE_PROCESSING_PROPERTIES_QCOM: Self = Self(1000440001);
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: Self = Self(1000335000);
    pub const PHYSICAL_DEVICE_IMAGE_SLICED_VIEW_OF_3D_FEATURES_EXT: Self = Self(1000418000);
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT: Self = Self(1000170000);
    pub const PHYSICAL_DEVICE_IMAGE_VIEW_MIN_LOD_FEATURES_EXT: Self = Self(1000391000);
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: Self = Self(1000265000);
    pub const PHYSICAL_DEVICE_INHERITED_VIEWPORT_SCISSOR_FEATURES_NV: Self = Self(1000278000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: Self = Self(1000138000);
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: Self = Self(1000138001);
    pub const PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR: Self = Self(1000504000);
    pub const PHYSICAL_DEVICE_INVOCATION_MASK_FEATURES_HUAWEI: Self = Self(1000370000);
    pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_KHR: Self = Self(1000562003);
    pub const PHYSICAL_DEVICE_LAYERED_API_PROPERTIES_LIST_KHR: Self = Self(1000562002);
    pub const PHYSICAL_DEVICE_LAYERED_API_VULKAN_PROPERTIES_KHR: Self = Self(1000562004);
    pub const PHYSICAL_DEVICE_LAYERED_DRIVER_PROPERTIES_MSFT: Self = Self(1000530000);
    pub const PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT: Self = Self(1000465000);
    pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_FEATURES_EXT: Self = Self(1000495000);
    pub const PHYSICAL_DEVICE_LEGACY_VERTEX_ATTRIBUTES_PROPERTIES_EXT: Self = Self(1000495001);
    pub const PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV: Self = Self(1000430000);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: Self = Self(1000259000);
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: Self = Self(1000259002);
    pub const PHYSICAL_DEVICE_MAINTENANCE_10_FEATURES_KHR: Self = Self(1000630000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_10_PROPERTIES_KHR: Self = Self(1000630001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: Self = Self(1000168000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: Self = Self(1000413000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: Self = Self(1000413001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES: Self = Self(1000470000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES: Self = Self(1000470001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES: Self = Self(1000545000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES: Self = Self(1000545001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_7_FEATURES_KHR: Self = Self(1000562000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_7_PROPERTIES_KHR: Self = Self(1000562001);
    pub const PHYSICAL_DEVICE_MAINTENANCE_8_FEATURES_KHR: Self = Self(1000574000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_9_FEATURES_KHR: Self = Self(1000584000);
    pub const PHYSICAL_DEVICE_MAINTENANCE_9_PROPERTIES_KHR: Self = Self(1000584001);
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_FEATURES_EXT: Self = Self(1000272000);
    pub const PHYSICAL_DEVICE_MAP_MEMORY_PLACED_PROPERTIES_EXT: Self = Self(1000272001);
    pub const PHYSICAL_DEVICE_MEMORY_BUDGET_PROPERTIES_EXT: Self = Self(1000237000);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT: Self = Self(1000427000);
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT: Self = Self(1000427001);
    pub const PHYSICAL_DEVICE_MEMORY_PRIORITY_FEATURES_EXT: Self = Self(1000238000);
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: Self = Self(1000059006);
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_EXT: Self = Self(1000328000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_FEATURES_NV: Self = Self(1000202000);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_EXT: Self = Self(1000328001);
    pub const PHYSICAL_DEVICE_MESH_SHADER_PROPERTIES_NV: Self = Self(1000202001);
    pub const PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_FEATURES_EXT: Self =
        Self(1000376000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES: Self = Self(1000053001);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_ATTRIBUTES_PROPERTIES_NVX: Self = Self(1000097000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_RENDER_AREAS_FEATURES_QCOM: Self =
        Self(1000510000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM: Self = Self(1000488000);
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: Self = Self(1000053002);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_FEATURES_EXT: Self = Self(1000392000);
    pub const PHYSICAL_DEVICE_MULTI_DRAW_PROPERTIES_EXT: Self = Self(1000392001);
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: Self = Self(1000351000);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_FEATURES_EXT: Self = Self(1000451000);
    pub const PHYSICAL_DEVICE_NESTED_COMMAND_BUFFER_PROPERTIES_EXT: Self = Self(1000451001);
    pub const PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT: Self = Self(1000422000);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_FEATURES_EXT: Self = Self(1000396005);
    pub const PHYSICAL_DEVICE_OPACITY_MICROMAP_PROPERTIES_EXT: Self = Self(1000396006);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_FEATURES_NV: Self = Self(1000464000);
    pub const PHYSICAL_DEVICE_OPTICAL_FLOW_PROPERTIES_NV: Self = Self(1000464001);
    pub const PHYSICAL_DEVICE_PAGEABLE_DEVICE_LOCAL_MEMORY_FEATURES_EXT: Self = Self(1000412000);
    pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_FEATURES_NV: Self =
        Self(1000570000);
    pub const PHYSICAL_DEVICE_PARTITIONED_ACCELERATION_STRUCTURE_PROPERTIES_NV: Self =
        Self(1000570001);
    pub const PHYSICAL_DEVICE_PCI_BUS_INFO_PROPERTIES_EXT: Self = Self(1000212000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_FEATURES_ARM: Self = Self(1000605000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_COUNTERS_BY_REGION_PROPERTIES_ARM: Self =
        Self(1000605001);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR: Self = Self(1000116000);
    pub const PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR: Self = Self(1000116001);
    pub const PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV: Self = Self(1000516000);
    pub const PHYSICAL_DEVICE_PIPELINE_BINARY_FEATURES_KHR: Self = Self(1000483000);
    pub const PHYSICAL_DEVICE_PIPELINE_BINARY_PROPERTIES_KHR: Self = Self(1000483004);
    pub const PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC: Self = Self(1000637000);
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: Self = Self(1000297000);
    pub const PHYSICAL_DEVICE_PIPELINE_EXECUTABLE_PROPERTIES_FEATURES_KHR: Self = Self(1000269000);
    pub const PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT: Self = Self(1000498000);
    pub const PHYSICAL_DEVICE_PIPELINE_OPACITY_MICROMAP_FEATURES_ARM: Self = Self(1000596000);
    pub const PHYSICAL_DEVICE_PIPELINE_PROPERTIES_FEATURES_EXT: Self = Self(1000372001);
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES: Self = Self(1000466000);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES: Self = Self(1000068001);
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES: Self = Self(1000068002);
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: Self = Self(1000117000);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_FEATURES_KHR: Self = Self(1000163000);
    pub const PHYSICAL_DEVICE_PORTABILITY_SUBSET_PROPERTIES_KHR: Self = Self(1000163001);
    pub const PHYSICAL_DEVICE_PRESENT_BARRIER_FEATURES_NV: Self = Self(1000292000);
    pub const PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR: Self = Self(1000479002);
    pub const PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR: Self = Self(1000294001);
    pub const PHYSICAL_DEVICE_PRESENT_METERING_FEATURES_NV: Self = Self(1000613001);
    pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR: Self = Self(1000361000);
    pub const PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT: Self = Self(1000208000);
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_2_FEATURES_KHR: Self = Self(1000480001);
    pub const PHYSICAL_DEVICE_PRESENT_WAIT_FEATURES_KHR: Self = Self(1000248000);
    pub const PHYSICAL_DEVICE_PRIMITIVES_GENERATED_QUERY_FEATURES_EXT: Self = Self(1000382000);
    pub const PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT: Self = Self(1000356000);
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: Self = Self(1000295000);
    pub const PHYSICAL_DEVICE_PROPERTIES_2: Self = Self(1000059001);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: Self = Self(1000145001);
    pub const PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: Self = Self(1000145002);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT: Self = Self(1000254000);
    pub const PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT: Self = Self(1000254002);
    pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_FEATURES_NV: Self = Self(1000580001);
    pub const PHYSICAL_DEVICE_PUSH_CONSTANT_BANK_PROPERTIES_NV: Self = Self(1000580002);
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: Self = Self(1000080000);
    pub const PHYSICAL_DEVICE_QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_INFO_ARM: Self =
        Self(1000507019);
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: Self =
        Self(1000342000);
    pub const PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV: Self = Self(1000555000);
    pub const PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR: Self = Self(1000348013);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT: Self = Self(1000581000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV: Self = Self(1000490000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT: Self =
        Self(1000581001);
    pub const PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV: Self = Self(1000490001);
    pub const PHYSICAL_DEVICE_RAY_TRACING_LINEAR_SWEPT_SPHERES_FEATURES_NV: Self = Self(1000429008);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000386000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_MOTION_BLUR_FEATURES_NV: Self = Self(1000327001);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_FEATURES_KHR: Self = Self(1000347000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PIPELINE_PROPERTIES_KHR: Self = Self(1000347001);
    pub const PHYSICAL_DEVICE_RAY_TRACING_POSITION_FETCH_FEATURES_KHR: Self = Self(1000481000);
    pub const PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV: Self = Self(1000165009);
    pub const PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV: Self = Self(1000568000);
    pub const PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG: Self = Self(1000110000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_FEATURES_ARM: Self = Self(1000424000);
    pub const PHYSICAL_DEVICE_RENDER_PASS_STRIPED_PROPERTIES_ARM: Self = Self(1000424001);
    pub const PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV: Self = Self(1000166000);
    pub const PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT: Self = Self(1000344000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR: Self = Self(1000286000);
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR: Self = Self(1000286001);
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: Self = Self(1000130000);
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: Self = Self(1000156004);
    pub const PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT: Self = Self(1000143003);
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: Self = Self(1000221000);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_FEATURES_ARM: Self = Self(1000417001);
    pub const PHYSICAL_DEVICE_SCHEDULING_CONTROLS_PROPERTIES_ARM: Self = Self(1000417002);
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: Self = Self(1000241000);
    pub const PHYSICAL_DEVICE_SHADER_64_BIT_INDEXING_FEATURES_EXT: Self = Self(1000627000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT16_VECTOR_FEATURES_NV: Self = Self(1000563000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT: Self = Self(1000273000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT: Self = Self(1000260000);
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: Self = Self(1000180000);
    pub const PHYSICAL_DEVICE_SHADER_BFLOAT16_FEATURES_KHR: Self = Self(1000141000);
    pub const PHYSICAL_DEVICE_SHADER_CLOCK_FEATURES_KHR: Self = Self(1000181000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM: Self = Self(1000497000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM: Self = Self(1000497001);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD: Self = Self(1000227000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD: Self = Self(1000185000);
    pub const PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM: Self = Self(1000415000);
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: Self = Self(1000276000);
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: Self = Self(1000063000);
    pub const PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD: Self =
        Self(1000321000);
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_FEATURES_AMDX: Self = Self(1000134000);
    pub const PHYSICAL_DEVICE_SHADER_ENQUEUE_PROPERTIES_AMDX: Self = Self(1000134001);
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES: Self = Self(1000544000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: Self = Self(1000082000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT8_FEATURES_EXT: Self = Self(1000567000);
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES: Self = Self(1000528000);
    pub const PHYSICAL_DEVICE_SHADER_FMA_FEATURES_KHR: Self = Self(1000579000);
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_ATOMIC_INT64_FEATURES_EXT: Self = Self(1000234000);
    pub const PHYSICAL_DEVICE_SHADER_IMAGE_FOOTPRINT_FEATURES_NV: Self = Self(1000204000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: Self = Self(1000280000);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: Self = Self(1000280001);
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL: Self = Self(1000209000);
    pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_FEATURES_EXT: Self = Self(1000635000);
    pub const PHYSICAL_DEVICE_SHADER_LONG_VECTOR_PROPERTIES_EXT: Self = Self(1000635001);
    pub const PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR: Self = Self(1000434000);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT: Self = Self(1000462000);
    pub const PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT: Self = Self(1000462001);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_FEATURES_EXT: Self = Self(1000482000);
    pub const PHYSICAL_DEVICE_SHADER_OBJECT_PROPERTIES_EXT: Self = Self(1000482001);
    pub const PHYSICAL_DEVICE_SHADER_QUAD_CONTROL_FEATURES_KHR: Self = Self(1000235000);
    pub const PHYSICAL_DEVICE_SHADER_RELAXED_EXTENDED_INSTRUCTION_FEATURES_KHR: Self =
        Self(1000558000);
    pub const PHYSICAL_DEVICE_SHADER_REPLICATED_COMPOSITES_FEATURES_EXT: Self = Self(1000564000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV: Self = Self(1000154000);
    pub const PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV: Self = Self(1000154001);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: Self = Self(1000175000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT: Self = Self(1000662000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES: Self = Self(1000416000);
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR: Self =
        Self(1000323000);
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: Self = Self(1000215000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_FEATURES_EXT: Self = Self(1000395000);
    pub const PHYSICAL_DEVICE_SHADER_TILE_IMAGE_PROPERTIES_EXT: Self = Self(1000395001);
    pub const PHYSICAL_DEVICE_SHADER_UNIFORM_BUFFER_UNSIZED_ARRAY_FEATURES_EXT: Self =
        Self(1000642000);
    pub const PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR: Self = Self(1000387000);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_FEATURES_NV: Self = Self(1000164001);
    pub const PHYSICAL_DEVICE_SHADING_RATE_IMAGE_PROPERTIES_NV: Self = Self(1000164002);
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: Self = Self(1000059008);
    pub const PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: Self = Self(1000094000);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: Self = Self(1000225002);
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: Self = Self(1000225000);
    pub const PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT: Self = Self(1000458000);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_FEATURES_HUAWEI: Self = Self(1000369001);
    pub const PHYSICAL_DEVICE_SUBPASS_SHADING_PROPERTIES_HUAWEI: Self = Self(1000369002);
    pub const PHYSICAL_DEVICE_SURFACE_INFO_2_KHR: Self = Self(1000119000);
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000275000);
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: Self = Self(1000314007);
    pub const PHYSICAL_DEVICE_TENSOR_FEATURES_ARM: Self = Self(1000460009);
    pub const PHYSICAL_DEVICE_TENSOR_PROPERTIES_ARM: Self = Self(1000460004);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_FEATURES_EXT: Self = Self(1000281000);
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: Self = Self(1000281001);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_3D_FEATURES_EXT: Self = Self(1000288000);
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: Self = Self(1000066000);
    pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_FEATURES_QCOM: Self = Self(1000547000);
    pub const PHYSICAL_DEVICE_TILE_MEMORY_HEAP_PROPERTIES_QCOM: Self = Self(1000547001);
    pub const PHYSICAL_DEVICE_TILE_PROPERTIES_FEATURES_QCOM: Self = Self(1000484000);
    pub const PHYSICAL_DEVICE_TILE_SHADING_FEATURES_QCOM: Self = Self(1000309000);
    pub const PHYSICAL_DEVICE_TILE_SHADING_PROPERTIES_QCOM: Self = Self(1000309001);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: Self = Self(1000207000);
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: Self = Self(1000207001);
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES: Self = Self(1000245000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT: Self = Self(1000028000);
    pub const PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT: Self = Self(1000028001);
    pub const PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR: Self = Self(1000527000);
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: Self = Self(1000253000);
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: Self = Self(1000120000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: Self = Self(1000190002);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: Self = Self(1000525000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT: Self = Self(1000190000);
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_ROBUSTNESS_FEATURES_EXT: Self = Self(1000608000);
    pub const PHYSICAL_DEVICE_VERTEX_INPUT_DYNAMIC_STATE_FEATURES_EXT: Self = Self(1000352000);
    pub const PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR: Self = Self(1000514000);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR: Self = Self(1000513004);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR: Self = Self(1000552004);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299006);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_QUANTIZATION_MAP_FEATURES_KHR: Self = Self(1000553009);
    pub const PHYSICAL_DEVICE_VIDEO_ENCODE_RGB_CONVERSION_FEATURES_VALVE: Self = Self(1000390000);
    pub const PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR: Self = Self(1000023014);
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR: Self = Self(1000515000);
    pub const PHYSICAL_DEVICE_VIDEO_MAINTENANCE_2_FEATURES_KHR: Self = Self(1000586000);
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: Self = Self(1000211000);
    pub const PHYSICAL_DEVICE_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_FEATURES_KHR: Self =
        Self(1000336000);
    pub const PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT: Self = Self(1000330000);
    pub const PHYSICAL_DEVICE_YCBCR_DEGAMMA_FEATURES_QCOM: Self = Self(1000520000);
    pub const PHYSICAL_DEVICE_YCBCR_IMAGE_ARRAYS_FEATURES_EXT: Self = Self(1000252000);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_DEVICE_MEMORY_FEATURES_EXT: Self = Self(1000620000);
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: Self = Self(1000325000);
    pub const PIPELINE_BINARY_CREATE_INFO_KHR: Self = Self(1000483001);
    pub const PIPELINE_BINARY_DATA_INFO_KHR: Self = Self(1000483006);
    pub const PIPELINE_BINARY_HANDLES_INFO_KHR: Self = Self(1000483009);
    pub const PIPELINE_BINARY_INFO_KHR: Self = Self(1000483002);
    pub const PIPELINE_BINARY_KEY_KHR: Self = Self(1000483003);
    pub const PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT: Self = Self(1000148002);
    pub const PIPELINE_COLOR_WRITE_CREATE_INFO_EXT: Self = Self(1000381001);
    pub const PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD: Self = Self(1000183000);
    pub const PIPELINE_COVERAGE_MODULATION_STATE_CREATE_INFO_NV: Self = Self(1000152000);
    pub const PIPELINE_COVERAGE_REDUCTION_STATE_CREATE_INFO_NV: Self = Self(1000250001);
    pub const PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV: Self = Self(1000149000);
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO: Self = Self(1000470005);
    pub const PIPELINE_CREATE_INFO_KHR: Self = Self(1000483007);
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO: Self = Self(1000192000);
    pub const PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT: Self = Self(1000099001);
    pub const PIPELINE_EXECUTABLE_INFO_KHR: Self = Self(1000269003);
    pub const PIPELINE_EXECUTABLE_INTERNAL_REPRESENTATION_KHR: Self = Self(1000269005);
    pub const PIPELINE_EXECUTABLE_PROPERTIES_KHR: Self = Self(1000269002);
    pub const PIPELINE_EXECUTABLE_STATISTIC_KHR: Self = Self(1000269004);
    pub const PIPELINE_FRAGMENT_DENSITY_MAP_LAYERED_CREATE_INFO_VALVE: Self = Self(1000611002);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_ENUM_STATE_CREATE_INFO_NV: Self = Self(1000326002);
    pub const PIPELINE_FRAGMENT_SHADING_RATE_STATE_CREATE_INFO_KHR: Self = Self(1000226001);
    pub const PIPELINE_INDIRECT_DEVICE_ADDRESS_INFO_NV: Self = Self(1000428002);
    pub const PIPELINE_INFO_KHR: Self = Self(1000269001);
    pub const PIPELINE_LIBRARY_CREATE_INFO_KHR: Self = Self(1000290000);
    pub const PIPELINE_PROPERTIES_IDENTIFIER_EXT: Self = Self(1000372000);
    pub const PIPELINE_RASTERIZATION_CONSERVATIVE_STATE_CREATE_INFO_EXT: Self = Self(1000101001);
    pub const PIPELINE_RASTERIZATION_DEPTH_CLIP_STATE_CREATE_INFO_EXT: Self = Self(1000102001);
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: Self = Self(1000259001);
    pub const PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT: Self =
        Self(1000254001);
    pub const PIPELINE_RASTERIZATION_STATE_RASTERIZATION_ORDER_AMD: Self = Self(1000018000);
    pub const PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT: Self = Self(1000028002);
    pub const PIPELINE_RENDERING_CREATE_INFO: Self = Self(1000044002);
    pub const PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV: Self = Self(1000166001);
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO: Self = Self(1000068000);
    pub const PIPELINE_SAMPLE_LOCATIONS_STATE_CREATE_INFO_EXT: Self = Self(1000143002);
    pub const PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT: Self = Self(1000462002);
    pub const PIPELINE_SHADER_STAGE_NODE_CREATE_INFO_AMDX: Self = Self(1000134004);
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: Self = Self(1000225001);
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: Self = Self(1000117003);
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: Self = Self(1000190001);
    pub const PIPELINE_VIEWPORT_COARSE_SAMPLE_ORDER_STATE_CREATE_INFO_NV: Self = Self(1000164005);
    pub const PIPELINE_VIEWPORT_DEPTH_CLAMP_CONTROL_CREATE_INFO_EXT: Self = Self(1000582001);
    pub const PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT: Self = Self(1000355001);
    pub const PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV: Self = Self(1000205000);
    pub const PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV: Self = Self(1000164000);
    pub const PIPELINE_VIEWPORT_SWIZZLE_STATE_CREATE_INFO_NV: Self = Self(1000098000);
    pub const PIPELINE_VIEWPORT_W_SCALING_STATE_CREATE_INFO_NV: Self = Self(1000087000);
    pub const PRESENT_FRAME_TOKEN_GGP: Self = Self(1000191000);
    pub const PRESENT_ID_2_KHR: Self = Self(1000479001);
    pub const PRESENT_ID_KHR: Self = Self(1000294000);
    pub const PRESENT_INFO_KHR: Self = Self(1000001001);
    pub const PRESENT_REGIONS_KHR: Self = Self(1000084000);
    pub const PRESENT_TIMES_INFO_GOOGLE: Self = Self(1000092000);
    pub const PRESENT_TIMINGS_INFO_EXT: Self = Self(1000208003);
    pub const PRESENT_TIMING_INFO_EXT: Self = Self(1000208004);
    pub const PRESENT_TIMING_SURFACE_CAPABILITIES_EXT: Self = Self(1000208008);
    pub const PRESENT_WAIT_2_INFO_KHR: Self = Self(1000480002);
    pub const PRIVATE_DATA_SLOT_CREATE_INFO: Self = Self(1000295002);
    pub const PROTECTED_SUBMIT_INFO: Self = Self(1000145000);
    pub const PUSH_CONSTANTS_INFO: Self = Self(1000545004);
    pub const PUSH_CONSTANT_BANK_INFO_NV: Self = Self(1000580000);
    pub const PUSH_DATA_INFO_EXT: Self = Self(1000135004);
    pub const PUSH_DESCRIPTOR_SET_INFO: Self = Self(1000545005);
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO: Self = Self(1000545006);
    pub const QUERY_LOW_LATENCY_SUPPORT_NV: Self = Self(1000310000);
    pub const QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR: Self = Self(1000116002);
    pub const QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL: Self = Self(1000210000);
    pub const QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR: Self = Self(1000299005);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: Self = Self(1000314008);
    pub const QUEUE_FAMILY_CHECKPOINT_PROPERTIES_NV: Self = Self(1000206001);
    pub const QUEUE_FAMILY_DATA_GRAPH_PROCESSING_ENGINE_PROPERTIES_ARM: Self = Self(1000507017);
    pub const QUEUE_FAMILY_DATA_GRAPH_PROPERTIES_ARM: Self = Self(1000507018);
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: Self = Self(1000388001);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_PROPERTIES_KHR: Self = Self(1000584002);
    pub const QUEUE_FAMILY_PROPERTIES_2: Self = Self(1000059005);
    pub const QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR: Self = Self(1000023016);
    pub const QUEUE_FAMILY_VIDEO_PROPERTIES_KHR: Self = Self(1000023012);
    pub const RAY_TRACING_PIPELINE_CLUSTER_ACCELERATION_STRUCTURE_CREATE_INFO_NV: Self =
        Self(1000569007);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_KHR: Self = Self(1000150015);
    pub const RAY_TRACING_PIPELINE_CREATE_INFO_NV: Self = Self(1000165000);
    pub const RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: Self = Self(1000150018);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: Self = Self(1000150016);
    pub const RAY_TRACING_SHADER_GROUP_CREATE_INFO_NV: Self = Self(1000165011);
    pub const RELEASE_CAPTURED_PIPELINE_DATA_INFO_KHR: Self = Self(1000483005);
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_KHR: Self = Self(1000275005);
    pub const RENDERING_AREA_INFO: Self = Self(1000470003);
    pub const RENDERING_ATTACHMENT_FLAGS_INFO_KHR: Self = Self(1000630002);
    pub const RENDERING_ATTACHMENT_INFO: Self = Self(1000044001);
    pub const RENDERING_ATTACHMENT_LOCATION_INFO: Self = Self(1000232001);
    pub const RENDERING_END_INFO_KHR: Self = Self(1000619003);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: Self = Self(1000044007);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: Self = Self(1000044006);
    pub const RENDERING_INFO: Self = Self(1000044000);
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO: Self = Self(1000232002);
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO: Self = Self(1000108003);
    pub const RENDER_PASS_CREATE_INFO_2: Self = Self(1000109004);
    pub const RENDER_PASS_CREATION_CONTROL_EXT: Self = Self(1000458001);
    pub const RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458002);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_CREATE_INFO_EXT: Self = Self(1000218002);
    pub const RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT: Self = Self(1000425002);
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: Self = Self(1000117001);
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO: Self = Self(1000053000);
    pub const RENDER_PASS_PERFORMANCE_COUNTERS_BY_REGION_BEGIN_INFO_ARM: Self = Self(1000605004);
    pub const RENDER_PASS_SAMPLE_LOCATIONS_BEGIN_INFO_EXT: Self = Self(1000143001);
    pub const RENDER_PASS_STRIPE_BEGIN_INFO_ARM: Self = Self(1000424002);
    pub const RENDER_PASS_STRIPE_INFO_ARM: Self = Self(1000424003);
    pub const RENDER_PASS_STRIPE_SUBMIT_INFO_ARM: Self = Self(1000424004);
    pub const RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT: Self = Self(1000458003);
    pub const RENDER_PASS_TILE_SHADING_CREATE_INFO_QCOM: Self = Self(1000309002);
    pub const RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM: Self = Self(1000282001);
    pub const RESOLVE_IMAGE_INFO_2: Self = Self(1000337005);
    pub const RESOLVE_IMAGE_MODE_INFO_KHR: Self = Self(1000630004);
    pub const RESOURCE_DESCRIPTOR_INFO_EXT: Self = Self(1000135002);
    pub const SAMPLER_BLOCK_MATCH_WINDOW_CREATE_INFO_QCOM: Self = Self(1000518002);
    pub const SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT: Self = Self(1000411001);
    pub const SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT: Self = Self(1000316008);
    pub const SAMPLER_CUBIC_WEIGHTS_CREATE_INFO_QCOM: Self = Self(1000519000);
    pub const SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT: Self = Self(1000287000);
    pub const SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT: Self = Self(1000135011);
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO: Self = Self(1000130001);
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO: Self = Self(1000156000);
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: Self = Self(1000156005);
    pub const SAMPLER_YCBCR_CONVERSION_INFO: Self = Self(1000156001);
    pub const SAMPLER_YCBCR_CONVERSION_YCBCR_DEGAMMA_CREATE_INFO_QCOM: Self = Self(1000520001);
    pub const SAMPLE_LOCATIONS_INFO_EXT: Self = Self(1000143000);
    pub const SCREEN_BUFFER_FORMAT_PROPERTIES_QNX: Self = Self(1000529001);
    pub const SCREEN_BUFFER_PROPERTIES_QNX: Self = Self(1000529000);
    pub const SCREEN_SURFACE_CREATE_INFO_QNX: Self = Self(1000378000);
    pub const SEMAPHORE_GET_FD_INFO_KHR: Self = Self(1000079001);
    pub const SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR: Self = Self(1000078003);
    pub const SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA: Self = Self(1000365001);
    pub const SEMAPHORE_SIGNAL_INFO: Self = Self(1000207005);
    pub const SEMAPHORE_SUBMIT_INFO: Self = Self(1000314005);
    pub const SEMAPHORE_TYPE_CREATE_INFO: Self = Self(1000207002);
    pub const SEMAPHORE_WAIT_INFO: Self = Self(1000207004);
    pub const SET_DESCRIPTOR_BUFFER_OFFSETS_INFO_EXT: Self = Self(1000545007);
    pub const SET_LATENCY_MARKER_INFO_NV: Self = Self(1000505002);
    pub const SET_PRESENT_CONFIG_NV: Self = Self(1000613000);
    pub const SHADER_CREATE_INFO_EXT: Self = Self(1000482002);
    pub const SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT: Self = Self(1000135006);
    pub const SHADER_MODULE_IDENTIFIER_EXT: Self = Self(1000462003);
    pub const SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160001);
    pub const SHARED_PRESENT_SURFACE_CAPABILITIES_KHR: Self = Self(1000111000);
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2: Self = Self(1000059007);
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: Self = Self(1000146004);
    pub const STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP: Self = Self(1000049000);
    pub const SUBMIT_INFO_2: Self = Self(1000314004);
    pub const SUBPASS_BEGIN_INFO: Self = Self(1000109005);
    pub const SUBPASS_DEPENDENCY_2: Self = Self(1000109003);
    pub const SUBPASS_DESCRIPTION_2: Self = Self(1000109002);
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: Self = Self(1000199001);
    pub const SUBPASS_END_INFO: Self = Self(1000109006);
    pub const SUBPASS_RESOLVE_PERFORMANCE_QUERY_EXT: Self = Self(1000376001);
    pub const SUBPASS_SHADING_PIPELINE_CREATE_INFO_HUAWEI: Self = Self(1000369000);
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE: Self = Self(1000270008);
    pub const SUBRESOURCE_LAYOUT_2: Self = Self(1000338002);
    pub const SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT: Self = Self(1000135013);
    pub const SURFACE_CAPABILITIES_2_EXT: Self = Self(1000090000);
    pub const SURFACE_CAPABILITIES_2_KHR: Self = Self(1000119001);
    pub const SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT: Self = Self(1000255002);
    pub const SURFACE_CAPABILITIES_PRESENT_BARRIER_NV: Self = Self(1000292001);
    pub const SURFACE_CAPABILITIES_PRESENT_ID_2_KHR: Self = Self(1000479000);
    pub const SURFACE_CAPABILITIES_PRESENT_WAIT_2_KHR: Self = Self(1000480000);
    pub const SURFACE_FORMAT_2_KHR: Self = Self(1000119002);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT: Self = Self(1000255000);
    pub const SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT: Self = Self(1000255001);
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_KHR: Self = Self(1000274002);
    pub const SURFACE_PRESENT_MODE_KHR: Self = Self(1000274000);
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_KHR: Self = Self(1000274001);
    pub const SURFACE_PROTECTED_CAPABILITIES_KHR: Self = Self(1000239000);
    pub const SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self(1000208009);
    pub const SWAPCHAIN_COUNTER_CREATE_INFO_EXT: Self = Self(1000091003);
    pub const SWAPCHAIN_CREATE_INFO_KHR: Self = Self(1000001000);
    pub const SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD: Self = Self(1000213001);
    pub const SWAPCHAIN_LATENCY_CREATE_INFO_NV: Self = Self(1000505007);
    pub const SWAPCHAIN_PRESENT_BARRIER_CREATE_INFO_NV: Self = Self(1000292002);
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_KHR: Self = Self(1000275001);
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR: Self = Self(1000275002);
    pub const SWAPCHAIN_PRESENT_MODE_INFO_KHR: Self = Self(1000275003);
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR: Self = Self(1000275004);
    pub const SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT: Self = Self(1000208002);
    pub const SWAPCHAIN_TIMING_PROPERTIES_EXT: Self = Self(1000208001);
    pub const SYSMEM_COLOR_SPACE_FUCHSIA: Self = Self(1000366008);
    pub const TENSOR_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460021);
    pub const TENSOR_COPY_ARM: Self = Self(1000460012);
    pub const TENSOR_CREATE_INFO_ARM: Self = Self(1000460000);
    pub const TENSOR_DEPENDENCY_INFO_ARM: Self = Self(1000460013);
    pub const TENSOR_DESCRIPTION_ARM: Self = Self(1000460006);
    pub const TENSOR_FORMAT_PROPERTIES_ARM: Self = Self(1000460005);
    pub const TENSOR_MEMORY_BARRIER_ARM: Self = Self(1000460008);
    pub const TENSOR_MEMORY_REQUIREMENTS_INFO_ARM: Self = Self(1000460007);
    pub const TENSOR_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_ARM: Self = Self(1000460022);
    pub const TENSOR_VIEW_CREATE_INFO_ARM: Self = Self(1000460001);
    pub const TEXEL_BUFFER_DESCRIPTOR_INFO_EXT: Self = Self(1000135000);
    pub const TEXTURE_LOD_GATHER_FORMAT_PROPERTIES_AMD: Self = Self(1000041000);
    pub const TILE_MEMORY_BIND_INFO_QCOM: Self = Self(1000547003);
    pub const TILE_MEMORY_REQUIREMENTS_QCOM: Self = Self(1000547002);
    pub const TILE_MEMORY_SIZE_INFO_QCOM: Self = Self(1000547004);
    pub const TILE_PROPERTIES_QCOM: Self = Self(1000484001);
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO: Self = Self(1000207003);
    pub const VALIDATION_CACHE_CREATE_INFO_EXT: Self = Self(1000160000);
    pub const VALIDATION_FEATURES_EXT: Self = Self(1000247000);
    pub const VALIDATION_FLAGS_EXT: Self = Self(1000061000);
    pub const VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: Self = Self(1000352002);
    pub const VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: Self = Self(1000352001);
    pub const VIDEO_BEGIN_CODING_INFO_KHR: Self = Self(1000023008);
    pub const VIDEO_CAPABILITIES_KHR: Self = Self(1000023001);
    pub const VIDEO_CODING_CONTROL_INFO_KHR: Self = Self(1000023010);
    pub const VIDEO_DECODE_AV1_CAPABILITIES_KHR: Self = Self(1000512000);
    pub const VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000512005);
    pub const VIDEO_DECODE_AV1_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586003);
    pub const VIDEO_DECODE_AV1_PICTURE_INFO_KHR: Self = Self(1000512001);
    pub const VIDEO_DECODE_AV1_PROFILE_INFO_KHR: Self = Self(1000512003);
    pub const VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000512004);
    pub const VIDEO_DECODE_CAPABILITIES_KHR: Self = Self(1000024001);
    pub const VIDEO_DECODE_H264_CAPABILITIES_KHR: Self = Self(1000040000);
    pub const VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000040006);
    pub const VIDEO_DECODE_H264_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586001);
    pub const VIDEO_DECODE_H264_PICTURE_INFO_KHR: Self = Self(1000040001);
    pub const VIDEO_DECODE_H264_PROFILE_INFO_KHR: Self = Self(1000040003);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000040005);
    pub const VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000040004);
    pub const VIDEO_DECODE_H265_CAPABILITIES_KHR: Self = Self(1000187000);
    pub const VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000187005);
    pub const VIDEO_DECODE_H265_INLINE_SESSION_PARAMETERS_INFO_KHR: Self = Self(1000586002);
    pub const VIDEO_DECODE_H265_PICTURE_INFO_KHR: Self = Self(1000187004);
    pub const VIDEO_DECODE_H265_PROFILE_INFO_KHR: Self = Self(1000187003);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000187002);
    pub const VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000187001);
    pub const VIDEO_DECODE_INFO_KHR: Self = Self(1000024000);
    pub const VIDEO_DECODE_USAGE_INFO_KHR: Self = Self(1000024002);
    pub const VIDEO_DECODE_VP9_CAPABILITIES_KHR: Self = Self(1000514001);
    pub const VIDEO_DECODE_VP9_PICTURE_INFO_KHR: Self = Self(1000514002);
    pub const VIDEO_DECODE_VP9_PROFILE_INFO_KHR: Self = Self(1000514003);
    pub const VIDEO_ENCODE_AV1_CAPABILITIES_KHR: Self = Self(1000513000);
    pub const VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR: Self = Self(1000513003);
    pub const VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000513010);
    pub const VIDEO_ENCODE_AV1_PICTURE_INFO_KHR: Self = Self(1000513002);
    pub const VIDEO_ENCODE_AV1_PROFILE_INFO_KHR: Self = Self(1000513005);
    pub const VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000513008);
    pub const VIDEO_ENCODE_AV1_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553007);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR: Self = Self(1000513006);
    pub const VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000513007);
    pub const VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR: Self = Self(1000513009);
    pub const VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000513001);
    pub const VIDEO_ENCODE_CAPABILITIES_KHR: Self = Self(1000299003);
    pub const VIDEO_ENCODE_H264_CAPABILITIES_KHR: Self = Self(1000038000);
    pub const VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR: Self = Self(1000038004);
    pub const VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000038006);
    pub const VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR: Self = Self(1000038005);
    pub const VIDEO_ENCODE_H264_PICTURE_INFO_KHR: Self = Self(1000038003);
    pub const VIDEO_ENCODE_H264_PROFILE_INFO_KHR: Self = Self(1000038007);
    pub const VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000038011);
    pub const VIDEO_ENCODE_H264_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553003);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR: Self = Self(1000038008);
    pub const VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000038009);
    pub const VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR: Self = Self(1000038010);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000038002);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000038001);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000038013);
    pub const VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000038012);
    pub const VIDEO_ENCODE_H265_CAPABILITIES_KHR: Self = Self(1000039000);
    pub const VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR: Self = Self(1000039004);
    pub const VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR: Self = Self(1000039006);
    pub const VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR: Self = Self(1000039005);
    pub const VIDEO_ENCODE_H265_PICTURE_INFO_KHR: Self = Self(1000039003);
    pub const VIDEO_ENCODE_H265_PROFILE_INFO_KHR: Self = Self(1000039007);
    pub const VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000039012);
    pub const VIDEO_ENCODE_H265_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553004);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR: Self = Self(1000039009);
    pub const VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000039010);
    pub const VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR: Self = Self(1000039011);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR: Self = Self(1000039002);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000039001);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000039014);
    pub const VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000039013);
    pub const VIDEO_ENCODE_INFO_KHR: Self = Self(1000299000);
    pub const VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR: Self = Self(1000552000);
    pub const VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552002);
    pub const VIDEO_ENCODE_PROFILE_RGB_CONVERSION_INFO_VALVE: Self = Self(1000390002);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR: Self = Self(1000299008);
    pub const VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR: Self = Self(1000299007);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_CAPABILITIES_KHR: Self = Self(1000553000);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_INFO_KHR: Self = Self(1000553002);
    pub const VIDEO_ENCODE_QUANTIZATION_MAP_SESSION_PARAMETERS_CREATE_INFO_KHR: Self =
        Self(1000553005);
    pub const VIDEO_ENCODE_RATE_CONTROL_INFO_KHR: Self = Self(1000299001);
    pub const VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR: Self = Self(1000299002);
    pub const VIDEO_ENCODE_RGB_CONVERSION_CAPABILITIES_VALVE: Self = Self(1000390001);
    pub const VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR: Self = Self(1000552001);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR: Self = Self(1000299010);
    pub const VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR: Self = Self(1000299009);
    pub const VIDEO_ENCODE_SESSION_RGB_CONVERSION_CREATE_INFO_VALVE: Self = Self(1000390003);
    pub const VIDEO_ENCODE_USAGE_INFO_KHR: Self = Self(1000299004);
    pub const VIDEO_END_CODING_INFO_KHR: Self = Self(1000023009);
    pub const VIDEO_FORMAT_AV1_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553008);
    pub const VIDEO_FORMAT_H265_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553006);
    pub const VIDEO_FORMAT_PROPERTIES_KHR: Self = Self(1000023015);
    pub const VIDEO_FORMAT_QUANTIZATION_MAP_PROPERTIES_KHR: Self = Self(1000553001);
    pub const VIDEO_INLINE_QUERY_INFO_KHR: Self = Self(1000515001);
    pub const VIDEO_PICTURE_RESOURCE_INFO_KHR: Self = Self(1000023002);
    pub const VIDEO_PROFILE_INFO_KHR: Self = Self(1000023000);
    pub const VIDEO_PROFILE_LIST_INFO_KHR: Self = Self(1000023013);
    pub const VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR: Self = Self(1000552003);
    pub const VIDEO_REFERENCE_SLOT_INFO_KHR: Self = Self(1000023011);
    pub const VIDEO_SESSION_CREATE_INFO_KHR: Self = Self(1000023005);
    pub const VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR: Self = Self(1000023003);
    pub const VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR: Self = Self(1000023006);
    pub const VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR: Self = Self(1000023007);
    pub const VI_SURFACE_CREATE_INFO_NN: Self = Self(1000062000);
    pub const WAYLAND_SURFACE_CREATE_INFO_KHR: Self = Self(1000006000);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_KHR: Self = Self(1000075000);
    pub const WIN32_KEYED_MUTEX_ACQUIRE_RELEASE_INFO_NV: Self = Self(1000058000);
    pub const WIN32_SURFACE_CREATE_INFO_KHR: Self = Self(1000009000);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR: Self = Self(1000150007);
    pub const WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_NV: Self = Self(1000165007);
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: Self = Self(1000138002);
    pub const WRITE_DESCRIPTOR_SET_PARTITIONED_ACCELERATION_STRUCTURE_NV: Self = Self(1000570002);
    pub const WRITE_DESCRIPTOR_SET_TENSOR_ARM: Self = Self(1000460003);
    pub const WRITE_INDIRECT_EXECUTION_SET_PIPELINE_EXT: Self = Self(1000572008);
    pub const WRITE_INDIRECT_EXECUTION_SET_SHADER_EXT: Self = Self(1000572009);
    pub const XCB_SURFACE_CREATE_INFO_KHR: Self = Self(1000005000);
    pub const XLIB_SURFACE_CREATE_INFO_KHR: Self = Self(1000004000);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_FEATURES: Self = Self(49);
    pub const PHYSICAL_DEVICE_VULKAN_1_1_PROPERTIES: Self = Self(50);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_FEATURES: Self = Self(51);
    pub const PHYSICAL_DEVICE_VULKAN_1_2_PROPERTIES: Self = Self(52);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_FEATURES: Self = Self(53);
    pub const PHYSICAL_DEVICE_VULKAN_1_3_PROPERTIES: Self = Self(54);
    pub const PHYSICAL_DEVICE_VULKAN_1_4_FEATURES: Self = Self(55);
    pub const PHYSICAL_DEVICE_VULKAN_1_4_PROPERTIES: Self = Self(56);
    pub const SURFACE_CREATE_INFO_OHOS: Self = Self(1000685000);
    pub const ATTACHMENT_DESCRIPTION_2_KHR: Self = Self::ATTACHMENT_DESCRIPTION_2;
    pub const ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT_KHR: Self =
        Self::ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT;
    pub const ATTACHMENT_REFERENCE_2_KHR: Self = Self::ATTACHMENT_REFERENCE_2;
    pub const ATTACHMENT_REFERENCE_STENCIL_LAYOUT_KHR: Self =
        Self::ATTACHMENT_REFERENCE_STENCIL_LAYOUT;
    pub const ATTACHMENT_SAMPLE_COUNT_INFO_NV: Self = Self::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
    pub const BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
        Self::BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO;
    pub const BIND_BUFFER_MEMORY_INFO_KHR: Self = Self::BIND_BUFFER_MEMORY_INFO;
    pub const BIND_DESCRIPTOR_SETS_INFO_KHR: Self = Self::BIND_DESCRIPTOR_SETS_INFO;
    pub const BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO_KHR: Self =
        Self::BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO;
    pub const BIND_IMAGE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_MEMORY_INFO;
    pub const BIND_IMAGE_PLANE_MEMORY_INFO_KHR: Self = Self::BIND_IMAGE_PLANE_MEMORY_INFO;
    pub const BIND_MEMORY_STATUS_KHR: Self = Self::BIND_MEMORY_STATUS;
    pub const BLIT_IMAGE_INFO_2_KHR: Self = Self::BLIT_IMAGE_INFO_2;
    pub const BUFFER_COPY_2_KHR: Self = Self::BUFFER_COPY_2;
    pub const BUFFER_DEVICE_ADDRESS_INFO_EXT: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_DEVICE_ADDRESS_INFO_KHR: Self = Self::BUFFER_DEVICE_ADDRESS_INFO;
    pub const BUFFER_IMAGE_COPY_2_KHR: Self = Self::BUFFER_IMAGE_COPY_2;
    pub const BUFFER_MEMORY_BARRIER_2_KHR: Self = Self::BUFFER_MEMORY_BARRIER_2;
    pub const BUFFER_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::BUFFER_MEMORY_REQUIREMENTS_INFO_2;
    pub const BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO_KHR: Self =
        Self::BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO;
    pub const BUFFER_USAGE_FLAGS_2_CREATE_INFO_KHR: Self = Self::BUFFER_USAGE_FLAGS_2_CREATE_INFO;
    pub const CALIBRATED_TIMESTAMP_INFO_EXT: Self = Self::CALIBRATED_TIMESTAMP_INFO_KHR;
    pub const COMMAND_BUFFER_INHERITANCE_RENDERING_INFO_KHR: Self =
        Self::COMMAND_BUFFER_INHERITANCE_RENDERING_INFO;
    pub const COMMAND_BUFFER_SUBMIT_INFO_KHR: Self = Self::COMMAND_BUFFER_SUBMIT_INFO;
    pub const COPY_BUFFER_INFO_2_KHR: Self = Self::COPY_BUFFER_INFO_2;
    pub const COPY_BUFFER_TO_IMAGE_INFO_2_KHR: Self = Self::COPY_BUFFER_TO_IMAGE_INFO_2;
    pub const COPY_IMAGE_INFO_2_KHR: Self = Self::COPY_IMAGE_INFO_2;
    pub const COPY_IMAGE_TO_BUFFER_INFO_2_KHR: Self = Self::COPY_IMAGE_TO_BUFFER_INFO_2;
    pub const COPY_IMAGE_TO_IMAGE_INFO_EXT: Self = Self::COPY_IMAGE_TO_IMAGE_INFO;
    pub const COPY_IMAGE_TO_MEMORY_INFO_EXT: Self = Self::COPY_IMAGE_TO_MEMORY_INFO;
    pub const COPY_MEMORY_TO_IMAGE_INFO_EXT: Self = Self::COPY_MEMORY_TO_IMAGE_INFO;
    pub const DEBUG_REPORT_CREATE_INFO_EXT: Self = Self::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT;
    pub const DEPENDENCY_INFO_KHR: Self = Self::DEPENDENCY_INFO;
    pub const DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO_EXT: Self =
        Self::DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO;
    pub const DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO_EXT: Self =
        Self::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO;
    pub const DESCRIPTOR_SET_LAYOUT_SUPPORT_KHR: Self = Self::DESCRIPTOR_SET_LAYOUT_SUPPORT;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO_EXT: Self =
        Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO;
    pub const DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT_EXT: Self =
        Self::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT;
    pub const DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR: Self =
        Self::DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO;
    pub const DEVICE_BUFFER_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_BUFFER_MEMORY_REQUIREMENTS;
    pub const DEVICE_GROUP_BIND_SPARSE_INFO_KHR: Self = Self::DEVICE_GROUP_BIND_SPARSE_INFO;
    pub const DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO_KHR: Self =
        Self::DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO;
    pub const DEVICE_GROUP_DEVICE_CREATE_INFO_KHR: Self = Self::DEVICE_GROUP_DEVICE_CREATE_INFO;
    pub const DEVICE_GROUP_RENDER_PASS_BEGIN_INFO_KHR: Self =
        Self::DEVICE_GROUP_RENDER_PASS_BEGIN_INFO;
    pub const DEVICE_GROUP_SUBMIT_INFO_KHR: Self = Self::DEVICE_GROUP_SUBMIT_INFO;
    pub const DEVICE_IMAGE_MEMORY_REQUIREMENTS_KHR: Self = Self::DEVICE_IMAGE_MEMORY_REQUIREMENTS;
    pub const DEVICE_IMAGE_SUBRESOURCE_INFO_KHR: Self = Self::DEVICE_IMAGE_SUBRESOURCE_INFO;
    pub const DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO_KHR: Self =
        Self::DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO;
    pub const DEVICE_PRIVATE_DATA_CREATE_INFO_EXT: Self = Self::DEVICE_PRIVATE_DATA_CREATE_INFO;
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_EXT: Self =
        Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO;
    pub const DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO_KHR: Self =
        Self::DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO;
    pub const EXPORT_FENCE_CREATE_INFO_KHR: Self = Self::EXPORT_FENCE_CREATE_INFO;
    pub const EXPORT_MEMORY_ALLOCATE_INFO_KHR: Self = Self::EXPORT_MEMORY_ALLOCATE_INFO;
    pub const EXPORT_SEMAPHORE_CREATE_INFO_KHR: Self = Self::EXPORT_SEMAPHORE_CREATE_INFO;
    pub const EXTERNAL_BUFFER_PROPERTIES_KHR: Self = Self::EXTERNAL_BUFFER_PROPERTIES;
    pub const EXTERNAL_FENCE_PROPERTIES_KHR: Self = Self::EXTERNAL_FENCE_PROPERTIES;
    pub const EXTERNAL_IMAGE_FORMAT_PROPERTIES_KHR: Self = Self::EXTERNAL_IMAGE_FORMAT_PROPERTIES;
    pub const EXTERNAL_MEMORY_BUFFER_CREATE_INFO_KHR: Self =
        Self::EXTERNAL_MEMORY_BUFFER_CREATE_INFO;
    pub const EXTERNAL_MEMORY_IMAGE_CREATE_INFO_KHR: Self = Self::EXTERNAL_MEMORY_IMAGE_CREATE_INFO;
    pub const EXTERNAL_SEMAPHORE_PROPERTIES_KHR: Self = Self::EXTERNAL_SEMAPHORE_PROPERTIES;
    pub const FORMAT_PROPERTIES_2_KHR: Self = Self::FORMAT_PROPERTIES_2;
    pub const FORMAT_PROPERTIES_3_KHR: Self = Self::FORMAT_PROPERTIES_3;
    pub const FRAMEBUFFER_ATTACHMENTS_CREATE_INFO_KHR: Self =
        Self::FRAMEBUFFER_ATTACHMENTS_CREATE_INFO;
    pub const FRAMEBUFFER_ATTACHMENT_IMAGE_INFO_KHR: Self = Self::FRAMEBUFFER_ATTACHMENT_IMAGE_INFO;
    pub const HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY_EXT: Self =
        Self::HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY;
    pub const HOST_IMAGE_LAYOUT_TRANSITION_INFO_EXT: Self = Self::HOST_IMAGE_LAYOUT_TRANSITION_INFO;
    pub const IMAGE_BLIT_2_KHR: Self = Self::IMAGE_BLIT_2;
    pub const IMAGE_COPY_2_KHR: Self = Self::IMAGE_COPY_2;
    pub const IMAGE_FORMAT_LIST_CREATE_INFO_KHR: Self = Self::IMAGE_FORMAT_LIST_CREATE_INFO;
    pub const IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::IMAGE_FORMAT_PROPERTIES_2;
    pub const IMAGE_MEMORY_BARRIER_2_KHR: Self = Self::IMAGE_MEMORY_BARRIER_2;
    pub const IMAGE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self = Self::IMAGE_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO_KHR: Self =
        Self::IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO;
    pub const IMAGE_RESOLVE_2_KHR: Self = Self::IMAGE_RESOLVE_2;
    pub const IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2_KHR: Self =
        Self::IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2;
    pub const IMAGE_STENCIL_USAGE_CREATE_INFO_EXT: Self = Self::IMAGE_STENCIL_USAGE_CREATE_INFO;
    pub const IMAGE_SUBRESOURCE_2_EXT: Self = Self::IMAGE_SUBRESOURCE_2;
    pub const IMAGE_SUBRESOURCE_2_KHR: Self = Self::IMAGE_SUBRESOURCE_2;
    pub const IMAGE_TO_MEMORY_COPY_EXT: Self = Self::IMAGE_TO_MEMORY_COPY;
    pub const IMAGE_VIEW_USAGE_CREATE_INFO_KHR: Self = Self::IMAGE_VIEW_USAGE_CREATE_INFO;
    pub const MEMORY_ALLOCATE_FLAGS_INFO_KHR: Self = Self::MEMORY_ALLOCATE_FLAGS_INFO;
    pub const MEMORY_BARRIER_2_KHR: Self = Self::MEMORY_BARRIER_2;
    pub const MEMORY_DEDICATED_ALLOCATE_INFO_KHR: Self = Self::MEMORY_DEDICATED_ALLOCATE_INFO;
    pub const MEMORY_DEDICATED_REQUIREMENTS_KHR: Self = Self::MEMORY_DEDICATED_REQUIREMENTS;
    pub const MEMORY_MAP_INFO_KHR: Self = Self::MEMORY_MAP_INFO;
    pub const MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO_KHR: Self =
        Self::MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO;
    pub const MEMORY_REQUIREMENTS_2_KHR: Self = Self::MEMORY_REQUIREMENTS_2;
    pub const MEMORY_TO_IMAGE_COPY_EXT: Self = Self::MEMORY_TO_IMAGE_COPY;
    pub const MEMORY_UNMAP_INFO_KHR: Self = Self::MEMORY_UNMAP_INFO;
    pub const MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_VALVE: Self =
        Self::MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT;
    pub const PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES;
    pub const PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES;
    pub const PHYSICAL_DEVICE_BUFFER_ADDRESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_EXT;
    pub const PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES;
    pub const PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_NV: Self =
        Self::PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR;
    pub const PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES;
    pub const PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES;
    pub const PHYSICAL_DEVICE_DRIVER_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_DRIVER_PROPERTIES;
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES;
    pub const PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES;
    pub const PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO;
    pub const PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO;
    pub const PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO;
    pub const PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO_KHR: Self =
        Self::PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO;
    pub const PHYSICAL_DEVICE_FEATURES_2_KHR: Self = Self::PHYSICAL_DEVICE_FEATURES_2;
    pub const PHYSICAL_DEVICE_FLOAT16_INT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES;
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_QCOM: Self =
        Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT;
    pub const PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_QCOM: Self =
        Self::PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT;
    pub const PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES;
    pub const PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES;
    pub const PHYSICAL_DEVICE_GROUP_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_GROUP_PROPERTIES;
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES;
    pub const PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES;
    pub const PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES;
    pub const PHYSICAL_DEVICE_ID_PROPERTIES_KHR: Self = Self::PHYSICAL_DEVICE_ID_PROPERTIES;
    pub const PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES;
    pub const PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2_KHR: Self =
        Self::PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2;
    pub const PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES;
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES;
    pub const PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES;
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES;
    pub const PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES;
    pub const PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES;
    pub const PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES;
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_NV: Self =
        Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT;
    pub const PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_NV: Self =
        Self::PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT;
    pub const PHYSICAL_DEVICE_MEMORY_PROPERTIES_2_KHR: Self =
        Self::PHYSICAL_DEVICE_MEMORY_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_MULTIVIEW_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_MULTIVIEW_FEATURES;
    pub const PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES;
    pub const PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_VALVE: Self =
        Self::PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT;
    pub const PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES;
    pub const PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES;
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES;
    pub const PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES;
    pub const PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES;
    pub const PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES;
    pub const PHYSICAL_DEVICE_PROPERTIES_2_KHR: Self = Self::PHYSICAL_DEVICE_PROPERTIES_2;
    pub const PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES;
    pub const PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_ARM: Self =
        Self::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT;
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR;
    pub const PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES;
    pub const PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES;
    pub const PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES;
    pub const PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_DRAW_PARAMETER_FEATURES: Self =
        Self::PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES;
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES;
    pub const PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES;
    pub const PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2_KHR: Self =
        Self::PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2;
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES;
    pub const PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES;
    pub const PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES;
    pub const PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES_EXT: Self =
        Self::PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES;
    pub const PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES;
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES;
    pub const PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES;
    pub const PHYSICAL_DEVICE_TOOL_PROPERTIES_EXT: Self = Self::PHYSICAL_DEVICE_TOOL_PROPERTIES;
    pub const PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES;
    pub const PHYSICAL_DEVICE_VARIABLE_POINTER_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES_KHR;
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT: Self =
        Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES;
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES;
    pub const PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_KHR: Self =
        Self::PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES;
    pub const PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES;
    pub const PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES_KHR: Self =
        Self::PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES;
    pub const PIPELINE_CREATE_FLAGS_2_CREATE_INFO_KHR: Self =
        Self::PIPELINE_CREATE_FLAGS_2_CREATE_INFO;
    pub const PIPELINE_CREATION_FEEDBACK_CREATE_INFO_EXT: Self =
        Self::PIPELINE_CREATION_FEEDBACK_CREATE_INFO;
    pub const PIPELINE_INFO_EXT: Self = Self::PIPELINE_INFO_KHR;
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO;
    pub const PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_KHR: Self =
        Self::PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO;
    pub const PIPELINE_RENDERING_CREATE_INFO_KHR: Self = Self::PIPELINE_RENDERING_CREATE_INFO;
    pub const PIPELINE_ROBUSTNESS_CREATE_INFO_EXT: Self = Self::PIPELINE_ROBUSTNESS_CREATE_INFO;
    pub const PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    pub const PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO_KHR: Self =
        Self::PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO;
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO;
    pub const PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_KHR: Self =
        Self::PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO;
    pub const PRIVATE_DATA_SLOT_CREATE_INFO_EXT: Self = Self::PRIVATE_DATA_SLOT_CREATE_INFO;
    pub const PUSH_CONSTANTS_INFO_KHR: Self = Self::PUSH_CONSTANTS_INFO;
    pub const PUSH_DESCRIPTOR_SET_INFO_KHR: Self = Self::PUSH_DESCRIPTOR_SET_INFO;
    pub const PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO_KHR: Self =
        Self::PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO;
    pub const QUERY_POOL_CREATE_INFO_INTEL: Self =
        Self::QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL;
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_EXT: Self =
        Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES;
    pub const QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES_KHR: Self =
        Self::QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES;
    pub const QUEUE_FAMILY_PROPERTIES_2_KHR: Self = Self::QUEUE_FAMILY_PROPERTIES_2;
    pub const RELEASE_SWAPCHAIN_IMAGES_INFO_EXT: Self = Self::RELEASE_SWAPCHAIN_IMAGES_INFO_KHR;
    pub const RENDERING_AREA_INFO_KHR: Self = Self::RENDERING_AREA_INFO;
    pub const RENDERING_ATTACHMENT_INFO_KHR: Self = Self::RENDERING_ATTACHMENT_INFO;
    pub const RENDERING_ATTACHMENT_LOCATION_INFO_KHR: Self =
        Self::RENDERING_ATTACHMENT_LOCATION_INFO;
    pub const RENDERING_END_INFO_EXT: Self = Self::RENDERING_END_INFO_KHR;
    pub const RENDERING_INFO_KHR: Self = Self::RENDERING_INFO;
    pub const RENDERING_INPUT_ATTACHMENT_INDEX_INFO_KHR: Self =
        Self::RENDERING_INPUT_ATTACHMENT_INDEX_INFO;
    pub const RENDER_PASS_ATTACHMENT_BEGIN_INFO_KHR: Self = Self::RENDER_PASS_ATTACHMENT_BEGIN_INFO;
    pub const RENDER_PASS_CREATE_INFO_2_KHR: Self = Self::RENDER_PASS_CREATE_INFO_2;
    pub const RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO_KHR: Self =
        Self::RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO;
    pub const RENDER_PASS_MULTIVIEW_CREATE_INFO_KHR: Self = Self::RENDER_PASS_MULTIVIEW_CREATE_INFO;
    pub const RESOLVE_IMAGE_INFO_2_KHR: Self = Self::RESOLVE_IMAGE_INFO_2;
    pub const SAMPLER_REDUCTION_MODE_CREATE_INFO_EXT: Self =
        Self::SAMPLER_REDUCTION_MODE_CREATE_INFO;
    pub const SAMPLER_YCBCR_CONVERSION_CREATE_INFO_KHR: Self =
        Self::SAMPLER_YCBCR_CONVERSION_CREATE_INFO;
    pub const SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES_KHR: Self =
        Self::SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES;
    pub const SAMPLER_YCBCR_CONVERSION_INFO_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION_INFO;
    pub const SEMAPHORE_SIGNAL_INFO_KHR: Self = Self::SEMAPHORE_SIGNAL_INFO;
    pub const SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::SEMAPHORE_SUBMIT_INFO;
    pub const SEMAPHORE_TYPE_CREATE_INFO_KHR: Self = Self::SEMAPHORE_TYPE_CREATE_INFO;
    pub const SEMAPHORE_WAIT_INFO_KHR: Self = Self::SEMAPHORE_WAIT_INFO;
    pub const SHADER_REQUIRED_SUBGROUP_SIZE_CREATE_INFO_EXT: Self =
        Self::PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO;
    pub const SPARSE_IMAGE_FORMAT_PROPERTIES_2_KHR: Self = Self::SPARSE_IMAGE_FORMAT_PROPERTIES_2;
    pub const SPARSE_IMAGE_MEMORY_REQUIREMENTS_2_KHR: Self =
        Self::SPARSE_IMAGE_MEMORY_REQUIREMENTS_2;
    pub const SUBMIT_INFO_2_KHR: Self = Self::SUBMIT_INFO_2;
    pub const SUBPASS_BEGIN_INFO_KHR: Self = Self::SUBPASS_BEGIN_INFO;
    pub const SUBPASS_DEPENDENCY_2_KHR: Self = Self::SUBPASS_DEPENDENCY_2;
    pub const SUBPASS_DESCRIPTION_2_KHR: Self = Self::SUBPASS_DESCRIPTION_2;
    pub const SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE_KHR: Self =
        Self::SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE;
    pub const SUBPASS_END_INFO_KHR: Self = Self::SUBPASS_END_INFO;
    pub const SUBPASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_QCOM: Self =
        Self::RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT;
    pub const SUBRESOURCE_HOST_MEMCPY_SIZE_EXT: Self = Self::SUBRESOURCE_HOST_MEMCPY_SIZE;
    pub const SUBRESOURCE_LAYOUT_2_EXT: Self = Self::SUBRESOURCE_LAYOUT_2;
    pub const SUBRESOURCE_LAYOUT_2_KHR: Self = Self::SUBRESOURCE_LAYOUT_2;
    pub const SURFACE_CAPABILITIES2_EXT: Self = Self::SURFACE_CAPABILITIES_2_EXT;
    pub const SURFACE_PRESENT_MODE_COMPATIBILITY_EXT: Self =
        Self::SURFACE_PRESENT_MODE_COMPATIBILITY_KHR;
    pub const SURFACE_PRESENT_MODE_EXT: Self = Self::SURFACE_PRESENT_MODE_KHR;
    pub const SURFACE_PRESENT_SCALING_CAPABILITIES_EXT: Self =
        Self::SURFACE_PRESENT_SCALING_CAPABILITIES_KHR;
    pub const SWAPCHAIN_PRESENT_FENCE_INFO_EXT: Self = Self::SWAPCHAIN_PRESENT_FENCE_INFO_KHR;
    pub const SWAPCHAIN_PRESENT_MODES_CREATE_INFO_EXT: Self =
        Self::SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR;
    pub const SWAPCHAIN_PRESENT_MODE_INFO_EXT: Self = Self::SWAPCHAIN_PRESENT_MODE_INFO_KHR;
    pub const SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_EXT: Self =
        Self::SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR;
    pub const TIMELINE_SEMAPHORE_SUBMIT_INFO_KHR: Self = Self::TIMELINE_SEMAPHORE_SUBMIT_INFO;
    pub const WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK_EXT: Self =
        Self::WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubpassContents(i32);
impl SubpassContents {
    pub const INLINE: Self = Self(0);
    pub const SECONDARY_COMMAND_BUFFERS: Self = Self(1);
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR: Self = Self(1000451000);
    pub const INLINE_AND_SECONDARY_COMMAND_BUFFERS_EXT: Self =
        Self::INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const ERROR_COMPRESSION_EXHAUSTED_EXT: Self = Self(-1000338000);
    pub const ERROR_FRAGMENTATION: Self = Self(-1000161000);
    pub const ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT: Self = Self(-1000255000);
    pub const ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR: Self = Self(-1000023000);
    pub const ERROR_INCOMPATIBLE_DISPLAY_KHR: Self = Self(-1000003001);
    pub const ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT: Self = Self(-1000158000);
    pub const ERROR_INVALID_EXTERNAL_HANDLE: Self = Self(-1000072003);
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: Self = Self(-1000257000);
    pub const ERROR_INVALID_SHADER_NV: Self = Self(-1000012000);
    pub const ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR: Self = Self(-1000299000);
    pub const ERROR_NATIVE_WINDOW_IN_USE_KHR: Self = Self(-1000000001);
    pub const ERROR_NOT_ENOUGH_SPACE_KHR: Self = Self(-1000483000);
    pub const ERROR_NOT_PERMITTED: Self = Self(-1000174001);
    pub const ERROR_OUT_OF_DATE_KHR: Self = Self(-1000001004);
    pub const ERROR_OUT_OF_POOL_MEMORY: Self = Self(-1000069000);
    pub const ERROR_PRESENT_TIMING_QUEUE_FULL_EXT: Self = Self(-1000208000);
    pub const ERROR_SURFACE_LOST_KHR: Self = Self(-1000000000);
    pub const ERROR_VALIDATION_FAILED: Self = Self(-1000011001);
    pub const ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR: Self = Self(-1000023001);
    pub const ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR: Self = Self(-1000023004);
    pub const ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR: Self = Self(-1000023003);
    pub const ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR: Self = Self(-1000023002);
    pub const ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR: Self = Self(-1000023005);
    pub const INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self(1000482000);
    pub const OPERATION_DEFERRED_KHR: Self = Self(1000268002);
    pub const OPERATION_NOT_DEFERRED_KHR: Self = Self(1000268003);
    pub const PIPELINE_BINARY_MISSING_KHR: Self = Self(1000483000);
    pub const PIPELINE_COMPILE_REQUIRED: Self = Self(1000297000);
    pub const SUBOPTIMAL_KHR: Self = Self(1000001003);
    pub const THREAD_DONE_KHR: Self = Self(1000268001);
    pub const THREAD_IDLE_KHR: Self = Self(1000268000);
    pub const ERROR_FRAGMENTATION_EXT: Self = Self::ERROR_FRAGMENTATION;
    pub const ERROR_INCOMPATIBLE_SHADER_BINARY_EXT: Self = Self::INCOMPATIBLE_SHADER_BINARY_EXT;
    pub const ERROR_INVALID_DEVICE_ADDRESS_EXT: Self = Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
    pub const ERROR_INVALID_EXTERNAL_HANDLE_KHR: Self = Self::ERROR_INVALID_EXTERNAL_HANDLE;
    pub const ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR: Self =
        Self::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
    pub const ERROR_NOT_PERMITTED_EXT: Self = Self::ERROR_NOT_PERMITTED;
    pub const ERROR_NOT_PERMITTED_KHR: Self = Self::ERROR_NOT_PERMITTED;
    pub const ERROR_OUT_OF_POOL_MEMORY_KHR: Self = Self::ERROR_OUT_OF_POOL_MEMORY;
    pub const ERROR_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
    pub const ERROR_VALIDATION_FAILED_EXT: Self = Self::ERROR_VALIDATION_FAILED;
    pub const PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::PIPELINE_COMPILE_REQUIRED;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynamicState(i32);
impl DynamicState {
    pub const VIEWPORT: Self = Self(0);
    pub const SCISSOR: Self = Self(1);
    pub const LINE_WIDTH: Self = Self(2);
    pub const DEPTH_BIAS: Self = Self(3);
    pub const BLEND_CONSTANTS: Self = Self(4);
    pub const DEPTH_BOUNDS: Self = Self(5);
    pub const STENCIL_COMPARE_MASK: Self = Self(6);
    pub const STENCIL_WRITE_MASK: Self = Self(7);
    pub const STENCIL_REFERENCE: Self = Self(8);
    pub const ALPHA_TO_COVERAGE_ENABLE_EXT: Self = Self(1000455007);
    pub const ALPHA_TO_ONE_ENABLE_EXT: Self = Self(1000455008);
    pub const ATTACHMENT_FEEDBACK_LOOP_ENABLE_EXT: Self = Self(1000524000);
    pub const COLOR_BLEND_ADVANCED_EXT: Self = Self(1000455018);
    pub const COLOR_BLEND_ENABLE_EXT: Self = Self(1000455010);
    pub const COLOR_BLEND_EQUATION_EXT: Self = Self(1000455011);
    pub const COLOR_WRITE_ENABLE_EXT: Self = Self(1000381000);
    pub const COLOR_WRITE_MASK_EXT: Self = Self(1000455012);
    pub const CONSERVATIVE_RASTERIZATION_MODE_EXT: Self = Self(1000455014);
    pub const COVERAGE_MODULATION_MODE_NV: Self = Self(1000455027);
    pub const COVERAGE_MODULATION_TABLE_ENABLE_NV: Self = Self(1000455028);
    pub const COVERAGE_MODULATION_TABLE_NV: Self = Self(1000455029);
    pub const COVERAGE_REDUCTION_MODE_NV: Self = Self(1000455032);
    pub const COVERAGE_TO_COLOR_ENABLE_NV: Self = Self(1000455025);
    pub const COVERAGE_TO_COLOR_LOCATION_NV: Self = Self(1000455026);
    pub const CULL_MODE: Self = Self(1000267000);
    pub const DEPTH_BIAS_ENABLE: Self = Self(1000377002);
    pub const DEPTH_BOUNDS_TEST_ENABLE: Self = Self(1000267009);
    pub const DEPTH_CLAMP_ENABLE_EXT: Self = Self(1000455003);
    pub const DEPTH_CLAMP_RANGE_EXT: Self = Self(1000582000);
    pub const DEPTH_CLIP_ENABLE_EXT: Self = Self(1000455016);
    pub const DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT: Self = Self(1000455022);
    pub const DEPTH_COMPARE_OP: Self = Self(1000267008);
    pub const DEPTH_TEST_ENABLE: Self = Self(1000267006);
    pub const DEPTH_WRITE_ENABLE: Self = Self(1000267007);
    pub const DISCARD_RECTANGLE_ENABLE_EXT: Self = Self(1000099001);
    pub const DISCARD_RECTANGLE_EXT: Self = Self(1000099000);
    pub const DISCARD_RECTANGLE_MODE_EXT: Self = Self(1000099002);
    pub const EXCLUSIVE_SCISSOR_ENABLE_NV: Self = Self(1000205000);
    pub const EXCLUSIVE_SCISSOR_NV: Self = Self(1000205001);
    pub const EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT: Self = Self(1000455015);
    pub const FRAGMENT_SHADING_RATE_KHR: Self = Self(1000226000);
    pub const FRONT_FACE: Self = Self(1000267001);
    pub const LINE_RASTERIZATION_MODE_EXT: Self = Self(1000455020);
    pub const LINE_STIPPLE: Self = Self(1000259000);
    pub const LINE_STIPPLE_ENABLE_EXT: Self = Self(1000455021);
    pub const LOGIC_OP_ENABLE_EXT: Self = Self(1000455009);
    pub const LOGIC_OP_EXT: Self = Self(1000377003);
    pub const PATCH_CONTROL_POINTS_EXT: Self = Self(1000377000);
    pub const POLYGON_MODE_EXT: Self = Self(1000455004);
    pub const PRIMITIVE_RESTART_ENABLE: Self = Self(1000377004);
    pub const PRIMITIVE_TOPOLOGY: Self = Self(1000267002);
    pub const PROVOKING_VERTEX_MODE_EXT: Self = Self(1000455019);
    pub const RASTERIZATION_SAMPLES_EXT: Self = Self(1000455005);
    pub const RASTERIZATION_STREAM_EXT: Self = Self(1000455013);
    pub const RASTERIZER_DISCARD_ENABLE: Self = Self(1000377001);
    pub const RAY_TRACING_PIPELINE_STACK_SIZE_KHR: Self = Self(1000347000);
    pub const REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV: Self = Self(1000455031);
    pub const SAMPLE_LOCATIONS_ENABLE_EXT: Self = Self(1000455017);
    pub const SAMPLE_LOCATIONS_EXT: Self = Self(1000143000);
    pub const SAMPLE_MASK_EXT: Self = Self(1000455006);
    pub const SCISSOR_WITH_COUNT: Self = Self(1000267004);
    pub const SHADING_RATE_IMAGE_ENABLE_NV: Self = Self(1000455030);
    pub const STENCIL_OP: Self = Self(1000267011);
    pub const STENCIL_TEST_ENABLE: Self = Self(1000267010);
    pub const TESSELLATION_DOMAIN_ORIGIN_EXT: Self = Self(1000455002);
    pub const VERTEX_INPUT_BINDING_STRIDE: Self = Self(1000267005);
    pub const VERTEX_INPUT_EXT: Self = Self(1000352000);
    pub const VIEWPORT_COARSE_SAMPLE_ORDER_NV: Self = Self(1000164006);
    pub const VIEWPORT_SHADING_RATE_PALETTE_NV: Self = Self(1000164004);
    pub const VIEWPORT_SWIZZLE_NV: Self = Self(1000455024);
    pub const VIEWPORT_WITH_COUNT: Self = Self(1000267003);
    pub const VIEWPORT_W_SCALING_ENABLE_NV: Self = Self(1000455023);
    pub const VIEWPORT_W_SCALING_NV: Self = Self(1000087000);
    pub const CULL_MODE_EXT: Self = Self::CULL_MODE;
    pub const DEPTH_BIAS_ENABLE_EXT: Self = Self::DEPTH_BIAS_ENABLE;
    pub const DEPTH_BOUNDS_TEST_ENABLE_EXT: Self = Self::DEPTH_BOUNDS_TEST_ENABLE;
    pub const DEPTH_COMPARE_OP_EXT: Self = Self::DEPTH_COMPARE_OP;
    pub const DEPTH_TEST_ENABLE_EXT: Self = Self::DEPTH_TEST_ENABLE;
    pub const DEPTH_WRITE_ENABLE_EXT: Self = Self::DEPTH_WRITE_ENABLE;
    pub const FRONT_FACE_EXT: Self = Self::FRONT_FACE;
    pub const LINE_STIPPLE_EXT: Self = Self::LINE_STIPPLE;
    pub const LINE_STIPPLE_KHR: Self = Self::LINE_STIPPLE;
    pub const PRIMITIVE_RESTART_ENABLE_EXT: Self = Self::PRIMITIVE_RESTART_ENABLE;
    pub const PRIMITIVE_TOPOLOGY_EXT: Self = Self::PRIMITIVE_TOPOLOGY;
    pub const RASTERIZER_DISCARD_ENABLE_EXT: Self = Self::RASTERIZER_DISCARD_ENABLE;
    pub const SCISSOR_WITH_COUNT_EXT: Self = Self::SCISSOR_WITH_COUNT;
    pub const STENCIL_OP_EXT: Self = Self::STENCIL_OP;
    pub const STENCIL_TEST_ENABLE_EXT: Self = Self::STENCIL_TEST_ENABLE;
    pub const VERTEX_INPUT_BINDING_STRIDE_EXT: Self = Self::VERTEX_INPUT_BINDING_STRIDE;
    pub const VIEWPORT_WITH_COUNT_EXT: Self = Self::VIEWPORT_WITH_COUNT;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const ACCELERATION_STRUCTURE_KHR: Self = Self(1000150000);
    pub const ACCELERATION_STRUCTURE_NV: Self = Self(1000165000);
    pub const BUFFER_COLLECTION_FUCHSIA: Self = Self(1000366000);
    pub const CUDA_FUNCTION_NV: Self = Self(1000307001);
    pub const CUDA_MODULE_NV: Self = Self(1000307000);
    pub const CU_FUNCTION_NVX: Self = Self(1000029001);
    pub const CU_MODULE_NVX: Self = Self(1000029000);
    pub const DATA_GRAPH_PIPELINE_SESSION_ARM: Self = Self(1000507000);
    pub const DEBUG_REPORT_CALLBACK_EXT: Self = Self(1000011000);
    pub const DEBUG_UTILS_MESSENGER_EXT: Self = Self(1000128000);
    pub const DEFERRED_OPERATION_KHR: Self = Self(1000268000);
    pub const DESCRIPTOR_UPDATE_TEMPLATE: Self = Self(1000085000);
    pub const DISPLAY_KHR: Self = Self(1000002000);
    pub const DISPLAY_MODE_KHR: Self = Self(1000002001);
    pub const EXTERNAL_COMPUTE_QUEUE_NV: Self = Self(1000556000);
    pub const INDIRECT_COMMANDS_LAYOUT_EXT: Self = Self(1000572000);
    pub const INDIRECT_COMMANDS_LAYOUT_NV: Self = Self(1000277000);
    pub const INDIRECT_EXECUTION_SET_EXT: Self = Self(1000572001);
    pub const MICROMAP_EXT: Self = Self(1000396000);
    pub const OPTICAL_FLOW_SESSION_NV: Self = Self(1000464000);
    pub const PERFORMANCE_CONFIGURATION_INTEL: Self = Self(1000210000);
    pub const PIPELINE_BINARY_KHR: Self = Self(1000483000);
    pub const PRIVATE_DATA_SLOT: Self = Self(1000295000);
    pub const SAMPLER_YCBCR_CONVERSION: Self = Self(1000156000);
    pub const SHADER_EXT: Self = Self(1000482000);
    pub const SURFACE_KHR: Self = Self(1000000000);
    pub const SWAPCHAIN_KHR: Self = Self(1000001000);
    pub const TENSOR_ARM: Self = Self(1000460000);
    pub const TENSOR_VIEW_ARM: Self = Self(1000460001);
    pub const VALIDATION_CACHE_EXT: Self = Self(1000160000);
    pub const VIDEO_SESSION_KHR: Self = Self(1000023000);
    pub const VIDEO_SESSION_PARAMETERS_KHR: Self = Self(1000023001);
    pub const DESCRIPTOR_UPDATE_TEMPLATE_KHR: Self = Self::DESCRIPTOR_UPDATE_TEMPLATE;
    pub const PRIVATE_DATA_SLOT_EXT: Self = Self::PRIVATE_DATA_SLOT;
    pub const SAMPLER_YCBCR_CONVERSION_KHR: Self = Self::SAMPLER_YCBCR_CONVERSION;
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FramebufferCreateFlags: Flags {
        const IMAGELESS = FramebufferCreateFlagBits::IMAGELESS.0;
        const IMAGELESS_KHR = Self::IMAGELESS.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FramebufferCreateFlagBits(u32);
impl FramebufferCreateFlagBits {
    pub const IMAGELESS: Self = Self(1 << 0);
    pub const IMAGELESS_KHR: Self = Self::IMAGELESS;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryPoolCreateFlags: Flags {
        const RESET_KHR = QueryPoolCreateFlagBits::RESET_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPoolCreateFlagBits(u32);
impl QueryPoolCreateFlagBits {
    pub const RESET_KHR: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RenderPassCreateFlags: Flags {
        const TRANSFORM_QCOM = RenderPassCreateFlagBits::TRANSFORM_QCOM.0;
        const PER_LAYER_FRAGMENT_DENSITY_VALVE = RenderPassCreateFlagBits::PER_LAYER_FRAGMENT_DENSITY_VALVE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderPassCreateFlagBits(u32);
impl RenderPassCreateFlagBits {
    pub const TRANSFORM_QCOM: Self = Self(1 << 1);
    pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SamplerCreateFlags: Flags {
        const SUBSAMPLED_EXT = SamplerCreateFlagBits::SUBSAMPLED_EXT.0;
        const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT = SamplerCreateFlagBits::SUBSAMPLED_COARSE_RECONSTRUCTION_EXT.0;
        const NON_SEAMLESS_CUBE_MAP_EXT = SamplerCreateFlagBits::NON_SEAMLESS_CUBE_MAP_EXT.0;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = SamplerCreateFlagBits::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0;
        const IMAGE_PROCESSING_QCOM = SamplerCreateFlagBits::IMAGE_PROCESSING_QCOM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerCreateFlagBits(u32);
impl SamplerCreateFlagBits {
    pub const SUBSAMPLED_EXT: Self = Self(1 << 0);
    pub const SUBSAMPLED_COARSE_RECONSTRUCTION_EXT: Self = Self(1 << 1);
    pub const NON_SEAMLESS_CUBE_MAP_EXT: Self = Self(1 << 2);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 3);
    pub const IMAGE_PROCESSING_QCOM: Self = Self(1 << 4);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineLayoutCreateFlags: Flags {
        const INDEPENDENT_SETS_EXT = PipelineLayoutCreateFlagBits::INDEPENDENT_SETS_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineLayoutCreateFlagBits(u32);
impl PipelineLayoutCreateFlagBits {
    pub const INDEPENDENT_SETS_EXT: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineCacheCreateFlags: Flags {
        const EXTERNALLY_SYNCHRONIZED = PipelineCacheCreateFlagBits::EXTERNALLY_SYNCHRONIZED.0;
        const INTERNALLY_SYNCHRONIZED_MERGE_KHR = PipelineCacheCreateFlagBits::INTERNALLY_SYNCHRONIZED_MERGE_KHR.0;
        const EXTERNALLY_SYNCHRONIZED_EXT = Self::EXTERNALLY_SYNCHRONIZED.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCacheCreateFlagBits(u32);
impl PipelineCacheCreateFlagBits {
    pub const EXTERNALLY_SYNCHRONIZED: Self = Self(1 << 0);
    pub const INTERNALLY_SYNCHRONIZED_MERGE_KHR: Self = Self(1 << 3);
    pub const EXTERNALLY_SYNCHRONIZED_EXT: Self = Self::EXTERNALLY_SYNCHRONIZED;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineDepthStencilStateCreateFlags: Flags {
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT = PipelineDepthStencilStateCreateFlagBits::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT.0;
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT = PipelineDepthStencilStateCreateFlagBits::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT.0;
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM = Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT.bits();
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM = Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineDepthStencilStateCreateFlagBits(u32);
impl PipelineDepthStencilStateCreateFlagBits {
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(1 << 0);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(1 << 1);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineDynamicStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineColorBlendStateCreateFlags: Flags {
        const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT = PipelineColorBlendStateCreateFlagBits::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT.0;
        const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM = Self::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineColorBlendStateCreateFlagBits(u32);
impl PipelineColorBlendStateCreateFlagBits {
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT: Self = Self(1 << 0);
    pub const RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_ACCESS_EXT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineMultisampleStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineRasterizationStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineViewportStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineTessellationStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineInputAssemblyStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineVertexInputStateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineShaderStageCreateFlags: Flags {
        const ALLOW_VARYING_SUBGROUP_SIZE = PipelineShaderStageCreateFlagBits::ALLOW_VARYING_SUBGROUP_SIZE.0;
        const REQUIRE_FULL_SUBGROUPS = PipelineShaderStageCreateFlagBits::REQUIRE_FULL_SUBGROUPS.0;
        const ALLOW_VARYING_SUBGROUP_SIZE_EXT = Self::ALLOW_VARYING_SUBGROUP_SIZE.bits();
        const REQUIRE_FULL_SUBGROUPS_EXT = Self::REQUIRE_FULL_SUBGROUPS.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineShaderStageCreateFlagBits(u32);
impl PipelineShaderStageCreateFlagBits {
    pub const ALLOW_VARYING_SUBGROUP_SIZE: Self = Self(1 << 0);
    pub const REQUIRE_FULL_SUBGROUPS: Self = Self(1 << 1);
    pub const ALLOW_VARYING_SUBGROUP_SIZE_EXT: Self = Self::ALLOW_VARYING_SUBGROUP_SIZE;
    pub const REQUIRE_FULL_SUBGROUPS_EXT: Self = Self::REQUIRE_FULL_SUBGROUPS;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DescriptorSetLayoutCreateFlags: Flags {
        const PUSH_DESCRIPTOR = DescriptorSetLayoutCreateFlagBits::PUSH_DESCRIPTOR.0;
        const UPDATE_AFTER_BIND_POOL = DescriptorSetLayoutCreateFlagBits::UPDATE_AFTER_BIND_POOL.0;
        const HOST_ONLY_POOL_EXT = DescriptorSetLayoutCreateFlagBits::HOST_ONLY_POOL_EXT.0;
        const DESCRIPTOR_BUFFER_EXT = DescriptorSetLayoutCreateFlagBits::DESCRIPTOR_BUFFER_EXT.0;
        const EMBEDDED_IMMUTABLE_SAMPLERS_EXT = DescriptorSetLayoutCreateFlagBits::EMBEDDED_IMMUTABLE_SAMPLERS_EXT.0;
        const PER_STAGE_NV = DescriptorSetLayoutCreateFlagBits::PER_STAGE_NV.0;
        const INDIRECT_BINDABLE_NV = DescriptorSetLayoutCreateFlagBits::INDIRECT_BINDABLE_NV.0;
        const HOST_ONLY_POOL_VALVE = Self::HOST_ONLY_POOL_EXT.bits();
        const PUSH_DESCRIPTOR_KHR = Self::PUSH_DESCRIPTOR.bits();
        const UPDATE_AFTER_BIND_POOL_EXT = Self::UPDATE_AFTER_BIND_POOL.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorSetLayoutCreateFlagBits(u32);
impl DescriptorSetLayoutCreateFlagBits {
    pub const PUSH_DESCRIPTOR: Self = Self(1 << 0);
    pub const UPDATE_AFTER_BIND_POOL: Self = Self(1 << 1);
    pub const HOST_ONLY_POOL_EXT: Self = Self(1 << 2);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 4);
    pub const EMBEDDED_IMMUTABLE_SAMPLERS_EXT: Self = Self(1 << 5);
    pub const PER_STAGE_NV: Self = Self(1 << 6);
    pub const INDIRECT_BINDABLE_NV: Self = Self(1 << 7);
    pub const HOST_ONLY_POOL_VALVE: Self = Self::HOST_ONLY_POOL_EXT;
    pub const PUSH_DESCRIPTOR_KHR: Self = Self::PUSH_DESCRIPTOR;
    pub const UPDATE_AFTER_BIND_POOL_EXT: Self = Self::UPDATE_AFTER_BIND_POOL;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BufferViewCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct InstanceCreateFlags: Flags {
        const ENUMERATE_PORTABILITY_KHR = InstanceCreateFlagBits::ENUMERATE_PORTABILITY_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstanceCreateFlagBits(u32);
impl InstanceCreateFlagBits {
    pub const ENUMERATE_PORTABILITY_KHR: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceQueueCreateFlags: Flags {
        const PROTECTED = DeviceQueueCreateFlagBits::PROTECTED.0;
        const INTERNALLY_SYNCHRONIZED_KHR = DeviceQueueCreateFlagBits::INTERNALLY_SYNCHRONIZED_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceQueueCreateFlagBits(u32);
impl DeviceQueueCreateFlagBits {
    pub const PROTECTED: Self = Self(1 << 0);
    pub const INTERNALLY_SYNCHRONIZED_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueueFlags: Flags {
        const GRAPHICS = QueueFlagBits::GRAPHICS.0;
        const COMPUTE = QueueFlagBits::COMPUTE.0;
        const TRANSFER = QueueFlagBits::TRANSFER.0;
        const SPARSE_BINDING = QueueFlagBits::SPARSE_BINDING.0;
        const PROTECTED = QueueFlagBits::PROTECTED.0;
        const VIDEO_DECODE_KHR = QueueFlagBits::VIDEO_DECODE_KHR.0;
        const VIDEO_ENCODE_KHR = QueueFlagBits::VIDEO_ENCODE_KHR.0;
        const OPTICAL_FLOW_NV = QueueFlagBits::OPTICAL_FLOW_NV.0;
        const DATA_GRAPH_ARM = QueueFlagBits::DATA_GRAPH_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueFlagBits(u32);
impl QueueFlagBits {
    pub const GRAPHICS: Self = Self(1 << 0);
    pub const COMPUTE: Self = Self(1 << 1);
    pub const TRANSFER: Self = Self(1 << 2);
    pub const SPARSE_BINDING: Self = Self(1 << 3);
    pub const PROTECTED: Self = Self(1 << 4);
    pub const VIDEO_DECODE_KHR: Self = Self(1 << 5);
    pub const VIDEO_ENCODE_KHR: Self = Self(1 << 6);
    pub const OPTICAL_FLOW_NV: Self = Self(1 << 8);
    pub const DATA_GRAPH_ARM: Self = Self(1 << 10);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryPropertyFlags: Flags {
        const DEVICE_LOCAL = MemoryPropertyFlagBits::DEVICE_LOCAL.0;
        const HOST_VISIBLE = MemoryPropertyFlagBits::HOST_VISIBLE.0;
        const HOST_COHERENT = MemoryPropertyFlagBits::HOST_COHERENT.0;
        const HOST_CACHED = MemoryPropertyFlagBits::HOST_CACHED.0;
        const LAZILY_ALLOCATED = MemoryPropertyFlagBits::LAZILY_ALLOCATED.0;
        const PROTECTED = MemoryPropertyFlagBits::PROTECTED.0;
        const DEVICE_COHERENT_AMD = MemoryPropertyFlagBits::DEVICE_COHERENT_AMD.0;
        const DEVICE_UNCACHED_AMD = MemoryPropertyFlagBits::DEVICE_UNCACHED_AMD.0;
        const RDMA_CAPABLE_NV = MemoryPropertyFlagBits::RDMA_CAPABLE_NV.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryPropertyFlagBits(u32);
impl MemoryPropertyFlagBits {
    pub const DEVICE_LOCAL: Self = Self(1 << 0);
    pub const HOST_VISIBLE: Self = Self(1 << 1);
    pub const HOST_COHERENT: Self = Self(1 << 2);
    pub const HOST_CACHED: Self = Self(1 << 3);
    pub const LAZILY_ALLOCATED: Self = Self(1 << 4);
    pub const PROTECTED: Self = Self(1 << 5);
    pub const DEVICE_COHERENT_AMD: Self = Self(1 << 6);
    pub const DEVICE_UNCACHED_AMD: Self = Self(1 << 7);
    pub const RDMA_CAPABLE_NV: Self = Self(1 << 8);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryHeapFlags: Flags {
        const DEVICE_LOCAL = MemoryHeapFlagBits::DEVICE_LOCAL.0;
        const MULTI_INSTANCE = MemoryHeapFlagBits::MULTI_INSTANCE.0;
        const TILE_MEMORY_QCOM = MemoryHeapFlagBits::TILE_MEMORY_QCOM.0;
        const MULTI_INSTANCE_KHR = Self::MULTI_INSTANCE.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryHeapFlagBits(u32);
impl MemoryHeapFlagBits {
    pub const DEVICE_LOCAL: Self = Self(1 << 0);
    pub const MULTI_INSTANCE: Self = Self(1 << 1);
    pub const TILE_MEMORY_QCOM: Self = Self(1 << 3);
    pub const MULTI_INSTANCE_KHR: Self = Self::MULTI_INSTANCE;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AccessFlags: Flags {
        const INDIRECT_COMMAND_READ = AccessFlagBits::INDIRECT_COMMAND_READ.0;
        const INDEX_READ = AccessFlagBits::INDEX_READ.0;
        const VERTEX_ATTRIBUTE_READ = AccessFlagBits::VERTEX_ATTRIBUTE_READ.0;
        const UNIFORM_READ = AccessFlagBits::UNIFORM_READ.0;
        const INPUT_ATTACHMENT_READ = AccessFlagBits::INPUT_ATTACHMENT_READ.0;
        const SHADER_READ = AccessFlagBits::SHADER_READ.0;
        const SHADER_WRITE = AccessFlagBits::SHADER_WRITE.0;
        const COLOR_ATTACHMENT_READ = AccessFlagBits::COLOR_ATTACHMENT_READ.0;
        const COLOR_ATTACHMENT_WRITE = AccessFlagBits::COLOR_ATTACHMENT_WRITE.0;
        const DEPTH_STENCIL_ATTACHMENT_READ = AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_READ.0;
        const DEPTH_STENCIL_ATTACHMENT_WRITE = AccessFlagBits::DEPTH_STENCIL_ATTACHMENT_WRITE.0;
        const TRANSFER_READ = AccessFlagBits::TRANSFER_READ.0;
        const TRANSFER_WRITE = AccessFlagBits::TRANSFER_WRITE.0;
        const HOST_READ = AccessFlagBits::HOST_READ.0;
        const HOST_WRITE = AccessFlagBits::HOST_WRITE.0;
        const MEMORY_READ = AccessFlagBits::MEMORY_READ.0;
        const MEMORY_WRITE = AccessFlagBits::MEMORY_WRITE.0;
        const COMMAND_PREPROCESS_READ_EXT = AccessFlagBits::COMMAND_PREPROCESS_READ_EXT.0;
        const COMMAND_PREPROCESS_WRITE_EXT = AccessFlagBits::COMMAND_PREPROCESS_WRITE_EXT.0;
        const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = AccessFlagBits::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0;
        const CONDITIONAL_RENDERING_READ_EXT = AccessFlagBits::CONDITIONAL_RENDERING_READ_EXT.0;
        const ACCELERATION_STRUCTURE_READ_KHR = AccessFlagBits::ACCELERATION_STRUCTURE_READ_KHR.0;
        const ACCELERATION_STRUCTURE_WRITE_KHR = AccessFlagBits::ACCELERATION_STRUCTURE_WRITE_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = AccessFlagBits::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.0;
        const FRAGMENT_DENSITY_MAP_READ_EXT = AccessFlagBits::FRAGMENT_DENSITY_MAP_READ_EXT.0;
        const TRANSFORM_FEEDBACK_WRITE_EXT = AccessFlagBits::TRANSFORM_FEEDBACK_WRITE_EXT.0;
        const TRANSFORM_FEEDBACK_COUNTER_READ_EXT = AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.0;
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = AccessFlagBits::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.0;
        const ACCELERATION_STRUCTURE_READ_NV = Self::ACCELERATION_STRUCTURE_READ_KHR.bits();
        const ACCELERATION_STRUCTURE_WRITE_NV = Self::ACCELERATION_STRUCTURE_WRITE_KHR.bits();
        const COMMAND_PREPROCESS_READ_NV = Self::COMMAND_PREPROCESS_READ_EXT.bits();
        const COMMAND_PREPROCESS_WRITE_NV = Self::COMMAND_PREPROCESS_WRITE_EXT.bits();
        const NONE_KHR = Self::NONE.bits();
        const SHADING_RATE_IMAGE_READ_NV = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.bits();
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlagBits(u32);
impl AccessFlagBits {
    pub const INDIRECT_COMMAND_READ: Self = Self(1 << 0);
    pub const INDEX_READ: Self = Self(1 << 1);
    pub const VERTEX_ATTRIBUTE_READ: Self = Self(1 << 2);
    pub const UNIFORM_READ: Self = Self(1 << 3);
    pub const INPUT_ATTACHMENT_READ: Self = Self(1 << 4);
    pub const SHADER_READ: Self = Self(1 << 5);
    pub const SHADER_WRITE: Self = Self(1 << 6);
    pub const COLOR_ATTACHMENT_READ: Self = Self(1 << 7);
    pub const COLOR_ATTACHMENT_WRITE: Self = Self(1 << 8);
    pub const DEPTH_STENCIL_ATTACHMENT_READ: Self = Self(1 << 9);
    pub const DEPTH_STENCIL_ATTACHMENT_WRITE: Self = Self(1 << 10);
    pub const TRANSFER_READ: Self = Self(1 << 11);
    pub const TRANSFER_WRITE: Self = Self(1 << 12);
    pub const HOST_READ: Self = Self(1 << 13);
    pub const HOST_WRITE: Self = Self(1 << 14);
    pub const MEMORY_READ: Self = Self(1 << 15);
    pub const MEMORY_WRITE: Self = Self(1 << 16);
    pub const COMMAND_PREPROCESS_READ_EXT: Self = Self(1 << 17);
    pub const COMMAND_PREPROCESS_WRITE_EXT: Self = Self(1 << 18);
    pub const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT: Self = Self(1 << 19);
    pub const CONDITIONAL_RENDERING_READ_EXT: Self = Self(1 << 20);
    pub const ACCELERATION_STRUCTURE_READ_KHR: Self = Self(1 << 21);
    pub const ACCELERATION_STRUCTURE_WRITE_KHR: Self = Self(1 << 22);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR: Self = Self(1 << 23);
    pub const FRAGMENT_DENSITY_MAP_READ_EXT: Self = Self(1 << 24);
    pub const TRANSFORM_FEEDBACK_WRITE_EXT: Self = Self(1 << 25);
    pub const TRANSFORM_FEEDBACK_COUNTER_READ_EXT: Self = Self(1 << 26);
    pub const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT: Self = Self(1 << 27);
    pub const ACCELERATION_STRUCTURE_READ_NV: Self = Self::ACCELERATION_STRUCTURE_READ_KHR;
    pub const ACCELERATION_STRUCTURE_WRITE_NV: Self = Self::ACCELERATION_STRUCTURE_WRITE_KHR;
    pub const COMMAND_PREPROCESS_READ_NV: Self = Self::COMMAND_PREPROCESS_READ_EXT;
    pub const COMMAND_PREPROCESS_WRITE_NV: Self = Self::COMMAND_PREPROCESS_WRITE_EXT;
    pub const SHADING_RATE_IMAGE_READ_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BufferUsageFlags: Flags {
        const TRANSFER_SRC = BufferUsageFlagBits::TRANSFER_SRC.0;
        const TRANSFER_DST = BufferUsageFlagBits::TRANSFER_DST.0;
        const UNIFORM_TEXEL_BUFFER = BufferUsageFlagBits::UNIFORM_TEXEL_BUFFER.0;
        const STORAGE_TEXEL_BUFFER = BufferUsageFlagBits::STORAGE_TEXEL_BUFFER.0;
        const UNIFORM_BUFFER = BufferUsageFlagBits::UNIFORM_BUFFER.0;
        const STORAGE_BUFFER = BufferUsageFlagBits::STORAGE_BUFFER.0;
        const INDEX_BUFFER = BufferUsageFlagBits::INDEX_BUFFER.0;
        const VERTEX_BUFFER = BufferUsageFlagBits::VERTEX_BUFFER.0;
        const INDIRECT_BUFFER = BufferUsageFlagBits::INDIRECT_BUFFER.0;
        const CONDITIONAL_RENDERING_EXT = BufferUsageFlagBits::CONDITIONAL_RENDERING_EXT.0;
        const SHADER_BINDING_TABLE_KHR = BufferUsageFlagBits::SHADER_BINDING_TABLE_KHR.0;
        const TRANSFORM_FEEDBACK_BUFFER_EXT = BufferUsageFlagBits::TRANSFORM_FEEDBACK_BUFFER_EXT.0;
        const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT = BufferUsageFlagBits::TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT.0;
        const VIDEO_DECODE_SRC_KHR = BufferUsageFlagBits::VIDEO_DECODE_SRC_KHR.0;
        const VIDEO_DECODE_DST_KHR = BufferUsageFlagBits::VIDEO_DECODE_DST_KHR.0;
        const VIDEO_ENCODE_DST_KHR = BufferUsageFlagBits::VIDEO_ENCODE_DST_KHR.0;
        const VIDEO_ENCODE_SRC_KHR = BufferUsageFlagBits::VIDEO_ENCODE_SRC_KHR.0;
        const SHADER_DEVICE_ADDRESS = BufferUsageFlagBits::SHADER_DEVICE_ADDRESS.0;
        const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR = BufferUsageFlagBits::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR.0;
        const ACCELERATION_STRUCTURE_STORAGE_KHR = BufferUsageFlagBits::ACCELERATION_STRUCTURE_STORAGE_KHR.0;
        const SAMPLER_DESCRIPTOR_BUFFER_EXT = BufferUsageFlagBits::SAMPLER_DESCRIPTOR_BUFFER_EXT.0;
        const RESOURCE_DESCRIPTOR_BUFFER_EXT = BufferUsageFlagBits::RESOURCE_DESCRIPTOR_BUFFER_EXT.0;
        const MICROMAP_BUILD_INPUT_READ_ONLY_EXT = BufferUsageFlagBits::MICROMAP_BUILD_INPUT_READ_ONLY_EXT.0;
        const MICROMAP_STORAGE_EXT = BufferUsageFlagBits::MICROMAP_STORAGE_EXT.0;
        const EXECUTION_GRAPH_SCRATCH_AMDX = BufferUsageFlagBits::EXECUTION_GRAPH_SCRATCH_AMDX.0;
        const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT = BufferUsageFlagBits::PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT.0;
        const TILE_MEMORY_QCOM = BufferUsageFlagBits::TILE_MEMORY_QCOM.0;
        const DESCRIPTOR_HEAP_EXT = BufferUsageFlagBits::DESCRIPTOR_HEAP_EXT.0;
        const RAY_TRACING_NV = Self::SHADER_BINDING_TABLE_KHR.bits();
        const SHADER_DEVICE_ADDRESS_EXT = Self::SHADER_DEVICE_ADDRESS.bits();
        const SHADER_DEVICE_ADDRESS_KHR = Self::SHADER_DEVICE_ADDRESS.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferUsageFlagBits(u32);
impl BufferUsageFlagBits {
    pub const TRANSFER_SRC: Self = Self(1 << 0);
    pub const TRANSFER_DST: Self = Self(1 << 1);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 2);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 3);
    pub const UNIFORM_BUFFER: Self = Self(1 << 4);
    pub const STORAGE_BUFFER: Self = Self(1 << 5);
    pub const INDEX_BUFFER: Self = Self(1 << 6);
    pub const VERTEX_BUFFER: Self = Self(1 << 7);
    pub const INDIRECT_BUFFER: Self = Self(1 << 8);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 9);
    pub const SHADER_BINDING_TABLE_KHR: Self = Self(1 << 10);
    pub const TRANSFORM_FEEDBACK_BUFFER_EXT: Self = Self(1 << 11);
    pub const TRANSFORM_FEEDBACK_COUNTER_BUFFER_EXT: Self = Self(1 << 12);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1 << 13);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1 << 14);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1 << 15);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1 << 16);
    pub const SHADER_DEVICE_ADDRESS: Self = Self(1 << 17);
    pub const ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR: Self = Self(1 << 19);
    pub const ACCELERATION_STRUCTURE_STORAGE_KHR: Self = Self(1 << 20);
    pub const SAMPLER_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 21);
    pub const RESOURCE_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 22);
    pub const MICROMAP_BUILD_INPUT_READ_ONLY_EXT: Self = Self(1 << 23);
    pub const MICROMAP_STORAGE_EXT: Self = Self(1 << 24);
    pub const EXECUTION_GRAPH_SCRATCH_AMDX: Self = Self(1 << 25);
    pub const PUSH_DESCRIPTORS_DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 26);
    pub const TILE_MEMORY_QCOM: Self = Self(1 << 27);
    pub const DESCRIPTOR_HEAP_EXT: Self = Self(1 << 28);
    pub const RAY_TRACING_NV: Self = Self::SHADER_BINDING_TABLE_KHR;
    pub const SHADER_DEVICE_ADDRESS_EXT: Self = Self::SHADER_DEVICE_ADDRESS;
    pub const SHADER_DEVICE_ADDRESS_KHR: Self = Self::SHADER_DEVICE_ADDRESS;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BufferCreateFlags: Flags {
        const SPARSE_BINDING = BufferCreateFlagBits::SPARSE_BINDING.0;
        const SPARSE_RESIDENCY = BufferCreateFlagBits::SPARSE_RESIDENCY.0;
        const SPARSE_ALIASED = BufferCreateFlagBits::SPARSE_ALIASED.0;
        const PROTECTED = BufferCreateFlagBits::PROTECTED.0;
        const DEVICE_ADDRESS_CAPTURE_REPLAY = BufferCreateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY.0;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = BufferCreateFlagBits::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0;
        const VIDEO_PROFILE_INDEPENDENT_KHR = BufferCreateFlagBits::VIDEO_PROFILE_INDEPENDENT_KHR.0;
        const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT = Self::DEVICE_ADDRESS_CAPTURE_REPLAY.bits();
        const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR = Self::DEVICE_ADDRESS_CAPTURE_REPLAY.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferCreateFlagBits(u32);
impl BufferCreateFlagBits {
    pub const SPARSE_BINDING: Self = Self(1 << 0);
    pub const SPARSE_RESIDENCY: Self = Self(1 << 1);
    pub const SPARSE_ALIASED: Self = Self(1 << 2);
    pub const PROTECTED: Self = Self(1 << 3);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1 << 4);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 5);
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(1 << 6);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_EXT: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self::DEVICE_ADDRESS_CAPTURE_REPLAY;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderStageFlags: Flags {
        const VERTEX = ShaderStageFlagBits::VERTEX.0;
        const TESSELLATION_CONTROL = ShaderStageFlagBits::TESSELLATION_CONTROL.0;
        const TESSELLATION_EVALUATION = ShaderStageFlagBits::TESSELLATION_EVALUATION.0;
        const GEOMETRY = ShaderStageFlagBits::GEOMETRY.0;
        const FRAGMENT = ShaderStageFlagBits::FRAGMENT.0;
        const COMPUTE = ShaderStageFlagBits::COMPUTE.0;
        const TASK_EXT = ShaderStageFlagBits::TASK_EXT.0;
        const MESH_EXT = ShaderStageFlagBits::MESH_EXT.0;
        const RAYGEN_KHR = ShaderStageFlagBits::RAYGEN_KHR.0;
        const ANY_HIT_KHR = ShaderStageFlagBits::ANY_HIT_KHR.0;
        const CLOSEST_HIT_KHR = ShaderStageFlagBits::CLOSEST_HIT_KHR.0;
        const MISS_KHR = ShaderStageFlagBits::MISS_KHR.0;
        const INTERSECTION_KHR = ShaderStageFlagBits::INTERSECTION_KHR.0;
        const CALLABLE_KHR = ShaderStageFlagBits::CALLABLE_KHR.0;
        const SUBPASS_SHADING_HUAWEI = ShaderStageFlagBits::SUBPASS_SHADING_HUAWEI.0;
        const CLUSTER_CULLING_HUAWEI = ShaderStageFlagBits::CLUSTER_CULLING_HUAWEI.0;
        const ANY_HIT_NV = Self::ANY_HIT_KHR.bits();
        const CALLABLE_NV = Self::CALLABLE_KHR.bits();
        const CLOSEST_HIT_NV = Self::CLOSEST_HIT_KHR.bits();
        const INTERSECTION_NV = Self::INTERSECTION_KHR.bits();
        const MESH_NV = Self::MESH_EXT.bits();
        const MISS_NV = Self::MISS_KHR.bits();
        const RAYGEN_NV = Self::RAYGEN_KHR.bits();
        const TASK_NV = Self::TASK_EXT.bits();
        const ALL_GRAPHICS = 0x0000001F;
        const ALL = 0x7FFFFFFF;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShaderStageFlagBits(u32);
impl ShaderStageFlagBits {
    pub const VERTEX: Self = Self(1 << 0);
    pub const TESSELLATION_CONTROL: Self = Self(1 << 1);
    pub const TESSELLATION_EVALUATION: Self = Self(1 << 2);
    pub const GEOMETRY: Self = Self(1 << 3);
    pub const FRAGMENT: Self = Self(1 << 4);
    pub const COMPUTE: Self = Self(1 << 5);
    pub const TASK_EXT: Self = Self(1 << 6);
    pub const MESH_EXT: Self = Self(1 << 7);
    pub const RAYGEN_KHR: Self = Self(1 << 8);
    pub const ANY_HIT_KHR: Self = Self(1 << 9);
    pub const CLOSEST_HIT_KHR: Self = Self(1 << 10);
    pub const MISS_KHR: Self = Self(1 << 11);
    pub const INTERSECTION_KHR: Self = Self(1 << 12);
    pub const CALLABLE_KHR: Self = Self(1 << 13);
    pub const SUBPASS_SHADING_HUAWEI: Self = Self(1 << 14);
    pub const CLUSTER_CULLING_HUAWEI: Self = Self(1 << 19);
    pub const ANY_HIT_NV: Self = Self::ANY_HIT_KHR;
    pub const CALLABLE_NV: Self = Self::CALLABLE_KHR;
    pub const CLOSEST_HIT_NV: Self = Self::CLOSEST_HIT_KHR;
    pub const INTERSECTION_NV: Self = Self::INTERSECTION_KHR;
    pub const MESH_NV: Self = Self::MESH_EXT;
    pub const MISS_NV: Self = Self::MISS_KHR;
    pub const RAYGEN_NV: Self = Self::RAYGEN_KHR;
    pub const TASK_NV: Self = Self::TASK_EXT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageUsageFlags: Flags {
        const TRANSFER_SRC = ImageUsageFlagBits::TRANSFER_SRC.0;
        const TRANSFER_DST = ImageUsageFlagBits::TRANSFER_DST.0;
        const SAMPLED = ImageUsageFlagBits::SAMPLED.0;
        const STORAGE = ImageUsageFlagBits::STORAGE.0;
        const COLOR_ATTACHMENT = ImageUsageFlagBits::COLOR_ATTACHMENT.0;
        const DEPTH_STENCIL_ATTACHMENT = ImageUsageFlagBits::DEPTH_STENCIL_ATTACHMENT.0;
        const TRANSIENT_ATTACHMENT = ImageUsageFlagBits::TRANSIENT_ATTACHMENT.0;
        const INPUT_ATTACHMENT = ImageUsageFlagBits::INPUT_ATTACHMENT.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = ImageUsageFlagBits::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
        const FRAGMENT_DENSITY_MAP_EXT = ImageUsageFlagBits::FRAGMENT_DENSITY_MAP_EXT.0;
        const VIDEO_DECODE_DST_KHR = ImageUsageFlagBits::VIDEO_DECODE_DST_KHR.0;
        const VIDEO_DECODE_SRC_KHR = ImageUsageFlagBits::VIDEO_DECODE_SRC_KHR.0;
        const VIDEO_DECODE_DPB_KHR = ImageUsageFlagBits::VIDEO_DECODE_DPB_KHR.0;
        const VIDEO_ENCODE_DST_KHR = ImageUsageFlagBits::VIDEO_ENCODE_DST_KHR.0;
        const VIDEO_ENCODE_SRC_KHR = ImageUsageFlagBits::VIDEO_ENCODE_SRC_KHR.0;
        const VIDEO_ENCODE_DPB_KHR = ImageUsageFlagBits::VIDEO_ENCODE_DPB_KHR.0;
        const INVOCATION_MASK_HUAWEI = ImageUsageFlagBits::INVOCATION_MASK_HUAWEI.0;
        const ATTACHMENT_FEEDBACK_LOOP_EXT = ImageUsageFlagBits::ATTACHMENT_FEEDBACK_LOOP_EXT.0;
        const SAMPLE_WEIGHT_QCOM = ImageUsageFlagBits::SAMPLE_WEIGHT_QCOM.0;
        const SAMPLE_BLOCK_MATCH_QCOM = ImageUsageFlagBits::SAMPLE_BLOCK_MATCH_QCOM.0;
        const HOST_TRANSFER = ImageUsageFlagBits::HOST_TRANSFER.0;
        const TENSOR_ALIASING_ARM = ImageUsageFlagBits::TENSOR_ALIASING_ARM.0;
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = ImageUsageFlagBits::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0;
        const VIDEO_ENCODE_EMPHASIS_MAP_KHR = ImageUsageFlagBits::VIDEO_ENCODE_EMPHASIS_MAP_KHR.0;
        const TILE_MEMORY_QCOM = ImageUsageFlagBits::TILE_MEMORY_QCOM.0;
        const HOST_TRANSFER_EXT = Self::HOST_TRANSFER.bits();
        const SHADING_RATE_IMAGE_NV = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageUsageFlagBits(u32);
impl ImageUsageFlagBits {
    pub const TRANSFER_SRC: Self = Self(1 << 0);
    pub const TRANSFER_DST: Self = Self(1 << 1);
    pub const SAMPLED: Self = Self(1 << 2);
    pub const STORAGE: Self = Self(1 << 3);
    pub const COLOR_ATTACHMENT: Self = Self(1 << 4);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(1 << 5);
    pub const TRANSIENT_ATTACHMENT: Self = Self(1 << 6);
    pub const INPUT_ATTACHMENT: Self = Self(1 << 7);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 8);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(1 << 9);
    pub const VIDEO_DECODE_DST_KHR: Self = Self(1 << 10);
    pub const VIDEO_DECODE_SRC_KHR: Self = Self(1 << 11);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1 << 12);
    pub const VIDEO_ENCODE_DST_KHR: Self = Self(1 << 13);
    pub const VIDEO_ENCODE_SRC_KHR: Self = Self(1 << 14);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1 << 15);
    pub const INVOCATION_MASK_HUAWEI: Self = Self(1 << 18);
    pub const ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 19);
    pub const SAMPLE_WEIGHT_QCOM: Self = Self(1 << 20);
    pub const SAMPLE_BLOCK_MATCH_QCOM: Self = Self(1 << 21);
    pub const HOST_TRANSFER: Self = Self(1 << 22);
    pub const TENSOR_ALIASING_ARM: Self = Self(1 << 23);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 25);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 26);
    pub const TILE_MEMORY_QCOM: Self = Self(1 << 27);
    pub const HOST_TRANSFER_EXT: Self = Self::HOST_TRANSFER;
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageCreateFlags: Flags {
        const SPARSE_BINDING = ImageCreateFlagBits::SPARSE_BINDING.0;
        const SPARSE_RESIDENCY = ImageCreateFlagBits::SPARSE_RESIDENCY.0;
        const SPARSE_ALIASED = ImageCreateFlagBits::SPARSE_ALIASED.0;
        const MUTABLE_FORMAT = ImageCreateFlagBits::MUTABLE_FORMAT.0;
        const CUBE_COMPATIBLE = ImageCreateFlagBits::CUBE_COMPATIBLE.0;
        const _2D_ARRAY_COMPATIBLE = ImageCreateFlagBits::_2D_ARRAY_COMPATIBLE.0;
        const SPLIT_INSTANCE_BIND_REGIONS = ImageCreateFlagBits::SPLIT_INSTANCE_BIND_REGIONS.0;
        const BLOCK_TEXEL_VIEW_COMPATIBLE = ImageCreateFlagBits::BLOCK_TEXEL_VIEW_COMPATIBLE.0;
        const EXTENDED_USAGE = ImageCreateFlagBits::EXTENDED_USAGE.0;
        const DISJOINT = ImageCreateFlagBits::DISJOINT.0;
        const ALIAS = ImageCreateFlagBits::ALIAS.0;
        const PROTECTED = ImageCreateFlagBits::PROTECTED.0;
        const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT = ImageCreateFlagBits::SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT.0;
        const CORNER_SAMPLED_NV = ImageCreateFlagBits::CORNER_SAMPLED_NV.0;
        const SUBSAMPLED_EXT = ImageCreateFlagBits::SUBSAMPLED_EXT.0;
        const FRAGMENT_DENSITY_MAP_OFFSET_EXT = ImageCreateFlagBits::FRAGMENT_DENSITY_MAP_OFFSET_EXT.0;
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT = ImageCreateFlagBits::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT.0;
        const _2D_VIEW_COMPATIBLE_EXT = ImageCreateFlagBits::_2D_VIEW_COMPATIBLE_EXT.0;
        const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT = ImageCreateFlagBits::MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT.0;
        const VIDEO_PROFILE_INDEPENDENT_KHR = ImageCreateFlagBits::VIDEO_PROFILE_INDEPENDENT_KHR.0;
        const _2D_ARRAY_COMPATIBLE_KHR = Self::_2D_ARRAY_COMPATIBLE.bits();
        const ALIAS_KHR = Self::ALIAS.bits();
        const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR = Self::BLOCK_TEXEL_VIEW_COMPATIBLE.bits();
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = Self::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT.bits();
        const DISJOINT_KHR = Self::DISJOINT.bits();
        const EXTENDED_USAGE_KHR = Self::EXTENDED_USAGE.bits();
        const FRAGMENT_DENSITY_MAP_OFFSET_QCOM = Self::FRAGMENT_DENSITY_MAP_OFFSET_EXT.bits();
        const SPLIT_INSTANCE_BIND_REGIONS_KHR = Self::SPLIT_INSTANCE_BIND_REGIONS.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageCreateFlagBits(u32);
impl ImageCreateFlagBits {
    pub const SPARSE_BINDING: Self = Self(1 << 0);
    pub const SPARSE_RESIDENCY: Self = Self(1 << 1);
    pub const SPARSE_ALIASED: Self = Self(1 << 2);
    pub const MUTABLE_FORMAT: Self = Self(1 << 3);
    pub const CUBE_COMPATIBLE: Self = Self(1 << 4);
    pub const _2D_ARRAY_COMPATIBLE: Self = Self(1 << 5);
    pub const SPLIT_INSTANCE_BIND_REGIONS: Self = Self(1 << 6);
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE: Self = Self(1 << 7);
    pub const EXTENDED_USAGE: Self = Self(1 << 8);
    pub const DISJOINT: Self = Self(1 << 9);
    pub const ALIAS: Self = Self(1 << 10);
    pub const PROTECTED: Self = Self(1 << 11);
    pub const SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_EXT: Self = Self(1 << 12);
    pub const CORNER_SAMPLED_NV: Self = Self(1 << 13);
    pub const SUBSAMPLED_EXT: Self = Self(1 << 14);
    pub const FRAGMENT_DENSITY_MAP_OFFSET_EXT: Self = Self(1 << 15);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT: Self = Self(1 << 16);
    pub const _2D_VIEW_COMPATIBLE_EXT: Self = Self(1 << 17);
    pub const MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_EXT: Self = Self(1 << 18);
    pub const VIDEO_PROFILE_INDEPENDENT_KHR: Self = Self(1 << 20);
    pub const _2D_ARRAY_COMPATIBLE_KHR: Self = Self::_2D_ARRAY_COMPATIBLE;
    pub const ALIAS_KHR: Self = Self::ALIAS;
    pub const BLOCK_TEXEL_VIEW_COMPATIBLE_KHR: Self = Self::BLOCK_TEXEL_VIEW_COMPATIBLE;
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self::DESCRIPTOR_HEAP_CAPTURE_REPLAY_EXT;
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    pub const EXTENDED_USAGE_KHR: Self = Self::EXTENDED_USAGE;
    pub const FRAGMENT_DENSITY_MAP_OFFSET_QCOM: Self = Self::FRAGMENT_DENSITY_MAP_OFFSET_EXT;
    pub const SPLIT_INSTANCE_BIND_REGIONS_KHR: Self = Self::SPLIT_INSTANCE_BIND_REGIONS;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageViewCreateFlags: Flags {
        const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT = ImageViewCreateFlagBits::FRAGMENT_DENSITY_MAP_DYNAMIC_EXT.0;
        const FRAGMENT_DENSITY_MAP_DEFERRED_EXT = ImageViewCreateFlagBits::FRAGMENT_DENSITY_MAP_DEFERRED_EXT.0;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT = ImageViewCreateFlagBits::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageViewCreateFlagBits(u32);
impl ImageViewCreateFlagBits {
    pub const FRAGMENT_DENSITY_MAP_DYNAMIC_EXT: Self = Self(1 << 0);
    pub const FRAGMENT_DENSITY_MAP_DEFERRED_EXT: Self = Self(1 << 1);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_EXT: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineCreateFlags: Flags {
        const DISABLE_OPTIMIZATION = PipelineCreateFlagBits::DISABLE_OPTIMIZATION.0;
        const ALLOW_DERIVATIVES = PipelineCreateFlagBits::ALLOW_DERIVATIVES.0;
        const DERIVATIVE = PipelineCreateFlagBits::DERIVATIVE.0;
        const VIEW_INDEX_FROM_DEVICE_INDEX = PipelineCreateFlagBits::VIEW_INDEX_FROM_DEVICE_INDEX.0;
        const DISPATCH_BASE = PipelineCreateFlagBits::DISPATCH_BASE.0;
        const DEFER_COMPILE_NV = PipelineCreateFlagBits::DEFER_COMPILE_NV.0;
        const CAPTURE_STATISTICS_KHR = PipelineCreateFlagBits::CAPTURE_STATISTICS_KHR.0;
        const CAPTURE_INTERNAL_REPRESENTATIONS_KHR = PipelineCreateFlagBits::CAPTURE_INTERNAL_REPRESENTATIONS_KHR.0;
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED = PipelineCreateFlagBits::FAIL_ON_PIPELINE_COMPILE_REQUIRED.0;
        const EARLY_RETURN_ON_FAILURE = PipelineCreateFlagBits::EARLY_RETURN_ON_FAILURE.0;
        const LINK_TIME_OPTIMIZATION_EXT = PipelineCreateFlagBits::LINK_TIME_OPTIMIZATION_EXT.0;
        const LIBRARY_KHR = PipelineCreateFlagBits::LIBRARY_KHR.0;
        const RAY_TRACING_SKIP_TRIANGLES_KHR = PipelineCreateFlagBits::RAY_TRACING_SKIP_TRIANGLES_KHR.0;
        const RAY_TRACING_SKIP_AABBS_KHR = PipelineCreateFlagBits::RAY_TRACING_SKIP_AABBS_KHR.0;
        const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR = PipelineCreateFlagBits::RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR.0;
        const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR = PipelineCreateFlagBits::RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR.0;
        const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR = PipelineCreateFlagBits::RAY_TRACING_NO_NULL_MISS_SHADERS_KHR.0;
        const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR = PipelineCreateFlagBits::RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR.0;
        const INDIRECT_BINDABLE_NV = PipelineCreateFlagBits::INDIRECT_BINDABLE_NV.0;
        const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR = PipelineCreateFlagBits::RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR.0;
        const RAY_TRACING_ALLOW_MOTION_NV = PipelineCreateFlagBits::RAY_TRACING_ALLOW_MOTION_NV.0;
        const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = PipelineCreateFlagBits::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
        const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = PipelineCreateFlagBits::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.0;
        const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT = PipelineCreateFlagBits::RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT.0;
        const RAY_TRACING_OPACITY_MICROMAP_EXT = PipelineCreateFlagBits::RAY_TRACING_OPACITY_MICROMAP_EXT.0;
        const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT = PipelineCreateFlagBits::COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT.0;
        const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT = PipelineCreateFlagBits::DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT.0;
        const NO_PROTECTED_ACCESS = PipelineCreateFlagBits::NO_PROTECTED_ACCESS.0;
        const RAY_TRACING_DISPLACEMENT_MICROMAP_NV = PipelineCreateFlagBits::RAY_TRACING_DISPLACEMENT_MICROMAP_NV.0;
        const DESCRIPTOR_BUFFER_EXT = PipelineCreateFlagBits::DESCRIPTOR_BUFFER_EXT.0;
        const PROTECTED_ACCESS_ONLY = PipelineCreateFlagBits::PROTECTED_ACCESS_ONLY.0;
        const DISPATCH_BASE_KHR = Self::DISPATCH_BASE.bits();
        const EARLY_RETURN_ON_FAILURE_EXT = Self::EARLY_RETURN_ON_FAILURE.bits();
        const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT = Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED.bits();
        const NO_PROTECTED_ACCESS_EXT = Self::NO_PROTECTED_ACCESS.bits();
        const PROTECTED_ACCESS_ONLY_EXT = Self::PROTECTED_ACCESS_ONLY.bits();
        const VIEW_INDEX_FROM_DEVICE_INDEX_KHR = Self::VIEW_INDEX_FROM_DEVICE_INDEX.bits();
        const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT = Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT.bits();
        const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreateFlagBits(u32);
impl PipelineCreateFlagBits {
    pub const DISABLE_OPTIMIZATION: Self = Self(1 << 0);
    pub const ALLOW_DERIVATIVES: Self = Self(1 << 1);
    pub const DERIVATIVE: Self = Self(1 << 2);
    pub const VIEW_INDEX_FROM_DEVICE_INDEX: Self = Self(1 << 3);
    pub const DISPATCH_BASE: Self = Self(1 << 4);
    pub const DEFER_COMPILE_NV: Self = Self(1 << 5);
    pub const CAPTURE_STATISTICS_KHR: Self = Self(1 << 6);
    pub const CAPTURE_INTERNAL_REPRESENTATIONS_KHR: Self = Self(1 << 7);
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED: Self = Self(1 << 8);
    pub const EARLY_RETURN_ON_FAILURE: Self = Self(1 << 9);
    pub const LINK_TIME_OPTIMIZATION_EXT: Self = Self(1 << 10);
    pub const LIBRARY_KHR: Self = Self(1 << 11);
    pub const RAY_TRACING_SKIP_TRIANGLES_KHR: Self = Self(1 << 12);
    pub const RAY_TRACING_SKIP_AABBS_KHR: Self = Self(1 << 13);
    pub const RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR: Self = Self(1 << 14);
    pub const RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR: Self = Self(1 << 15);
    pub const RAY_TRACING_NO_NULL_MISS_SHADERS_KHR: Self = Self(1 << 16);
    pub const RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR: Self = Self(1 << 17);
    pub const INDIRECT_BINDABLE_NV: Self = Self(1 << 18);
    pub const RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_KHR: Self = Self(1 << 19);
    pub const RAY_TRACING_ALLOW_MOTION_NV: Self = Self(1 << 20);
    pub const RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 21);
    pub const RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self = Self(1 << 22);
    pub const RETAIN_LINK_TIME_OPTIMIZATION_INFO_EXT: Self = Self(1 << 23);
    pub const RAY_TRACING_OPACITY_MICROMAP_EXT: Self = Self(1 << 24);
    pub const COLOR_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 25);
    pub const DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_EXT: Self = Self(1 << 26);
    pub const NO_PROTECTED_ACCESS: Self = Self(1 << 27);
    pub const RAY_TRACING_DISPLACEMENT_MICROMAP_NV: Self = Self(1 << 28);
    pub const DESCRIPTOR_BUFFER_EXT: Self = Self(1 << 29);
    pub const PROTECTED_ACCESS_ONLY: Self = Self(1 << 30);
    pub const DISPATCH_BASE_KHR: Self = Self::DISPATCH_BASE;
    pub const EARLY_RETURN_ON_FAILURE_EXT: Self = Self::EARLY_RETURN_ON_FAILURE;
    pub const FAIL_ON_PIPELINE_COMPILE_REQUIRED_EXT: Self = Self::FAIL_ON_PIPELINE_COMPILE_REQUIRED;
    pub const NO_PROTECTED_ACCESS_EXT: Self = Self::NO_PROTECTED_ACCESS;
    pub const PROTECTED_ACCESS_ONLY_EXT: Self = Self::PROTECTED_ACCESS_ONLY;
    pub const VIEW_INDEX_FROM_DEVICE_INDEX_KHR: Self = Self::VIEW_INDEX_FROM_DEVICE_INDEX;
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT: Self =
        Self::RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_EXT;
    pub const PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self =
        Self::RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ColorComponentFlags: Flags {
        const R = ColorComponentFlagBits::R.0;
        const G = ColorComponentFlagBits::G.0;
        const B = ColorComponentFlagBits::B.0;
        const A = ColorComponentFlagBits::A.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ColorComponentFlagBits(u32);
impl ColorComponentFlagBits {
    pub const R: Self = Self(1 << 0);
    pub const G: Self = Self(1 << 1);
    pub const B: Self = Self(1 << 2);
    pub const A: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FenceCreateFlags: Flags {
        const SIGNALED = FenceCreateFlagBits::SIGNALED.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceCreateFlagBits(u32);
impl FenceCreateFlagBits {
    pub const SIGNALED: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SemaphoreCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FormatFeatureFlags: Flags {
        const SAMPLED_IMAGE = FormatFeatureFlagBits::SAMPLED_IMAGE.0;
        const STORAGE_IMAGE = FormatFeatureFlagBits::STORAGE_IMAGE.0;
        const STORAGE_IMAGE_ATOMIC = FormatFeatureFlagBits::STORAGE_IMAGE_ATOMIC.0;
        const UNIFORM_TEXEL_BUFFER = FormatFeatureFlagBits::UNIFORM_TEXEL_BUFFER.0;
        const STORAGE_TEXEL_BUFFER = FormatFeatureFlagBits::STORAGE_TEXEL_BUFFER.0;
        const STORAGE_TEXEL_BUFFER_ATOMIC = FormatFeatureFlagBits::STORAGE_TEXEL_BUFFER_ATOMIC.0;
        const VERTEX_BUFFER = FormatFeatureFlagBits::VERTEX_BUFFER.0;
        const COLOR_ATTACHMENT = FormatFeatureFlagBits::COLOR_ATTACHMENT.0;
        const COLOR_ATTACHMENT_BLEND = FormatFeatureFlagBits::COLOR_ATTACHMENT_BLEND.0;
        const DEPTH_STENCIL_ATTACHMENT = FormatFeatureFlagBits::DEPTH_STENCIL_ATTACHMENT.0;
        const BLIT_SRC = FormatFeatureFlagBits::BLIT_SRC.0;
        const BLIT_DST = FormatFeatureFlagBits::BLIT_DST.0;
        const SAMPLED_IMAGE_FILTER_LINEAR = FormatFeatureFlagBits::SAMPLED_IMAGE_FILTER_LINEAR.0;
        const SAMPLED_IMAGE_FILTER_CUBIC_EXT = FormatFeatureFlagBits::SAMPLED_IMAGE_FILTER_CUBIC_EXT.0;
        const TRANSFER_SRC = FormatFeatureFlagBits::TRANSFER_SRC.0;
        const TRANSFER_DST = FormatFeatureFlagBits::TRANSFER_DST.0;
        const SAMPLED_IMAGE_FILTER_MINMAX = FormatFeatureFlagBits::SAMPLED_IMAGE_FILTER_MINMAX.0;
        const MIDPOINT_CHROMA_SAMPLES = FormatFeatureFlagBits::MIDPOINT_CHROMA_SAMPLES.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = FormatFeatureFlagBits::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = FormatFeatureFlagBits::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = FormatFeatureFlagBits::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = FormatFeatureFlagBits::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE.0;
        const DISJOINT = FormatFeatureFlagBits::DISJOINT.0;
        const COSITED_CHROMA_SAMPLES = FormatFeatureFlagBits::COSITED_CHROMA_SAMPLES.0;
        const FRAGMENT_DENSITY_MAP_EXT = FormatFeatureFlagBits::FRAGMENT_DENSITY_MAP_EXT.0;
        const VIDEO_DECODE_OUTPUT_KHR = FormatFeatureFlagBits::VIDEO_DECODE_OUTPUT_KHR.0;
        const VIDEO_DECODE_DPB_KHR = FormatFeatureFlagBits::VIDEO_DECODE_DPB_KHR.0;
        const VIDEO_ENCODE_INPUT_KHR = FormatFeatureFlagBits::VIDEO_ENCODE_INPUT_KHR.0;
        const VIDEO_ENCODE_DPB_KHR = FormatFeatureFlagBits::VIDEO_ENCODE_DPB_KHR.0;
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = FormatFeatureFlagBits::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = FormatFeatureFlagBits::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
        const COSITED_CHROMA_SAMPLES_KHR = Self::COSITED_CHROMA_SAMPLES.bits();
        const DISJOINT_KHR = Self::DISJOINT.bits();
        const MIDPOINT_CHROMA_SAMPLES_KHR = Self::MIDPOINT_CHROMA_SAMPLES.bits();
        const SAMPLED_IMAGE_FILTER_CUBIC_IMG = Self::SAMPLED_IMAGE_FILTER_CUBIC_EXT.bits();
        const SAMPLED_IMAGE_FILTER_MINMAX_EXT = Self::SAMPLED_IMAGE_FILTER_MINMAX.bits();
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.bits();
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE.bits();
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.bits();
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR = Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.bits();
        const TRANSFER_DST_KHR = Self::TRANSFER_DST.bits();
        const TRANSFER_SRC_KHR = Self::TRANSFER_SRC.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatFeatureFlagBits(u32);
impl FormatFeatureFlagBits {
    pub const SAMPLED_IMAGE: Self = Self(1 << 0);
    pub const STORAGE_IMAGE: Self = Self(1 << 1);
    pub const STORAGE_IMAGE_ATOMIC: Self = Self(1 << 2);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 3);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 4);
    pub const STORAGE_TEXEL_BUFFER_ATOMIC: Self = Self(1 << 5);
    pub const VERTEX_BUFFER: Self = Self(1 << 6);
    pub const COLOR_ATTACHMENT: Self = Self(1 << 7);
    pub const COLOR_ATTACHMENT_BLEND: Self = Self(1 << 8);
    pub const DEPTH_STENCIL_ATTACHMENT: Self = Self(1 << 9);
    pub const BLIT_SRC: Self = Self(1 << 10);
    pub const BLIT_DST: Self = Self(1 << 11);
    pub const SAMPLED_IMAGE_FILTER_LINEAR: Self = Self(1 << 12);
    pub const SAMPLED_IMAGE_FILTER_CUBIC_EXT: Self = Self(1 << 13);
    pub const TRANSFER_SRC: Self = Self(1 << 14);
    pub const TRANSFER_DST: Self = Self(1 << 15);
    pub const SAMPLED_IMAGE_FILTER_MINMAX: Self = Self(1 << 16);
    pub const MIDPOINT_CHROMA_SAMPLES: Self = Self(1 << 17);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER: Self = Self(1 << 18);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER: Self = Self(1 << 19);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT: Self = Self(1 << 20);
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE: Self =
        Self(1 << 21);
    pub const DISJOINT: Self = Self(1 << 22);
    pub const COSITED_CHROMA_SAMPLES: Self = Self(1 << 23);
    pub const FRAGMENT_DENSITY_MAP_EXT: Self = Self(1 << 24);
    pub const VIDEO_DECODE_OUTPUT_KHR: Self = Self(1 << 25);
    pub const VIDEO_DECODE_DPB_KHR: Self = Self(1 << 26);
    pub const VIDEO_ENCODE_INPUT_KHR: Self = Self(1 << 27);
    pub const VIDEO_ENCODE_DPB_KHR: Self = Self(1 << 28);
    pub const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR: Self = Self(1 << 29);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 30);
    pub const COSITED_CHROMA_SAMPLES_KHR: Self = Self::COSITED_CHROMA_SAMPLES;
    pub const DISJOINT_KHR: Self = Self::DISJOINT;
    pub const MIDPOINT_CHROMA_SAMPLES_KHR: Self = Self::MIDPOINT_CHROMA_SAMPLES;
    pub const SAMPLED_IMAGE_FILTER_CUBIC_IMG: Self = Self::SAMPLED_IMAGE_FILTER_CUBIC_EXT;
    pub const SAMPLED_IMAGE_FILTER_MINMAX_EXT: Self = Self::SAMPLED_IMAGE_FILTER_MINMAX;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER;
    pub const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_KHR: Self =
        Self::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER;
    pub const TRANSFER_DST_KHR: Self = Self::TRANSFER_DST;
    pub const TRANSFER_SRC_KHR: Self = Self::TRANSFER_SRC;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryControlFlags: Flags {
        const PRECISE = QueryControlFlagBits::PRECISE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryControlFlagBits(u32);
impl QueryControlFlagBits {
    pub const PRECISE: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryResultFlags: Flags {
        const _64 = QueryResultFlagBits::_64.0;
        const WAIT = QueryResultFlagBits::WAIT.0;
        const WITH_AVAILABILITY = QueryResultFlagBits::WITH_AVAILABILITY.0;
        const PARTIAL = QueryResultFlagBits::PARTIAL.0;
        const WITH_STATUS_KHR = QueryResultFlagBits::WITH_STATUS_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryResultFlagBits(u32);
impl QueryResultFlagBits {
    pub const _64: Self = Self(1 << 0);
    pub const WAIT: Self = Self(1 << 1);
    pub const WITH_AVAILABILITY: Self = Self(1 << 2);
    pub const PARTIAL: Self = Self(1 << 3);
    pub const WITH_STATUS_KHR: Self = Self(1 << 4);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ShaderModuleCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct EventCreateFlags: Flags {
        const DEVICE_ONLY = EventCreateFlagBits::DEVICE_ONLY.0;
        const DEVICE_ONLY_KHR = Self::DEVICE_ONLY.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventCreateFlagBits(u32);
impl EventCreateFlagBits {
    pub const DEVICE_ONLY: Self = Self(1 << 0);
    pub const DEVICE_ONLY_KHR: Self = Self::DEVICE_ONLY;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommandPoolCreateFlags: Flags {
        const TRANSIENT = CommandPoolCreateFlagBits::TRANSIENT.0;
        const RESET_COMMAND_BUFFER = CommandPoolCreateFlagBits::RESET_COMMAND_BUFFER.0;
        const PROTECTED = CommandPoolCreateFlagBits::PROTECTED.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolCreateFlagBits(u32);
impl CommandPoolCreateFlagBits {
    pub const TRANSIENT: Self = Self(1 << 0);
    pub const RESET_COMMAND_BUFFER: Self = Self(1 << 1);
    pub const PROTECTED: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommandPoolResetFlags: Flags {
        const RELEASE_RESOURCES = CommandPoolResetFlagBits::RELEASE_RESOURCES.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandPoolResetFlagBits(u32);
impl CommandPoolResetFlagBits {
    pub const RELEASE_RESOURCES: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommandBufferResetFlags: Flags {
        const RELEASE_RESOURCES = CommandBufferResetFlagBits::RELEASE_RESOURCES.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferResetFlagBits(u32);
impl CommandBufferResetFlagBits {
    pub const RELEASE_RESOURCES: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CommandBufferUsageFlags: Flags {
        const ONE_TIME_SUBMIT = CommandBufferUsageFlagBits::ONE_TIME_SUBMIT.0;
        const RENDER_PASS_CONTINUE = CommandBufferUsageFlagBits::RENDER_PASS_CONTINUE.0;
        const SIMULTANEOUS_USE = CommandBufferUsageFlagBits::SIMULTANEOUS_USE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommandBufferUsageFlagBits(u32);
impl CommandBufferUsageFlagBits {
    pub const ONE_TIME_SUBMIT: Self = Self(1 << 0);
    pub const RENDER_PASS_CONTINUE: Self = Self(1 << 1);
    pub const SIMULTANEOUS_USE: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueryPipelineStatisticFlags: Flags {
        const INPUT_ASSEMBLY_VERTICES = QueryPipelineStatisticFlagBits::INPUT_ASSEMBLY_VERTICES.0;
        const INPUT_ASSEMBLY_PRIMITIVES = QueryPipelineStatisticFlagBits::INPUT_ASSEMBLY_PRIMITIVES.0;
        const VERTEX_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits::VERTEX_SHADER_INVOCATIONS.0;
        const GEOMETRY_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits::GEOMETRY_SHADER_INVOCATIONS.0;
        const GEOMETRY_SHADER_PRIMITIVES = QueryPipelineStatisticFlagBits::GEOMETRY_SHADER_PRIMITIVES.0;
        const CLIPPING_INVOCATIONS = QueryPipelineStatisticFlagBits::CLIPPING_INVOCATIONS.0;
        const CLIPPING_PRIMITIVES = QueryPipelineStatisticFlagBits::CLIPPING_PRIMITIVES.0;
        const FRAGMENT_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits::FRAGMENT_SHADER_INVOCATIONS.0;
        const TESSELLATION_CONTROL_SHADER_PATCHES = QueryPipelineStatisticFlagBits::TESSELLATION_CONTROL_SHADER_PATCHES.0;
        const TESSELLATION_EVALUATION_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits::TESSELLATION_EVALUATION_SHADER_INVOCATIONS.0;
        const COMPUTE_SHADER_INVOCATIONS = QueryPipelineStatisticFlagBits::COMPUTE_SHADER_INVOCATIONS.0;
        const TASK_SHADER_INVOCATIONS_EXT = QueryPipelineStatisticFlagBits::TASK_SHADER_INVOCATIONS_EXT.0;
        const MESH_SHADER_INVOCATIONS_EXT = QueryPipelineStatisticFlagBits::MESH_SHADER_INVOCATIONS_EXT.0;
        const CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI = QueryPipelineStatisticFlagBits::CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryPipelineStatisticFlagBits(u32);
impl QueryPipelineStatisticFlagBits {
    pub const INPUT_ASSEMBLY_VERTICES: Self = Self(1 << 0);
    pub const INPUT_ASSEMBLY_PRIMITIVES: Self = Self(1 << 1);
    pub const VERTEX_SHADER_INVOCATIONS: Self = Self(1 << 2);
    pub const GEOMETRY_SHADER_INVOCATIONS: Self = Self(1 << 3);
    pub const GEOMETRY_SHADER_PRIMITIVES: Self = Self(1 << 4);
    pub const CLIPPING_INVOCATIONS: Self = Self(1 << 5);
    pub const CLIPPING_PRIMITIVES: Self = Self(1 << 6);
    pub const FRAGMENT_SHADER_INVOCATIONS: Self = Self(1 << 7);
    pub const TESSELLATION_CONTROL_SHADER_PATCHES: Self = Self(1 << 8);
    pub const TESSELLATION_EVALUATION_SHADER_INVOCATIONS: Self = Self(1 << 9);
    pub const COMPUTE_SHADER_INVOCATIONS: Self = Self(1 << 10);
    pub const TASK_SHADER_INVOCATIONS_EXT: Self = Self(1 << 11);
    pub const MESH_SHADER_INVOCATIONS_EXT: Self = Self(1 << 12);
    pub const CLUSTER_CULLING_SHADER_INVOCATIONS_HUAWEI: Self = Self(1 << 13);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MemoryMapFlags: Flags {
        const PLACED_EXT = MemoryMapFlagBits::PLACED_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryMapFlagBits(u32);
impl MemoryMapFlagBits {
    pub const PLACED_EXT: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageAspectFlags: Flags {
        const COLOR = ImageAspectFlagBits::COLOR.0;
        const DEPTH = ImageAspectFlagBits::DEPTH.0;
        const STENCIL = ImageAspectFlagBits::STENCIL.0;
        const METADATA = ImageAspectFlagBits::METADATA.0;
        const PLANE_0 = ImageAspectFlagBits::PLANE_0.0;
        const PLANE_1 = ImageAspectFlagBits::PLANE_1.0;
        const PLANE_2 = ImageAspectFlagBits::PLANE_2.0;
        const MEMORY_PLANE_0_EXT = ImageAspectFlagBits::MEMORY_PLANE_0_EXT.0;
        const MEMORY_PLANE_1_EXT = ImageAspectFlagBits::MEMORY_PLANE_1_EXT.0;
        const MEMORY_PLANE_2_EXT = ImageAspectFlagBits::MEMORY_PLANE_2_EXT.0;
        const MEMORY_PLANE_3_EXT = ImageAspectFlagBits::MEMORY_PLANE_3_EXT.0;
        const NONE_KHR = Self::NONE.bits();
        const PLANE_0_KHR = Self::PLANE_0.bits();
        const PLANE_1_KHR = Self::PLANE_1.bits();
        const PLANE_2_KHR = Self::PLANE_2.bits();
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ImageAspectFlagBits(u32);
impl ImageAspectFlagBits {
    pub const COLOR: Self = Self(1 << 0);
    pub const DEPTH: Self = Self(1 << 1);
    pub const STENCIL: Self = Self(1 << 2);
    pub const METADATA: Self = Self(1 << 3);
    pub const PLANE_0: Self = Self(1 << 4);
    pub const PLANE_1: Self = Self(1 << 5);
    pub const PLANE_2: Self = Self(1 << 6);
    pub const MEMORY_PLANE_0_EXT: Self = Self(1 << 7);
    pub const MEMORY_PLANE_1_EXT: Self = Self(1 << 8);
    pub const MEMORY_PLANE_2_EXT: Self = Self(1 << 9);
    pub const MEMORY_PLANE_3_EXT: Self = Self(1 << 10);
    pub const PLANE_0_KHR: Self = Self::PLANE_0;
    pub const PLANE_1_KHR: Self = Self::PLANE_1;
    pub const PLANE_2_KHR: Self = Self::PLANE_2;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SparseMemoryBindFlags: Flags {
        const METADATA = SparseMemoryBindFlagBits::METADATA.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseMemoryBindFlagBits(u32);
impl SparseMemoryBindFlagBits {
    pub const METADATA: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SparseImageFormatFlags: Flags {
        const SINGLE_MIPTAIL = SparseImageFormatFlagBits::SINGLE_MIPTAIL.0;
        const ALIGNED_MIP_SIZE = SparseImageFormatFlagBits::ALIGNED_MIP_SIZE.0;
        const NONSTANDARD_BLOCK_SIZE = SparseImageFormatFlagBits::NONSTANDARD_BLOCK_SIZE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SparseImageFormatFlagBits(u32);
impl SparseImageFormatFlagBits {
    pub const SINGLE_MIPTAIL: Self = Self(1 << 0);
    pub const ALIGNED_MIP_SIZE: Self = Self(1 << 1);
    pub const NONSTANDARD_BLOCK_SIZE: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SubpassDescriptionFlags: Flags {
        const PER_VIEW_ATTRIBUTES_NVX = SubpassDescriptionFlagBits::PER_VIEW_ATTRIBUTES_NVX.0;
        const PER_VIEW_POSITION_X_ONLY_NVX = SubpassDescriptionFlagBits::PER_VIEW_POSITION_X_ONLY_NVX.0;
        const FRAGMENT_REGION_EXT = SubpassDescriptionFlagBits::FRAGMENT_REGION_EXT.0;
        const CUSTOM_RESOLVE_EXT = SubpassDescriptionFlagBits::CUSTOM_RESOLVE_EXT.0;
        const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT = SubpassDescriptionFlagBits::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT.0;
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT = SubpassDescriptionFlagBits::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT.0;
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT = SubpassDescriptionFlagBits::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT.0;
        const ENABLE_LEGACY_DITHERING_EXT = SubpassDescriptionFlagBits::ENABLE_LEGACY_DITHERING_EXT.0;
        const TILE_SHADING_APRON_QCOM = SubpassDescriptionFlagBits::TILE_SHADING_APRON_QCOM.0;
        const FRAGMENT_REGION_QCOM = Self::FRAGMENT_REGION_EXT.bits();
        const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM = Self::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT.bits();
        const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM = Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT.bits();
        const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM = Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT.bits();
        const SHADER_RESOLVE_QCOM = Self::CUSTOM_RESOLVE_EXT.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubpassDescriptionFlagBits(u32);
impl SubpassDescriptionFlagBits {
    pub const PER_VIEW_ATTRIBUTES_NVX: Self = Self(1 << 0);
    pub const PER_VIEW_POSITION_X_ONLY_NVX: Self = Self(1 << 1);
    pub const FRAGMENT_REGION_EXT: Self = Self(1 << 2);
    pub const CUSTOM_RESOLVE_EXT: Self = Self(1 << 3);
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT: Self = Self(1 << 4);
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT: Self = Self(1 << 5);
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT: Self = Self(1 << 6);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(1 << 7);
    pub const TILE_SHADING_APRON_QCOM: Self = Self(1 << 8);
    pub const FRAGMENT_REGION_QCOM: Self = Self::FRAGMENT_REGION_EXT;
    pub const RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_EXT;
    pub const RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_EXT;
    pub const RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM: Self =
        Self::RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_EXT;
    pub const SHADER_RESOLVE_QCOM: Self = Self::CUSTOM_RESOLVE_EXT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineStageFlags: Flags {
        const TOP_OF_PIPE = PipelineStageFlagBits::TOP_OF_PIPE.0;
        const DRAW_INDIRECT = PipelineStageFlagBits::DRAW_INDIRECT.0;
        const VERTEX_INPUT = PipelineStageFlagBits::VERTEX_INPUT.0;
        const VERTEX_SHADER = PipelineStageFlagBits::VERTEX_SHADER.0;
        const TESSELLATION_CONTROL_SHADER = PipelineStageFlagBits::TESSELLATION_CONTROL_SHADER.0;
        const TESSELLATION_EVALUATION_SHADER = PipelineStageFlagBits::TESSELLATION_EVALUATION_SHADER.0;
        const GEOMETRY_SHADER = PipelineStageFlagBits::GEOMETRY_SHADER.0;
        const FRAGMENT_SHADER = PipelineStageFlagBits::FRAGMENT_SHADER.0;
        const EARLY_FRAGMENT_TESTS = PipelineStageFlagBits::EARLY_FRAGMENT_TESTS.0;
        const LATE_FRAGMENT_TESTS = PipelineStageFlagBits::LATE_FRAGMENT_TESTS.0;
        const COLOR_ATTACHMENT_OUTPUT = PipelineStageFlagBits::COLOR_ATTACHMENT_OUTPUT.0;
        const COMPUTE_SHADER = PipelineStageFlagBits::COMPUTE_SHADER.0;
        const TRANSFER = PipelineStageFlagBits::TRANSFER.0;
        const BOTTOM_OF_PIPE = PipelineStageFlagBits::BOTTOM_OF_PIPE.0;
        const HOST = PipelineStageFlagBits::HOST.0;
        const ALL_GRAPHICS = PipelineStageFlagBits::ALL_GRAPHICS.0;
        const ALL_COMMANDS = PipelineStageFlagBits::ALL_COMMANDS.0;
        const COMMAND_PREPROCESS_EXT = PipelineStageFlagBits::COMMAND_PREPROCESS_EXT.0;
        const CONDITIONAL_RENDERING_EXT = PipelineStageFlagBits::CONDITIONAL_RENDERING_EXT.0;
        const TASK_SHADER_EXT = PipelineStageFlagBits::TASK_SHADER_EXT.0;
        const MESH_SHADER_EXT = PipelineStageFlagBits::MESH_SHADER_EXT.0;
        const RAY_TRACING_SHADER_KHR = PipelineStageFlagBits::RAY_TRACING_SHADER_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = PipelineStageFlagBits::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
        const FRAGMENT_DENSITY_PROCESS_EXT = PipelineStageFlagBits::FRAGMENT_DENSITY_PROCESS_EXT.0;
        const TRANSFORM_FEEDBACK_EXT = PipelineStageFlagBits::TRANSFORM_FEEDBACK_EXT.0;
        const ACCELERATION_STRUCTURE_BUILD_KHR = PipelineStageFlagBits::ACCELERATION_STRUCTURE_BUILD_KHR.0;
        const ACCELERATION_STRUCTURE_BUILD_NV = Self::ACCELERATION_STRUCTURE_BUILD_KHR.bits();
        const COMMAND_PREPROCESS_NV = Self::COMMAND_PREPROCESS_EXT.bits();
        const MESH_SHADER_NV = Self::MESH_SHADER_EXT.bits();
        const NONE_KHR = Self::NONE.bits();
        const RAY_TRACING_SHADER_NV = Self::RAY_TRACING_SHADER_KHR.bits();
        const SHADING_RATE_IMAGE_NV = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.bits();
        const TASK_SHADER_NV = Self::TASK_SHADER_EXT.bits();
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineStageFlagBits(u32);
impl PipelineStageFlagBits {
    pub const TOP_OF_PIPE: Self = Self(1 << 0);
    pub const DRAW_INDIRECT: Self = Self(1 << 1);
    pub const VERTEX_INPUT: Self = Self(1 << 2);
    pub const VERTEX_SHADER: Self = Self(1 << 3);
    pub const TESSELLATION_CONTROL_SHADER: Self = Self(1 << 4);
    pub const TESSELLATION_EVALUATION_SHADER: Self = Self(1 << 5);
    pub const GEOMETRY_SHADER: Self = Self(1 << 6);
    pub const FRAGMENT_SHADER: Self = Self(1 << 7);
    pub const EARLY_FRAGMENT_TESTS: Self = Self(1 << 8);
    pub const LATE_FRAGMENT_TESTS: Self = Self(1 << 9);
    pub const COLOR_ATTACHMENT_OUTPUT: Self = Self(1 << 10);
    pub const COMPUTE_SHADER: Self = Self(1 << 11);
    pub const TRANSFER: Self = Self(1 << 12);
    pub const BOTTOM_OF_PIPE: Self = Self(1 << 13);
    pub const HOST: Self = Self(1 << 14);
    pub const ALL_GRAPHICS: Self = Self(1 << 15);
    pub const ALL_COMMANDS: Self = Self(1 << 16);
    pub const COMMAND_PREPROCESS_EXT: Self = Self(1 << 17);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 18);
    pub const TASK_SHADER_EXT: Self = Self(1 << 19);
    pub const MESH_SHADER_EXT: Self = Self(1 << 20);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(1 << 21);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 22);
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(1 << 23);
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(1 << 24);
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(1 << 25);
    pub const ACCELERATION_STRUCTURE_BUILD_NV: Self = Self::ACCELERATION_STRUCTURE_BUILD_KHR;
    pub const COMMAND_PREPROCESS_NV: Self = Self::COMMAND_PREPROCESS_EXT;
    pub const MESH_SHADER_NV: Self = Self::MESH_SHADER_EXT;
    pub const RAY_TRACING_SHADER_NV: Self = Self::RAY_TRACING_SHADER_KHR;
    pub const SHADING_RATE_IMAGE_NV: Self = Self::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR;
    pub const TASK_SHADER_NV: Self = Self::TASK_SHADER_EXT;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SampleCountFlags: Flags {
        const _1 = SampleCountFlagBits::_1.0;
        const _2 = SampleCountFlagBits::_2.0;
        const _4 = SampleCountFlagBits::_4.0;
        const _8 = SampleCountFlagBits::_8.0;
        const _16 = SampleCountFlagBits::_16.0;
        const _32 = SampleCountFlagBits::_32.0;
        const _64 = SampleCountFlagBits::_64.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SampleCountFlagBits(u32);
impl SampleCountFlagBits {
    pub const _1: Self = Self(1 << 0);
    pub const _2: Self = Self(1 << 1);
    pub const _4: Self = Self(1 << 2);
    pub const _8: Self = Self(1 << 3);
    pub const _16: Self = Self(1 << 4);
    pub const _32: Self = Self(1 << 5);
    pub const _64: Self = Self(1 << 6);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AttachmentDescriptionFlags: Flags {
        const MAY_ALIAS = AttachmentDescriptionFlagBits::MAY_ALIAS.0;
        const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR = AttachmentDescriptionFlagBits::RESOLVE_SKIP_TRANSFER_FUNCTION_KHR.0;
        const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR = AttachmentDescriptionFlagBits::RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AttachmentDescriptionFlagBits(u32);
impl AttachmentDescriptionFlagBits {
    pub const MAY_ALIAS: Self = Self(1 << 0);
    pub const RESOLVE_SKIP_TRANSFER_FUNCTION_KHR: Self = Self(1 << 1);
    pub const RESOLVE_ENABLE_TRANSFER_FUNCTION_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StencilFaceFlags: Flags {
        const FRONT = StencilFaceFlagBits::FRONT.0;
        const BACK = StencilFaceFlagBits::BACK.0;
        const STENCIL_FRONT_AND_BACK = Self::FRONT_AND_BACK.bits();
        const FRONT_AND_BACK = 0x00000003;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StencilFaceFlagBits(u32);
impl StencilFaceFlagBits {
    pub const FRONT: Self = Self(1 << 0);
    pub const BACK: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct CullModeFlags: Flags {
        const FRONT = CullModeFlagBits::FRONT.0;
        const BACK = CullModeFlagBits::BACK.0;
        const NONE = 0;
        const FRONT_AND_BACK = 0x00000003;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CullModeFlagBits(u32);
impl CullModeFlagBits {
    pub const FRONT: Self = Self(1 << 0);
    pub const BACK: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DescriptorPoolCreateFlags: Flags {
        const FREE_DESCRIPTOR_SET = DescriptorPoolCreateFlagBits::FREE_DESCRIPTOR_SET.0;
        const UPDATE_AFTER_BIND = DescriptorPoolCreateFlagBits::UPDATE_AFTER_BIND.0;
        const HOST_ONLY_EXT = DescriptorPoolCreateFlagBits::HOST_ONLY_EXT.0;
        const ALLOW_OVERALLOCATION_SETS_NV = DescriptorPoolCreateFlagBits::ALLOW_OVERALLOCATION_SETS_NV.0;
        const ALLOW_OVERALLOCATION_POOLS_NV = DescriptorPoolCreateFlagBits::ALLOW_OVERALLOCATION_POOLS_NV.0;
        const HOST_ONLY_VALVE = Self::HOST_ONLY_EXT.bits();
        const UPDATE_AFTER_BIND_EXT = Self::UPDATE_AFTER_BIND.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorPoolCreateFlagBits(u32);
impl DescriptorPoolCreateFlagBits {
    pub const FREE_DESCRIPTOR_SET: Self = Self(1 << 0);
    pub const UPDATE_AFTER_BIND: Self = Self(1 << 1);
    pub const HOST_ONLY_EXT: Self = Self(1 << 2);
    pub const ALLOW_OVERALLOCATION_SETS_NV: Self = Self(1 << 3);
    pub const ALLOW_OVERALLOCATION_POOLS_NV: Self = Self(1 << 4);
    pub const HOST_ONLY_VALVE: Self = Self::HOST_ONLY_EXT;
    pub const UPDATE_AFTER_BIND_EXT: Self = Self::UPDATE_AFTER_BIND;
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DescriptorPoolResetFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DependencyFlags: Flags {
        const BY_REGION = DependencyFlagBits::BY_REGION.0;
        const VIEW_LOCAL = DependencyFlagBits::VIEW_LOCAL.0;
        const DEVICE_GROUP = DependencyFlagBits::DEVICE_GROUP.0;
        const FEEDBACK_LOOP_EXT = DependencyFlagBits::FEEDBACK_LOOP_EXT.0;
        const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR = DependencyFlagBits::QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR.0;
        const ASYMMETRIC_EVENT_KHR = DependencyFlagBits::ASYMMETRIC_EVENT_KHR.0;
        const DEVICE_GROUP_KHR = Self::DEVICE_GROUP.bits();
        const VIEW_LOCAL_KHR = Self::VIEW_LOCAL.bits();
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DependencyFlagBits(u32);
impl DependencyFlagBits {
    pub const BY_REGION: Self = Self(1 << 0);
    pub const VIEW_LOCAL: Self = Self(1 << 1);
    pub const DEVICE_GROUP: Self = Self(1 << 2);
    pub const FEEDBACK_LOOP_EXT: Self = Self(1 << 3);
    pub const QUEUE_FAMILY_OWNERSHIP_TRANSFER_USE_ALL_STAGES_KHR: Self = Self(1 << 5);
    pub const ASYMMETRIC_EVENT_KHR: Self = Self(1 << 6);
    pub const DEVICE_GROUP_KHR: Self = Self::DEVICE_GROUP;
    pub const VIEW_LOCAL_KHR: Self = Self::VIEW_LOCAL;
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
    p_create_info: *const InstanceCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_instance: *mut Instance,
) -> Result;
pub type PFN_vkDestroyInstance =
    unsafe extern "system" fn(instance: Instance, p_allocator: *const AllocationCallbacks<'_>);
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
    p_create_info: *const DeviceCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_device: *mut Device,
) -> Result;
pub type PFN_vkDestroyDevice =
    unsafe extern "system" fn(device: Device, p_allocator: *const AllocationCallbacks<'_>);
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
    p_submits: *const SubmitInfo<'_>,
    fence: Fence,
) -> Result;
pub type PFN_vkQueueWaitIdle = unsafe extern "system" fn(queue: Queue) -> Result;
pub type PFN_vkDeviceWaitIdle = unsafe extern "system" fn(device: Device) -> Result;
pub type PFN_vkAllocateMemory = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const MemoryAllocateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_memory: *mut DeviceMemory,
) -> Result;
pub type PFN_vkFreeMemory = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    p_allocator: *const AllocationCallbacks<'_>,
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
    p_memory_ranges: *const MappedMemoryRange<'_>,
) -> Result;
pub type PFN_vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: Device,
    memory_range_count: u32,
    p_memory_ranges: *const MappedMemoryRange<'_>,
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
    samples: SampleCountFlagBits,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties,
);
pub type PFN_vkQueueBindSparse = unsafe extern "system" fn(
    queue: Queue,
    bind_info_count: u32,
    p_bind_info: *const BindSparseInfo<'_>,
    fence: Fence,
) -> Result;
pub type PFN_vkCreateFence = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const FenceCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_fence: *mut Fence,
) -> Result;
pub type PFN_vkDestroyFence = unsafe extern "system" fn(
    device: Device,
    fence: Fence,
    p_allocator: *const AllocationCallbacks<'_>,
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
    p_create_info: *const SemaphoreCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_semaphore: *mut Semaphore,
) -> Result;
pub type PFN_vkDestroySemaphore = unsafe extern "system" fn(
    device: Device,
    semaphore: Semaphore,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateEvent = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const EventCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_event: *mut Event,
) -> Result;
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
    device: Device,
    event: Event,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetEventStatus = unsafe extern "system" fn(device: Device, event: Event) -> Result;
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: Device, event: Event) -> Result;
pub type PFN_vkResetEvent = unsafe extern "system" fn(device: Device, event: Event) -> Result;
pub type PFN_vkCreateQueryPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const QueryPoolCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_query_pool: *mut QueryPool,
) -> Result;
pub type PFN_vkDestroyQueryPool = unsafe extern "system" fn(
    device: Device,
    query_pool: QueryPool,
    p_allocator: *const AllocationCallbacks<'_>,
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
    p_create_info: *const BufferCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_buffer: *mut Buffer,
) -> Result;
pub type PFN_vkDestroyBuffer = unsafe extern "system" fn(
    device: Device,
    buffer: Buffer,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferViewCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_view: *mut BufferView,
) -> Result;
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
    device: Device,
    buffer_view: BufferView,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateImage = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_image: *mut Image,
) -> Result;
pub type PFN_vkDestroyImage = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetImageSubresourceLayout = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource,
    p_layout: *mut SubresourceLayout,
);
pub type PFN_vkCreateImageView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ImageViewCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_view: *mut ImageView,
) -> Result;
pub type PFN_vkDestroyImageView = unsafe extern "system" fn(
    device: Device,
    image_view: ImageView,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_shader_module: *mut ShaderModule,
) -> Result;
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
    device: Device,
    shader_module: ShaderModule,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineCacheCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipeline_cache: *mut PipelineCache,
) -> Result;
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
    device: Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    p_src_caches: *const PipelineCache,
) -> Result;
pub type PFN_vkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const GraphicsPipelineCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ComputePipelineCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineLayoutCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipeline_layout: *mut PipelineLayout,
) -> Result;
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
    device: Device,
    pipeline_layout: PipelineLayout,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateSampler = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_sampler: *mut Sampler,
) -> Result;
pub type PFN_vkDestroySampler = unsafe extern "system" fn(
    device: Device,
    sampler: Sampler,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_set_layout: *mut DescriptorSetLayout,
) -> Result;
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: Device,
    descriptor_set_layout: DescriptorSetLayout,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorPoolCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_descriptor_pool: *mut DescriptorPool,
) -> Result;
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> Result;
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const DescriptorSetAllocateInfo<'_>,
    p_descriptor_sets: *mut DescriptorSet,
) -> Result;
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
) -> Result;
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
    device: Device,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet<'_>,
    descriptor_copy_count: u32,
    p_descriptor_copies: *const CopyDescriptorSet<'_>,
);
pub type PFN_vkCreateFramebuffer = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const FramebufferCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_framebuffer: *mut Framebuffer,
) -> Result;
pub type PFN_vkDestroyFramebuffer = unsafe extern "system" fn(
    device: Device,
    framebuffer: Framebuffer,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateRenderPass = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_render_pass: *mut RenderPass,
) -> Result;
pub type PFN_vkDestroyRenderPass = unsafe extern "system" fn(
    device: Device,
    render_pass: RenderPass,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetRenderAreaGranularity = unsafe extern "system" fn(
    device: Device,
    render_pass: RenderPass,
    p_granularity: *mut Extent2D,
);
pub type PFN_vkCreateCommandPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const CommandPoolCreateInfo<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_command_pool: *mut CommandPool,
) -> Result;
pub type PFN_vkDestroyCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkResetCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> Result;
pub type PFN_vkAllocateCommandBuffers = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const CommandBufferAllocateInfo<'_>,
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
    p_begin_info: *const CommandBufferBeginInfo<'_>,
) -> Result;
pub type PFN_vkEndCommandBuffer =
    unsafe extern "system" fn(command_buffer: CommandBuffer) -> Result;
pub type PFN_vkResetCommandBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    flags: CommandBufferResetFlags,
) -> Result;
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
pub type PFN_vkCmdSetViewport = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewports: *const Viewport,
);
pub type PFN_vkCmdSetScissor = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);
pub type PFN_vkCmdSetLineWidth =
    unsafe extern "system" fn(command_buffer: CommandBuffer, line_width: f32);
pub type PFN_vkCmdSetDepthBias = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);
pub type PFN_vkCmdSetBlendConstants =
    unsafe extern "system" fn(command_buffer: CommandBuffer, blend_constants: *const [f32; 4]);
pub type PFN_vkCmdSetDepthBounds = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
);
pub type PFN_vkCmdSetStencilCompareMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
);
pub type PFN_vkCmdSetStencilWriteMask = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
);
pub type PFN_vkCmdSetStencilReference = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
);
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    p_dynamic_offsets: *const u32,
);
pub type PFN_vkCmdBindIndexBuffer = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
);
pub type PFN_vkCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
);
pub type PFN_vkCmdDraw = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);
pub type PFN_vkCmdDrawIndexed = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);
pub type PFN_vkCmdDrawIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
pub type PFN_vkCmdDispatchIndirect =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
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
pub type PFN_vkCmdBlitImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageBlit,
    filter: Filter,
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
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_color: *const ClearColorValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
pub type PFN_vkCmdClearDepthStencilImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
pub type PFN_vkCmdClearAttachments = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    attachment_count: u32,
    p_attachments: *const ClearAttachment,
    rect_count: u32,
    p_rects: *const ClearRect,
);
pub type PFN_vkCmdResolveImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    p_regions: *const ImageResolve,
);
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier<'_>,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
);
pub type PFN_vkCmdPipelineBarrier = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier<'_>,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier<'_>,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier<'_>,
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
    pipeline_stage: PipelineStageFlagBits,
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
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: *const c_void,
);
pub type PFN_vkCmdBeginRenderPass = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo<'_>,
    contents: SubpassContents,
);
pub type PFN_vkCmdNextSubpass =
    unsafe extern "system" fn(command_buffer: CommandBuffer, contents: SubpassContents);
pub type PFN_vkCmdEndRenderPass = unsafe extern "system" fn(command_buffer: CommandBuffer);
pub type PFN_vkCmdExecuteCommands = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    command_buffer_count: u32,
    p_command_buffers: *const CommandBuffer,
);
