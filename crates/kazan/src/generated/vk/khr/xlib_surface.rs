#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    get_physical_device_xlib_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_xlib_surface_khr: transmute(
                    load(c"vkCreateXlibSurfaceKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_xlib_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceXlibPresentationSupportKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_xlib_surface_khr(
        &self,
        instance: Instance,
        create_info: &XlibSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_xlib_surface_khr)(
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
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_xlib_presentation_support_khr)(
                physical_device,
                queue_family_index,
                dpy,
                visual_id,
            )
        }
    }
}
