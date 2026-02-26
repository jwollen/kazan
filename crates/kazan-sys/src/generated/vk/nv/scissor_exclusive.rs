#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub exclusive_scissor: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceExclusiveScissorFeaturesNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_EXCLUSIVE_SCISSOR_FEATURES_NV,
            p_next: core::ptr::null_mut(),
            exclusive_scissor: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub exclusive_scissor_count: u32,
    pub p_exclusive_scissors: *const Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineViewportExclusiveScissorStateCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VIEWPORT_EXCLUSIVE_SCISSOR_STATE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            exclusive_scissor_count: Default::default(),
            p_exclusive_scissors: core::ptr::null(),
            _marker: PhantomData,
        }
    }
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
