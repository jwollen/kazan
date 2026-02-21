#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub exclusive_scissor: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub exclusive_scissor_count: u32,
    pub p_exclusive_scissors: *const Rect2D,
}
pub type PFN_vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissors: *const Rect2D,
);
pub type PFN_vkCmdSetExclusiveScissorEnableNV = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissor_enables: *const Bool32,
);
