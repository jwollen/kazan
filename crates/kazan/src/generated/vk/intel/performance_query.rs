#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    initialize_performance_api_intel: PFN_vkInitializePerformanceApiINTEL,
    uninitialize_performance_api_intel: PFN_vkUninitializePerformanceApiINTEL,
    cmd_set_performance_marker_intel: PFN_vkCmdSetPerformanceMarkerINTEL,
    cmd_set_performance_stream_marker_intel: PFN_vkCmdSetPerformanceStreamMarkerINTEL,
    cmd_set_performance_override_intel: PFN_vkCmdSetPerformanceOverrideINTEL,
    acquire_performance_configuration_intel: PFN_vkAcquirePerformanceConfigurationINTEL,
    release_performance_configuration_intel: PFN_vkReleasePerformanceConfigurationINTEL,
    queue_set_performance_configuration_intel: PFN_vkQueueSetPerformanceConfigurationINTEL,
    get_performance_parameter_intel: PFN_vkGetPerformanceParameterINTEL,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                initialize_performance_api_intel: transmute(
                    load(c"vkInitializePerformanceApiINTEL").ok_or(LoadingError)?,
                ),
                uninitialize_performance_api_intel: transmute(
                    load(c"vkUninitializePerformanceApiINTEL").ok_or(LoadingError)?,
                ),
                cmd_set_performance_marker_intel: transmute(
                    load(c"vkCmdSetPerformanceMarkerINTEL").ok_or(LoadingError)?,
                ),
                cmd_set_performance_stream_marker_intel: transmute(
                    load(c"vkCmdSetPerformanceStreamMarkerINTEL").ok_or(LoadingError)?,
                ),
                cmd_set_performance_override_intel: transmute(
                    load(c"vkCmdSetPerformanceOverrideINTEL").ok_or(LoadingError)?,
                ),
                acquire_performance_configuration_intel: transmute(
                    load(c"vkAcquirePerformanceConfigurationINTEL").ok_or(LoadingError)?,
                ),
                release_performance_configuration_intel: transmute(
                    load(c"vkReleasePerformanceConfigurationINTEL").ok_or(LoadingError)?,
                ),
                queue_set_performance_configuration_intel: transmute(
                    load(c"vkQueueSetPerformanceConfigurationINTEL").ok_or(LoadingError)?,
                ),
                get_performance_parameter_intel: transmute(
                    load(c"vkGetPerformanceParameterINTEL").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn initialize_performance_api_intel(
        &self,
        device: Device,
        initialize_info: &InitializePerformanceApiInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.initialize_performance_api_intel)(device, initialize_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn uninitialize_performance_api_intel(&self, device: Device) {
        unsafe { (self.uninitialize_performance_api_intel)(device) }
    }
    pub unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &PerformanceMarkerInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.cmd_set_performance_marker_intel)(command_buffer, marker_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &PerformanceStreamMarkerInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.cmd_set_performance_stream_marker_intel)(command_buffer, marker_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: CommandBuffer,
        override_info: &PerformanceOverrideInfoINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.cmd_set_performance_override_intel)(command_buffer, override_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn acquire_performance_configuration_intel(
        &self,
        device: Device,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    ) -> crate::Result<PerformanceConfigurationINTEL> {
        unsafe {
            let mut configuration = core::mem::MaybeUninit::uninit();
            let result = (self.acquire_performance_configuration_intel)(
                device,
                acquire_info,
                configuration.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(configuration.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn release_performance_configuration_intel(
        &self,
        device: Device,
        configuration: PerformanceConfigurationINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.release_performance_configuration_intel)(device, configuration);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.queue_set_performance_configuration_intel)(queue, configuration);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_performance_parameter_intel(
        &self,
        device: Device,
        parameter: PerformanceParameterTypeINTEL,
    ) -> crate::Result<PerformanceValueINTEL> {
        unsafe {
            let mut value = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_performance_parameter_intel)(device, parameter, value.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(value.assume_init()),
                err => Err(err),
            }
        }
    }
}
