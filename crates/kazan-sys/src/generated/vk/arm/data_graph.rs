#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DataGraphPipelineSessionARM(u64);
#[repr(C)]
pub struct PhysicalDeviceDataGraphFeaturesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph: Bool32,
    pub data_graph_update_after_bind: Bool32,
    pub data_graph_specialization_constants: Bool32,
    pub data_graph_descriptor_buffer: Bool32,
    pub data_graph_shader_module: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dimension: u32,
    pub zero_count: u32,
    pub group_size: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineConstantARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub id: u32,
    pub p_constant_data: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineResourceInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set: u32,
    pub binding: u32,
    pub array_element: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineCompilerControlCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_vendor_options: *const c_char,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags2KHR,
    pub layout: PipelineLayout,
    pub resource_info_count: u32,
    pub p_resource_infos: *const DataGraphPipelineResourceInfoARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineShaderModuleCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo<'a>,
    pub constant_count: u32,
    pub p_constants: *const DataGraphPipelineConstantARM<'a>,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineSessionCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DataGraphPipelineSessionCreateFlagsARM,
    pub data_graph_pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineSessionBindPointRequirementsInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineSessionBindPointRequirementARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub bind_point_type: DataGraphPipelineSessionBindPointTypeARM,
    pub num_objects: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineSessionMemoryRequirementsInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub object_index: u32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct BindDataGraphPipelineSessionMemoryInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub object_index: u32,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_graph_pipeline: Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelinePropertyQueryResultARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub property: DataGraphPipelinePropertyARM,
    pub is_text: Bool32,
    pub data_size: usize,
    pub p_data: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineIdentifierCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphPipelineDispatchInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DataGraphPipelineDispatchFlagsARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceDataGraphProcessingEngineARM {
    pub ty: PhysicalDeviceDataGraphProcessingEngineTypeARM,
    pub is_foreign: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceDataGraphOperationSupportARM {
    pub operation_type: PhysicalDeviceDataGraphOperationTypeARM,
    pub name: [c_char; MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM as usize],
    pub version: u32,
}
#[repr(C)]
pub struct QueueFamilyDataGraphPropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub engine: PhysicalDeviceDataGraphProcessingEngineARM,
    pub operation: PhysicalDeviceDataGraphOperationSupportARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub engine_type: PhysicalDeviceDataGraphProcessingEngineTypeARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct QueueFamilyDataGraphProcessingEnginePropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub foreign_semaphore_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub foreign_memory_handle_types: ExternalMemoryHandleTypeFlags,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DataGraphProcessingEngineCreateInfoARM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub processing_engine_count: u32,
    pub p_processing_engines: *mut PhysicalDeviceDataGraphProcessingEngineARM,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineSessionBindPointARM(i32);
impl DataGraphPipelineSessionBindPointARM {
    pub const TRANSIENT_ARM: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineSessionBindPointTypeARM(i32);
impl DataGraphPipelineSessionBindPointTypeARM {
    pub const MEMORY_ARM: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelinePropertyARM(i32);
impl DataGraphPipelinePropertyARM {
    pub const CREATION_LOG_ARM: Self = Self(0);
    pub const IDENTIFIER_ARM: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceDataGraphProcessingEngineTypeARM(i32);
impl PhysicalDeviceDataGraphProcessingEngineTypeARM {
    pub const DEFAULT_ARM: Self = Self(0);
    pub const COMPUTE_QCOM: Self = Self(1000629001);
    pub const NEURAL_QCOM: Self = Self(1000629000);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceDataGraphOperationTypeARM(i32);
impl PhysicalDeviceDataGraphOperationTypeARM {
    pub const SPIRV_EXTENDED_INSTRUCTION_SET_ARM: Self = Self(0);
    pub const BUILTIN_MODEL_QCOM: Self = Self(1000629001);
    pub const NEURAL_MODEL_QCOM: Self = Self(1000629000);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphPipelineSessionCreateFlagsARM: Flags64 {
        const PROTECTED_ARM = DataGraphPipelineSessionCreateFlagBitsARM::PROTECTED_ARM.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineSessionCreateFlagBitsARM(u64);
impl DataGraphPipelineSessionCreateFlagBitsARM {
    pub const PROTECTED_ARM: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DataGraphPipelineDispatchFlagsARM: Flags64 {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineDispatchFlagBitsARM(u64);
impl DataGraphPipelineDispatchFlagBitsARM {}
pub type PFN_vkCreateDataGraphPipelinesARM = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const DataGraphPipelineCreateInfoARM<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_pipelines: *mut Pipeline,
) -> Result;
pub type PFN_vkCreateDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DataGraphPipelineSessionCreateInfoARM<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_session: *mut DataGraphPipelineSessionARM,
) -> Result;
pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM =
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM<'_>,
        p_bind_point_requirement_count: *mut u32,
        p_bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM<'_>,
    ) -> Result;
pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM<'_>,
    p_memory_requirements: *mut MemoryRequirements2<'_>,
);
pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM<'_>,
) -> Result;
pub type PFN_vkDestroyDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: Device,
    session: DataGraphPipelineSessionARM,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCmdDispatchDataGraphARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    session: DataGraphPipelineSessionARM,
    p_info: *const DataGraphPipelineDispatchInfoARM<'_>,
);
pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
    p_properties_count: *mut u32,
    p_properties: *mut DataGraphPipelinePropertyARM,
) -> Result;
pub type PFN_vkGetDataGraphPipelinePropertiesARM = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const DataGraphPipelineInfoARM<'_>,
    properties_count: u32,
    p_properties: *mut DataGraphPipelinePropertyQueryResultARM<'_>,
) -> Result;
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_queue_family_data_graph_property_count: *mut u32,
        p_queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM<'_>,
    ) -> Result;
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM<'_>,
    p_queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>,
);
