#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineSessionARM(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDataGraphFeaturesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub data_graph: Bool32,
    pub data_graph_update_after_bind: Bool32,
    pub data_graph_specialization_constants: Bool32,
    pub data_graph_descriptor_buffer: Bool32,
    pub data_graph_shader_module: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dimension: u32,
    pub zero_count: u32,
    pub group_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineConstantARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub id: u32,
    pub p_constant_data: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineResourceInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub descriptor_set: u32,
    pub binding: u32,
    pub array_element: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineCompilerControlCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_vendor_options: *const c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineCreateFlags2KHR,
    pub layout: PipelineLayout,
    pub resource_info_count: u32,
    pub p_resource_infos: *const DataGraphPipelineResourceInfoARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineShaderModuleCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub module: ShaderModule,
    pub p_name: *const c_char,
    pub p_specialization_info: *const SpecializationInfo,
    pub constant_count: u32,
    pub p_constants: *const DataGraphPipelineConstantARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineSessionCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DataGraphPipelineSessionCreateFlagsARM,
    pub data_graph_pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineSessionBindPointRequirementsInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineSessionBindPointRequirementARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub bind_point_type: DataGraphPipelineSessionBindPointTypeARM,
    pub num_objects: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineSessionMemoryRequirementsInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub object_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindDataGraphPipelineSessionMemoryInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub session: DataGraphPipelineSessionARM,
    pub bind_point: DataGraphPipelineSessionBindPointARM,
    pub object_index: u32,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub data_graph_pipeline: Pipeline,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelinePropertyQueryResultARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub property: DataGraphPipelinePropertyARM,
    pub is_text: Bool32,
    pub data_size: usize,
    pub p_data: *mut c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineIdentifierCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphPipelineDispatchInfoARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: DataGraphPipelineDispatchFlagsARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDataGraphProcessingEngineARM {
    pub ty: PhysicalDeviceDataGraphProcessingEngineTypeARM,
    pub is_foreign: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDataGraphOperationSupportARM {
    pub operation_type: PhysicalDeviceDataGraphOperationTypeARM,
    pub name: [c_char; MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM as usize],
    pub version: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyDataGraphPropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub engine: PhysicalDeviceDataGraphProcessingEngineARM,
    pub operation: PhysicalDeviceDataGraphOperationSupportARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub engine_type: PhysicalDeviceDataGraphProcessingEngineTypeARM,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyDataGraphProcessingEnginePropertiesARM {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub foreign_semaphore_handle_types: ExternalSemaphoreHandleTypeFlags,
    pub foreign_memory_handle_types: ExternalMemoryHandleTypeFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DataGraphProcessingEngineCreateInfoARM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub processing_engine_count: u32,
    pub p_processing_engines: *mut PhysicalDeviceDataGraphProcessingEngineARM,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineSessionBindPointARM(i32);
impl DataGraphPipelineSessionBindPointARM {
    pub const TRANSIENT_ARM: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelineSessionBindPointTypeARM(i32);
impl DataGraphPipelineSessionBindPointTypeARM {
    pub const MEMORY_ARM: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DataGraphPipelinePropertyARM(i32);
impl DataGraphPipelinePropertyARM {
    pub const CREATION_LOG_ARM: Self = Self(0);
    pub const IDENTIFIER_ARM: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceDataGraphProcessingEngineTypeARM(i32);
impl PhysicalDeviceDataGraphProcessingEngineTypeARM {
    pub const DEFAULT_ARM: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDeviceDataGraphOperationTypeARM(i32);
impl PhysicalDeviceDataGraphOperationTypeARM {
    pub const SPIRV_EXTENDED_INSTRUCTION_SET_ARM: Self = Self(0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DataGraphPipelineSessionCreateFlagsARM: Flags64 {
        const PROTECTED_ARM = 1 << 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DataGraphPipelineDispatchFlagsARM: Flags64 {
    }
}
pub type PFN_vkCreateDataGraphPipelinesARM = unsafe extern "system" fn(
    device: Device,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    p_create_infos: *const DataGraphPipelineCreateInfoARM,
    p_allocator: *const AllocationCallbacks,
    p_pipelines: *mut Pipeline,
) -> Result;
pub type PFN_vkCreateDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const DataGraphPipelineSessionCreateInfoARM,
    p_allocator: *const AllocationCallbacks,
    p_session: *mut DataGraphPipelineSessionARM,
) -> Result;
pub type PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM =
    unsafe extern "system" fn(
        device: Device,
        p_info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM,
        p_bind_point_requirement_count: *mut u32,
        p_bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM,
    ) -> Result;
pub type PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM = unsafe extern "system" fn(
    device: Device,
    p_info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM,
    p_memory_requirements: *mut MemoryRequirements2,
);
pub type PFN_vkBindDataGraphPipelineSessionMemoryARM = unsafe extern "system" fn(
    device: Device,
    bind_info_count: u32,
    p_bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM,
) -> Result;
pub type PFN_vkDestroyDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: Device,
    session: DataGraphPipelineSessionARM,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCmdDispatchDataGraphARM = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    session: DataGraphPipelineSessionARM,
    p_info: *const DataGraphPipelineDispatchInfoARM,
);
pub type PFN_vkGetDataGraphPipelineAvailablePropertiesARM = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const DataGraphPipelineInfoARM,
    p_properties_count: *mut u32,
    p_properties: *mut DataGraphPipelinePropertyARM,
) -> Result;
pub type PFN_vkGetDataGraphPipelinePropertiesARM = unsafe extern "system" fn(
    device: Device,
    p_pipeline_info: *const DataGraphPipelineInfoARM,
    properties_count: u32,
    p_properties: *mut DataGraphPipelinePropertyQueryResultARM,
) -> Result;
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_queue_family_data_graph_property_count: *mut u32,
        p_queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM,
    ) -> Result;
pub type PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
    p_queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM,
);
