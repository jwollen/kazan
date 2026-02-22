#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    create_screen_surface_qnx: PFN_vkCreateScreenSurfaceQNX,
    get_physical_device_screen_presentation_support_qnx:
        PFN_vkGetPhysicalDeviceScreenPresentationSupportQNX,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_screen_surface_qnx: transmute(
                    load(c"vkCreateScreenSurfaceQNX").ok_or(LoadingError)?,
                ),
                get_physical_device_screen_presentation_support_qnx: transmute(
                    load(c"vkGetPhysicalDeviceScreenPresentationSupportQNX").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_screen_surface_qnx(
        &self,
        instance: Instance,
        create_info: &ScreenSurfaceCreateInfoQNX,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.create_screen_surface_qnx)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface,
            ))
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
