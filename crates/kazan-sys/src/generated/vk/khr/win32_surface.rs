#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Win32SurfaceCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: Win32SurfaceCreateFlagsKHR,
    pub hinstance: HINSTANCE,
    pub hwnd: HWND,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for Win32SurfaceCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            hinstance: Default::default(),
            hwnd: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> Win32SurfaceCreateInfoKHR<'a> {
    pub fn flags(mut self, flags: Win32SurfaceCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn hinstance(mut self, hinstance: HINSTANCE) -> Self {
        self.hinstance = hinstance;
        self
    }
    pub fn hwnd(mut self, hwnd: HWND) -> Self {
        self.hwnd = hwnd;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Win32SurfaceCreateFlagsKHR: Flags {
    }
}
pub type PFN_vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const Win32SurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "system" fn(physical_device: PhysicalDevice, queue_family_index: u32) -> Bool32;
