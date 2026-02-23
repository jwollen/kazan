#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct AttachmentSampleCountInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_samples: *const SampleCountFlagBits,
    pub depth_stencil_attachment_samples: SampleCountFlagBits,
}
