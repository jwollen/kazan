#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub unified_image_layouts: Bool32,
    pub unified_image_layouts_video: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct AttachmentFeedbackLoopInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub feedback_loop_enable: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
