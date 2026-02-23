#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type CAMetalLayer = *const c_void;
#[repr(C)]
pub struct MetalSurfaceCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MetalSurfaceCreateFlagsEXT,
    pub p_layer: *const CAMetalLayer,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct MetalSurfaceCreateFlagsEXT: Flags {
    }
}
pub type PFN_vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MetalSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
