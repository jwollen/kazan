#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct TensorViewARM(u64);
#[repr(C)]
pub struct TensorDescriptionARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tiling: TensorTilingARM,
    pub format: Format,
    pub dimension_count: u32,
    pub p_dimensions: *const i64,
    pub p_strides: *const i64,
    pub usage: TensorUsageFlagsARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TensorCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorCreateFlagsARM,
    pub p_description: *const TensorDescriptionARM<'a>,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TensorMemoryRequirementsInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BindTensorMemoryInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct WriteDescriptorSetTensorARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view_count: u32,
    pub p_tensor_views: *const TensorViewARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TensorFormatPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_tiling_tensor_features: FormatFeatureFlags2,
    pub linear_tiling_tensor_features: FormatFeatureFlags2,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceTensorPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_tensor_dimension_count: u32,
    pub max_tensor_elements: u64,
    pub max_per_dimension_tensor_elements: u64,
    pub max_tensor_stride: i64,
    pub max_tensor_size: u64,
    pub max_tensor_shader_access_array_length: u32,
    pub max_tensor_shader_access_size: u32,
    pub max_descriptor_set_storage_tensors: u32,
    pub max_per_stage_descriptor_set_storage_tensors: u32,
    pub max_descriptor_set_update_after_bind_storage_tensors: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_tensors: u32,
    pub shader_storage_tensor_array_non_uniform_indexing_native: Bool32,
    pub shader_tensor_supported_stages: ShaderStageFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TensorMemoryBarrierARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub tensor: TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TensorDependencyInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_memory_barrier_count: u32,
    pub p_tensor_memory_barriers: *const TensorMemoryBarrierARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceTensorFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_non_packed: Bool32,
    pub shader_tensor_access: Bool32,
    pub shader_storage_tensor_array_dynamic_indexing: Bool32,
    pub shader_storage_tensor_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_storage_tensor_update_after_bind: Bool32,
    pub tensors: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DeviceTensorMemoryRequirementsARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const TensorCreateInfoARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CopyTensorInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_tensor: TensorARM,
    pub dst_tensor: TensorARM,
    pub region_count: u32,
    pub p_regions: *const TensorCopyARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TensorCopyARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dimension_count: u32,
    pub p_src_offset: *const u64,
    pub p_dst_offset: *const u64,
    pub p_extent: *const u64,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct MemoryDedicatedAllocateInfoTensorARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferTensorPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_capture_replay_descriptor_data_size: usize,
    pub tensor_view_capture_replay_descriptor_data_size: usize,
    pub tensor_descriptor_size: usize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceDescriptorBufferTensorFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_buffer_tensor_descriptors: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TensorCaptureDescriptorDataInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct TensorViewCaptureDescriptorDataInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view: TensorViewARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DescriptorGetTensorInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view: TensorViewARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct FrameBoundaryTensorsARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_count: u32,
    pub p_tensors: *const TensorARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceExternalTensorInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorCreateFlagsARM,
    pub p_description: *const TensorDescriptionARM<'a>,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExternalTensorPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_memory_properties: ExternalMemoryProperties,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ExternalMemoryTensorCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorTilingARM(i32);
impl TensorTilingARM {
    pub const OPTIMAL_ARM: Self = Self(0);
    pub const LINEAR_ARM: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TensorCreateFlagsARM: Flags64 {
        const MUTABLE_FORMAT_ARM = TensorCreateFlagBitsARM::MUTABLE_FORMAT_ARM.0;
        const PROTECTED_ARM = TensorCreateFlagBitsARM::PROTECTED_ARM.0;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = TensorCreateFlagBitsARM::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM.0;
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM = TensorCreateFlagBitsARM::DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorCreateFlagBitsARM(u64);
impl TensorCreateFlagBitsARM {
    pub const MUTABLE_FORMAT_ARM: Self = Self(1 << 0);
    pub const PROTECTED_ARM: Self = Self(1 << 1);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1 << 2);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TensorUsageFlagsARM: Flags64 {
        const SHADER_ARM = TensorUsageFlagBitsARM::SHADER_ARM.0;
        const TRANSFER_SRC_ARM = TensorUsageFlagBitsARM::TRANSFER_SRC_ARM.0;
        const TRANSFER_DST_ARM = TensorUsageFlagBitsARM::TRANSFER_DST_ARM.0;
        const IMAGE_ALIASING_ARM = TensorUsageFlagBitsARM::IMAGE_ALIASING_ARM.0;
        const DATA_GRAPH_ARM = TensorUsageFlagBitsARM::DATA_GRAPH_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorUsageFlagBitsARM(u64);
impl TensorUsageFlagBitsARM {
    pub const SHADER_ARM: Self = Self(1 << 1);
    pub const TRANSFER_SRC_ARM: Self = Self(1 << 2);
    pub const TRANSFER_DST_ARM: Self = Self(1 << 3);
    pub const IMAGE_ALIASING_ARM: Self = Self(1 << 4);
    pub const DATA_GRAPH_ARM: Self = Self(1 << 5);
}
pub type PFN_vkCreateTensorARM = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const TensorCreateInfoARM<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_tensor: *mut TensorARM,
) -> Result;
pub type PFN_vkDestroyTensorARM = unsafe extern "system" fn(
    device: Device,
    tensor: TensorARM,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateTensorViewARM = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const TensorViewCreateInfoARM<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_view: *mut TensorViewARM,
) -> Result;
pub type PFN_vkDestroyTensorViewARM = unsafe extern "system" fn(
    device: Device,
    tensor_view: TensorViewARM,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorMemoryRequirementsInfoARM<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkBindTensorMemoryARM = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindTensorMemoryInfoARM<'_>,
) -> Result;
pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceTensorMemoryRequirementsARM<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkCmdCopyTensorARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_tensor_info: *const CopyTensorInfoARM<'_>,
);
pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorCaptureDescriptorDataInfoARM<'_>,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorViewCaptureDescriptorDataInfoARM<'_>,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM<'_>,
    p_external_tensor_properties: *mut ExternalTensorPropertiesARM<'_>,
);
