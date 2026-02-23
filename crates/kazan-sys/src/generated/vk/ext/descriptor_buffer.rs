#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_buffer: Bool32,
    pub descriptor_buffer_capture_replay: Bool32,
    pub descriptor_buffer_image_layout_ignored: Bool32,
    pub descriptor_buffer_push_descriptors: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_descriptor_single_array: Bool32,
    pub bufferless_push_descriptors: Bool32,
    pub allow_sampler_image_view_post_submit_creation: Bool32,
    pub descriptor_buffer_offset_alignment: DeviceSize,
    pub max_descriptor_buffer_bindings: u32,
    pub max_resource_descriptor_buffer_bindings: u32,
    pub max_sampler_descriptor_buffer_bindings: u32,
    pub max_embedded_immutable_sampler_bindings: u32,
    pub max_embedded_immutable_samplers: u32,
    pub buffer_capture_replay_descriptor_data_size: usize,
    pub image_capture_replay_descriptor_data_size: usize,
    pub image_view_capture_replay_descriptor_data_size: usize,
    pub sampler_capture_replay_descriptor_data_size: usize,
    pub acceleration_structure_capture_replay_descriptor_data_size: usize,
    pub sampler_descriptor_size: usize,
    pub combined_image_sampler_descriptor_size: usize,
    pub sampled_image_descriptor_size: usize,
    pub storage_image_descriptor_size: usize,
    pub uniform_texel_buffer_descriptor_size: usize,
    pub robust_uniform_texel_buffer_descriptor_size: usize,
    pub storage_texel_buffer_descriptor_size: usize,
    pub robust_storage_texel_buffer_descriptor_size: usize,
    pub uniform_buffer_descriptor_size: usize,
    pub robust_uniform_buffer_descriptor_size: usize,
    pub storage_buffer_descriptor_size: usize,
    pub robust_storage_buffer_descriptor_size: usize,
    pub input_attachment_descriptor_size: usize,
    pub acceleration_structure_descriptor_size: usize,
    pub max_sampler_descriptor_buffer_range: DeviceSize,
    pub max_resource_descriptor_buffer_range: DeviceSize,
    pub sampler_descriptor_buffer_address_space_size: DeviceSize,
    pub resource_descriptor_buffer_address_space_size: DeviceSize,
    pub descriptor_buffer_address_space_size: DeviceSize,
}
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_density_map_descriptor_size: usize,
}
#[repr(C)]
pub struct DescriptorAddressInfoEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub address: DeviceAddress,
    pub range: DeviceSize,
    pub format: Format,
}
#[repr(C)]
pub struct DescriptorBufferBindingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address: DeviceAddress,
    pub usage: BufferUsageFlags,
}
#[repr(C)]
pub struct DescriptorBufferBindingPushDescriptorBufferHandleEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
#[repr(C)]
pub struct DescriptorGetInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: DescriptorType,
    pub data: DescriptorDataEXT,
}
#[repr(C)]
pub struct BufferCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
}
#[repr(C)]
pub struct ImageCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
}
#[repr(C)]
pub struct ImageViewCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
}
#[repr(C)]
pub struct SamplerCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sampler: Sampler,
}
#[repr(C)]
pub struct AccelerationStructureCaptureDescriptorDataInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureKHR,
    pub acceleration_structure_nv: AccelerationStructureNV,
}
#[repr(C)]
pub struct OpaqueCaptureDescriptorDataCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_descriptor_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DescriptorDataEXT {
    pub p_sampler: *const Sampler,
    pub p_combined_image_sampler: *const DescriptorImageInfo,
    pub p_input_attachment_image: *const DescriptorImageInfo,
    pub p_sampled_image: *const DescriptorImageInfo,
    pub p_storage_image: *const DescriptorImageInfo,
    pub p_uniform_texel_buffer: *const DescriptorAddressInfoEXT,
    pub p_storage_texel_buffer: *const DescriptorAddressInfoEXT,
    pub p_uniform_buffer: *const DescriptorAddressInfoEXT,
    pub p_storage_buffer: *const DescriptorAddressInfoEXT,
    pub acceleration_structure: DeviceAddress,
}
pub type PFN_vkGetDescriptorSetLayoutSizeEXT = unsafe extern "system" fn(
    device: Device,
    layout: DescriptorSetLayout,
    p_layout_size_in_bytes: *mut DeviceSize,
);
pub type PFN_vkGetDescriptorSetLayoutBindingOffsetEXT = unsafe extern "system" fn(
    device: Device,
    layout: DescriptorSetLayout,
    binding: u32,
    p_offset: *mut DeviceSize,
);
pub type PFN_vkGetDescriptorEXT = unsafe extern "system" fn(
    device: Device,
    p_descriptor_info: *const DescriptorGetInfoEXT,
    data_size: usize,
    p_descriptor: *mut c_void,
);
pub type PFN_vkCmdBindDescriptorBuffersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer_count: u32,
    p_binding_infos: *const DescriptorBufferBindingInfoEXT,
);
pub type PFN_vkCmdSetDescriptorBufferOffsetsEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    set_count: u32,
    p_buffer_indices: *const u32,
    p_offsets: *const DeviceSize,
);
pub type PFN_vkCmdBindDescriptorBufferEmbeddedSamplersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
);
pub type PFN_vkGetBufferOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const BufferCaptureDescriptorDataInfoEXT,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageCaptureDescriptorDataInfoEXT,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageViewCaptureDescriptorDataInfoEXT,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const SamplerCaptureDescriptorDataInfoEXT,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT,
        p_data: *mut c_void,
    ) -> Result;
