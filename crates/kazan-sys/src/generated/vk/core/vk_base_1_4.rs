#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
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
pub struct PhysicalDeviceIndexTypeUint8Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub index_type_uint8: Bool32,
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
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueueGlobalPriority(i32);
impl QueueGlobalPriority {
    pub const LOW: Self = Self(128);
    pub const MEDIUM: Self = Self(256);
    pub const HIGH: Self = Self(512);
    pub const REALTIME: Self = Self(1024);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MemoryUnmapFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct BufferUsageFlags2: Flags64 {
        const TRANSFER_SRC = 1 << 0;
        const TRANSFER_DST = 1 << 1;
        const UNIFORM_TEXEL_BUFFER = 1 << 2;
        const STORAGE_TEXEL_BUFFER = 1 << 3;
        const UNIFORM_BUFFER = 1 << 4;
        const STORAGE_BUFFER = 1 << 5;
        const INDEX_BUFFER = 1 << 6;
        const VERTEX_BUFFER = 1 << 7;
        const INDIRECT_BUFFER = 1 << 8;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct HostImageCopyFlags: Flags {
    }
}
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
