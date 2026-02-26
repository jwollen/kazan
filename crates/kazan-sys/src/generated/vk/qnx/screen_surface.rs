#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScreenSurfaceCreateInfoQNX<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ScreenSurfaceCreateFlagsQNX,
    pub context: *mut _screen_context,
    pub window: *mut _screen_window,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ScreenSurfaceCreateInfoQNX<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SCREEN_SURFACE_CREATE_INFO_QNX,
            p_next: core::ptr::null(),
            flags: Default::default(),
            context: core::ptr::null_mut(),
            window: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ScreenSurfaceCreateFlagsQNX: Flags {
    }
}
pub type PFN_vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
    instance: Instance,
    p_create_info: *const ScreenSurfaceCreateInfoQNX<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_surface: *mut SurfaceKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    queue_family_index: u32,
    window: *mut _screen_window,
) -> Bool32;
