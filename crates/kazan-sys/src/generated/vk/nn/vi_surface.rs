#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ViSurfaceCreateInfoNN<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ViSurfaceCreateFlagsNN,
    pub window: *mut c_void,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ViSurfaceCreateInfoNN<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VI_SURFACE_CREATE_INFO_NN,
            p_next: core::ptr::null(),
            flags: Default::default(),
            window: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ViSurfaceCreateInfoNN<'a> {
    pub fn flags(mut self, flags: ViSurfaceCreateFlagsNN) -> Self {
        self.flags = flags;
        self
    }
    pub fn window(mut self, window: &'a mut c_void) -> Self {
        self.window = window;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ViSurfaceCreateFlagsNN: Flags {
    }
}
pub type PFN_vkCreateViSurfaceNN = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ViSurfaceCreateInfoNN<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
