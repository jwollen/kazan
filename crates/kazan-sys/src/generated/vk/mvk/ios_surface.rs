#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct IOSSurfaceCreateInfoMVK<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: IOSSurfaceCreateFlagsMVK,
    pub p_view: *const c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for IOSSurfaceCreateInfoMVK<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IOS_SURFACE_CREATE_INFO_MVK,
            p_next: core::ptr::null(),
            flags: Default::default(),
            p_view: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> IOSSurfaceCreateInfoMVK<'a> {
    pub fn flags(mut self, flags: IOSSurfaceCreateFlagsMVK) -> Self {
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
    pub struct IOSSurfaceCreateFlagsMVK: Flags {
    }
}
pub type PFN_vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const IOSSurfaceCreateInfoMVK<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
