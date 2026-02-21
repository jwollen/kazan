#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_draw_mesh_tasks_ext: PFN_vkCmdDrawMeshTasksEXT,
    cmd_draw_mesh_tasks_indirect_ext: PFN_vkCmdDrawMeshTasksIndirectEXT,
    cmd_draw_mesh_tasks_indirect_count_ext: Option<PFN_vkCmdDrawMeshTasksIndirectCountEXT>,
}
impl DeviceFn {
    pub unsafe fn cmd_draw_mesh_tasks_ext(
        &self,
        command_buffer: CommandBuffer,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.cmd_draw_mesh_tasks_ext)(
                command_buffer,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        }
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_ext(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_mesh_tasks_indirect_ext)(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,
            )
        }
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_ext(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_mesh_tasks_indirect_count_ext.unwrap())(
                command_buffer,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        }
    }
}
