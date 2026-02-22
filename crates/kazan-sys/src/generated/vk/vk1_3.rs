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
pub struct PhysicalDeviceInlineUniformBlockFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceInlineUniformBlockProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSetInlineUniformBlock {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_size: u32,
    pub p_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolInlineUniformBlockCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_inline_uniform_block_bindings: u32,
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
pub struct PipelineCreationFeedback {
    pub flags: PipelineCreationFeedbackFlags,
    pub duration: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCreationFeedbackCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_pipeline_creation_feedback: *mut PipelineCreationFeedback,
    pub pipeline_stage_creation_feedback_count: u32,
    pub p_pipeline_stage_creation_feedbacks: *mut PipelineCreationFeedback,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_demote_to_helper_invocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTexelBufferAlignmentProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: DeviceSize,
    pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSubgroupSizeControlFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSubgroupSizeControlProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: ShaderStageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageRequiredSubgroupSizeCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub required_subgroup_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineCreationCacheControlFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_creation_cache_control: Bool32,
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
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_zero_initialize_workgroup_memory: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageRobustnessFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub robust_image_access: Bool32,
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
pub struct ImageBlit2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3D; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3D; 2],
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
pub struct ImageResolve2 {
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
pub struct BlitImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageBlit2,
    pub filter: Filter,
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
pub struct ResolveImageInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_image: Image,
    pub src_image_layout: ImageLayout,
    pub dst_image: Image,
    pub dst_image_layout: ImageLayout,
    pub region_count: u32,
    pub p_regions: *const ImageResolve2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderTerminateInvocationFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_terminate_invocation: Bool32,
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
pub struct PhysicalDeviceShaderIntegerDotProductFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_integer_dot_product: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderIntegerDotProductProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRenderingCreateInfo {
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
pub struct RenderingInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderingFlags,
    pub render_area: Rect2D,
    pub layer_count: u32,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const RenderingAttachmentInfo,
    pub p_depth_attachment: *const RenderingAttachmentInfo,
    pub p_stencil_attachment: *const RenderingAttachmentInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingAttachmentInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub resolve_mode: ResolveModeFlagBits,
    pub resolve_image_view: ImageView,
    pub resolve_image_layout: ImageLayout,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub clear_value: ClearValue,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDynamicRenderingFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_rendering: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceRenderingInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderingFlags,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
    pub rasterization_samples: SampleCountFlagBits,
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
    pub struct PipelineCreationFeedbackFlags: Flags {
        const VALID = PipelineCreationFeedbackFlagBits::VALID.0;
        const APPLICATION_PIPELINE_CACHE_HIT = PipelineCreationFeedbackFlagBits::APPLICATION_PIPELINE_CACHE_HIT.0;
        const BASE_PIPELINE_ACCELERATION = PipelineCreationFeedbackFlagBits::BASE_PIPELINE_ACCELERATION.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCreationFeedbackFlagBits(u32);
impl PipelineCreationFeedbackFlagBits {
    pub const VALID: Self = Self(1 << 0);
    pub const APPLICATION_PIPELINE_CACHE_HIT: Self = Self(1 << 1);
    pub const BASE_PIPELINE_ACCELERATION: Self = Self(1 << 2);
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
        const COMMAND_PREPROCESS_READ_EXT = AccessFlagBits2::COMMAND_PREPROCESS_READ_EXT.0;
        const COMMAND_PREPROCESS_WRITE_EXT = AccessFlagBits2::COMMAND_PREPROCESS_WRITE_EXT.0;
        const COLOR_ATTACHMENT_READ_NONCOHERENT_EXT = AccessFlagBits2::COLOR_ATTACHMENT_READ_NONCOHERENT_EXT.0;
        const CONDITIONAL_RENDERING_READ_EXT = AccessFlagBits2::CONDITIONAL_RENDERING_READ_EXT.0;
        const ACCELERATION_STRUCTURE_READ_KHR = AccessFlagBits2::ACCELERATION_STRUCTURE_READ_KHR.0;
        const ACCELERATION_STRUCTURE_WRITE_KHR = AccessFlagBits2::ACCELERATION_STRUCTURE_WRITE_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR = AccessFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_READ_KHR.0;
        const FRAGMENT_DENSITY_MAP_READ_EXT = AccessFlagBits2::FRAGMENT_DENSITY_MAP_READ_EXT.0;
        const TRANSFORM_FEEDBACK_WRITE_EXT = AccessFlagBits2::TRANSFORM_FEEDBACK_WRITE_EXT.0;
        const TRANSFORM_FEEDBACK_COUNTER_READ_EXT = AccessFlagBits2::TRANSFORM_FEEDBACK_COUNTER_READ_EXT.0;
        const TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT = AccessFlagBits2::TRANSFORM_FEEDBACK_COUNTER_WRITE_EXT.0;
        const SHADER_SAMPLED_READ = AccessFlagBits2::SHADER_SAMPLED_READ.0;
        const SHADER_STORAGE_READ = AccessFlagBits2::SHADER_STORAGE_READ.0;
        const SHADER_STORAGE_WRITE = AccessFlagBits2::SHADER_STORAGE_WRITE.0;
        const VIDEO_DECODE_READ_KHR = AccessFlagBits2::VIDEO_DECODE_READ_KHR.0;
        const VIDEO_DECODE_WRITE_KHR = AccessFlagBits2::VIDEO_DECODE_WRITE_KHR.0;
        const VIDEO_ENCODE_READ_KHR = AccessFlagBits2::VIDEO_ENCODE_READ_KHR.0;
        const VIDEO_ENCODE_WRITE_KHR = AccessFlagBits2::VIDEO_ENCODE_WRITE_KHR.0;
        const INVOCATION_MASK_READ_HUAWEI = AccessFlagBits2::INVOCATION_MASK_READ_HUAWEI.0;
        const SHADER_BINDING_TABLE_READ_KHR = AccessFlagBits2::SHADER_BINDING_TABLE_READ_KHR.0;
        const DESCRIPTOR_BUFFER_READ_EXT = AccessFlagBits2::DESCRIPTOR_BUFFER_READ_EXT.0;
        const OPTICAL_FLOW_READ_NV = AccessFlagBits2::OPTICAL_FLOW_READ_NV.0;
        const OPTICAL_FLOW_WRITE_NV = AccessFlagBits2::OPTICAL_FLOW_WRITE_NV.0;
        const MICROMAP_READ_EXT = AccessFlagBits2::MICROMAP_READ_EXT.0;
        const MICROMAP_WRITE_EXT = AccessFlagBits2::MICROMAP_WRITE_EXT.0;
        const DATA_GRAPH_READ_ARM = AccessFlagBits2::DATA_GRAPH_READ_ARM.0;
        const DATA_GRAPH_WRITE_ARM = AccessFlagBits2::DATA_GRAPH_WRITE_ARM.0;
        const SHADER_TILE_ATTACHMENT_READ_QCOM = AccessFlagBits2::SHADER_TILE_ATTACHMENT_READ_QCOM.0;
        const SHADER_TILE_ATTACHMENT_WRITE_QCOM = AccessFlagBits2::SHADER_TILE_ATTACHMENT_WRITE_QCOM.0;
        const MEMORY_DECOMPRESSION_READ_EXT = AccessFlagBits2::MEMORY_DECOMPRESSION_READ_EXT.0;
        const MEMORY_DECOMPRESSION_WRITE_EXT = AccessFlagBits2::MEMORY_DECOMPRESSION_WRITE_EXT.0;
        const SAMPLER_HEAP_READ_EXT = AccessFlagBits2::SAMPLER_HEAP_READ_EXT.0;
        const RESOURCE_HEAP_READ_EXT = AccessFlagBits2::RESOURCE_HEAP_READ_EXT.0;
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
    pub const SHADER_SAMPLED_READ: Self = Self(1 << 32);
    pub const SHADER_STORAGE_READ: Self = Self(1 << 33);
    pub const SHADER_STORAGE_WRITE: Self = Self(1 << 34);
    pub const VIDEO_DECODE_READ_KHR: Self = Self(1 << 35);
    pub const VIDEO_DECODE_WRITE_KHR: Self = Self(1 << 36);
    pub const VIDEO_ENCODE_READ_KHR: Self = Self(1 << 37);
    pub const VIDEO_ENCODE_WRITE_KHR: Self = Self(1 << 38);
    pub const INVOCATION_MASK_READ_HUAWEI: Self = Self(1 << 39);
    pub const SHADER_BINDING_TABLE_READ_KHR: Self = Self(1 << 40);
    pub const DESCRIPTOR_BUFFER_READ_EXT: Self = Self(1 << 41);
    pub const OPTICAL_FLOW_READ_NV: Self = Self(1 << 42);
    pub const OPTICAL_FLOW_WRITE_NV: Self = Self(1 << 43);
    pub const MICROMAP_READ_EXT: Self = Self(1 << 44);
    pub const MICROMAP_WRITE_EXT: Self = Self(1 << 45);
    pub const DATA_GRAPH_READ_ARM: Self = Self(1 << 47);
    pub const DATA_GRAPH_WRITE_ARM: Self = Self(1 << 48);
    pub const SHADER_TILE_ATTACHMENT_READ_QCOM: Self = Self(1 << 51);
    pub const SHADER_TILE_ATTACHMENT_WRITE_QCOM: Self = Self(1 << 52);
    pub const MEMORY_DECOMPRESSION_READ_EXT: Self = Self(1 << 55);
    pub const MEMORY_DECOMPRESSION_WRITE_EXT: Self = Self(1 << 56);
    pub const SAMPLER_HEAP_READ_EXT: Self = Self(1 << 57);
    pub const RESOURCE_HEAP_READ_EXT: Self = Self(1 << 58);
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
        const COMMAND_PREPROCESS_EXT = PipelineStageFlagBits2::COMMAND_PREPROCESS_EXT.0;
        const CONDITIONAL_RENDERING_EXT = PipelineStageFlagBits2::CONDITIONAL_RENDERING_EXT.0;
        const TASK_SHADER_EXT = PipelineStageFlagBits2::TASK_SHADER_EXT.0;
        const MESH_SHADER_EXT = PipelineStageFlagBits2::MESH_SHADER_EXT.0;
        const RAY_TRACING_SHADER_KHR = PipelineStageFlagBits2::RAY_TRACING_SHADER_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = PipelineStageFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
        const FRAGMENT_DENSITY_PROCESS_EXT = PipelineStageFlagBits2::FRAGMENT_DENSITY_PROCESS_EXT.0;
        const TRANSFORM_FEEDBACK_EXT = PipelineStageFlagBits2::TRANSFORM_FEEDBACK_EXT.0;
        const ACCELERATION_STRUCTURE_BUILD_KHR = PipelineStageFlagBits2::ACCELERATION_STRUCTURE_BUILD_KHR.0;
        const VIDEO_DECODE_KHR = PipelineStageFlagBits2::VIDEO_DECODE_KHR.0;
        const VIDEO_ENCODE_KHR = PipelineStageFlagBits2::VIDEO_ENCODE_KHR.0;
        const ACCELERATION_STRUCTURE_COPY_KHR = PipelineStageFlagBits2::ACCELERATION_STRUCTURE_COPY_KHR.0;
        const OPTICAL_FLOW_NV = PipelineStageFlagBits2::OPTICAL_FLOW_NV.0;
        const MICROMAP_BUILD_EXT = PipelineStageFlagBits2::MICROMAP_BUILD_EXT.0;
        const COPY = PipelineStageFlagBits2::COPY.0;
        const RESOLVE = PipelineStageFlagBits2::RESOLVE.0;
        const BLIT = PipelineStageFlagBits2::BLIT.0;
        const CLEAR = PipelineStageFlagBits2::CLEAR.0;
        const INDEX_INPUT = PipelineStageFlagBits2::INDEX_INPUT.0;
        const VERTEX_ATTRIBUTE_INPUT = PipelineStageFlagBits2::VERTEX_ATTRIBUTE_INPUT.0;
        const PRE_RASTERIZATION_SHADERS = PipelineStageFlagBits2::PRE_RASTERIZATION_SHADERS.0;
        const SUBPASS_SHADER_HUAWEI = PipelineStageFlagBits2::SUBPASS_SHADER_HUAWEI.0;
        const INVOCATION_MASK_HUAWEI = PipelineStageFlagBits2::INVOCATION_MASK_HUAWEI.0;
        const CLUSTER_CULLING_SHADER_HUAWEI = PipelineStageFlagBits2::CLUSTER_CULLING_SHADER_HUAWEI.0;
        const DATA_GRAPH_ARM = PipelineStageFlagBits2::DATA_GRAPH_ARM.0;
        const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV = PipelineStageFlagBits2::CONVERT_COOPERATIVE_VECTOR_MATRIX_NV.0;
        const MEMORY_DECOMPRESSION_EXT = PipelineStageFlagBits2::MEMORY_DECOMPRESSION_EXT.0;
        const COPY_INDIRECT_KHR = PipelineStageFlagBits2::COPY_INDIRECT_KHR.0;
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
    pub const COMMAND_PREPROCESS_EXT: Self = Self(1 << 17);
    pub const CONDITIONAL_RENDERING_EXT: Self = Self(1 << 18);
    pub const TASK_SHADER_EXT: Self = Self(1 << 19);
    pub const MESH_SHADER_EXT: Self = Self(1 << 20);
    pub const RAY_TRACING_SHADER_KHR: Self = Self(1 << 21);
    pub const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR: Self = Self(1 << 22);
    pub const FRAGMENT_DENSITY_PROCESS_EXT: Self = Self(1 << 23);
    pub const TRANSFORM_FEEDBACK_EXT: Self = Self(1 << 24);
    pub const ACCELERATION_STRUCTURE_BUILD_KHR: Self = Self(1 << 25);
    pub const VIDEO_DECODE_KHR: Self = Self(1 << 26);
    pub const VIDEO_ENCODE_KHR: Self = Self(1 << 27);
    pub const ACCELERATION_STRUCTURE_COPY_KHR: Self = Self(1 << 28);
    pub const OPTICAL_FLOW_NV: Self = Self(1 << 29);
    pub const MICROMAP_BUILD_EXT: Self = Self(1 << 30);
    pub const COPY: Self = Self(1 << 32);
    pub const RESOLVE: Self = Self(1 << 33);
    pub const BLIT: Self = Self(1 << 34);
    pub const CLEAR: Self = Self(1 << 35);
    pub const INDEX_INPUT: Self = Self(1 << 36);
    pub const VERTEX_ATTRIBUTE_INPUT: Self = Self(1 << 37);
    pub const PRE_RASTERIZATION_SHADERS: Self = Self(1 << 38);
    pub const SUBPASS_SHADER_HUAWEI: Self = Self(1 << 39);
    pub const INVOCATION_MASK_HUAWEI: Self = Self(1 << 40);
    pub const CLUSTER_CULLING_SHADER_HUAWEI: Self = Self(1 << 41);
    pub const DATA_GRAPH_ARM: Self = Self(1 << 42);
    pub const CONVERT_COOPERATIVE_VECTOR_MATRIX_NV: Self = Self(1 << 44);
    pub const MEMORY_DECOMPRESSION_EXT: Self = Self(1 << 45);
    pub const COPY_INDIRECT_KHR: Self = Self(1 << 46);
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
        const SAMPLED_IMAGE_FILTER_CUBIC = FormatFeatureFlagBits2::SAMPLED_IMAGE_FILTER_CUBIC.0;
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
        const FRAGMENT_DENSITY_MAP_EXT = FormatFeatureFlagBits2::FRAGMENT_DENSITY_MAP_EXT.0;
        const VIDEO_DECODE_OUTPUT_KHR = FormatFeatureFlagBits2::VIDEO_DECODE_OUTPUT_KHR.0;
        const VIDEO_DECODE_DPB_KHR = FormatFeatureFlagBits2::VIDEO_DECODE_DPB_KHR.0;
        const VIDEO_ENCODE_INPUT_KHR = FormatFeatureFlagBits2::VIDEO_ENCODE_INPUT_KHR.0;
        const VIDEO_ENCODE_DPB_KHR = FormatFeatureFlagBits2::VIDEO_ENCODE_DPB_KHR.0;
        const ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR = FormatFeatureFlagBits2::ACCELERATION_STRUCTURE_VERTEX_BUFFER_KHR.0;
        const FRAGMENT_SHADING_RATE_ATTACHMENT_KHR = FormatFeatureFlagBits2::FRAGMENT_SHADING_RATE_ATTACHMENT_KHR.0;
        const STORAGE_READ_WITHOUT_FORMAT = FormatFeatureFlagBits2::STORAGE_READ_WITHOUT_FORMAT.0;
        const STORAGE_WRITE_WITHOUT_FORMAT = FormatFeatureFlagBits2::STORAGE_WRITE_WITHOUT_FORMAT.0;
        const SAMPLED_IMAGE_DEPTH_COMPARISON = FormatFeatureFlagBits2::SAMPLED_IMAGE_DEPTH_COMPARISON.0;
        const WEIGHT_IMAGE_QCOM = FormatFeatureFlagBits2::WEIGHT_IMAGE_QCOM.0;
        const WEIGHT_SAMPLED_IMAGE_QCOM = FormatFeatureFlagBits2::WEIGHT_SAMPLED_IMAGE_QCOM.0;
        const BLOCK_MATCHING_QCOM = FormatFeatureFlagBits2::BLOCK_MATCHING_QCOM.0;
        const BOX_FILTER_SAMPLED_QCOM = FormatFeatureFlagBits2::BOX_FILTER_SAMPLED_QCOM.0;
        const LINEAR_COLOR_ATTACHMENT_NV = FormatFeatureFlagBits2::LINEAR_COLOR_ATTACHMENT_NV.0;
        const TENSOR_SHADER_ARM = FormatFeatureFlagBits2::TENSOR_SHADER_ARM.0;
        const OPTICAL_FLOW_IMAGE_NV = FormatFeatureFlagBits2::OPTICAL_FLOW_IMAGE_NV.0;
        const OPTICAL_FLOW_VECTOR_NV = FormatFeatureFlagBits2::OPTICAL_FLOW_VECTOR_NV.0;
        const OPTICAL_FLOW_COST_NV = FormatFeatureFlagBits2::OPTICAL_FLOW_COST_NV.0;
        const TENSOR_IMAGE_ALIASING_ARM = FormatFeatureFlagBits2::TENSOR_IMAGE_ALIASING_ARM.0;
        const HOST_IMAGE_TRANSFER = FormatFeatureFlagBits2::HOST_IMAGE_TRANSFER.0;
        const TENSOR_DATA_GRAPH_ARM = FormatFeatureFlagBits2::TENSOR_DATA_GRAPH_ARM.0;
        const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR = FormatFeatureFlagBits2::VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0;
        const VIDEO_ENCODE_EMPHASIS_MAP_KHR = FormatFeatureFlagBits2::VIDEO_ENCODE_EMPHASIS_MAP_KHR.0;
        const ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV = FormatFeatureFlagBits2::ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV.0;
        const DEPTH_COPY_ON_COMPUTE_QUEUE_KHR = FormatFeatureFlagBits2::DEPTH_COPY_ON_COMPUTE_QUEUE_KHR.0;
        const DEPTH_COPY_ON_TRANSFER_QUEUE_KHR = FormatFeatureFlagBits2::DEPTH_COPY_ON_TRANSFER_QUEUE_KHR.0;
        const STENCIL_COPY_ON_COMPUTE_QUEUE_KHR = FormatFeatureFlagBits2::STENCIL_COPY_ON_COMPUTE_QUEUE_KHR.0;
        const STENCIL_COPY_ON_TRANSFER_QUEUE_KHR = FormatFeatureFlagBits2::STENCIL_COPY_ON_TRANSFER_QUEUE_KHR.0;
        const COPY_IMAGE_INDIRECT_DST_KHR = FormatFeatureFlagBits2::COPY_IMAGE_INDIRECT_DST_KHR.0;
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
    pub const SAMPLED_IMAGE_FILTER_CUBIC: Self = Self(1 << 13);
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
    pub const STORAGE_READ_WITHOUT_FORMAT: Self = Self(1 << 31);
    pub const STORAGE_WRITE_WITHOUT_FORMAT: Self = Self(1 << 32);
    pub const SAMPLED_IMAGE_DEPTH_COMPARISON: Self = Self(1 << 33);
    pub const WEIGHT_IMAGE_QCOM: Self = Self(1 << 34);
    pub const WEIGHT_SAMPLED_IMAGE_QCOM: Self = Self(1 << 35);
    pub const BLOCK_MATCHING_QCOM: Self = Self(1 << 36);
    pub const BOX_FILTER_SAMPLED_QCOM: Self = Self(1 << 37);
    pub const LINEAR_COLOR_ATTACHMENT_NV: Self = Self(1 << 38);
    pub const TENSOR_SHADER_ARM: Self = Self(1 << 39);
    pub const OPTICAL_FLOW_IMAGE_NV: Self = Self(1 << 40);
    pub const OPTICAL_FLOW_VECTOR_NV: Self = Self(1 << 41);
    pub const OPTICAL_FLOW_COST_NV: Self = Self(1 << 42);
    pub const TENSOR_IMAGE_ALIASING_ARM: Self = Self(1 << 43);
    pub const HOST_IMAGE_TRANSFER: Self = Self(1 << 46);
    pub const TENSOR_DATA_GRAPH_ARM: Self = Self(1 << 48);
    pub const VIDEO_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 49);
    pub const VIDEO_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 50);
    pub const ACCELERATION_STRUCTURE_RADIUS_BUFFER_NV: Self = Self(1 << 51);
    pub const DEPTH_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(1 << 52);
    pub const DEPTH_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(1 << 53);
    pub const STENCIL_COPY_ON_COMPUTE_QUEUE_KHR: Self = Self(1 << 54);
    pub const STENCIL_COPY_ON_TRANSFER_QUEUE_KHR: Self = Self(1 << 55);
    pub const COPY_IMAGE_INDIRECT_DST_KHR: Self = Self(1 << 59);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct RenderingFlags: Flags {
        const CONTENTS_SECONDARY_COMMAND_BUFFERS = RenderingFlagBits::CONTENTS_SECONDARY_COMMAND_BUFFERS.0;
        const SUSPENDING = RenderingFlagBits::SUSPENDING.0;
        const RESUMING = RenderingFlagBits::RESUMING.0;
        const ENABLE_LEGACY_DITHERING_EXT = RenderingFlagBits::ENABLE_LEGACY_DITHERING_EXT.0;
        const CONTENTS_INLINE_KHR = RenderingFlagBits::CONTENTS_INLINE_KHR.0;
        const PER_LAYER_FRAGMENT_DENSITY_VALVE = RenderingFlagBits::PER_LAYER_FRAGMENT_DENSITY_VALVE.0;
        const FRAGMENT_REGION_EXT = RenderingFlagBits::FRAGMENT_REGION_EXT.0;
        const CUSTOM_RESOLVE_EXT = RenderingFlagBits::CUSTOM_RESOLVE_EXT.0;
        const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR = RenderingFlagBits::LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RenderingFlagBits(u32);
impl RenderingFlagBits {
    pub const CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1 << 0);
    pub const SUSPENDING: Self = Self(1 << 1);
    pub const RESUMING: Self = Self(1 << 2);
    pub const ENABLE_LEGACY_DITHERING_EXT: Self = Self(1 << 3);
    pub const CONTENTS_INLINE_KHR: Self = Self(1 << 4);
    pub const PER_LAYER_FRAGMENT_DENSITY_VALVE: Self = Self(1 << 5);
    pub const FRAGMENT_REGION_EXT: Self = Self(1 << 6);
    pub const CUSTOM_RESOLVE_EXT: Self = Self(1 << 7);
    pub const LOCAL_READ_CONCURRENT_ACCESS_CONTROL_KHR: Self = Self(1 << 8);
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
        const DEBUG_REPORTING_EXT = ToolPurposeFlagBits::DEBUG_REPORTING_EXT.0;
        const DEBUG_MARKERS_EXT = ToolPurposeFlagBits::DEBUG_MARKERS_EXT.0;
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
    pub const DEBUG_REPORTING_EXT: Self = Self(1 << 5);
    pub const DEBUG_MARKERS_EXT: Self = Self(1 << 6);
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
pub type PFN_vkCmdSetCullMode =
    unsafe extern "system" fn(command_buffer: CommandBuffer, cull_mode: CullModeFlags);
pub type PFN_vkCmdSetFrontFace =
    unsafe extern "system" fn(command_buffer: CommandBuffer, front_face: FrontFace);
pub type PFN_vkCmdSetPrimitiveTopology =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_topology: PrimitiveTopology);
pub type PFN_vkCmdSetViewportWithCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    viewport_count: u32,
    p_viewports: *const Viewport,
);
pub type PFN_vkCmdSetScissorWithCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    scissor_count: u32,
    p_scissors: *const Rect2D,
);
pub type PFN_vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
    p_sizes: *const DeviceSize,
    p_strides: *const DeviceSize,
);
pub type PFN_vkCmdSetDepthTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_test_enable: Bool32);
pub type PFN_vkCmdSetDepthWriteEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_write_enable: Bool32);
pub type PFN_vkCmdSetDepthCompareOp =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_compare_op: CompareOp);
pub type PFN_vkCmdSetDepthBoundsTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bounds_test_enable: Bool32);
pub type PFN_vkCmdSetStencilTestEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, stencil_test_enable: Bool32);
pub type PFN_vkCmdSetStencilOp = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
);
pub type PFN_vkCmdSetRasterizerDiscardEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, rasterizer_discard_enable: Bool32);
pub type PFN_vkCmdSetDepthBiasEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, depth_bias_enable: Bool32);
pub type PFN_vkCmdSetPrimitiveRestartEnable =
    unsafe extern "system" fn(command_buffer: CommandBuffer, primitive_restart_enable: Bool32);
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
pub type PFN_vkCmdBlitImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_blit_image_info: *const BlitImageInfo2,
);
pub type PFN_vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
);
pub type PFN_vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
);
pub type PFN_vkCmdResolveImage2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_resolve_image_info: *const ResolveImageInfo2,
);
pub type PFN_vkCmdSetEvent2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    p_dependency_info: *const DependencyInfo,
);
pub type PFN_vkCmdResetEvent2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags2,
);
pub type PFN_vkCmdWaitEvents2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    p_dependency_infos: *const DependencyInfo,
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
pub type PFN_vkCmdBeginRendering = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_rendering_info: *const RenderingInfo,
);
pub type PFN_vkCmdEndRendering = unsafe extern "system" fn(command_buffer: CommandBuffer);
