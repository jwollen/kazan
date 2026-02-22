#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    get_physical_device_surface_capabilities2_ext: PFN_vkGetPhysicalDeviceSurfaceCapabilities2EXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_surface_capabilities2_ext: transmute(
                    load(c"vkGetPhysicalDeviceSurfaceCapabilities2EXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_surface_capabilities2_ext(
        &self,
        physical_device: PhysicalDevice,
        surface: SurfaceKHR,
    ) -> crate::Result<SurfaceCapabilities2EXT> {
        unsafe {
            let mut surface_capabilities = core::mem::MaybeUninit::uninit();
            let result = (self.get_physical_device_surface_capabilities2_ext)(
                physical_device,
                surface,
                surface_capabilities.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(surface_capabilities.assume_init()),
                err => Err(err),
            }
        }
    }
}
