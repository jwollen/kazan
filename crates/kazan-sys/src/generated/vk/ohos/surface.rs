#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub type OHNativeWindow = *const c_void;
#[repr(C)]
pub struct SurfaceCreateInfoOHOS {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SurfaceCreateFlagsOHOS,
    pub window: *mut OHNativeWindow,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct SurfaceCreateFlagsOHOS: Flags {
    }
}
pub type PFN_vkCreateSurfaceOHOS = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const SurfaceCreateInfoOHOS,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
