#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_xlib_surface_khr: PFN_vkCreateXlibSurfaceKHR,
    get_physical_device_xlib_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXlibPresentationSupportKHR,
}
impl InstanceFn {
    pub unsafe fn create_xlib_surface_khr(
        &self,
        instance: Instance,
        create_info: &XlibSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_xlib_surface_khr)(instance, create_info, allocator.to_raw_ptr(), surface)
        }
    }
    pub unsafe fn get_physical_device_xlib_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dpy: &mut Display,
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
