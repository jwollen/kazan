#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_queue_family_data_graph_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM,
    get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: impl ExtendUninit<QueueFamilyDataGraphPropertiesARM>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                queue_family_data_graph_properties,
                |queue_family_data_graph_property_count, queue_family_data_graph_properties| {
                    (self.get_physical_device_queue_family_data_graph_properties_arm)(
                        physical_device,
                        queue_family_index,
                        queue_family_data_graph_property_count,
                        queue_family_data_graph_properties as _,
                    )
                },
            )
        }
    }
    pub unsafe fn get_physical_device_queue_family_data_graph_processing_engine_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_data_graph_processing_engine_info: &PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
        queue_family_data_graph_processing_engine_properties: &mut QueueFamilyDataGraphProcessingEnginePropertiesARM,
    ) {
        unsafe {
            (self.get_physical_device_queue_family_data_graph_processing_engine_properties_arm)(
                physical_device,
                queue_family_data_graph_processing_engine_info,
                queue_family_data_graph_processing_engine_properties,
            )
        }
    }
}
pub struct DeviceFn {
    create_data_graph_pipelines_arm: PFN_vkCreateDataGraphPipelinesARM,
    create_data_graph_pipeline_session_arm: PFN_vkCreateDataGraphPipelineSessionARM,
    get_data_graph_pipeline_session_bind_point_requirements_arm:
        PFN_vkGetDataGraphPipelineSessionBindPointRequirementsARM,
    get_data_graph_pipeline_session_memory_requirements_arm:
        PFN_vkGetDataGraphPipelineSessionMemoryRequirementsARM,
    bind_data_graph_pipeline_session_memory_arm: PFN_vkBindDataGraphPipelineSessionMemoryARM,
    destroy_data_graph_pipeline_session_arm: PFN_vkDestroyDataGraphPipelineSessionARM,
    cmd_dispatch_data_graph_arm: PFN_vkCmdDispatchDataGraphARM,
    get_data_graph_pipeline_available_properties_arm:
        PFN_vkGetDataGraphPipelineAvailablePropertiesARM,
    get_data_graph_pipeline_properties_arm: PFN_vkGetDataGraphPipelinePropertiesARM,
}
impl DeviceFn {
    pub unsafe fn create_data_graph_pipelines_arm(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[DataGraphPipelineCreateInfoARM],
        allocator: &AllocationCallbacks,
        pipelines: &mut [Pipeline],
    ) -> Result {
        unsafe {
            (self.create_data_graph_pipelines_arm)(
                device,
                deferred_operation,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator,
                pipelines.as_mut_ptr() as _,
            )
        }
    }
    pub unsafe fn create_data_graph_pipeline_session_arm(
        &self,
        device: Device,
        create_info: &DataGraphPipelineSessionCreateInfoARM,
        allocator: &AllocationCallbacks,
        session: &mut DataGraphPipelineSessionARM,
    ) -> Result {
        unsafe {
            (self.create_data_graph_pipeline_session_arm)(device, create_info, allocator, session)
        }
    }
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm(
        &self,
        device: Device,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirements: impl ExtendUninit<DataGraphPipelineSessionBindPointRequirementARM>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                bind_point_requirements,
                |bind_point_requirement_count, bind_point_requirements| {
                    (self.get_data_graph_pipeline_session_bind_point_requirements_arm)(
                        device,
                        info,
                        bind_point_requirement_count,
                        bind_point_requirements as _,
                    )
                },
            )
        }
    }
    pub unsafe fn get_data_graph_pipeline_session_memory_requirements_arm(
        &self,
        device: Device,
        info: &DataGraphPipelineSessionMemoryRequirementsInfoARM,
        memory_requirements: &mut MemoryRequirements2,
    ) {
        unsafe {
            (self.get_data_graph_pipeline_session_memory_requirements_arm)(
                device,
                info,
                memory_requirements,
            )
        }
    }
    pub unsafe fn bind_data_graph_pipeline_session_memory_arm(
        &self,
        device: Device,
        bind_infos: &[BindDataGraphPipelineSessionMemoryInfoARM],
    ) -> Result {
        unsafe {
            (self.bind_data_graph_pipeline_session_memory_arm)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn destroy_data_graph_pipeline_session_arm(
        &self,
        device: Device,
        session: DataGraphPipelineSessionARM,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_data_graph_pipeline_session_arm)(device, session, allocator) }
    }
    pub unsafe fn cmd_dispatch_data_graph_arm(
        &self,
        command_buffer: CommandBuffer,
        session: DataGraphPipelineSessionARM,
        info: &DataGraphPipelineDispatchInfoARM,
    ) {
        unsafe { (self.cmd_dispatch_data_graph_arm)(command_buffer, session, info) }
    }
    pub unsafe fn get_data_graph_pipeline_available_properties_arm(
        &self,
        device: Device,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: impl ExtendUninit<DataGraphPipelinePropertyARM>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |properties_count, properties| {
                (self.get_data_graph_pipeline_available_properties_arm)(
                    device,
                    pipeline_info,
                    properties_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_data_graph_pipeline_properties_arm(
        &self,
        device: Device,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: &mut [DataGraphPipelinePropertyQueryResultARM],
    ) -> Result {
        unsafe {
            (self.get_data_graph_pipeline_properties_arm)(
                device,
                pipeline_info,
                properties.len().try_into().unwrap(),
                properties.as_mut_ptr() as _,
            )
        }
    }
}
