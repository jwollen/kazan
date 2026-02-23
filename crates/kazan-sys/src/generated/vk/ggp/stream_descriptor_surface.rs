#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct StreamDescriptorSurfaceCreateInfoGGP {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: StreamDescriptorSurfaceCreateFlagsGGP,
    pub stream_descriptor: GgpStreamDescriptor,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct StreamDescriptorSurfaceCreateFlagsGGP: Flags {
    }
}
pub type PFN_vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
