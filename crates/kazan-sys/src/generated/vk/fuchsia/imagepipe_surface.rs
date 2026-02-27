#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIA<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImagePipeSurfaceCreateFlagsFUCHSIA,
    pub image_pipe_handle: zx_handle_t,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ImagePipeSurfaceCreateInfoFUCHSIA<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA,
            p_next: core::ptr::null(),
            flags: Default::default(),
            image_pipe_handle: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ImagePipeSurfaceCreateInfoFUCHSIA<'a> {
    pub fn flags(mut self, flags: ImagePipeSurfaceCreateFlagsFUCHSIA) -> Self {
        self.flags = flags;
        self
    }
    pub fn image_pipe_handle(mut self, image_pipe_handle: zx_handle_t) -> Self {
        self.image_pipe_handle = image_pipe_handle;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImagePipeSurfaceCreateFlagsFUCHSIA: Flags {
    }
}
pub type PFN_vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
