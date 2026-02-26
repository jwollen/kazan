#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_video_encode_quality_level_properties_khr:
        PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_video_encode_quality_level_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_video_encode_quality_level_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
    ) -> crate::Result<VideoEncodeQualityLevelPropertiesKHR<'_>> {
        unsafe {
            let mut quality_level_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_video_encode_quality_level_properties_khr)(
                physical_device,
                quality_level_info,
                quality_level_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(quality_level_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
pub struct DeviceFn {
    get_encoded_video_session_parameters_khr: PFN_vkGetEncodedVideoSessionParametersKHR,
    cmd_encode_video_khr: PFN_vkCmdEncodeVideoKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_encoded_video_session_parameters_khr: transmute(
                    load(c"vkGetEncodedVideoSessionParametersKHR").ok_or(LoadingError)?,
                ),
                cmd_encode_video_khr: transmute(load(c"vkCmdEncodeVideoKHR").ok_or(LoadingError)?),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_encoded_video_session_parameters_khr<'a>(
        &self,
        device: Device,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR<'_>,
        feedback_info: Option<&mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>>,
        data: impl ExtendUninit<u8>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(data, |data_size, data| {
                let result = (self.get_encoded_video_session_parameters_khr)(
                    device,
                    video_session_parameters_info,
                    todo!("output parameters in enumeration commands"),
                    data_size,
                    data as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            })
        }
    }
    pub unsafe fn cmd_encode_video_khr(
        &self,
        command_buffer: CommandBuffer,
        encode_info: &VideoEncodeInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_encode_video_khr)(command_buffer, encode_info) }
    }
}
