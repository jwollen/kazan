#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferUsageFlags2CreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: BufferUsageFlags2,
}
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
pub struct PhysicalDeviceMaintenance5Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance5: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance5Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub early_fragment_multisample_coverage_after_sample_counting: Bool32,
    pub early_fragment_sample_mask_test_before_sample_counting: Bool32,
    pub depth_stencil_swizzle_one_support: Bool32,
    pub polygon_mode_point_size: Bool32,
    pub non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
    pub non_strict_wide_lines_use_parallelogram: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance6Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance6: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance6Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub block_texel_view_compatible_multiple_layers: Bool32,
    pub max_combined_image_sampler_descriptor_count: u32,
    pub fragment_shading_rate_clamp_combiner_inputs: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingAreaInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueGlobalPriorityCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub global_priority: QueueGlobalPriority,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGlobalPriorityQueryFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub global_priority_query: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyGlobalPriorityProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub priority_count: u32,
    pub priorities: [QueueGlobalPriority; MAX_GLOBAL_PRIORITY_SIZE as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDivisorDescription {
    pub binding: u32,
    pub divisor: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineVertexInputDivisorStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_binding_divisor_count: u32,
    pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescription,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVertexAttributeDivisorProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertex_attrib_divisor: u32,
    pub supports_non_zero_first_instance: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVertexAttributeDivisorFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_attribute_instance_rate_divisor: Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceIndexTypeUint8Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub index_type_uint8: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLineRasterizationFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rectangular_lines: Bool32,
    pub bresenham_lines: Bool32,
    pub smooth_lines: Bool32,
    pub stippled_rectangular_lines: Bool32,
    pub stippled_bresenham_lines: Bool32,
    pub stippled_smooth_lines: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLineRasterizationProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub line_sub_pixel_precision_bits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationLineStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub line_rasterization_mode: LineRasterizationMode,
    pub stippled_line_enable: Bool32,
    pub line_stipple_factor: u32,
    pub line_stipple_pattern: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVulkan14Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub global_priority_query: Bool32,
    pub shader_subgroup_rotate: Bool32,
    pub shader_subgroup_rotate_clustered: Bool32,
    pub shader_float_controls2: Bool32,
    pub shader_expect_assume: Bool32,
    pub rectangular_lines: Bool32,
    pub bresenham_lines: Bool32,
    pub smooth_lines: Bool32,
    pub stippled_rectangular_lines: Bool32,
    pub stippled_bresenham_lines: Bool32,
    pub stippled_smooth_lines: Bool32,
    pub vertex_attribute_instance_rate_divisor: Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
    pub index_type_uint8: Bool32,
    pub dynamic_rendering_local_read: Bool32,
    pub maintenance5: Bool32,
    pub maintenance6: Bool32,
    pub pipeline_protected_access: Bool32,
    pub pipeline_robustness: Bool32,
    pub host_image_copy: Bool32,
    pub push_descriptor: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVulkan14Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub line_sub_pixel_precision_bits: u32,
    pub max_vertex_attrib_divisor: u32,
    pub supports_non_zero_first_instance: Bool32,
    pub max_push_descriptors: u32,
    pub dynamic_rendering_local_read_depth_stencil_attachments: Bool32,
    pub dynamic_rendering_local_read_multisampled_attachments: Bool32,
    pub early_fragment_multisample_coverage_after_sample_counting: Bool32,
    pub early_fragment_sample_mask_test_before_sample_counting: Bool32,
    pub depth_stencil_swizzle_one_support: Bool32,
    pub polygon_mode_point_size: Bool32,
    pub non_strict_single_pixel_wide_lines_use_parallelogram: Bool32,
    pub non_strict_wide_lines_use_parallelogram: Bool32,
    pub block_texel_view_compatible_multiple_layers: Bool32,
    pub max_combined_image_sampler_descriptor_count: u32,
    pub fragment_shading_rate_clamp_combiner_inputs: Bool32,
    pub default_robustness_storage_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_uniform_buffers: PipelineRobustnessBufferBehavior,
    pub default_robustness_vertex_inputs: PipelineRobustnessBufferBehavior,
    pub default_robustness_images: PipelineRobustnessImageBehavior,
    pub copy_src_layout_count: u32,
    pub p_copy_src_layouts: *mut ImageLayout,
    pub copy_dst_layout_count: u32,
    pub p_copy_dst_layouts: *mut ImageLayout,
    pub optimal_tiling_layout_uuid: [u8; UUID_SIZE as usize],
    pub identical_memory_type_requirements: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceHostImageCopyFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub host_image_copy: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceHostImageCopyProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub copy_src_layout_count: u32,
    pub p_copy_src_layouts: *mut ImageLayout,
    pub copy_dst_layout_count: u32,
    pub p_copy_dst_layouts: *mut ImageLayout,
    pub optimal_tiling_layout_uuid: [u8; UUID_SIZE as usize],
    pub identical_memory_type_requirements: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryToImageCopy {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_host_pointer: *const c_void,
    pub memory_row_length: u32,
    pub memory_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageToMemoryCopy {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_host_pointer: *mut c_void,
    pub memory_row_length: u32,
    pub memory_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyMemoryToImageInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HostImageCopyFlags,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const MemoryToImageCopy,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyImageToMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HostImageCopyFlags,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageToMemoryCopy,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyImageToImageInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HostImageCopyFlags,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageCopy2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostImageLayoutTransitionInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub subresource_range: ImageSubresourceRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubresourceHostMemcpySize {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostImageCopyDevicePerformanceQuery {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_device_access: Bool32,
    pub identical_memory_layout: Bool32,
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
pub struct ImageSubresource2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_subresource: ImageSubresource,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubresourceLayout2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subresource_layout: SubresourceLayout,
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
pub struct DeviceImageSubresourceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const ImageCreateInfo,
    pub p_subresource: *const ImageSubresource2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryMapInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryMapFlags,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryUnmapInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryUnmapFlags,
    pub memory: DeviceMemory,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindMemoryStatus {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_result: *mut Result,
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDynamicRenderingLocalReadFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_rendering_local_read: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingAttachmentLocationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_locations: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingInputAttachmentIndexInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_input_indices: *const u32,
    pub p_depth_input_attachment_index: *const u32,
    pub p_stencil_input_attachment_index: *const u32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueGlobalPriority(i32);
impl QueueGlobalPriority {
    pub const LOW: Self = Self(128);
    pub const MEDIUM: Self = Self(256);
    pub const HIGH: Self = Self(512);
    pub const REALTIME: Self = Self(1024);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineRasterizationMode(i32);
impl LineRasterizationMode {
    pub const DEFAULT: Self = Self(0);
    pub const RECTANGULAR: Self = Self(1);
    pub const BRESENHAM: Self = Self(2);
    pub const RECTANGULAR_SMOOTH: Self = Self(3);
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
    pub struct MemoryUnmapFlags: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryUnmapFlagBits(u32);
impl MemoryUnmapFlagBits {}
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct BufferUsageFlags2: Flags64 {
        const TRANSFER_SRC = BufferUsageFlagBits2::TRANSFER_SRC.0;
        const TRANSFER_DST = BufferUsageFlagBits2::TRANSFER_DST.0;
        const UNIFORM_TEXEL_BUFFER = BufferUsageFlagBits2::UNIFORM_TEXEL_BUFFER.0;
        const STORAGE_TEXEL_BUFFER = BufferUsageFlagBits2::STORAGE_TEXEL_BUFFER.0;
        const UNIFORM_BUFFER = BufferUsageFlagBits2::UNIFORM_BUFFER.0;
        const STORAGE_BUFFER = BufferUsageFlagBits2::STORAGE_BUFFER.0;
        const INDEX_BUFFER = BufferUsageFlagBits2::INDEX_BUFFER.0;
        const VERTEX_BUFFER = BufferUsageFlagBits2::VERTEX_BUFFER.0;
        const INDIRECT_BUFFER = BufferUsageFlagBits2::INDIRECT_BUFFER.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BufferUsageFlagBits2(u64);
impl BufferUsageFlagBits2 {
    pub const TRANSFER_SRC: Self = Self(1 << 0);
    pub const TRANSFER_DST: Self = Self(1 << 1);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(1 << 2);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(1 << 3);
    pub const UNIFORM_BUFFER: Self = Self(1 << 4);
    pub const STORAGE_BUFFER: Self = Self(1 << 5);
    pub const INDEX_BUFFER: Self = Self(1 << 6);
    pub const VERTEX_BUFFER: Self = Self(1 << 7);
    pub const INDIRECT_BUFFER: Self = Self(1 << 8);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct HostImageCopyFlags: Flags {
        const MEMCPY = HostImageCopyFlagBits::MEMCPY.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostImageCopyFlagBits(u32);
impl HostImageCopyFlagBits {
    pub const MEMCPY: Self = Self(1 << 0);
}
pub type PFN_vkGetRenderingAreaGranularity = unsafe extern "system" fn(
    device: Device,
    p_rendering_area_info: *const RenderingAreaInfo,
    p_granularity: *mut Extent2D,
);
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
pub type PFN_vkCmdSetLineStipple = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
);
pub type PFN_vkCmdBindIndexBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
);
pub type PFN_vkCopyMemoryToImage = unsafe extern "system" fn(
    device: Device,
    p_copy_memory_to_image_info: *const CopyMemoryToImageInfo,
) -> Result;
pub type PFN_vkCopyImageToMemory = unsafe extern "system" fn(
    device: Device,
    p_copy_image_to_memory_info: *const CopyImageToMemoryInfo,
) -> Result;
pub type PFN_vkCopyImageToImage = unsafe extern "system" fn(
    device: Device,
    p_copy_image_to_image_info: *const CopyImageToImageInfo,
) -> Result;
pub type PFN_vkTransitionImageLayout = unsafe extern "system" fn(
    device: Device,
    transition_count: u32,
    p_transitions: *const HostImageLayoutTransitionInfo,
) -> Result;
pub type PFN_vkGetImageSubresourceLayout2 = unsafe extern "system" fn(
    device: Device,
    image: Image,
    p_subresource: *const ImageSubresource2,
    p_layout: *mut SubresourceLayout2,
);
pub type PFN_vkGetDeviceImageSubresourceLayout = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageSubresourceInfo,
    p_layout: *mut SubresourceLayout2,
);
pub type PFN_vkMapMemory2 = unsafe extern "system" fn(
    device: Device,
    p_memory_map_info: *const MemoryMapInfo,
    pp_data: *mut *mut c_void,
) -> Result;
pub type PFN_vkUnmapMemory2 = unsafe extern "system" fn(
    device: Device,
    p_memory_unmap_info: *const MemoryUnmapInfo,
) -> Result;
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
pub type PFN_vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_location_info: *const RenderingAttachmentLocationInfo,
);
pub type PFN_vkCmdSetRenderingInputAttachmentIndices = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
);
