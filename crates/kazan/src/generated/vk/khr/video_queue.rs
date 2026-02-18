#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_video_capabilities_khr: PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR,
    get_physical_device_video_format_properties_khr:
        PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_video_capabilities_khr(
        &self,
        physical_device: PhysicalDevice,
        video_profile: &VideoProfileInfoKHR,
        capabilities: &mut VideoCapabilitiesKHR,
    ) -> Result {
        unsafe {
            (self.get_physical_device_video_capabilities_khr)(
                physical_device,
                video_profile,
                capabilities,
            )
        }
    }
    pub unsafe fn get_physical_device_video_format_properties_khr(
        &self,
        physical_device: PhysicalDevice,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
        video_format_properties: impl ExtendUninit<VideoFormatPropertiesKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                video_format_properties,
                |video_format_property_count, video_format_properties| {
                    (self.get_physical_device_video_format_properties_khr)(
                        physical_device,
                        video_format_info,
                        video_format_property_count,
                        video_format_properties as _,
                    )
                },
            )
        }
    }
}
pub struct DeviceFn {
    create_video_session_khr: PFN_vkCreateVideoSessionKHR,
    destroy_video_session_khr: PFN_vkDestroyVideoSessionKHR,
    create_video_session_parameters_khr: PFN_vkCreateVideoSessionParametersKHR,
    update_video_session_parameters_khr: PFN_vkUpdateVideoSessionParametersKHR,
    destroy_video_session_parameters_khr: PFN_vkDestroyVideoSessionParametersKHR,
    get_video_session_memory_requirements_khr: PFN_vkGetVideoSessionMemoryRequirementsKHR,
    bind_video_session_memory_khr: PFN_vkBindVideoSessionMemoryKHR,
    cmd_begin_video_coding_khr: PFN_vkCmdBeginVideoCodingKHR,
    cmd_control_video_coding_khr: PFN_vkCmdControlVideoCodingKHR,
    cmd_end_video_coding_khr: PFN_vkCmdEndVideoCodingKHR,
}
impl DeviceFn {
    pub unsafe fn create_video_session_khr(
        &self,
        device: Device,
        create_info: &VideoSessionCreateInfoKHR,
        allocator: &AllocationCallbacks,
        video_session: &mut VideoSessionKHR,
    ) -> Result {
        unsafe { (self.create_video_session_khr)(device, create_info, allocator, video_session) }
    }
    pub unsafe fn destroy_video_session_khr(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_video_session_khr)(device, video_session, allocator) }
    }
    pub unsafe fn create_video_session_parameters_khr(
        &self,
        device: Device,
        create_info: &VideoSessionParametersCreateInfoKHR,
        allocator: &AllocationCallbacks,
        video_session_parameters: &mut VideoSessionParametersKHR,
    ) -> Result {
        unsafe {
            (self.create_video_session_parameters_khr)(
                device,
                create_info,
                allocator,
                video_session_parameters,
            )
        }
    }
    pub unsafe fn update_video_session_parameters_khr(
        &self,
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR,
    ) -> Result {
        unsafe {
            (self.update_video_session_parameters_khr)(
                device,
                video_session_parameters,
                update_info,
            )
        }
    }
    pub unsafe fn destroy_video_session_parameters_khr(
        &self,
        device: Device,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: &AllocationCallbacks,
    ) {
        unsafe {
            (self.destroy_video_session_parameters_khr)(device, video_session_parameters, allocator)
        }
    }
    pub unsafe fn get_video_session_memory_requirements_khr(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        memory_requirements: impl ExtendUninit<VideoSessionMemoryRequirementsKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                memory_requirements,
                |memory_requirements_count, memory_requirements| {
                    (self.get_video_session_memory_requirements_khr)(
                        device,
                        video_session,
                        memory_requirements_count,
                        memory_requirements as _,
                    )
                },
            )
        }
    }
    pub unsafe fn bind_video_session_memory_khr(
        &self,
        device: Device,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR],
    ) -> Result {
        unsafe {
            (self.bind_video_session_memory_khr)(
                device,
                video_session,
                bind_session_memory_infos.len().try_into().unwrap(),
                bind_session_memory_infos.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_begin_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        begin_info: &VideoBeginCodingInfoKHR,
    ) {
        unsafe { (self.cmd_begin_video_coding_khr)(command_buffer, begin_info) }
    }
    pub unsafe fn cmd_control_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        coding_control_info: &VideoCodingControlInfoKHR,
    ) {
        unsafe { (self.cmd_control_video_coding_khr)(command_buffer, coding_control_info) }
    }
    pub unsafe fn cmd_end_video_coding_khr(
        &self,
        command_buffer: CommandBuffer,
        end_coding_info: &VideoEndCodingInfoKHR,
    ) {
        unsafe { (self.cmd_end_video_coding_khr)(command_buffer, end_coding_info) }
    }
}
