#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clip_control: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineViewportDepthClipControlCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub negative_one_to_one: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
