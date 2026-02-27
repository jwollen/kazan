#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XcbSurfaceCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *mut xcb_connection_t,
    pub window: xcb_window_t,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for XcbSurfaceCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::XCB_SURFACE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            connection: core::ptr::null_mut(),
            window: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> XcbSurfaceCreateInfoKHR<'a> {
    pub fn flags(mut self, flags: XcbSurfaceCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn connection(mut self, connection: *mut xcb_connection_t) -> Self {
        self.connection = connection;
        self
    }
    pub fn window(mut self, window: xcb_window_t) -> Self {
        self.window = window;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct XcbSurfaceCreateFlagsKHR: Flags {
    }
}
pub type PFN_vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const XcbSurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> Bool32;
