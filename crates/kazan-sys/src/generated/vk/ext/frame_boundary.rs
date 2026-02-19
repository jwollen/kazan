#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FrameBoundaryEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: FrameBoundaryFlagsEXT,
    pub frame_id: u64,
    pub image_count: u32,
    pub p_images: *const Image,
    pub buffer_count: u32,
    pub p_buffers: *const Buffer,
    pub tag_name: u64,
    pub tag_size: usize,
    pub p_tag: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFrameBoundaryFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub frame_boundary: Bool32,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct FrameBoundaryFlagsEXT: Flags {
        const FRAME_END_EXT = 1 << 0;
    }
}
