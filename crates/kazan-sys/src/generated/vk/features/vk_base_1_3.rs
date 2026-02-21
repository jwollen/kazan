#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
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
    pub plane_aspect: ImageAspectFlagBits,
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
        const INDIRECT_COMMAND_READ = AccessFlagBits2::INDIRECT_COMMAND_READ.0;
        const INDEX_READ = AccessFlagBits2::INDEX_READ.0;
        const VERTEX_ATTRIBUTE_READ = AccessFlagBits2::VERTEX_ATTRIBUTE_READ.0;
        const UNIFORM_READ = AccessFlagBits2::UNIFORM_READ.0;
        const INPUT_ATTACHMENT_READ = AccessFlagBits2::INPUT_ATTACHMENT_READ.0;
        const SHADER_READ = AccessFlagBits2::SHADER_READ.0;
        const SHADER_WRITE = AccessFlagBits2::SHADER_WRITE.0;
        const COLOR_ATTACHMENT_READ = AccessFlagBits2::COLOR_ATTACHMENT_READ.0;
        const COLOR_ATTACHMENT_WRITE = AccessFlagBits2::COLOR_ATTACHMENT_WRITE.0;
        const DEPTH_STENCIL_ATTACHMENT_READ = AccessFlagBits2::DEPTH_STENCIL_ATTACHMENT_READ.0;
        const DEPTH_STENCIL_ATTACHMENT_WRITE = AccessFlagBits2::DEPTH_STENCIL_ATTACHMENT_WRITE.0;
        const TRANSFER_READ = AccessFlagBits2::TRANSFER_READ.0;
        const TRANSFER_WRITE = AccessFlagBits2::TRANSFER_WRITE.0;
        const HOST_READ = AccessFlagBits2::HOST_READ.0;
        const HOST_WRITE = AccessFlagBits2::HOST_WRITE.0;
        const MEMORY_READ = AccessFlagBits2::MEMORY_READ.0;
        const MEMORY_WRITE = AccessFlagBits2::MEMORY_WRITE.0;
        const SHADER_SAMPLED_READ = AccessFlagBits2::SHADER_SAMPLED_READ.0;
        const SHADER_STORAGE_READ = AccessFlagBits2::SHADER_STORAGE_READ.0;
        const SHADER_STORAGE_WRITE = AccessFlagBits2::SHADER_STORAGE_WRITE.0;
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AccessFlagBits2(u64);
impl AccessFlagBits2 {
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
    pub const SHADER_SAMPLED_READ: Self = Self(1 << 32);
    pub const SHADER_STORAGE_READ: Self = Self(1 << 33);
    pub const SHADER_STORAGE_WRITE: Self = Self(1 << 34);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineStageFlags2: Flags64 {
        const TOP_OF_PIPE = PipelineStageFlagBits2::TOP_OF_PIPE.0;
        const DRAW_INDIRECT = PipelineStageFlagBits2::DRAW_INDIRECT.0;
        const VERTEX_INPUT = PipelineStageFlagBits2::VERTEX_INPUT.0;
        const VERTEX_SHADER = PipelineStageFlagBits2::VERTEX_SHADER.0;
        const TESSELLATION_CONTROL_SHADER = PipelineStageFlagBits2::TESSELLATION_CONTROL_SHADER.0;
        const TESSELLATION_EVALUATION_SHADER = PipelineStageFlagBits2::TESSELLATION_EVALUATION_SHADER.0;
        const GEOMETRY_SHADER = PipelineStageFlagBits2::GEOMETRY_SHADER.0;
        const FRAGMENT_SHADER = PipelineStageFlagBits2::FRAGMENT_SHADER.0;
        const EARLY_FRAGMENT_TESTS = PipelineStageFlagBits2::EARLY_FRAGMENT_TESTS.0;
        const LATE_FRAGMENT_TESTS = PipelineStageFlagBits2::LATE_FRAGMENT_TESTS.0;
        const COLOR_ATTACHMENT_OUTPUT = PipelineStageFlagBits2::COLOR_ATTACHMENT_OUTPUT.0;
        const COMPUTE_SHADER = PipelineStageFlagBits2::COMPUTE_SHADER.0;
        const ALL_TRANSFER = PipelineStageFlagBits2::ALL_TRANSFER.0;
        const BOTTOM_OF_PIPE = PipelineStageFlagBits2::BOTTOM_OF_PIPE.0;
        const HOST = PipelineStageFlagBits2::HOST.0;
        const ALL_GRAPHICS = PipelineStageFlagBits2::ALL_GRAPHICS.0;
        const ALL_COMMANDS = PipelineStageFlagBits2::ALL_COMMANDS.0;
        const COPY = PipelineStageFlagBits2::COPY.0;
        const RESOLVE = PipelineStageFlagBits2::RESOLVE.0;
        const BLIT = PipelineStageFlagBits2::BLIT.0;
        const CLEAR = PipelineStageFlagBits2::CLEAR.0;
        const INDEX_INPUT = PipelineStageFlagBits2::INDEX_INPUT.0;
        const VERTEX_ATTRIBUTE_INPUT = PipelineStageFlagBits2::VERTEX_ATTRIBUTE_INPUT.0;
        const PRE_RASTERIZATION_SHADERS = PipelineStageFlagBits2::PRE_RASTERIZATION_SHADERS.0;
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineStageFlagBits2(u64);
impl PipelineStageFlagBits2 {
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
    pub const ALL_TRANSFER: Self = Self(1 << 12);
    pub const BOTTOM_OF_PIPE: Self = Self(1 << 13);
    pub const HOST: Self = Self(1 << 14);
    pub const ALL_GRAPHICS: Self = Self(1 << 15);
    pub const ALL_COMMANDS: Self = Self(1 << 16);
    pub const COPY: Self = Self(1 << 32);
    pub const RESOLVE: Self = Self(1 << 33);
    pub const BLIT: Self = Self(1 << 34);
    pub const CLEAR: Self = Self(1 << 35);
    pub const INDEX_INPUT: Self = Self(1 << 36);
    pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(1 << 37);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(1 << 38);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FormatFeatureFlags2: Flags64 {
        const SAMPLED_IMAGE = FormatFeatureFlagBits2::SAMPLED_IMAGE.0;
        const STORAGE_IMAGE = FormatFeatureFlagBits2::STORAGE_IMAGE.0;
        const STORAGE_IMAGE_ATOMIC = FormatFeatureFlagBits2::STORAGE_IMAGE_ATOMIC.0;
        const UNIFORM_TEXEL_BUFFER = FormatFeatureFlagBits2::UNIFORM_TEXEL_BUFFER.0;
        const STORAGE_TEXEL_BUFFER = FormatFeatureFlagBits2::STORAGE_TEXEL_BUFFER.0;
        const STORAGE_TEXEL_BUFFER_ATOMIC = FormatFeatureFlagBits2::STORAGE_TEXEL_BUFFER_ATOMIC.0;
        const VERTEX_BUFFER = FormatFeatureFlagBits2::VERTEX_BUFFER.0;
        const COLOR_ATTACHMENT = FormatFeatureFlagBits2::COLOR_ATTACHMENT.0;
        const COLOR_ATTACHMENT_BLEND = FormatFeatureFlagBits2::COLOR_ATTACHMENT_BLEND.0;
        const DEPTH_STENCIL_ATTACHMENT = FormatFeatureFlagBits2::DEPTH_STENCIL_ATTACHMENT.0;
        const BLIT_SRC = FormatFeatureFlagBits2::BLIT_SRC.0;
        const BLIT_DST = FormatFeatureFlagBits2::BLIT_DST.0;
        const SAMPLED_IMAGE_FILTER_LINEAR = FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_LINEAR.0;
        const TRANSFER_SRC = FormatFeatureFlagBits2::TRANSFER_SRC.0;
        const TRANSFER_DST = FormatFeatureFlagBits2::TRANSFER_DST.0;
        const SAMPLED_IMAGE_FILTER_MINMAX = FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_MINMAX.0;
        const MIDPOINT_CHROMA_SAMPLES = FormatFeatureFlagBits2::MIDPOINT_CHROMA_SAMPLES.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER = FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER = FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT = FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT.0;
        const SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE = FormatFeatureFlagBits2::SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE.0;
        const DISJOINT = FormatFeatureFlagBits2::DISJOINT.0;
        const COSITED_CHROMA_SAMPLES = FormatFeatureFlagBits2::COSITED_CHROMA_SAMPLES.0;
        const STORAGE_READ_WITHOUT_FORMAT = FormatFeatureFlagBits2::STORAGE_READ_WITHOUT_FORMAT.0;
        const STORAGE_WRITE_WITHOUT_FORMAT = FormatFeatureFlagBits2::STORAGE_WRITE_WITHOUT_FORMAT.0;
        const SAMPLED_IMAGE_DEPTH_COMPARISON = FormatFeatureFlagBits2::SAMPLED_IMAGE_DEPTH_COMPARISON.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatFeatureFlagBits2(u64);
impl FormatFeatureFlagBits2 {
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
    pub const STORAGE_READ_WITHOUT_FORMAT: Self = Self(1 << 31);
    pub const STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(1 << 32);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(1 << 33);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ToolPurposeFlags: Flags {
        const VALIDATION = ToolPurposeFlagBits::VALIDATION.0;
        const PROFILING = ToolPurposeFlagBits::PROFILING.0;
        const TRACING = ToolPurposeFlagBits::TRACING.0;
        const ADDITIONAL_FEATURES = ToolPurposeFlagBits::ADDITIONAL_FEATURES.0;
        const MODIFYING_FEATURES = ToolPurposeFlagBits::MODIFYING_FEATURES.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ToolPurposeFlagBits(u32);
impl ToolPurposeFlagBits {
    pub const VALIDATION: Self = Self(1 << 0);
    pub const PROFILING: Self = Self(1 << 1);
    pub const TRACING: Self = Self(1 << 2);
    pub const ADDITIONAL_FEATURES: Self = Self(1 << 3);
    pub const MODIFYING_FEATURES: Self = Self(1 << 4);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SubmitFlags: Flags {
        const PROTECTED = SubmitFlagBits::PROTECTED.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubmitFlagBits(u32);
impl SubmitFlagBits {
    pub const PROTECTED: Self = Self(1 << 0);
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
