#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_xcb_surface_khr: PFN_vkCreateXcbSurfaceKHR,
    get_physical_device_xcb_presentation_support_khr:
        PFN_vkGetPhysicalDeviceXcbPresentationSupportKHR,
}
impl InstanceFn {
    pub unsafe fn create_xcb_surface_khr(
        &self,
        instance: Instance,
        create_info: &XcbSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_xcb_surface_khr)(instance, create_info, allocator.to_raw_ptr(), surface)
        }
    }
    pub unsafe fn get_physical_device_xcb_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        connection: &mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_xcb_presentation_support_khr)(
                physical_device,
                queue_family_index,
                connection,
                visual_id,
            )
        }
    }
}
