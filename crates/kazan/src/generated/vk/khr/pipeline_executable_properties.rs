#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_pipeline_executable_properties_khr: PFN_vkGetPipelineExecutablePropertiesKHR,
    get_pipeline_executable_statistics_khr: PFN_vkGetPipelineExecutableStatisticsKHR,
    get_pipeline_executable_internal_representations_khr:
        PFN_vkGetPipelineExecutableInternalRepresentationsKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_pipeline_executable_properties_khr: transmute(
                    load(c"vkGetPipelineExecutablePropertiesKHR").ok_or(LoadingError)?,
                ),
                get_pipeline_executable_statistics_khr: transmute(
                    load(c"vkGetPipelineExecutableStatisticsKHR").ok_or(LoadingError)?,
                ),
                get_pipeline_executable_internal_representations_khr: transmute(
                    load(c"vkGetPipelineExecutableInternalRepresentationsKHR")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_pipeline_executable_properties_khr<'a>(
        &self,
        device: Device,
        pipeline_info: &PipelineInfoKHR<'_>,
        properties: impl ExtendUninit<PipelineExecutablePropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(properties, |executable_count, properties| {
                let result = (self.get_pipeline_executable_properties_khr)(
                    device,
                    pipeline_info,
                    executable_count,
                    properties as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_pipeline_executable_statistics_khr<'a>(
        &self,
        device: Device,
        executable_info: &PipelineExecutableInfoKHR<'_>,
        statistics: impl ExtendUninit<PipelineExecutableStatisticKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(statistics, |statistic_count, statistics| {
                let result = (self.get_pipeline_executable_statistics_khr)(
                    device,
                    executable_info,
                    statistic_count,
                    statistics as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn get_pipeline_executable_internal_representations_khr<'a>(
        &self,
        device: Device,
        executable_info: &PipelineExecutableInfoKHR<'_>,
        internal_representations: impl ExtendUninit<PipelineExecutableInternalRepresentationKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                internal_representations,
                |internal_representation_count, internal_representations| {
                    let result = (self.get_pipeline_executable_internal_representations_khr)(
                        device,
                        executable_info,
                        internal_representation_count,
                        internal_representations as _,
                    );

                    match result {
                        VkResult::SUCCESS => Ok(()),
                        VkResult::INCOMPLETE => Ok(()),
                        err => Err(err),
                    }
                },
            )
        }
    }
}
