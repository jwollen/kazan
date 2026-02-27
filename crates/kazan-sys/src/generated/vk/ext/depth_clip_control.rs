#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub depth_clip_control: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDepthClipControlFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DEPTH_CLIP_CONTROL_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            depth_clip_control: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceDepthClipControlFeaturesEXT<'a> {
    pub fn depth_clip_control(mut self, depth_clip_control: Bool32) -> Self {
        self.depth_clip_control = depth_clip_control;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineViewportDepthClipControlCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub negative_one_to_one: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineViewportDepthClipControlCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_VIEWPORT_DEPTH_CLIP_CONTROL_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            negative_one_to_one: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineViewportDepthClipControlCreateInfoEXT<'a> {
    pub fn negative_one_to_one(mut self, negative_one_to_one: Bool32) -> Self {
        self.negative_one_to_one = negative_one_to_one;
        self
    }
}
