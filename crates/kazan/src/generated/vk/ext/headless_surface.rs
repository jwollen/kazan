#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    create_headless_surface_ext: PFN_vkCreateHeadlessSurfaceEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_headless_surface_ext: transmute(
                    load(c"vkCreateHeadlessSurfaceEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_headless_surface_ext(
        &self,
        instance: Instance,
        create_info: &HeadlessSurfaceCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_headless_surface_ext)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(surface.assume_init()),
                err => Err(err),
            }
        }
    }
}
