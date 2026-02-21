#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
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
    pub unsafe fn initialize_performance_api_intel(
        &self,
        device: Device,
        initialize_info: &InitializePerformanceApiInfoINTEL,
    ) -> Result {
        unsafe { (self.initialize_performance_api_intel)(device, initialize_info) }
    }
    pub unsafe fn uninitialize_performance_api_intel(&self, device: Device) {
        unsafe { (self.uninitialize_performance_api_intel)(device) }
    }
    pub unsafe fn cmd_set_performance_marker_intel(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &PerformanceMarkerInfoINTEL,
    ) -> Result {
        unsafe { (self.cmd_set_performance_marker_intel)(command_buffer, marker_info) }
    }
    pub unsafe fn cmd_set_performance_stream_marker_intel(
        &self,
        command_buffer: CommandBuffer,
        marker_info: &PerformanceStreamMarkerInfoINTEL,
    ) -> Result {
        unsafe { (self.cmd_set_performance_stream_marker_intel)(command_buffer, marker_info) }
    }
    pub unsafe fn cmd_set_performance_override_intel(
        &self,
        command_buffer: CommandBuffer,
        override_info: &PerformanceOverrideInfoINTEL,
    ) -> Result {
        unsafe { (self.cmd_set_performance_override_intel)(command_buffer, override_info) }
    }
    pub unsafe fn acquire_performance_configuration_intel(
        &self,
        device: Device,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
        configuration: &mut PerformanceConfigurationINTEL,
    ) -> Result {
        unsafe {
            (self.acquire_performance_configuration_intel)(device, acquire_info, configuration)
        }
    }
    pub unsafe fn release_performance_configuration_intel(
        &self,
        device: Device,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result {
        unsafe { (self.release_performance_configuration_intel)(device, configuration) }
    }
    pub unsafe fn queue_set_performance_configuration_intel(
        &self,
        queue: Queue,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result {
        unsafe { (self.queue_set_performance_configuration_intel)(queue, configuration) }
    }
    pub unsafe fn get_performance_parameter_intel(
        &self,
        device: Device,
        parameter: PerformanceParameterTypeINTEL,
        value: &mut PerformanceValueINTEL,
    ) -> Result {
        unsafe { (self.get_performance_parameter_intel)(device, parameter, value) }
    }
}
