#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MacOSSurfaceCreateInfoMVK<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: MacOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for MacOSSurfaceCreateInfoMVK<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::MACOS_SURFACE_CREATE_INFO_MVK,
            p_next: core::ptr::null(),
            flags: Default::default(),
            p_view: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> MacOSSurfaceCreateInfoMVK<'a> {
    pub fn flags(mut self, flags: MacOSSurfaceCreateFlagsMVK) -> Self {
        self.flags = flags;
        self
    }
    pub fn view(mut self, view: &'a c_void) -> Self {
        self.p_view = view;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MacOSSurfaceCreateFlagsMVK: Flags {
    }
}
pub type PFN_vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const MacOSSurfaceCreateInfoMVK<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
