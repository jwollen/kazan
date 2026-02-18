#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct InstanceFn {
    create_win32_surface_khr: PFN_vkCreateWin32SurfaceKHR,
    get_physical_device_win32_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWin32PresentationSupportKHR,
}
impl InstanceFn {
    pub unsafe fn create_win32_surface_khr(
        &self,
        instance: Instance,
        create_info: &Win32SurfaceCreateInfoKHR,
        allocator: &AllocationCallbacks,
        surface: &mut SurfaceKHR,
    ) -> Result {
        unsafe { (self.create_win32_surface_khr)(instance, create_info, allocator, surface) }
    }
    pub unsafe fn get_physical_device_win32_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_win32_presentation_support_khr)(
                physical_device,
                queue_family_index,
            )
        }
    }
}
