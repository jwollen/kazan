#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for FrameBoundaryEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::FRAME_BOUNDARY_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            frame_id: Default::default(),
            image_count: Default::default(),
            p_images: core::ptr::null(),
            buffer_count: Default::default(),
            p_buffers: core::ptr::null(),
            tag_name: Default::default(),
            tag_size: Default::default(),
            p_tag: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> FrameBoundaryEXT<'a> {
    pub fn flags(mut self, flags: FrameBoundaryFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    pub fn frame_id(mut self, frame_id: u64) -> Self {
        self.frame_id = frame_id;
        self
    }
    pub fn images(mut self, images: &'a [Image]) -> Self {
        self.image_count = images.len().try_into().unwrap();
        self.p_images = images.as_ptr();
        self
    }
    pub fn buffers(mut self, buffers: &'a [Buffer]) -> Self {
        self.buffer_count = buffers.len().try_into().unwrap();
        self.p_buffers = buffers.as_ptr();
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.tag_name = tag_name;
        self
    }
    pub fn tag(mut self, tag: &'a [u8]) -> Self {
        self.tag_size = tag.len().try_into().unwrap();
        self.p_tag = tag.as_ptr() as _;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub frame_boundary: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceFrameBoundaryFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_FRAME_BOUNDARY_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            frame_boundary: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceFrameBoundaryFeaturesEXT<'a> {
    pub fn frame_boundary(mut self, frame_boundary: Bool32) -> Self {
        self.frame_boundary = frame_boundary;
        self
    }
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
