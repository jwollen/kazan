#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct WaylandSurfaceCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display,
    pub surface: *mut wl_surface,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct WaylandSurfaceCreateFlagsKHR: Flags {
    }
}
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const WaylandSurfaceCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_surface: *mut SurfaceKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
)
    -> Bool32;
