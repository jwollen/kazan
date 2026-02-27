#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub unified_image_layouts: Bool32,
    pub unified_image_layouts_video: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            unified_image_layouts: Default::default(),
            unified_image_layouts_video: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'a> {
    pub fn unified_image_layouts(mut self, unified_image_layouts: Bool32) -> Self {
        self.unified_image_layouts = unified_image_layouts;
        self
    }
    pub fn unified_image_layouts_video(mut self, unified_image_layouts_video: Bool32) -> Self {
        self.unified_image_layouts_video = unified_image_layouts_video;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentFeedbackLoopInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub feedback_loop_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AttachmentFeedbackLoopInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ATTACHMENT_FEEDBACK_LOOP_INFO_EXT,
            p_next: core::ptr::null(),
            feedback_loop_enable: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> AttachmentFeedbackLoopInfoEXT<'a> {
    pub fn feedback_loop_enable(mut self, feedback_loop_enable: Bool32) -> Self {
        self.feedback_loop_enable = feedback_loop_enable;
        self
    }
}
