#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clip_enable: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    pub depth_clip_enable: Bool32,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT: Flags {
    }
}
