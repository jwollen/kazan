#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceUnifiedImageLayoutsFeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub unified_image_layouts: Bool32,
    pub unified_image_layouts_video: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentFeedbackLoopInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub feedback_loop_enable: Bool32,
}
