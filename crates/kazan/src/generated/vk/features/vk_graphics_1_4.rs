#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_set_line_stipple: PFN_vkCmdSetLineStipple,
    cmd_bind_index_buffer2: PFN_vkCmdBindIndexBuffer2,
    get_rendering_area_granularity: PFN_vkGetRenderingAreaGranularity,
    cmd_set_rendering_attachment_locations: PFN_vkCmdSetRenderingAttachmentLocations,
    cmd_set_rendering_input_attachment_indices: PFN_vkCmdSetRenderingInputAttachmentIndices,
}
impl DeviceFn {
    pub unsafe fn cmd_set_line_stipple(
        &self,
        command_buffer: CommandBuffer,
        line_stipple_factor: u32,
        line_stipple_pattern: u16,
    ) {
        unsafe {
            (self.cmd_set_line_stipple)(command_buffer, line_stipple_factor, line_stipple_pattern)
        }
    }
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
    pub unsafe fn cmd_set_rendering_attachment_locations(
        &self,
        command_buffer: CommandBuffer,
        location_info: &RenderingAttachmentLocationInfo,
    ) {
        unsafe { (self.cmd_set_rendering_attachment_locations)(command_buffer, location_info) }
    }
    pub unsafe fn cmd_set_rendering_input_attachment_indices(
        &self,
        command_buffer: CommandBuffer,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) {
        unsafe {
            (self.cmd_set_rendering_input_attachment_indices)(
                command_buffer,
                input_attachment_index_info,
            )
        }
    }
}
