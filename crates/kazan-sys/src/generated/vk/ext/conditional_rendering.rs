#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConditionalRenderingBeginInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub flags: ConditionalRenderingFlagsEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conditional_rendering_enable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub conditional_rendering: Bool32,
    pub inherited_conditional_rendering: Bool32,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ConditionalRenderingFlagsEXT: Flags {
        const INVERTED_EXT = ConditionalRenderingFlagBitsEXT::INVERTED_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalRenderingFlagBitsEXT(u32);
impl ConditionalRenderingFlagBitsEXT {
    pub const INVERTED_EXT: Self = Self(1 << 0);
}
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
);
pub type PFN_vkCmdEndConditionalRenderingEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer);
