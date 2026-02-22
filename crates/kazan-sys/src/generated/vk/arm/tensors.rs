#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorViewARM(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorDescriptionARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tiling: TensorTilingARM,
    pub format: Format,
    pub dimension_count: u32,
    pub p_dimensions: *const i64,
    pub p_strides: *const i64,
    pub usage: TensorUsageFlagsARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorCreateFlagsARM,
    pub p_description: *const TensorDescriptionARM,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub p_queue_family_indices: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorMemoryRequirementsInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindTensorMemoryInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSetTensorARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view_count: u32,
    pub p_tensor_views: *const TensorViewARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorFormatPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub optimal_tiling_tensor_features: FormatFeatureFlags2,
    pub linear_tiling_tensor_features: FormatFeatureFlags2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTensorPropertiesARM {
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
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorMemoryBarrierARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub tensor: TensorARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorDependencyInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_memory_barrier_count: u32,
    pub p_tensor_memory_barriers: *const TensorMemoryBarrierARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceTensorFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_non_packed: Bool32,
    pub shader_tensor_access: Bool32,
    pub shader_storage_tensor_array_dynamic_indexing: Bool32,
    pub shader_storage_tensor_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_storage_tensor_update_after_bind: Bool32,
    pub tensors: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceTensorMemoryRequirementsARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_create_info: *const TensorCreateInfoARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyTensorInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_tensor: TensorARM,
    pub dst_tensor: TensorARM,
    pub region_count: u32,
    pub p_regions: *const TensorCopyARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorCopyARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dimension_count: u32,
    pub p_src_offset: *const u64,
    pub p_dst_offset: *const u64,
    pub p_extent: *const u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDedicatedAllocateInfoTensorARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorBufferTensorPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub tensor_capture_replay_descriptor_data_size: usize,
    pub tensor_view_capture_replay_descriptor_data_size: usize,
    pub tensor_descriptor_size: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDescriptorBufferTensorFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub descriptor_buffer_tensor_descriptors: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorCaptureDescriptorDataInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor: TensorARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TensorViewCaptureDescriptorDataInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view: TensorViewARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorGetTensorInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_view: TensorViewARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameBoundaryTensorsARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub tensor_count: u32,
    pub p_tensors: *const TensorARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExternalTensorInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: TensorCreateFlagsARM,
    pub p_description: *const TensorDescriptionARM,
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalTensorPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub external_memory_properties: ExternalMemoryProperties,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryTensorCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorTilingARM(i32);
impl TensorTilingARM {
    pub const OPTIMAL_ARM: Self = Self(0);
    pub const LINEAR_ARM: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct TensorCreateFlagsARM: Flags64 {
        const MUTABLE_FORMAT_ARM = TensorCreateFlagBitsARM::MUTABLE_FORMAT_ARM.0;
        const PROTECTED_ARM = TensorCreateFlagBitsARM::PROTECTED_ARM.0;
        const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM = TensorCreateFlagBitsARM::DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM.0;
        const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM = TensorCreateFlagBitsARM::DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TensorCreateFlagBitsARM(u64);
impl TensorCreateFlagBitsARM {
    pub const MUTABLE_FORMAT_ARM: Self = Self(1 << 0);
    pub const PROTECTED_ARM: Self = Self(1 << 1);
    pub const DESCRIPTOR_BUFFER_CAPTURE_REPLAY_ARM: Self = Self(1 << 2);
    pub const DESCRIPTOR_HEAP_CAPTURE_REPLAY_ARM: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct TensorUsageFlagsARM: Flags64 {
        const SHADER_ARM = TensorUsageFlagBitsARM::SHADER_ARM.0;
        const TRANSFER_SRC_ARM = TensorUsageFlagBitsARM::TRANSFER_SRC_ARM.0;
        const TRANSFER_DST_ARM = TensorUsageFlagBitsARM::TRANSFER_DST_ARM.0;
        const IMAGE_ALIASING_ARM = TensorUsageFlagBitsARM::IMAGE_ALIASING_ARM.0;
        const DATA_GRAPH_ARM = TensorUsageFlagBitsARM::DATA_GRAPH_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    p_create_info: *const TensorCreateInfoARM,
    p_allocator: *const AllocationCallbacks,
    p_tensor: *mut TensorARM,
) -> Result;
pub type PFN_vkDestroyTensorARM = unsafe extern "system" fn(
    device: Device,
    tensor: TensorARM,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateTensorViewARM = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const TensorViewCreateInfoARM,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut TensorViewARM,
) -> Result;
pub type PFN_vkDestroyTensorViewARM = unsafe extern "system" fn(
    device: Device,
    tensor_view: TensorViewARM,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorMemoryRequirementsInfoARM,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkBindTensorMemoryARM = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindTensorMemoryInfoARM,
) -> Result;
pub type PFN_vkGetDeviceTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const DeviceTensorMemoryRequirementsARM,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkCmdCopyTensorARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_copy_tensor_info: *const CopyTensorInfoARM,
);
pub type PFN_vkGetTensorOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorCaptureDescriptorDataInfoARM,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetTensorViewOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const TensorViewCaptureDescriptorDataInfoARM,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkGetPhysicalDeviceExternalTensorPropertiesARM = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM,
    p_external_tensor_properties: *mut ExternalTensorPropertiesARM,
);
