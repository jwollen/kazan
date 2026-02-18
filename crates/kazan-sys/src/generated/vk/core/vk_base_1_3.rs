#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub type Flags64 = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PrivateDataSlot(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DevicePrivateDataCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub private_data_slot_request_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PrivateDataSlotCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PrivateDataSlotCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePrivateDataFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub private_data: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceBufferMemoryRequirements {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const BufferCreateInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceImageMemoryRequirements {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const ImageCreateInfo,
    pub plane_aspect: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance4Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub maintenance4: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance4Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_buffer_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub texture_compression_astc_hdr: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVulkan13Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_image_access: Bool32,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
    pub pipeline_creation_cache_control: Bool32,
    pub private_data: Bool32,
    pub shader_demote_to_helper_invocation: Bool32,
    pub shader_terminate_invocation: Bool32,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
    pub synchronization2: Bool32,
    pub texture_compression_astc_hdr: Bool32,
    pub shader_zero_initialize_workgroup_memory: Bool32,
    pub dynamic_rendering: Bool32,
    pub shader_integer_dot_product: Bool32,
    pub maintenance4: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVulkan13Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: ShaderStageFlags,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
    pub max_inline_uniform_total_size: u32,
    pub integer_dot_product8_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product8_bit_signed_accelerated: Bool32,
    pub integer_dot_product8_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product4x8_bit_packed_unsigned_accelerated: Bool32,
    pub integer_dot_product4x8_bit_packed_signed_accelerated: Bool32,
    pub integer_dot_product4x8_bit_packed_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product16_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product16_bit_signed_accelerated: Bool32,
    pub integer_dot_product16_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product32_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product32_bit_signed_accelerated: Bool32,
    pub integer_dot_product32_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product64_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product64_bit_signed_accelerated: Bool32,
    pub integer_dot_product64_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating8_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated:
        Bool32,
    pub integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating16_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating32_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating64_bit_signed_accelerated: Bool32,
    pub integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated: Bool32,
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
    pub max_buffer_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceToolProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub name: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub version: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
    pub purposes: ToolPurposeFlags,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub layer: [c_char; MAX_EXTENSION_NAME_SIZE as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCopy2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCopy2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferImageCopy2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyBufferInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_buffer: Buffer,
    pub dst_buffer: Buffer,
    pub region_count: u32,
    pub p_regions: *const BufferCopy2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageCopy2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyBufferToImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_buffer: Buffer,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const BufferImageCopy2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyImageToBufferInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_buffer: Buffer,
    pub region_count: u32,
    pub p_regions: *const BufferImageCopy2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryBarrier2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DependencyInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dependency_flags: DependencyFlags,
    pub memory_barrier_count: u32,
    pub p_memory_barriers: *const MemoryBarrier2,
    pub buffer_memory_barrier_count: u32,
    pub p_buffer_memory_barriers: *const BufferMemoryBarrier2,
    pub image_memory_barrier_count: u32,
    pub p_image_memory_barriers: *const ImageMemoryBarrier2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SemaphoreSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
    pub stage_mask: PipelineStageFlags2,
    pub device_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub command_buffer: CommandBuffer,
    pub device_mask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubmitInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SubmitFlags,
    pub wait_semaphore_info_count: u32,
    pub p_wait_semaphore_infos: *const SemaphoreSubmitInfo,
    pub command_buffer_info_count: u32,
    pub p_command_buffer_infos: *const CommandBufferSubmitInfo,
    pub signal_semaphore_info_count: u32,
    pub p_signal_semaphore_infos: *const SemaphoreSubmitInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSynchronization2Features {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub synchronization2: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties3 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub linear_tiling_features: FormatFeatureFlags2,
    pub optimal_tiling_features: FormatFeatureFlags2,
    pub buffer_features: FormatFeatureFlags2,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PrivateDataSlotCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct AccessFlags2: Flags64 {
        const INDIRECT_COMMAND_READ = 1 << 0;
        const INDEX_READ = 1 << 1;
        const VERTEX_ATTRIBUTE_READ = 1 << 2;
        const UNIFORM_READ = 1 << 3;
        const INPUT_ATTACHMENT_READ = 1 << 4;
        const SHADER_READ = 1 << 5;
        const SHADER_WRITE = 1 << 6;
        const COLOR_ATTACHMENT_READ = 1 << 7;
        const COLOR_ATTACHMENT_WRITE = 1 << 8;
        const DEPTH_STENCIL_ATTACHMENT_READ = 1 << 9;
        const DEPTH_STENCIL_ATTACHMENT_WRITE = 1 << 10;
        const TRANSFER_READ = 1 << 11;
        const TRANSFER_WRITE = 1 << 12;
        const HOST_READ = 1 << 13;
        const HOST_WRITE = 1 << 14;
        const MEMORY_READ = 1 << 15;
        const MEMORY_WRITE = 1 << 16;
        const SHADER_SAMPLED_READ = 1 << 32;
        const SHADER_STORAGE_READ = 1 << 33;
        const SHADER_STORAGE_WRITE = 1 << 34;
        const NONE = 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineStageFlags2: Flags64 {
        const TOP_OF_PIPE = 1 << 0;
        const DRAW_INDIRECT = 1 << 1;
        const VERTEX_INPUT = 1 << 2;
        const VERTEX_SHADER = 1 << 3;
        const TESSELLATION_CONTROL_SHADER = 1 << 4;
        const TESSELLATION_EVALUATION_SHADER = 1 << 5;
        const GEOMETRY_SHADER = 1 << 6;
        const FRAGMENT_SHADER = 1 << 7;
        const EARLY_FRAGMENT_TESTS = 1 << 8;
        const LATE_FRAGMENT_TESTS = 1 << 9;
        const COLOR_ATTACHMENT_OUTPUT = 1 << 10;
        const COMPUTE_SHADER = 1 << 11;
        const ALL_TRANSFER = 1 << 12;
        const BOTTOM_OF_PIPE = 1 << 13;
        const HOST = 1 << 14;
        const ALL_GRAPHICS = 1 << 15;
        const ALL_COMMANDS = 1 << 16;
        const COPY = 1 << 32;
        const RESOLVE = 1 << 33;
        const BLIT = 1 << 34;
        const CLEAR = 1 << 35;
        const INDEX_INPUT = 1 << 36;
        const VERTEX_ATTRIBUTE_INPUT = 1 << 37;
        const PRE_RASTERIZATION_SHADERS = 1 << 38;
        const NONE = 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FormatFeatureFlags2: Flags64 {
        const SAMPLED_IMAGE = 1 << 0;
        const STORAGE_IMAGE = 1 << 1;
        const STORAGE_IMAGE_ATOMIC = 1 << 2;
        const UNIFORM_TEXEL_BUFFER = 1 << 3;
        const STORAGE_TEXEL_BUFFER = 1 << 4;
        const STORAGE_TEXEL_BUFFER_ATOMIC = 1 << 5;
        const VERTEX_BUFFER = 1 << 6;
        const COLOR_ATTACHMENT = 1 << 7;
        const COLOR_ATTACHMENT_BLEND = 1 << 8;
        const DEPTH_STENCIL_ATTACHMENT = 1 << 9;
        const BLIT_SRC = 1 << 10;
        const BLIT_DST = 1 << 11;
        const SAMPLED_IMAGE_FILTER_LINEAR = 1 << 12;
        const TRANSFER_SRC = 1 << 14;
        const TRANSFER_DST = 1 << 15;
        const SAMPLED_IMAGE_FILTER_MINMAX = 1 << 16;
        const MIDPOINT_CHROMA_SAMPLES = 1 << 17;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = 1 << 18;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = 1 << 19;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = 1 << 20;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = 1 << 21;
        const DISJOINT = 1 << 22;
        const COSITED_CHROMA_SAMPLES = 1 << 23;
        const STORAGE_READ_WITHOUT_FORMAT = 1 << 31;
        const STORAGE_WRITE_WITHOUT_FORMAT = 1 << 32;
        const SAMPLED_IMAGE_DEPTH_COMPARISON = 1 << 33;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ToolPurposeFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SubmitFlags: Flags {
    }
}
pub type PFN_vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceBufferMemoryRequirements,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageMemoryRequirements,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceImageMemoryRequirements,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
pub type PFN_vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_tool_count: *mut u32,
    p_tool_properties: *mut PhysicalDeviceToolProperties,
) -> Result;
pub type PFN_vkCreatePrivateDataSlot = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PrivateDataSlotCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_private_data_slot: *mut PrivateDataSlot,
) -> Result;
pub type PFN_vkDestroyPrivateDataSlot = unsafe extern "system" fn(
    device: Device,
    private_data_slot: PrivateDataSlot,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkSetPrivateData = unsafe extern "system" fn(
    device: Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> Result;
pub type PFN_vkGetPrivateData = unsafe extern "system" fn(
    device: Device,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    p_data: *mut u64,
);
pub type PFN_vkCmdCopyBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_buffer_info: *const CopyBufferInfo2,
);
pub type PFN_vkCmdCopyImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_image_info: *const CopyImageInfo2,
);
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
);
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
);
pub type PFN_vkCmdPipelineBarrier2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_dependency_info: *const DependencyInfo,
);
pub type PFN_vkQueueSubmit2 = unsafe extern "system" fn(
    queue: Queue,
    submit_count: u32,
    p_submits: *const SubmitInfo2,
    fence: Fence,
) -> Result;
pub type PFN_vkCmdWriteTimestamp2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
);
