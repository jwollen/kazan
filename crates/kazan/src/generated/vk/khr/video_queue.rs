#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_video_capabilities_khr: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
    get_physical_device_video_format_properties_khr:
        PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_video_capabilities_khr: transmute(
                    load(c"vkGetPhysicalDeviceVideoCapabilitiesKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_video_format_properties_khr: transmute(
                    load(c"vkGetPhysicalDeviceVideoFormatPropertiesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_video_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        video_profile: &VideoProfileInfoKHR<'_>,
    ) -> crate::Result<VideoCapabilitiesKHR<'_>> {
        unsafe {
            let mut capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_video_capabilities_khr)(
                physical_device,
                video_profile,
                capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(capabilities.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_physical_device_video_format_properties_khr<'a>(
        &self,
        physical_device: PhysicalDevice,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR<'_>,
        video_format_properties: impl ExtendUninit<VideoFormatPropertiesKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                video_format_properties,
                |video_format_property_count, video_format_properties| {
                    let result = (self.get_physical_device_video_format_properties_khr)(
                        physical_device,
                        video_format_info,
                        video_format_property_count,
                        video_format_properties as _,
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
pub struct DeviceFn {
    create_video_session_khr: PFN_vkCreateVideoSessionKHR,
    destroy_video_session_khr: PFN_vkDestroyVideoSessionKHR,
    get_video_session_memory_requirements_khr: PFN_vkGetVideoSessionMemoryRequirementsKHR,
    bind_video_session_memory_khr: PFN_vkBindVideoSessionMemoryKHR,
    create_video_session_parameters_khr: PFN_vkCreateVideoSessionParametersKHR,
    update_video_session_parameters_khr: PFN_vkUpdateVideoSessionParametersKHR,
    destroy_video_session_parameters_khr: PFN_vkDestroyVideoSessionParametersKHR,
    cmd_begin_video_coding_khr: PFN_vkCmdBeginVideoCodingKHR,
    cmd_end_video_coding_khr: PFN_vkCmdEndVideoCodingKHR,
    cmd_control_video_coding_khr: PFN_vkCmdControlVideoCodingKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_video_session_khr: transmute(
                    load(c"vkCreateVideoSessionKHR").ok_or(LoadingError)?,
                ),
                destroy_video_session_khr: transmute(
                    load(c"vkDestroyVideoSessionKHR").ok_or(LoadingError)?,
                ),
                get_video_session_memory_requirements_khr: transmute(
                    load(c"vkGetVideoSessionMemoryRequirementsKHR").ok_or(LoadingError)?,
                ),
                bind_video_session_memory_khr: transmute(
                    load(c"vkBindVideoSessionMemoryKHR").ok_or(LoadingError)?,
                ),
                create_video_session_parameters_khr: transmute(
                    load(c"vkCreateVideoSessionParametersKHR").ok_or(LoadingError)?,
                ),
                update_video_session_parameters_khr: transmute(
                    load(c"vkUpdateVideoSessionParametersKHR").ok_or(LoadingError)?,
                ),
                destroy_video_session_parameters_khr: transmute(
                    load(c"vkDestroyVideoSessionParametersKHR").ok_or(LoadingError)?,
                ),
                cmd_begin_video_coding_khr: transmute(
                    load(c"vkCmdBeginVideoCodingKHR").ok_or(LoadingError)?,
                ),
                cmd_end_video_coding_khr: transmute(
                    load(c"vkCmdEndVideoCodingKHR").ok_or(LoadingError)?,
                ),
                cmd_control_video_coding_khr: transmute(
                    load(c"vkCmdControlVideoCodingKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn create_video_session_khr(
        &self,
        device: Device,
        create_info: &VideoSessionCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<VideoSessionKHR> {
        unsafe {
            let mut video_session = core::mem::MaybeUninit::uninit();
            let result = (self.create_video_session_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                video_session.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(video_session.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_video_session_khr(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_video_session_khr)(device, video_session, allocator.to_raw_ptr()) }
    }
    pub unsafe fn get_video_session_memory_requirements_khr<'a>(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        memory_requirements: impl ExtendUninit<VideoSessionMemoryRequirementsKHR<'a>>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                memory_requirements,
                |memory_requirements_count, memory_requirements| {
                    let result = (self.get_video_session_memory_requirements_khr)(
                        device,
                        video_session,
                        memory_requirements_count,
                        memory_requirements as _,
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
    pub unsafe fn bind_video_session_memory_khr(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR<'_>],
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.bind_video_session_memory_khr)(
                device,
                video_session,
                bind_session_memory_infos.len().try_into().unwrap(),
                bind_session_memory_infos.as_ptr() as _,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn create_video_session_parameters_khr(
        &self,
        device: Device,
        create_info: &VideoSessionParametersCreateInfoKHR<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<VideoSessionParametersKHR> {
        unsafe {
            let mut video_session_parameters = core::mem::MaybeUninit::uninit();
            let result = (self.create_video_session_parameters_khr)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                video_session_parameters.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(video_session_parameters.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn update_video_session_parameters_khr(
        &self,
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.update_video_session_parameters_khr)(
                device,
                video_session_parameters,
                update_info,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn destroy_video_session_parameters_khr(
        &self,
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe {
            (self.destroy_video_session_parameters_khr)(
                device,
                video_session_parameters,
                allocator.to_raw_ptr(),
            )
        }
    }
    pub unsafe fn cmd_begin_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &VideoBeginCodingInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_begin_video_coding_khr)(command_buffer, begin_info) }
    }
    pub unsafe fn cmd_end_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        end_coding_info: &VideoEndCodingInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_end_video_coding_khr)(command_buffer, end_coding_info) }
    }
    pub unsafe fn cmd_control_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        coding_control_info: &VideoCodingControlInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_control_video_coding_khr)(command_buffer, coding_control_info) }
    }
}
