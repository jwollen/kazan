#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HeadlessSurfaceCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: HeadlessSurfaceCreateFlagsEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for HeadlessSurfaceCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::HEADLESS_SURFACE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct HeadlessSurfaceCreateFlagsEXT: Flags {
    }
}
pub type PFN_vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const HeadlessSurfaceCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
