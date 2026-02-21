#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
    get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
    get_pipeline_executable_internal_representations_khr:
        PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
}
impl DeviceFn {
    pub unsafe fn get_pipeline_executable_properties_khr(
        &self,
        device: Device,
        pipeline_info: &PipelineInfoKHR,
        properties: impl ExtendUninit<PipelineExecutablePropertiesKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(properties, |executable_count, properties| {
                (self.get_pipeline_executable_properties_khr)(
                    device,
                    pipeline_info,
                    executable_count,
                    properties as _,
                )
            })
        }
    }
    pub unsafe fn get_pipeline_executable_statistics_khr(
        &self,
        device: Device,
        executable_info: &PipelineExecutableInfoKHR,
        statistics: impl ExtendUninit<PipelineExecutableStatisticKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(statistics, |statistic_count, statistics| {
                (self.get_pipeline_executable_statistics_khr)(
                    device,
                    executable_info,
                    statistic_count,
                    statistics as _,
                )
            })
        }
    }
    pub unsafe fn get_pipeline_executable_internal_representations_khr(
        &self,
        device: Device,
        executable_info: &PipelineExecutableInfoKHR,
        internal_representations: impl ExtendUninit<PipelineExecutableInternalRepresentationKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                internal_representations,
                |internal_representation_count, internal_representations| {
                    (self.get_pipeline_executable_internal_representations_khr)(
                        device,
                        executable_info,
                        internal_representation_count,
                        internal_representations as _,
                    )
                },
            )
        }
    }
}
