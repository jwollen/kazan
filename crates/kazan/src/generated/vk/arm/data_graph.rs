#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_queue_family_data_graph_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM,
    get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
        PFN_vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_queue_family_data_graph_properties_arm: transmute(
                    load(c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM")
                        .ok_or(LoadingError)?,
                ),
                get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
                    transmute(
                        load(
                            c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM",
                        )
                        .ok_or(LoadingError)?,
                    ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_queue_family_data_graph_properties_arm(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_properties: impl ExtendUninit<QueueFamilyDataGraphPropertiesARM>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                queue_family_data_graph_properties,
                |queue_family_data_graph_property_count, queue_family_data_graph_properties| {
                    result((self
                        .get_physical_device_queue_family_data_graph_properties_arm)(
                        physical_device,
                        queue_family_index,
                        queue_family_data_graph_property_count,
                        queue_family_data_graph_properties as _,
                    ))
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
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_data_graph_pipelines_arm: transmute(
                    load(c"vkCreateDataGraphPipelinesARM").ok_or(LoadingError)?,
                ),
                create_data_graph_pipeline_session_arm: transmute(
                    load(c"vkCreateDataGraphPipelineSessionARM").ok_or(LoadingError)?,
                ),
                get_data_graph_pipeline_session_bind_point_requirements_arm: transmute(
                    load(c"vkGetDataGraphPipelineSessionBindPointRequirementsARM")
                        .ok_or(LoadingError)?,
                ),
                get_data_graph_pipeline_session_memory_requirements_arm: transmute(
                    load(c"vkGetDataGraphPipelineSessionMemoryRequirementsARM")
                        .ok_or(LoadingError)?,
                ),
                bind_data_graph_pipeline_session_memory_arm: transmute(
                    load(c"vkBindDataGraphPipelineSessionMemoryARM").ok_or(LoadingError)?,
                ),
                destroy_data_graph_pipeline_session_arm: transmute(
                    load(c"vkDestroyDataGraphPipelineSessionARM").ok_or(LoadingError)?,
                ),
                cmd_dispatch_data_graph_arm: transmute(
                    load(c"vkCmdDispatchDataGraphARM").ok_or(LoadingError)?,
                ),
                get_data_graph_pipeline_available_properties_arm: transmute(
                    load(c"vkGetDataGraphPipelineAvailablePropertiesARM").ok_or(LoadingError)?,
                ),
                get_data_graph_pipeline_properties_arm: transmute(
                    load(c"vkGetDataGraphPipelinePropertiesARM").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_data_graph_pipelines_arm(
        &self,
        device: Device,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[DataGraphPipelineCreateInfoARM],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> crate::Result<()> {
        unsafe {
            result((self.create_data_graph_pipelines_arm)(
                device,
                deferred_operation,
                pipeline_cache,
                create_infos.len().try_into().unwrap(),
                create_infos.as_ptr() as _,
                allocator.to_raw_ptr(),
                pipelines.as_mut_ptr() as _,
            ))
        }
    }
    pub unsafe fn create_data_graph_pipeline_session_arm(
        &self,
        device: Device,
        create_info: &DataGraphPipelineSessionCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
        session: &mut DataGraphPipelineSessionARM,
    ) -> crate::Result<()> {
        unsafe {
            result((self.create_data_graph_pipeline_session_arm)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                session,
            ))
        }
    }
    pub unsafe fn get_data_graph_pipeline_session_bind_point_requirements_arm(
        &self,
        device: Device,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirements: impl ExtendUninit<DataGraphPipelineSessionBindPointRequirementARM>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                bind_point_requirements,
                |bind_point_requirement_count, bind_point_requirements| {
                    result((self
                        .get_data_graph_pipeline_session_bind_point_requirements_arm)(
                        device,
                        info,
                        bind_point_requirement_count,
                        bind_point_requirements as _,
                    ))
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
    ) -> crate::Result<()> {
        unsafe {
            result((self.bind_data_graph_pipeline_session_memory_arm)(
                device,
                bind_infos.len().try_into().unwrap(),
                bind_infos.as_ptr() as _,
            ))
        }
    }
    pub unsafe fn destroy_data_graph_pipeline_session_arm(
        &self,
        device: Device,
        session: DataGraphPipelineSessionARM,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.destroy_data_graph_pipeline_session_arm)(device, session, allocator.to_raw_ptr())
        }
    }
    pub unsafe fn cmd_dispatch_data_graph_arm(
        &self,
        command_buffer: CommandBuffer,
        session: DataGraphPipelineSessionARM,
        info: Option<&DataGraphPipelineDispatchInfoARM>,
    ) {
        unsafe { (self.cmd_dispatch_data_graph_arm)(command_buffer, session, info.to_raw_ptr()) }
    }
    pub unsafe fn get_data_graph_pipeline_available_properties_arm(
        &self,
        device: Device,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: impl ExtendUninit<DataGraphPipelinePropertyARM>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |properties_count, properties| {
                result((self.get_data_graph_pipeline_available_properties_arm)(
                    device,
                    pipeline_info,
                    properties_count,
                    properties as _,
                ))
            })
        }
    }
    pub unsafe fn get_data_graph_pipeline_properties_arm(
        &self,
        device: Device,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: &mut [DataGraphPipelinePropertyQueryResultARM],
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_data_graph_pipeline_properties_arm)(
                device,
                pipeline_info,
                properties.len().try_into().unwrap(),
                properties.as_mut_ptr() as _,
            ))
        }
    }
}
