#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub type OHNativeWindow = *const c_void;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SurfaceCreateInfoOHOS<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SurfaceCreateFlagsOHOS,
    pub window: *mut OHNativeWindow,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SurfaceCreateInfoOHOS<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SURFACE_CREATE_INFO_OHOS,
            p_next: core::ptr::null(),
            flags: Default::default(),
            window: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SurfaceCreateInfoOHOS<'a> {
    pub fn flags(mut self, flags: SurfaceCreateFlagsOHOS) -> Self {
        self.flags = flags;
        self
    }
    pub fn window(mut self, window: &'a mut OHNativeWindow) -> Self {
        self.window = window;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SurfaceCreateFlagsOHOS: Flags {
    }
}
pub type PFN_vkCreateSurfaceOHOS = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const SurfaceCreateInfoOHOS<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
