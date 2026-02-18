#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_video_encode_quality_level_properties_khr:
        PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_video_encode_quality_level_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        quality_level_properties: &mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> Result {
        unsafe {
            (self.get_physical_device_video_encode_quality_level_properties_khr)(
                physical_device,
                quality_level_info,
                quality_level_properties,
            )
        }
    }
}
pub struct DeviceFn {
    get_encoded_video_session_parameters_khr: PFN_vkGetEncodedVideoSessionParametersKHR,
    cmd_encode_video_khr: PFN_vkCmdEncodeVideoKHR,
}
impl DeviceFn {
    pub unsafe fn get_encoded_video_session_parameters_khr(
        &self,
        device: Device,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: &mut VideoEncodeSessionParametersFeedbackInfoKHR,
        data: impl ExtendUninit<u8>,
    ) -> Result {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                (self.get_encoded_video_session_parameters_khr)(
                    device,
                    video_session_parameters_info,
                    feedback_info,
                    data_size,
                    data as _,
                )
            })
        }
    }
    pub unsafe fn cmd_encode_video_khr(
        &self,
        command_buffer: CommandBuffer,
        encode_info: &VideoEncodeInfoKHR,
    ) {
        unsafe { (self.cmd_encode_video_khr)(command_buffer, encode_info) }
    }
}
