#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDiscardRectanglePropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_discard_rectangles: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDiscardRectanglePropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DISCARD_RECTANGLE_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            max_discard_rectangles: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineDiscardRectangleStateCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineDiscardRectangleStateCreateFlagsEXT,
    pub discard_rectangle_mode: DiscardRectangleModeEXT,
    pub discard_rectangle_count: u32,
    pub p_discard_rectangles: *const Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineDiscardRectangleStateCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_DISCARD_RECTANGLE_STATE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            discard_rectangle_mode: Default::default(),
            discard_rectangle_count: Default::default(),
            p_discard_rectangles: core::ptr::null(),
            _marker: PhantomData,
        }
    }
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
