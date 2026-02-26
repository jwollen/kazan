#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct ConditionalRenderingBeginInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub flags: ConditionalRenderingFlagsEXT,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conditional_rendering_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub conditional_rendering: Bool32,
    pub inherited_conditional_rendering: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ConditionalRenderingFlagsEXT: Flags {
        const INVERTED_EXT = ConditionalRenderingFlagBitsEXT::INVERTED_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConditionalRenderingFlagBitsEXT(u32);
impl ConditionalRenderingFlagBitsEXT {
    pub const INVERTED_EXT: Self = Self(1 << 0);
}
pub type PFN_vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT<'_>,
);
pub type PFN_vkCmdEndConditionalRenderingEXT =
    unsafe extern "system" fn(command_buffer: CommandBuffer);
