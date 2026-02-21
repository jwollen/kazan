#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_bind_index_buffer2_khr: PFN_vkCmdBindIndexBuffer2,
    get_rendering_area_granularity_khr: PFN_vkGetRenderingAreaGranularity,
    get_device_image_subresource_layout_khr: PFN_vkGetDeviceImageSubresourceLayout,
    get_image_subresource_layout2_khr: PFN_vkGetImageSubresourceLayout2,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_bind_index_buffer2_khr: transmute(
                    load(c"vkCmdBindIndexBuffer2KHR").ok_or(LoadingError)?,
                ),
                get_rendering_area_granularity_khr: transmute(
                    load(c"vkGetRenderingAreaGranularityKHR").ok_or(LoadingError)?,
                ),
                get_device_image_subresource_layout_khr: transmute(
                    load(c"vkGetDeviceImageSubresourceLayoutKHR").ok_or(LoadingError)?,
                ),
                get_image_subresource_layout2_khr: transmute(
                    load(c"vkGetImageSubresourceLayout2KHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_bind_index_buffer2_khr(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe {
            (self.cmd_bind_index_buffer2_khr)(command_buffer, buffer, offset, size, index_type)
        }
    }
    pub unsafe fn get_rendering_area_granularity_khr(
        &self,
        device: Device,
        rendering_area_info: &RenderingAreaInfo,
        granularity: &mut Extent2D,
    ) {
        unsafe {
            (self.get_rendering_area_granularity_khr)(device, rendering_area_info, granularity)
        }
    }
    pub unsafe fn get_device_image_subresource_layout_khr(
        &self,
        device: Device,
        info: &DeviceImageSubresourceInfo,
        layout: &mut SubresourceLayout2,
    ) {
        unsafe { (self.get_device_image_subresource_layout_khr)(device, info, layout) }
    }
    pub unsafe fn get_image_subresource_layout2_khr(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) {
        unsafe { (self.get_image_subresource_layout2_khr)(device, image, subresource, layout) }
    }
}
