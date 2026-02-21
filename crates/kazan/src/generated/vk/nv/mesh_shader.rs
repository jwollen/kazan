#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    cmd_draw_mesh_tasks_nv: PFN_vkCmdDrawMeshTasksNV,
    cmd_draw_mesh_tasks_indirect_nv: PFN_vkCmdDrawMeshTasksIndirectNV,
    cmd_draw_mesh_tasks_indirect_count_nv: Option<PFN_vkCmdDrawMeshTasksIndirectCountNV>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_draw_mesh_tasks_nv: transmute(
                    load(c"vkCmdDrawMeshTasksNV").ok_or(LoadingError)?,
                ),
                cmd_draw_mesh_tasks_indirect_nv: transmute(
                    load(c"vkCmdDrawMeshTasksIndirectNV").ok_or(LoadingError)?,
                ),
                cmd_draw_mesh_tasks_indirect_count_nv: transmute(load(
                    c"vkCmdDrawMeshTasksIndirectCountNV",
                )),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_draw_mesh_tasks_nv(
        &self,
        command_buffer: CommandBuffer,
        task_count: u32,
        first_task: u32,
    ) {
        unsafe { (self.cmd_draw_mesh_tasks_nv)(command_buffer, task_count, first_task) }
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_nv(
        &self,
        command_buffer: CommandBuffer,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.cmd_draw_mesh_tasks_indirect_nv)(
                command_buffer,
                buffer,
                offset,
                draw_count,
                stride,
            )
        }
    }
    pub unsafe fn cmd_draw_mesh_tasks_indirect_count_nv(
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
            (self.cmd_draw_mesh_tasks_indirect_count_nv.unwrap())(
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
