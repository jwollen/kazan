#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorBufferFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_buffer: Bool32,
    pub descriptor_buffer_capture_replay: Bool32,
    pub descriptor_buffer_image_layout_ignored: Bool32,
    pub descriptor_buffer_push_descriptors: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorBufferFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            descriptor_buffer: Default::default(),
            descriptor_buffer_capture_replay: Default::default(),
            descriptor_buffer_image_layout_ignored: Default::default(),
            descriptor_buffer_push_descriptors: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorBufferPropertiesEXT<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorBufferPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            combined_image_sampler_descriptor_single_array: Default::default(),
            bufferless_push_descriptors: Default::default(),
            allow_sampler_image_view_post_submit_creation: Default::default(),
            descriptor_buffer_offset_alignment: Default::default(),
            max_descriptor_buffer_bindings: Default::default(),
            max_resource_descriptor_buffer_bindings: Default::default(),
            max_sampler_descriptor_buffer_bindings: Default::default(),
            max_embedded_immutable_sampler_bindings: Default::default(),
            max_embedded_immutable_samplers: Default::default(),
            buffer_capture_replay_descriptor_data_size: Default::default(),
            image_capture_replay_descriptor_data_size: Default::default(),
            image_view_capture_replay_descriptor_data_size: Default::default(),
            sampler_capture_replay_descriptor_data_size: Default::default(),
            acceleration_structure_capture_replay_descriptor_data_size: Default::default(),
            sampler_descriptor_size: Default::default(),
            combined_image_sampler_descriptor_size: Default::default(),
            sampled_image_descriptor_size: Default::default(),
            storage_image_descriptor_size: Default::default(),
            uniform_texel_buffer_descriptor_size: Default::default(),
            robust_uniform_texel_buffer_descriptor_size: Default::default(),
            storage_texel_buffer_descriptor_size: Default::default(),
            robust_storage_texel_buffer_descriptor_size: Default::default(),
            uniform_buffer_descriptor_size: Default::default(),
            robust_uniform_buffer_descriptor_size: Default::default(),
            storage_buffer_descriptor_size: Default::default(),
            robust_storage_buffer_descriptor_size: Default::default(),
            input_attachment_descriptor_size: Default::default(),
            acceleration_structure_descriptor_size: Default::default(),
            max_sampler_descriptor_buffer_range: Default::default(),
            max_resource_descriptor_buffer_range: Default::default(),
            sampler_descriptor_buffer_address_space_size: Default::default(),
            resource_descriptor_buffer_address_space_size: Default::default(),
            descriptor_buffer_address_space_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub combined_image_sampler_density_map_descriptor_size: usize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_BUFFER_DENSITY_MAP_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            combined_image_sampler_density_map_descriptor_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorAddressInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub address: DeviceAddress,
    pub range: DeviceSize,
    pub format: Format,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorAddressInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_ADDRESS_INFO_EXT,
            p_next: core::ptr::null_mut(),
            address: Default::default(),
            range: Default::default(),
            format: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorBufferBindingInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub address: DeviceAddress,
    pub usage: BufferUsageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorBufferBindingInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_BUFFER_BINDING_INFO_EXT,
            p_next: core::ptr::null(),
            address: Default::default(),
            usage: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorBufferBindingPushDescriptorBufferHandleEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorBufferBindingPushDescriptorBufferHandleEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_BUFFER_BINDING_PUSH_DESCRIPTOR_BUFFER_HANDLE_EXT,
            p_next: core::ptr::null(),
            buffer: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorGetInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: DescriptorType,
    pub data: DescriptorDataEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorGetInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_GET_INFO_EXT,
            p_next: core::ptr::null(),
            ty: Default::default(),
            data: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferCaptureDescriptorDataInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BufferCaptureDescriptorDataInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BUFFER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
            p_next: core::ptr::null(),
            buffer: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageCaptureDescriptorDataInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageCaptureDescriptorDataInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
            p_next: core::ptr::null(),
            image: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageViewCaptureDescriptorDataInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_view: ImageView,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageViewCaptureDescriptorDataInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_VIEW_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
            p_next: core::ptr::null(),
            image_view: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCaptureDescriptorDataInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub sampler: Sampler,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerCaptureDescriptorDataInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
            p_next: core::ptr::null(),
            sampler: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AccelerationStructureCaptureDescriptorDataInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acceleration_structure: AccelerationStructureKHR,
    pub acceleration_structure_nv: AccelerationStructureNV,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AccelerationStructureCaptureDescriptorDataInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ACCELERATION_STRUCTURE_CAPTURE_DESCRIPTOR_DATA_INFO_EXT,
            p_next: core::ptr::null(),
            acceleration_structure: Default::default(),
            acceleration_structure_nv: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpaqueCaptureDescriptorDataCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub opaque_capture_descriptor_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for OpaqueCaptureDescriptorDataCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::OPAQUE_CAPTURE_DESCRIPTOR_DATA_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            opaque_capture_descriptor_data: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DescriptorDataEXT<'a> {
    pub p_sampler: *const Sampler,
    pub p_combined_image_sampler: *const DescriptorImageInfo,
    pub p_input_attachment_image: *const DescriptorImageInfo,
    pub p_sampled_image: *const DescriptorImageInfo,
    pub p_storage_image: *const DescriptorImageInfo,
    pub p_uniform_texel_buffer: *const DescriptorAddressInfoEXT<'a>,
    pub p_storage_texel_buffer: *const DescriptorAddressInfoEXT<'a>,
    pub p_uniform_buffer: *const DescriptorAddressInfoEXT<'a>,
    pub p_storage_buffer: *const DescriptorAddressInfoEXT<'a>,
    pub acceleration_structure: DeviceAddress,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorDataEXT<'_> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
    p_descriptor_info: *const DescriptorGetInfoEXT<'_>,
    data_size: usize,
    p_descriptor: *mut c_void,
);
pub type PFN_vkCmdBindDescriptorBuffersEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer_count: u32,
    p_binding_infos: *const DescriptorBufferBindingInfoEXT<'_>,
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
    p_info: *const BufferCaptureDescriptorDataInfoEXT<'_>,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageCaptureDescriptorDataInfoEXT<'_>,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const ImageViewCaptureDescriptorDataInfoEXT<'_>,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: Device,
    p_info: *const SamplerCaptureDescriptorDataInfoEXT<'_>,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
    unsafe extern "system" fn(
        device: Device,
        p_info: *const AccelerationStructureCaptureDescriptorDataInfoEXT<'_>,
        p_data: *mut c_void,
    ) -> Result;
