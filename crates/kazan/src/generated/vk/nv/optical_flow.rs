#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    get_physical_device_optical_flow_image_formats_nv:
        PFN_vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_optical_flow_image_formats_nv(
        &self,
        physical_device: PhysicalDevice,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
        image_format_properties: impl ExtendUninit<OpticalFlowImageFormatPropertiesNV>,
    ) -> Result {
        unsafe {
            try_extend_uninit(
                image_format_properties,
                |format_count, image_format_properties| {
                    (self.get_physical_device_optical_flow_image_formats_nv)(
                        physical_device,
                        optical_flow_image_format_info,
                        format_count,
                        image_format_properties as _,
                    )
                },
            )
        }
    }
}
pub struct DeviceFn {
    create_optical_flow_session_nv: PFN_vkCreateOpticalFlowSessionNV,
    destroy_optical_flow_session_nv: PFN_vkDestroyOpticalFlowSessionNV,
    bind_optical_flow_session_image_nv: PFN_vkBindOpticalFlowSessionImageNV,
    cmd_optical_flow_execute_nv: PFN_vkCmdOpticalFlowExecuteNV,
}
impl DeviceFn {
    pub unsafe fn create_optical_flow_session_nv(
        &self,
        device: Device,
        create_info: &OpticalFlowSessionCreateInfoNV,
        allocator: &AllocationCallbacks,
        session: &mut OpticalFlowSessionNV,
    ) -> Result {
        unsafe { (self.create_optical_flow_session_nv)(device, create_info, allocator, session) }
    }
    pub unsafe fn destroy_optical_flow_session_nv(
        &self,
        device: Device,
        session: OpticalFlowSessionNV,
        allocator: &AllocationCallbacks,
    ) {
        unsafe { (self.destroy_optical_flow_session_nv)(device, session, allocator) }
    }
    pub unsafe fn bind_optical_flow_session_image_nv(
        &self,
        device: Device,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> Result {
        unsafe {
            (self.bind_optical_flow_session_image_nv)(device, session, binding_point, view, layout)
        }
    }
    pub unsafe fn cmd_optical_flow_execute_nv(
        &self,
        command_buffer: CommandBuffer,
        session: OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV,
    ) {
        unsafe { (self.cmd_optical_flow_execute_nv)(command_buffer, session, execute_info) }
    }
}
