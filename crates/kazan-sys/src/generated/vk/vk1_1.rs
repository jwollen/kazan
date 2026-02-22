#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const LUID_SIZE: u32 = 8;
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorUpdateTemplate(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversion(u64);
pub type PhysicalDeviceVariablePointerFeatures = PhysicalDeviceVariablePointersFeatures;
pub type PhysicalDeviceShaderDrawParameterFeatures = PhysicalDeviceShaderDrawParametersFeatures;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub features: PhysicalDeviceFeatures,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: PhysicalDeviceProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format_properties: FormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub image_format_properties: ImageFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub flags: ImageCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub queue_family_properties: QueueFamilyProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMemoryProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_properties: PhysicalDeviceMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageFormatProperties2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub properties: SparseImageFormatProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSparseImageFormatInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ty: ImageType,
    pub samples: SampleCountFlagBits,
    pub usage: ImageUsageFlags,
    pub tiling: ImageTiling,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVariablePointersFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryProperties {
    pub external_memory_features: ExternalMemoryFeatureFlags,
    pub export_from_imported_handle_types: ExternalMemoryHandleTypeFlags,
    pub compatible_handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalImageFormatInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalBufferInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferCreateFlags,
    pub usage: BufferUsageFlags,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalBufferProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceIDProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub device_uuid: [u8; UUID_SIZE as usize],
    pub driver_uuid: [u8; UUID_SIZE as usize],
    pub device_luid: [u8; LUID_SIZE as usize],
    pub device_node_mask: u32,
    pub device_luid_valid: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryImageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryBufferCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportMemoryAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalSemaphoreInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalSemaphoreProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub compatible_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub external_semaphore_features: ExternalSemaphoreFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportSemaphoreCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalSemaphoreHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalFenceInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalFenceProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub export_from_imported_handle_types: ExternalFenceHandleTypeFlags,
    pub compatible_handle_types: ExternalFenceHandleTypeFlags,
    pub external_fence_features: ExternalFenceFeatureFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExportFenceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalFenceHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub physical_device_count: u32,
    pub physical_devices: [PhysicalDevice; MAX_DEVICE_GROUP_SIZE as usize],
    pub subset_allocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryAllocateFlagsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MemoryAllocateFlags,
    pub device_mask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindBufferMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImageMemoryDeviceGroupInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_index_count: u32,
    pub p_device_indices: *const u32,
    pub split_instance_bind_region_count: u32,
    pub p_split_instance_bind_regions: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupCommandBufferBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub wait_semaphore_count: u32,
    pub p_wait_semaphore_device_indices: *const u32,
    pub command_buffer_count: u32,
    pub p_command_buffer_device_masks: *const u32,
    pub signal_semaphore_count: u32,
    pub p_signal_semaphore_device_indices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupBindSparseInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub resource_device_index: u32,
    pub memory_device_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupDeviceCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub physical_device_count: u32,
    pub p_physical_devices: *const PhysicalDevice,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorUpdateTemplateEntry {
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub offset: usize,
    pub stride: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorUpdateTemplateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorUpdateTemplateCreateFlags,
    pub descriptor_update_entry_count: u32,
    pub p_descriptor_update_entries: *const DescriptorUpdateTemplateEntry,
    pub template_type: DescriptorUpdateTemplateType,
    pub descriptor_set_layout: DescriptorSetLayout,
    pub pipeline_bind_point: PipelineBindPoint,
    pub pipeline_layout: PipelineLayout,
    pub set: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const InputAttachmentAspectReference,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub storage_buffer16_bit_access: Bool32,
    pub uniform_and_storage_buffer16_bit_access: Bool32,
    pub storage_push_constant16: Bool32,
    pub storage_input_output16: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subgroup_size: u32,
    pub supported_stages: ShaderStageFlags,
    pub supported_operations: SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageSparseMemoryRequirementsInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: MemoryRequirements,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SparseImageMemoryRequirements2 {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_requirements: SparseImageMemoryRequirements,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub point_clipping_behavior: PointClippingBehavior,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedRequirements {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub prefers_dedicated_allocation: Bool32,
    pub requires_dedicated_allocation: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub usage: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub domain_origin: TessellationDomainOrigin,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conversion: SamplerYcbcrConversion,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub ycbcr_model: SamplerYcbcrModelConversion,
    pub ycbcr_range: SamplerYcbcrRange,
    pub components: ComponentMapping,
    pub x_chroma_offset: ChromaLocation,
    pub y_chroma_offset: ChromaLocation,
    pub chroma_filter: Filter,
    pub force_explicit_reconstruction: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindImagePlaneMemoryInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImagePlaneMemoryRequirementsInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub plane_aspect: ImageAspectFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_ycbcr_conversion: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerYcbcrConversionImageFormatProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_descriptor_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProtectedSubmitInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub protected_submit: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_memory: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub protected_no_fault: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceQueueInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutSupport {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_draw_parameters: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorUpdateTemplateType(i32);
impl DescriptorUpdateTemplateType {
    pub const DESCRIPTOR_SET: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointClippingBehavior(i32);
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TessellationDomainOrigin(i32);
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerYcbcrModelConversion(i32);
impl SamplerYcbcrModelConversion {
    pub const RGB_IDENTITY: Self = Self(0);
    pub const YCBCR_IDENTITY: Self = Self(1);
    pub const YCBCR_709: Self = Self(2);
    pub const YCBCR_601: Self = Self(3);
    pub const YCBCR_2020: Self = Self(4);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerYcbcrRange(i32);
impl SamplerYcbcrRange {
    pub const ITU_FULL: Self = Self(0);
    pub const ITU_NARROW: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChromaLocation(i32);
impl ChromaLocation {
    pub const COSITED_EVEN: Self = Self(0);
    pub const MIDPOINT: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SubgroupFeatureFlags: Flags {
        const BASIC = SubgroupFeatureFlagBits::BASIC.0;
        const VOTE = SubgroupFeatureFlagBits::VOTE.0;
        const ARITHMETIC = SubgroupFeatureFlagBits::ARITHMETIC.0;
        const BALLOT = SubgroupFeatureFlagBits::BALLOT.0;
        const SHUFFLE = SubgroupFeatureFlagBits::SHUFFLE.0;
        const SHUFFLE_RELATIVE = SubgroupFeatureFlagBits::SHUFFLE_RELATIVE.0;
        const CLUSTERED = SubgroupFeatureFlagBits::CLUSTERED.0;
        const QUAD = SubgroupFeatureFlagBits::QUAD.0;
        const PARTITIONED_EXT = SubgroupFeatureFlagBits::PARTITIONED_EXT.0;
        const ROTATE = SubgroupFeatureFlagBits::ROTATE.0;
        const ROTATE_CLUSTERED = SubgroupFeatureFlagBits::ROTATE_CLUSTERED.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubgroupFeatureFlagBits(u32);
impl SubgroupFeatureFlagBits {
    pub const BASIC: Self = Self(1 << 0);
    pub const VOTE: Self = Self(1 << 1);
    pub const ARITHMETIC: Self = Self(1 << 2);
    pub const BALLOT: Self = Self(1 << 3);
    pub const SHUFFLE: Self = Self(1 << 4);
    pub const SHUFFLE_RELATIVE: Self = Self(1 << 5);
    pub const CLUSTERED: Self = Self(1 << 6);
    pub const QUAD: Self = Self(1 << 7);
    pub const PARTITIONED_EXT: Self = Self(1 << 8);
    pub const ROTATE: Self = Self(1 << 9);
    pub const ROTATE_CLUSTERED: Self = Self(1 << 10);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DescriptorUpdateTemplateCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PeerMemoryFeatureFlags: Flags {
        const COPY_SRC = PeerMemoryFeatureFlagBits::COPY_SRC.0;
        const COPY_DST = PeerMemoryFeatureFlagBits::COPY_DST.0;
        const GENERIC_SRC = PeerMemoryFeatureFlagBits::GENERIC_SRC.0;
        const GENERIC_DST = PeerMemoryFeatureFlagBits::GENERIC_DST.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerMemoryFeatureFlagBits(u32);
impl PeerMemoryFeatureFlagBits {
    pub const COPY_SRC: Self = Self(1 << 0);
    pub const COPY_DST: Self = Self(1 << 1);
    pub const GENERIC_SRC: Self = Self(1 << 2);
    pub const GENERIC_DST: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MemoryAllocateFlags: Flags {
        const DEVICE_MASK = MemoryAllocateFlagBits::DEVICE_MASK.0;
        const DEVICE_ADDRESS = MemoryAllocateFlagBits::DEVICE_ADDRESS.0;
        const DEVICE_ADDRESS_CAPTURE_REPLAY = MemoryAllocateFlagBits::DEVICE_ADDRESS_CAPTURE_REPLAY.0;
        const ZERO_INITIALIZE_EXT = MemoryAllocateFlagBits::ZERO_INITIALIZE_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryAllocateFlagBits(u32);
impl MemoryAllocateFlagBits {
    pub const DEVICE_MASK: Self = Self(1 << 0);
    pub const DEVICE_ADDRESS: Self = Self(1 << 1);
    pub const DEVICE_ADDRESS_CAPTURE_REPLAY: Self = Self(1 << 2);
    pub const ZERO_INITIALIZE_EXT: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct CommandPoolTrimFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalMemoryHandleTypeFlags: Flags {
        const OPAQUE_FD = ExternalMemoryHandleTypeFlagBits::OPAQUE_FD.0;
        const OPAQUE_WIN32 = ExternalMemoryHandleTypeFlagBits::OPAQUE_WIN32.0;
        const OPAQUE_WIN32_KMT = ExternalMemoryHandleTypeFlagBits::OPAQUE_WIN32_KMT.0;
        const D3D11_TEXTURE = ExternalMemoryHandleTypeFlagBits::D3D11_TEXTURE.0;
        const D3D11_TEXTURE_KMT = ExternalMemoryHandleTypeFlagBits::D3D11_TEXTURE_KMT.0;
        const D3D12_HEAP = ExternalMemoryHandleTypeFlagBits::D3D12_HEAP.0;
        const D3D12_RESOURCE = ExternalMemoryHandleTypeFlagBits::D3D12_RESOURCE.0;
        const HOST_ALLOCATION_EXT = ExternalMemoryHandleTypeFlagBits::HOST_ALLOCATION_EXT.0;
        const HOST_MAPPED_FOREIGN_MEMORY_EXT = ExternalMemoryHandleTypeFlagBits::HOST_MAPPED_FOREIGN_MEMORY_EXT.0;
        const DMA_BUF_EXT = ExternalMemoryHandleTypeFlagBits::DMA_BUF_EXT.0;
        const ANDROID_HARDWARE_BUFFER_ANDROID = ExternalMemoryHandleTypeFlagBits::ANDROID_HARDWARE_BUFFER_ANDROID.0;
        const ZIRCON_VMO_FUCHSIA = ExternalMemoryHandleTypeFlagBits::ZIRCON_VMO_FUCHSIA.0;
        const RDMA_ADDRESS_NV = ExternalMemoryHandleTypeFlagBits::RDMA_ADDRESS_NV.0;
        const SCREEN_BUFFER_QNX = ExternalMemoryHandleTypeFlagBits::SCREEN_BUFFER_QNX.0;
        const OH_NATIVE_BUFFER_OHOS = ExternalMemoryHandleTypeFlagBits::OH_NATIVE_BUFFER_OHOS.0;
        const MTLBUFFER_EXT = ExternalMemoryHandleTypeFlagBits::MTLBUFFER_EXT.0;
        const MTLTEXTURE_EXT = ExternalMemoryHandleTypeFlagBits::MTLTEXTURE_EXT.0;
        const MTLHEAP_EXT = ExternalMemoryHandleTypeFlagBits::MTLHEAP_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryHandleTypeFlagBits(u32);
impl ExternalMemoryHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const D3D11_TEXTURE: Self = Self(1 << 3);
    pub const D3D11_TEXTURE_KMT: Self = Self(1 << 4);
    pub const D3D12_HEAP: Self = Self(1 << 5);
    pub const D3D12_RESOURCE: Self = Self(1 << 6);
    pub const HOST_ALLOCATION_EXT: Self = Self(1 << 7);
    pub const HOST_MAPPED_FOREIGN_MEMORY_EXT: Self = Self(1 << 8);
    pub const DMA_BUF_EXT: Self = Self(1 << 9);
    pub const ANDROID_HARDWARE_BUFFER_ANDROID: Self = Self(1 << 10);
    pub const ZIRCON_VMO_FUCHSIA: Self = Self(1 << 11);
    pub const RDMA_ADDRESS_NV: Self = Self(1 << 12);
    pub const SCREEN_BUFFER_QNX: Self = Self(1 << 14);
    pub const OH_NATIVE_BUFFER_OHOS: Self = Self(1 << 15);
    pub const MTLBUFFER_EXT: Self = Self(1 << 16);
    pub const MTLTEXTURE_EXT: Self = Self(1 << 17);
    pub const MTLHEAP_EXT: Self = Self(1 << 18);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalMemoryFeatureFlags: Flags {
        const DEDICATED_ONLY = ExternalMemoryFeatureFlagBits::DEDICATED_ONLY.0;
        const EXPORTABLE = ExternalMemoryFeatureFlagBits::EXPORTABLE.0;
        const IMPORTABLE = ExternalMemoryFeatureFlagBits::IMPORTABLE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalMemoryFeatureFlagBits(u32);
impl ExternalMemoryFeatureFlagBits {
    pub const DEDICATED_ONLY: Self = Self(1 << 0);
    pub const EXPORTABLE: Self = Self(1 << 1);
    pub const IMPORTABLE: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalSemaphoreHandleTypeFlags: Flags {
        const OPAQUE_FD = ExternalSemaphoreHandleTypeFlagBits::OPAQUE_FD.0;
        const OPAQUE_WIN32 = ExternalSemaphoreHandleTypeFlagBits::OPAQUE_WIN32.0;
        const OPAQUE_WIN32_KMT = ExternalSemaphoreHandleTypeFlagBits::OPAQUE_WIN32_KMT.0;
        const D3D12_FENCE = ExternalSemaphoreHandleTypeFlagBits::D3D12_FENCE.0;
        const SYNC_FD = ExternalSemaphoreHandleTypeFlagBits::SYNC_FD.0;
        const ZIRCON_EVENT_FUCHSIA = ExternalSemaphoreHandleTypeFlagBits::ZIRCON_EVENT_FUCHSIA.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreHandleTypeFlagBits(u32);
impl ExternalSemaphoreHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const D3D12_FENCE: Self = Self(1 << 3);
    pub const SYNC_FD: Self = Self(1 << 4);
    pub const ZIRCON_EVENT_FUCHSIA: Self = Self(1 << 7);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalSemaphoreFeatureFlags: Flags {
        const EXPORTABLE = ExternalSemaphoreFeatureFlagBits::EXPORTABLE.0;
        const IMPORTABLE = ExternalSemaphoreFeatureFlagBits::IMPORTABLE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalSemaphoreFeatureFlagBits(u32);
impl ExternalSemaphoreFeatureFlagBits {
    pub const EXPORTABLE: Self = Self(1 << 0);
    pub const IMPORTABLE: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SemaphoreImportFlags: Flags {
        const TEMPORARY = SemaphoreImportFlagBits::TEMPORARY.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SemaphoreImportFlagBits(u32);
impl SemaphoreImportFlagBits {
    pub const TEMPORARY: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalFenceHandleTypeFlags: Flags {
        const OPAQUE_FD = ExternalFenceHandleTypeFlagBits::OPAQUE_FD.0;
        const OPAQUE_WIN32 = ExternalFenceHandleTypeFlagBits::OPAQUE_WIN32.0;
        const OPAQUE_WIN32_KMT = ExternalFenceHandleTypeFlagBits::OPAQUE_WIN32_KMT.0;
        const SYNC_FD = ExternalFenceHandleTypeFlagBits::SYNC_FD.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceHandleTypeFlagBits(u32);
impl ExternalFenceHandleTypeFlagBits {
    pub const OPAQUE_FD: Self = Self(1 << 0);
    pub const OPAQUE_WIN32: Self = Self(1 << 1);
    pub const OPAQUE_WIN32_KMT: Self = Self(1 << 2);
    pub const SYNC_FD: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ExternalFenceFeatureFlags: Flags {
        const EXPORTABLE = ExternalFenceFeatureFlagBits::EXPORTABLE.0;
        const IMPORTABLE = ExternalFenceFeatureFlagBits::IMPORTABLE.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFenceFeatureFlagBits(u32);
impl ExternalFenceFeatureFlagBits {
    pub const EXPORTABLE: Self = Self(1 << 0);
    pub const IMPORTABLE: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FenceImportFlags: Flags {
        const TEMPORARY = FenceImportFlagBits::TEMPORARY.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FenceImportFlagBits(u32);
impl FenceImportFlagBits {
    pub const TEMPORARY: Self = Self(1 << 0);
}
pub type PFN_vkEnumerateInstanceVersion =
    unsafe extern "system" fn(p_api_version: *mut u32) -> Result;
pub type PFN_vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_features: *mut PhysicalDeviceFeatures2,
);
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_properties: *mut PhysicalDeviceProperties2,
);
pub type PFN_vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    format: Format,
    p_format_properties: *mut FormatProperties2,
);
pub type PFN_vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_image_format_info: *const PhysicalDeviceImageFormatInfo2,
    p_image_format_properties: *mut ImageFormatProperties2,
) -> Result;
pub type PFN_vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_property_count: *mut u32,
    p_queue_family_properties: *mut QueueFamilyProperties2,
);
pub type PFN_vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_memory_properties: *mut PhysicalDeviceMemoryProperties2,
);
pub type PFN_vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    p_property_count: *mut u32,
    p_properties: *mut SparseImageFormatProperties2,
);
pub type PFN_vkTrimCommandPool = unsafe extern "system" fn(
    device: Device,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
);
pub type PFN_vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    p_external_buffer_properties: *mut ExternalBufferProperties,
);
pub type PFN_vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    p_external_semaphore_properties: *mut ExternalSemaphoreProperties,
);
pub type PFN_vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    p_external_fence_properties: *mut ExternalFenceProperties,
);
pub type PFN_vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: Instance,
    p_physical_device_group_count: *mut u32,
    p_physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> Result;
pub type PFN_vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: Device,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    p_peer_memory_features: *mut PeerMemoryFeatureFlags,
);
pub type PFN_vkBindBufferMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindBufferMemoryInfo,
) -> Result;
pub type PFN_vkBindImageMemory2 = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindImageMemoryInfo,
) -> Result;
pub type PFN_vkCmdSetDeviceMask =
    unsafe extern "system" fn(command_buffer: CommandBuffer, device_mask: u32);
pub type PFN_vkCmdDispatchBase = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
pub type PFN_vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorUpdateTemplateCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> Result;
pub type PFN_vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: Device,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    p_data: *const c_void,
);
pub type PFN_vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageMemoryRequirementsInfo2,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageSparseMemoryRequirementsInfo2,
    p_sparse_memory_requirement_count: *mut u32,
    p_sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);
pub type PFN_vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerYcbcrConversionCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> Result;
pub type PFN_vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: Device,
    ycbcr_conversion: SamplerYcbcrConversion,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: Device,
    p_queue_info: *const DeviceQueueInfo2,
    p_queue: *mut Queue,
);
pub type PFN_vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_support: *mut DescriptorSetLayoutSupport,
);
