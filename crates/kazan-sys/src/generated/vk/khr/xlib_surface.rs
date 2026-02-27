#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XlibSurfaceCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XlibSurfaceCreateFlagsKHR,
    pub dpy: *mut Display,
    pub window: Window,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for XlibSurfaceCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::XLIB_SURFACE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            dpy: core::ptr::null_mut(),
            window: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> XlibSurfaceCreateInfoKHR<'a> {
    pub fn flags(mut self, flags: XlibSurfaceCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn dpy(mut self, dpy: *mut Display) -> Self {
        self.dpy = dpy;
        self
    }
    pub fn window(mut self, window: Window) -> Self {
        self.window = window;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct XlibSurfaceCreateFlagsKHR: Flags {
    }
}
pub type PFN_vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XlibSurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    dpy: *mut Display,
    visual_id: VisualID,
) -> Bool32;
