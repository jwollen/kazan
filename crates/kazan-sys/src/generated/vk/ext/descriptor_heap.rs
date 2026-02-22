#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorARM(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorViewCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorViewCreateFlagsARM,
    pub tensor: TensorARM,
    pub format: Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostAddressRangeEXT {
    pub address: *mut c_void,
    pub size: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostAddressRangeConstEXT {
    pub address: *const c_void,
    pub size: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceAddressRangeEXT {
    pub address: DeviceAddress,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TexelBufferDescriptorInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub address_range: DeviceAddressRangeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageDescriptorInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_view: *const ImageViewCreateInfo,
    pub layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ResourceDescriptorInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: DescriptorType,
    pub data: ResourceDescriptorDataEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindHeapInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub heap_range: DeviceAddressRangeEXT,
    pub reserved_range_offset: DeviceSize,
    pub reserved_range_size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub offset: u32,
    pub data: HostAddressRangeConstEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceConstantOffsetEXT {
    pub heap_offset: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub sampler_heap_offset: u32,
    pub sampler_heap_array_stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourcePushIndexEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceIndirectIndexEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub address_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_address_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceIndirectIndexArrayEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub address_offset: u32,
    pub heap_index_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_address_offset: u32,
    pub sampler_heap_index_stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceHeapDataEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceShaderRecordIndexEXT {
    pub heap_offset: u32,
    pub shader_record_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_shader_record_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceIndirectAddressEXT {
    pub push_offset: u32,
    pub address_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetAndBindingMappingEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set: u32,
    pub first_binding: u32,
    pub binding_count: u32,
    pub resource_mask: SpirvResourceTypeFlagsEXT,
    pub source: DescriptorMappingSourceEXT,
    pub source_data: DescriptorMappingSourceDataEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderDescriptorSetAndBindingMappingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mapping_count: u32,
    pub p_mappings: *const DescriptorSetAndBindingMappingEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCustomBorderColorIndexCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpaqueCaptureDataCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_data: *const HostAddressRangeConstEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutPushDataTokenNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub push_data_offset: u32,
    pub push_data_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubsampledImageFormatPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subsampled_image_descriptor_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorHeapFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_heap: Bool32,
    pub descriptor_heap_capture_replay: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorHeapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub sampler_heap_alignment: DeviceSize,
    pub resource_heap_alignment: DeviceSize,
    pub max_sampler_heap_size: DeviceSize,
    pub max_resource_heap_size: DeviceSize,
    pub min_sampler_heap_reserved_range: DeviceSize,
    pub min_sampler_heap_reserved_range_with_embedded: DeviceSize,
    pub min_resource_heap_reserved_range: DeviceSize,
    pub sampler_descriptor_size: DeviceSize,
    pub image_descriptor_size: DeviceSize,
    pub buffer_descriptor_size: DeviceSize,
    pub sampler_descriptor_alignment: DeviceSize,
    pub image_descriptor_alignment: DeviceSize,
    pub buffer_descriptor_alignment: DeviceSize,
    pub max_push_data_size: DeviceSize,
    pub image_capture_replay_opaque_data_size: usize,
    pub max_descriptor_heap_embedded_samplers: u32,
    pub sampler_ycbcr_conversion_count: u32,
    pub sparse_descriptor_heaps: Bool32,
    pub protected_descriptor_heaps: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceDescriptorHeapInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_sampler_heap_bind_info: *const BindHeapInfoEXT,
    pub p_resource_heap_bind_info: *const BindHeapInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorHeapTensorPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_descriptor_size: DeviceSize,
    pub tensor_descriptor_alignment: DeviceSize,
    pub tensor_capture_replay_opaque_data_size: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ResourceDescriptorDataEXT {
    pub p_image: *const ImageDescriptorInfoEXT,
    pub p_texel_buffer: *const TexelBufferDescriptorInfoEXT,
    pub p_address_range: *const DeviceAddressRangeEXT,
    pub p_tensor_arm: *const TensorViewCreateInfoARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DescriptorMappingSourceDataEXT {
    pub constant_offset: DescriptorMappingSourceConstantOffsetEXT,
    pub push_index: DescriptorMappingSourcePushIndexEXT,
    pub indirect_index: DescriptorMappingSourceIndirectIndexEXT,
    pub indirect_index_array: DescriptorMappingSourceIndirectIndexArrayEXT,
    pub heap_data: DescriptorMappingSourceHeapDataEXT,
    pub push_data_offset: u32,
    pub push_address_offset: u32,
    pub indirect_address: DescriptorMappingSourceIndirectAddressEXT,
    pub shader_record_index: DescriptorMappingSourceShaderRecordIndexEXT,
    pub shader_record_data_offset: u32,
    pub shader_record_address_offset: u32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorMappingSourceEXT(i32);
impl DescriptorMappingSourceEXT {
    pub const HEAP_WITH_CONSTANT_OFFSET_EXT: Self = Self(0);
    pub const HEAP_WITH_PUSH_INDEX_EXT: Self = Self(1);
    pub const HEAP_WITH_INDIRECT_INDEX_EXT: Self = Self(2);
    pub const HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT: Self = Self(3);
    pub const RESOURCE_HEAP_DATA_EXT: Self = Self(4);
    pub const PUSH_DATA_EXT: Self = Self(5);
    pub const PUSH_ADDRESS_EXT: Self = Self(6);
    pub const INDIRECT_ADDRESS_EXT: Self = Self(7);
    pub const HEAP_WITH_SHADER_RECORD_INDEX_EXT: Self = Self(8);
    pub const SHADER_RECORD_ADDRESS_EXT: Self = Self(10);
    pub const SHADER_RECORD_DATA_EXT: Self = Self(9);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct TensorViewCreateFlagsARM: Flags64 {
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = TensorViewCreateFlagBitsARM::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorViewCreateFlagBitsARM(u64);
impl TensorViewCreateFlagBitsARM {
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SpirvResourceTypeFlagsEXT: Flags {
        const SAMPLER_EXT = SpirvResourceTypeFlagBitsEXT::SAMPLER_EXT.0;
        const SAMPLED_IMAGE_EXT = SpirvResourceTypeFlagBitsEXT::SAMPLED_IMAGE_EXT.0;
        const READ_ONLY_IMAGE_EXT = SpirvResourceTypeFlagBitsEXT::READ_ONLY_IMAGE_EXT.0;
        const READ_WRITE_IMAGE_EXT = SpirvResourceTypeFlagBitsEXT::READ_WRITE_IMAGE_EXT.0;
        const COMBINED_SAMPLED_IMAGE_EXT = SpirvResourceTypeFlagBitsEXT::COMBINED_SAMPLED_IMAGE_EXT.0;
        const UNIFORM_BUFFER_EXT = SpirvResourceTypeFlagBitsEXT::UNIFORM_BUFFER_EXT.0;
        const READ_ONLY_STORAGE_BUFFER_EXT = SpirvResourceTypeFlagBitsEXT::READ_ONLY_STORAGE_BUFFER_EXT.0;
        const READ_WRITE_STORAGE_BUFFER_EXT = SpirvResourceTypeFlagBitsEXT::READ_WRITE_STORAGE_BUFFER_EXT.0;
        const ACCELERATION_STRUCTURE_EXT = SpirvResourceTypeFlagBitsEXT::ACCELERATION_STRUCTURE_EXT.0;
        const TENSOR_ARM = SpirvResourceTypeFlagBitsEXT::TENSOR_ARM.0;
        const ALL = 0x7FFFFFFF;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpirvResourceTypeFlagBitsEXT(u32);
impl SpirvResourceTypeFlagBitsEXT {
    pub const SAMPLER_EXT: Self = Self(1 << 0);
    pub const SAMPLED_IMAGE_EXT: Self = Self(1 << 1);
    pub const READ_ONLY_IMAGE_EXT: Self = Self(1 << 2);
    pub const READ_WRITE_IMAGE_EXT: Self = Self(1 << 3);
    pub const COMBINED_SAMPLED_IMAGE_EXT: Self = Self(1 << 4);
    pub const UNIFORM_BUFFER_EXT: Self = Self(1 << 5);
    pub const READ_ONLY_STORAGE_BUFFER_EXT: Self = Self(1 << 6);
    pub const READ_WRITE_STORAGE_BUFFER_EXT: Self = Self(1 << 7);
    pub const ACCELERATION_STRUCTURE_EXT: Self = Self(1 << 8);
    pub const TENSOR_ARM: Self = Self(1 << 9);
}
pub type PFN_vkWriteSamplerDescriptorsEXT = unsafe extern "system" fn(
    device: Device,
    sampler_count: u32,
    p_samplers: *const SamplerCreateInfo,
    p_descriptors: *const HostAddressRangeEXT,
) -> Result;
pub type PFN_vkWriteResourceDescriptorsEXT = unsafe extern "system" fn(
    device: Device,
    resource_count: u32,
    p_resources: *const ResourceDescriptorInfoEXT,
    p_descriptors: *const HostAddressRangeEXT,
) -> Result;
pub type PFN_vkCmdBindSamplerHeapEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_bind_info: *const BindHeapInfoEXT);
pub type PFN_vkCmdBindResourceHeapEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_bind_info: *const BindHeapInfoEXT);
pub type PFN_vkCmdPushDataEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_data_info: *const PushDataInfoEXT,
);
pub type PFN_vkRegisterCustomBorderColorEXT = unsafe extern "system" fn(
    device: Device,
    p_border_color: *const SamplerCustomBorderColorCreateInfoEXT,
    request_index: Bool32,
    p_index: *mut u32,
) -> Result;
pub type PFN_vkUnregisterCustomBorderColorEXT =
    unsafe extern "system" fn(device: Device, index: u32);
pub type PFN_vkGetImageOpaqueCaptureDataEXT = unsafe extern "system" fn(
    device: Device,
    image_count: u32,
    p_images: *const Image,
    p_datas: *mut HostAddressRangeEXT,
) -> Result;
pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    descriptor_type: DescriptorType,
) -> DeviceSize;
pub type PFN_vkGetTensorOpaqueCaptureDataARM = unsafe extern "system" fn(
    device: Device,
    tensor_count: u32,
    p_tensors: *const TensorARM,
    p_datas: *mut HostAddressRangeEXT,
) -> Result;
