#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    cmd_bind_transform_feedback_buffers_ext: PFN_vkCmdBindTransformFeedbackBuffersEXT,
    cmd_begin_transform_feedback_ext: PFN_vkCmdBeginTransformFeedbackEXT,
    cmd_end_transform_feedback_ext: PFN_vkCmdEndTransformFeedbackEXT,
    cmd_begin_query_indexed_ext: PFN_vkCmdBeginQueryIndexedEXT,
    cmd_end_query_indexed_ext: PFN_vkCmdEndQueryIndexedEXT,
    cmd_draw_indirect_byte_count_ext: PFN_vkCmdDrawIndirectByteCountEXT,
}
impl DeviceFn {
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        &self,
        command_buffer: CommandBuffer,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: &[DeviceSize],
    ) {
        unsafe {
            (self.cmd_bind_transform_feedback_buffers_ext)(
                command_buffer,
                first_binding,
                buffers.len().try_into().unwrap(),
                buffers.as_ptr() as _,
                offsets.as_ptr() as _,
                sizes.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_begin_transform_feedback_ext(
        &self,
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: &[DeviceSize],
    ) {
        unsafe {
            (self.cmd_begin_transform_feedback_ext)(
                command_buffer,
                first_counter_buffer,
                counter_buffers.len().try_into().unwrap(),
                counter_buffers.as_ptr() as _,
                counter_buffer_offsets.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_end_transform_feedback_ext(
        &self,
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: &[DeviceSize],
    ) {
        unsafe {
            (self.cmd_end_transform_feedback_ext)(
                command_buffer,
                first_counter_buffer,
                counter_buffers.len().try_into().unwrap(),
                counter_buffers.as_ptr() as _,
                counter_buffer_offsets.as_ptr() as _,
            )
        }
    }
    pub unsafe fn cmd_begin_query_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ) {
        unsafe {
            (self.cmd_begin_query_indexed_ext)(command_buffer, query_pool, query, flags, index)
        }
    }
    pub unsafe fn cmd_end_query_indexed_ext(
        &self,
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        index: u32,
    ) {
        unsafe { (self.cmd_end_query_indexed_ext)(command_buffer, query_pool, query, index) }
    }
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        &self,
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_indirect_byte_count_ext)(
                command_buffer,
                instance_count,
                first_instance,
                counter_buffer,
                counter_buffer_offset,
                counter_offset,
                vertex_stride,
            )
        }
    }
}
