#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConditionalRenderingBeginInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub buffer: Buffer,
    pub offset: DeviceSize,
    pub flags: ConditionalRenderingFlagsEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ConditionalRenderingBeginInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::CONDITIONAL_RENDERING_BEGIN_INFO_EXT,
            p_next: core::ptr::null(),
            buffer: Default::default(),
            offset: Default::default(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub conditional_rendering_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CommandBufferInheritanceConditionalRenderingInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COMMAND_BUFFER_INHERITANCE_CONDITIONAL_RENDERING_INFO_EXT,
            p_next: core::ptr::null(),
            conditional_rendering_enable: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub conditional_rendering: Bool32,
    pub inherited_conditional_rendering: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceConditionalRenderingFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_CONDITIONAL_RENDERING_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            conditional_rendering: Default::default(),
            inherited_conditional_rendering: Default::default(),
            _marker: PhantomData,
        }
    }
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
