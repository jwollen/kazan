#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct DirectFBSurfaceCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DirectFBSurfaceCreateFlagsEXT,
    pub dfb: *mut IDirectFB,
    pub surface: *mut IDirectFBSurface,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct DirectFBSurfaceCreateFlagsEXT: Flags {
    }
}
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DirectFBSurfaceCreateInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32;
