#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TensorARM(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorViewCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorViewCreateFlagsARM,
    pub tensor: TensorARM,
    pub format: Format,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TensorViewCreateInfoARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TENSOR_VIEW_CREATE_INFO_ARM,
            p_next: core::ptr::null(),
            flags: Default::default(),
            tensor: Default::default(),
            format: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostAddressRangeEXT<'a> {
    pub address: *mut c_void,
    pub size: usize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for HostAddressRangeEXT<'_> {
    fn default() -> Self {
        Self {
            address: core::ptr::null_mut(),
            size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HostAddressRangeConstEXT<'a> {
    pub address: *const c_void,
    pub size: usize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for HostAddressRangeConstEXT<'_> {
    fn default() -> Self {
        Self {
            address: core::ptr::null(),
            size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DeviceAddressRangeEXT {
    pub address: DeviceAddress,
    pub size: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TexelBufferDescriptorInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub format: Format,
    pub address_range: DeviceAddressRangeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for TexelBufferDescriptorInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::TEXEL_BUFFER_DESCRIPTOR_INFO_EXT,
            p_next: core::ptr::null(),
            format: Default::default(),
            address_range: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageDescriptorInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_view: *const ImageViewCreateInfo<'a>,
    pub layout: ImageLayout,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImageDescriptorInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGE_DESCRIPTOR_INFO_EXT,
            p_next: core::ptr::null(),
            p_view: core::ptr::null(),
            layout: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ResourceDescriptorInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub ty: DescriptorType,
    pub data: ResourceDescriptorDataEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ResourceDescriptorInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RESOURCE_DESCRIPTOR_INFO_EXT,
            p_next: core::ptr::null(),
            ty: Default::default(),
            data: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindHeapInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub heap_range: DeviceAddressRangeEXT,
    pub reserved_range_offset: DeviceSize,
    pub reserved_range_size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindHeapInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_HEAP_INFO_EXT,
            p_next: core::ptr::null(),
            heap_range: Default::default(),
            reserved_range_offset: Default::default(),
            reserved_range_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushDataInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub offset: u32,
    pub data: HostAddressRangeConstEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PushDataInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PUSH_DATA_INFO_EXT,
            p_next: core::ptr::null(),
            offset: Default::default(),
            data: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceConstantOffsetEXT<'a> {
    pub heap_offset: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
    pub sampler_heap_offset: u32,
    pub sampler_heap_array_stride: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorMappingSourceConstantOffsetEXT<'_> {
    fn default() -> Self {
        Self {
            heap_offset: Default::default(),
            heap_array_stride: Default::default(),
            p_embedded_sampler: core::ptr::null(),
            sampler_heap_offset: Default::default(),
            sampler_heap_array_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourcePushIndexEXT<'a> {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorMappingSourcePushIndexEXT<'_> {
    fn default() -> Self {
        Self {
            heap_offset: Default::default(),
            push_offset: Default::default(),
            heap_index_stride: Default::default(),
            heap_array_stride: Default::default(),
            p_embedded_sampler: core::ptr::null(),
            use_combined_image_sampler_index: Default::default(),
            sampler_heap_offset: Default::default(),
            sampler_push_offset: Default::default(),
            sampler_heap_index_stride: Default::default(),
            sampler_heap_array_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceIndirectIndexEXT<'a> {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub address_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_address_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorMappingSourceIndirectIndexEXT<'_> {
    fn default() -> Self {
        Self {
            heap_offset: Default::default(),
            push_offset: Default::default(),
            address_offset: Default::default(),
            heap_index_stride: Default::default(),
            heap_array_stride: Default::default(),
            p_embedded_sampler: core::ptr::null(),
            use_combined_image_sampler_index: Default::default(),
            sampler_heap_offset: Default::default(),
            sampler_push_offset: Default::default(),
            sampler_address_offset: Default::default(),
            sampler_heap_index_stride: Default::default(),
            sampler_heap_array_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceIndirectIndexArrayEXT<'a> {
    pub heap_offset: u32,
    pub push_offset: u32,
    pub address_offset: u32,
    pub heap_index_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_push_offset: u32,
    pub sampler_address_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorMappingSourceIndirectIndexArrayEXT<'_> {
    fn default() -> Self {
        Self {
            heap_offset: Default::default(),
            push_offset: Default::default(),
            address_offset: Default::default(),
            heap_index_stride: Default::default(),
            p_embedded_sampler: core::ptr::null(),
            use_combined_image_sampler_index: Default::default(),
            sampler_heap_offset: Default::default(),
            sampler_push_offset: Default::default(),
            sampler_address_offset: Default::default(),
            sampler_heap_index_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DescriptorMappingSourceHeapDataEXT {
    pub heap_offset: u32,
    pub push_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorMappingSourceShaderRecordIndexEXT<'a> {
    pub heap_offset: u32,
    pub shader_record_offset: u32,
    pub heap_index_stride: u32,
    pub heap_array_stride: u32,
    pub p_embedded_sampler: *const SamplerCreateInfo<'a>,
    pub use_combined_image_sampler_index: Bool32,
    pub sampler_heap_offset: u32,
    pub sampler_shader_record_offset: u32,
    pub sampler_heap_index_stride: u32,
    pub sampler_heap_array_stride: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorMappingSourceShaderRecordIndexEXT<'_> {
    fn default() -> Self {
        Self {
            heap_offset: Default::default(),
            shader_record_offset: Default::default(),
            heap_index_stride: Default::default(),
            heap_array_stride: Default::default(),
            p_embedded_sampler: core::ptr::null(),
            use_combined_image_sampler_index: Default::default(),
            sampler_heap_offset: Default::default(),
            sampler_shader_record_offset: Default::default(),
            sampler_heap_index_stride: Default::default(),
            sampler_heap_array_stride: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DescriptorMappingSourceIndirectAddressEXT {
    pub push_offset: u32,
    pub address_offset: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetAndBindingMappingEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set: u32,
    pub first_binding: u32,
    pub binding_count: u32,
    pub resource_mask: SpirvResourceTypeFlagsEXT,
    pub source: DescriptorMappingSourceEXT,
    pub source_data: DescriptorMappingSourceDataEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorSetAndBindingMappingEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DESCRIPTOR_SET_AND_BINDING_MAPPING_EXT,
            p_next: core::ptr::null(),
            descriptor_set: Default::default(),
            first_binding: Default::default(),
            binding_count: Default::default(),
            resource_mask: Default::default(),
            source: Default::default(),
            source_data: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderDescriptorSetAndBindingMappingInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub mapping_count: u32,
    pub p_mappings: *const DescriptorSetAndBindingMappingEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ShaderDescriptorSetAndBindingMappingInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SHADER_DESCRIPTOR_SET_AND_BINDING_MAPPING_INFO_EXT,
            p_next: core::ptr::null(),
            mapping_count: Default::default(),
            p_mappings: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCustomBorderColorIndexCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub index: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SamplerCustomBorderColorIndexCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SAMPLER_CUSTOM_BORDER_COLOR_INDEX_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            index: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OpaqueCaptureDataCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_data: *const HostAddressRangeConstEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for OpaqueCaptureDataCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::OPAQUE_CAPTURE_DATA_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            p_data: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IndirectCommandsLayoutPushDataTokenNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub push_data_offset: u32,
    pub push_data_size: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IndirectCommandsLayoutPushDataTokenNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::INDIRECT_COMMANDS_LAYOUT_PUSH_DATA_TOKEN_NV,
            p_next: core::ptr::null(),
            push_data_offset: Default::default(),
            push_data_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubsampledImageFormatPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subsampled_image_descriptor_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SubsampledImageFormatPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SUBSAMPLED_IMAGE_FORMAT_PROPERTIES_EXT,
            p_next: core::ptr::null(),
            subsampled_image_descriptor_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorHeapFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_heap: Bool32,
    pub descriptor_heap_capture_replay: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorHeapFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            descriptor_heap: Default::default(),
            descriptor_heap_capture_replay: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorHeapPropertiesEXT<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorHeapPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            sampler_heap_alignment: Default::default(),
            resource_heap_alignment: Default::default(),
            max_sampler_heap_size: Default::default(),
            max_resource_heap_size: Default::default(),
            min_sampler_heap_reserved_range: Default::default(),
            min_sampler_heap_reserved_range_with_embedded: Default::default(),
            min_resource_heap_reserved_range: Default::default(),
            sampler_descriptor_size: Default::default(),
            image_descriptor_size: Default::default(),
            buffer_descriptor_size: Default::default(),
            sampler_descriptor_alignment: Default::default(),
            image_descriptor_alignment: Default::default(),
            buffer_descriptor_alignment: Default::default(),
            max_push_data_size: Default::default(),
            image_capture_replay_opaque_data_size: Default::default(),
            max_descriptor_heap_embedded_samplers: Default::default(),
            sampler_ycbcr_conversion_count: Default::default(),
            sparse_descriptor_heaps: Default::default(),
            protected_descriptor_heaps: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceDescriptorHeapInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_sampler_heap_bind_info: *const BindHeapInfoEXT<'a>,
    pub p_resource_heap_bind_info: *const BindHeapInfoEXT<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CommandBufferInheritanceDescriptorHeapInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_BUFFER_INHERITANCE_DESCRIPTOR_HEAP_INFO_EXT,
            p_next: core::ptr::null(),
            p_sampler_heap_bind_info: core::ptr::null(),
            p_resource_heap_bind_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorHeapTensorPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_descriptor_size: DeviceSize,
    pub tensor_descriptor_alignment: DeviceSize,
    pub tensor_capture_replay_opaque_data_size: usize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDescriptorHeapTensorPropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DESCRIPTOR_HEAP_TENSOR_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
            tensor_descriptor_size: Default::default(),
            tensor_descriptor_alignment: Default::default(),
            tensor_capture_replay_opaque_data_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ResourceDescriptorDataEXT<'a> {
    pub p_image: *const ImageDescriptorInfoEXT<'a>,
    pub p_texel_buffer: *const TexelBufferDescriptorInfoEXT<'a>,
    pub p_address_range: *const DeviceAddressRangeEXT,
    pub p_tensor_arm: *const TensorViewCreateInfoARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ResourceDescriptorDataEXT<'_> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union DescriptorMappingSourceDataEXT<'a> {
    pub constant_offset: DescriptorMappingSourceConstantOffsetEXT<'a>,
    pub push_index: DescriptorMappingSourcePushIndexEXT<'a>,
    pub indirect_index: DescriptorMappingSourceIndirectIndexEXT<'a>,
    pub indirect_index_array: DescriptorMappingSourceIndirectIndexArrayEXT<'a>,
    pub heap_data: DescriptorMappingSourceHeapDataEXT,
    pub push_data_offset: u32,
    pub push_address_offset: u32,
    pub indirect_address: DescriptorMappingSourceIndirectAddressEXT,
    pub shader_record_index: DescriptorMappingSourceShaderRecordIndexEXT<'a>,
    pub shader_record_data_offset: u32,
    pub shader_record_address_offset: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DescriptorMappingSourceDataEXT<'_> {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TensorViewCreateFlagsARM: Flags64 {
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = TensorViewCreateFlagBitsARM::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorViewCreateFlagBitsARM(u64);
impl TensorViewCreateFlagBitsARM {
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    p_samplers: *const SamplerCreateInfo<'_>,
    p_descriptors: *const HostAddressRangeEXT<'_>,
) -> Result;
pub type PFN_vkWriteResourceDescriptorsEXT = unsafe extern "system" fn(
    device: Device,
    resource_count: u32,
    p_resources: *const ResourceDescriptorInfoEXT<'_>,
    p_descriptors: *const HostAddressRangeEXT<'_>,
) -> Result;
pub type PFN_vkCmdBindSamplerHeapEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_bind_info: *const BindHeapInfoEXT<'_>,
);
pub type PFN_vkCmdBindResourceHeapEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_bind_info: *const BindHeapInfoEXT<'_>,
);
pub type PFN_vkCmdPushDataEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_push_data_info: *const PushDataInfoEXT<'_>,
);
pub type PFN_vkRegisterCustomBorderColorEXT = unsafe extern "system" fn(
    device: Device,
    p_border_color: *const SamplerCustomBorderColorCreateInfoEXT<'_>,
    request_index: Bool32,
    p_index: *mut u32,
) -> Result;
pub type PFN_vkUnregisterCustomBorderColorEXT =
    unsafe extern "system" fn(device: Device, index: u32);
pub type PFN_vkGetImageOpaqueCaptureDataEXT = unsafe extern "system" fn(
    device: Device,
    image_count: u32,
    p_images: *const Image,
    p_datas: *mut HostAddressRangeEXT<'_>,
) -> Result;
pub type PFN_vkGetPhysicalDeviceDescriptorSizeEXT = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    descriptor_type: DescriptorType,
) -> DeviceSize;
pub type PFN_vkGetTensorOpaqueCaptureDataARM = unsafe extern "system" fn(
    device: Device,
    tensor_count: u32,
    p_tensors: *const TensorARM,
    p_datas: *mut HostAddressRangeEXT<'_>,
) -> Result;
