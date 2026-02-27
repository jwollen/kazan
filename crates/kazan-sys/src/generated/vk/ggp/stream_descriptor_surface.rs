#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StreamDescriptorSurfaceCreateInfoGGP<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: StreamDescriptorSurfaceCreateFlagsGGP,
    pub stream_descriptor: GgpStreamDescriptor,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for StreamDescriptorSurfaceCreateInfoGGP<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::STREAM_DESCRIPTOR_SURFACE_CREATE_INFO_GGP,
            p_next: core::ptr::null(),
            flags: Default::default(),
            stream_descriptor: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> StreamDescriptorSurfaceCreateInfoGGP<'a> {
    pub fn flags(mut self, flags: StreamDescriptorSurfaceCreateFlagsGGP) -> Self {
        self.flags = flags;
        self
    }
    pub fn stream_descriptor(mut self, stream_descriptor: GgpStreamDescriptor) -> Self {
        self.stream_descriptor = stream_descriptor;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StreamDescriptorSurfaceCreateFlagsGGP: Flags {
    }
}
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
