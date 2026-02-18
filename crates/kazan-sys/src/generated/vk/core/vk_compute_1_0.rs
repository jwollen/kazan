#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferView(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModule(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pipeline(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineLayout(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sampler(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSet(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayout(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPool(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Event(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCache(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WriteDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub p_image_info: *const DescriptorImageInfo,
    pub p_buffer_info: *const DescriptorBufferInfo,
    pub p_texel_buffer_view: *const BufferView,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyDescriptorSet {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BufferViewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModuleCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub p_code: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub p_immutable_samplers: *const Sampler,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub p_bindings: *const DescriptorSetLayoutBinding,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolSize {
    pub ty: DescriptorType,
    pub descriptor_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorPoolCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub p_pool_sizes: *const DescriptorPoolSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DescriptorSetAllocateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub p_map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub p_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputePipelineCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCacheCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub p_initial_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCacheHeaderVersionOne {
    pub header_size: u32,
    pub header_version: PipelineCacheHeaderVersion,
    pub vendor_id: u32,
    pub device_id: u32,
    pub pipeline_cache_uuid: [u8; UUID_SIZE as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub p_set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub p_push_constant_ranges: *const PushConstantRange,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SamplerCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EventCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: EventCreateFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DescriptorType(i32);
impl DescriptorType {
    pub const SAMPLER: Self = Self(0);
    pub const COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const SAMPLED_IMAGE: Self = Self(2);
    pub const STORAGE_IMAGE: Self = Self(3);
    pub const UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const UNIFORM_BUFFER: Self = Self(6);
    pub const STORAGE_BUFFER: Self = Self(7);
    pub const UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const INPUT_ATTACHMENT: Self = Self(10);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BorderColor(i32);
impl BorderColor {
    pub const FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const INT_OPAQUE_BLACK: Self = Self(3);
    pub const FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const INT_OPAQUE_WHITE: Self = Self(5);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineBindPoint(i32);
impl PipelineBindPoint {
    pub const GRAPHICS: Self = Self(0);
    pub const COMPUTE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCacheHeaderVersion(i32);
impl PipelineCacheHeaderVersion {
    pub const ONE: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Filter(i32);
impl Filter {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerMipmapMode(i32);
impl SamplerMipmapMode {
    pub const NEAREST: Self = Self(0);
    pub const LINEAR: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SamplerAddressMode(i32);
impl SamplerAddressMode {
    pub const REPEAT: Self = Self(0);
    pub const MIRRORED_REPEAT: Self = Self(1);
    pub const CLAMP_TO_EDGE: Self = Self(2);
    pub const CLAMP_TO_BORDER: Self = Self(3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SamplerCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineLayoutCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineCacheCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineShaderStageCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DescriptorSetLayoutCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct BufferViewCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ShaderStageFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ShaderModuleCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct EventCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct QueryPipelineStatisticFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DescriptorPoolCreateFlags: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DescriptorPoolResetFlags: Flags {
    }
}
pub type PFN_vkCreateEvent = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const EventCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_event: *mut Event,
) -> Result;
pub type PFN_vkDestroyEvent = unsafe extern "system" fn(
    device: Device,
    event: Event,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetEventStatus = unsafe extern "system" fn(device: Device, event: Event) -> Result;
pub type PFN_vkSetEvent = unsafe extern "system" fn(device: Device, event: Event) -> Result;
pub type PFN_vkResetEvent = unsafe extern "system" fn(device: Device, event: Event) -> Result;
pub type PFN_vkCreateBufferView = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const BufferViewCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_view: *mut BufferView,
) -> Result;
pub type PFN_vkDestroyBufferView = unsafe extern "system" fn(
    device: Device,
    buffer_view: BufferView,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateShaderModule = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_shader_module: *mut ShaderModule,
) -> Result;
pub type PFN_vkDestroyShaderModule = unsafe extern "system" fn(
    device: Device,
    shader_module: ShaderModule,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreatePipelineCache = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineCacheCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_cache: *mut PipelineCache,
) -> Result;
pub type PFN_vkDestroyPipelineCache = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetPipelineCacheData = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkMergePipelineCaches = unsafe extern "system" fn(
    device: Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    p_src_caches: *const PipelineCache,
) -> Result;
pub type PFN_vkCreateComputePipelines = unsafe extern "system" fn(
    device: Device,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const ComputePipelineCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
pub type PFN_vkDestroyPipeline = unsafe extern "system" fn(
    device: Device,
    pipeline: Pipeline,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreatePipelineLayout = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const PipelineLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_pipeline_layout: *mut PipelineLayout,
) -> Result;
pub type PFN_vkDestroyPipelineLayout = unsafe extern "system" fn(
    device: Device,
    pipeline_layout: PipelineLayout,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateSampler = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const SamplerCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_sampler: *mut Sampler,
) -> Result;
pub type PFN_vkDestroySampler = unsafe extern "system" fn(
    device: Device,
    sampler: Sampler,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorSetLayoutCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_set_layout: *mut DescriptorSetLayout,
) -> Result;
pub type PFN_vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: Device,
    descriptor_set_layout: DescriptorSetLayout,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateDescriptorPool = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DescriptorPoolCreateInfo,
    p_allocator: *const AllocationCallbacks,
    p_descriptor_pool: *mut DescriptorPool,
) -> Result;
pub type PFN_vkDestroyDescriptorPool = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkResetDescriptorPool = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> Result;
pub type PFN_vkAllocateDescriptorSets = unsafe extern "system" fn(
    device: Device,
    p_allocate_info: *const DescriptorSetAllocateInfo,
    p_descriptor_sets: *mut DescriptorSet,
) -> Result;
pub type PFN_vkFreeDescriptorSets = unsafe extern "system" fn(
    device: Device,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
) -> Result;
pub type PFN_vkUpdateDescriptorSets = unsafe extern "system" fn(
    device: Device,
    descriptor_write_count: u32,
    p_descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    p_descriptor_copies: *const CopyDescriptorSet,
);
pub type PFN_vkCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);
pub type PFN_vkCmdBindDescriptorSets = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    p_descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    p_dynamic_offsets: *const u32,
);
pub type PFN_vkCmdDispatch = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);
pub type PFN_vkCmdDispatchIndirect =
    unsafe extern "system" fn(command_buffer: CommandBuffer, buffer: Buffer, offset: DeviceSize);
pub type PFN_vkCmdClearColorImage = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    p_color: *const ClearColorValue,
    range_count: u32,
    p_ranges: *const ImageSubresourceRange,
);
pub type PFN_vkCmdSetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
pub type PFN_vkCmdResetEvent = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);
pub type PFN_vkCmdWaitEvents = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    event_count: u32,
    p_events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    p_memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    p_buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    p_image_memory_barriers: *const ImageMemoryBarrier,
);
pub type PFN_vkCmdPushConstants = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    p_values: *const c_void,
);
