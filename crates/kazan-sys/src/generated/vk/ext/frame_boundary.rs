#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct FrameBoundaryEXT<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub frame_boundary: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct FrameBoundaryFlagsEXT: Flags {
        const FRAME_END_EXT = FrameBoundaryFlagBitsEXT::FRAME_END_EXT.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FrameBoundaryFlagBitsEXT(u32);
impl FrameBoundaryFlagBitsEXT {
    pub const FRAME_END_EXT: Self = Self(1 << 0);
}
