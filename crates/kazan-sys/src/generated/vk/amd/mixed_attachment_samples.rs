#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentSampleCountInfoAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_samples: *const SampleCountFlagBits,
    pub depth_stencil_attachment_samples: SampleCountFlagBits,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AttachmentSampleCountInfoAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ATTACHMENT_SAMPLE_COUNT_INFO_AMD,
            p_next: core::ptr::null(),
            color_attachment_count: Default::default(),
            p_color_attachment_samples: core::ptr::null(),
            depth_stencil_attachment_samples: Default::default(),
            _marker: PhantomData,
        }
    }
}
