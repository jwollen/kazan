#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WaylandSurfaceCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: WaylandSurfaceCreateFlagsKHR,
    pub display: *mut wl_display,
    pub surface: *mut wl_surface,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for WaylandSurfaceCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            display: core::ptr::null_mut(),
            surface: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct WaylandSurfaceCreateFlagsKHR: Flags {
    }
}
pub type PFN_vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const WaylandSurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
)
    -> Bool32;
