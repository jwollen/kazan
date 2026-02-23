#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_discard_rectangles: u32,
}
#[repr(C)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    pub discard_rectangle_count: u32,
    pub p_discard_rectangles: *const Rect2D,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DiscardRectangleModeEXT(i32);
impl DiscardRectangleModeEXT {
    pub const INCLUSIVE_EXT: Self = Self(0);
    pub const EXCLUSIVE_EXT: Self = Self(1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineDiscardRectangleStateCreateFlagsEXT: Flags {
    }
}
pub type PFN_vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    p_discard_rectangles: *const Rect2D,
);
pub type PFN_vkCmdSetDiscardRectangleEnableEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer, discard_rectangle_enable: Bool32);
pub type PFN_vkCmdSetDiscardRectangleModeEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    discard_rectangle_mode: DiscardRectangleModeEXT,
);
