#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_screen_surface_qnx: PFN_vkCreateScreenSurfaceQNX,
    get_physical_device_screen_presentation_support_qnx:
        PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
}
impl InstanceFn {
    pub unsafe fn create_screen_surface_qnx(
        &self,
        instance: Instance,
        create_info: &ScreenSurfaceCreateInfoQNX,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe {
            (self.create_screen_surface_qnx)(instance, create_info, allocator.to_raw_ptr(), surface)
        }
    }
    pub unsafe fn get_physical_device_screen_presentation_support_qnx(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        window: &mut _screen_window,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_screen_presentation_support_qnx)(
                physical_device,
                queue_family_index,
                window,
            )
        }
    }
}
