#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type ANativeWindow = *const c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AndroidSurfaceCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AndroidSurfaceCreateFlagsKHR,
    pub window: *mut ANativeWindow,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for AndroidSurfaceCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::ANDROID_SURFACE_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            window: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
impl<'a> AndroidSurfaceCreateInfoKHR<'a> {
    pub fn flags(mut self, flags: AndroidSurfaceCreateFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn window(mut self, window: *mut ANativeWindow) -> Self {
        self.window = window;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct AndroidSurfaceCreateFlagsKHR: Flags {
    }
}
pub type PFN_vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const AndroidSurfaceCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
