#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DirectFBSurfaceCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: DirectFBSurfaceCreateFlagsEXT,
    pub dfb: *mut IDirectFB,
    pub surface: *mut IDirectFBSurface,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DirectFBSurfaceCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DIRECTFB_SURFACE_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            flags: Default::default(),
            dfb: core::ptr::null_mut(),
            surface: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DirectFBSurfaceCreateFlagsEXT: Flags {
    }
}
pub type PFN_vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const DirectFBSurfaceCreateInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32;
