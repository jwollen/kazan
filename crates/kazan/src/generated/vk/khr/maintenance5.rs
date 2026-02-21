#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_bind_index_buffer2: PFN_vkCmdBindIndexBuffer2,
    get_rendering_area_granularity: PFN_vkGetRenderingAreaGranularity,
    get_device_image_subresource_layout: PFN_vkGetDeviceImageSubresourceLayout,
    get_image_subresource_layout2: PFN_vkGetImageSubresourceLayout2,
}
impl DeviceFn {
    pub unsafe fn cmd_bind_index_buffer2(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe { (self.cmd_bind_index_buffer2)(command_buffer, buffer, offset, size, index_type) }
    }
    pub unsafe fn get_rendering_area_granularity(
        &self,
        device: Device,
        rendering_area_info: &RenderingAreaInfo,
        granularity: &mut Extent2D,
    ) {
        unsafe { (self.get_rendering_area_granularity)(device, rendering_area_info, granularity) }
    }
    pub unsafe fn get_device_image_subresource_layout(
        &self,
        device: Device,
        info: &DeviceImageSubresourceInfo,
        layout: &mut SubresourceLayout2,
    ) {
        unsafe { (self.get_device_image_subresource_layout)(device, info, layout) }
    }
    pub unsafe fn get_image_subresource_layout2(
        &self,
        device: Device,
        image: Image,
        subresource: &ImageSubresource2,
        layout: &mut SubresourceLayout2,
    ) {
        unsafe { (self.get_image_subresource_layout2)(device, image, subresource, layout) }
    }
}
