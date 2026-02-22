#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct InstanceFn {
    create_wayland_surface_khr: PFN_vkCreateWaylandSurfaceKHR,
    get_physical_device_wayland_presentation_support_khr:
        PFN_vkGetPhysicalDeviceWaylandPresentationSupportKHR,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_wayland_surface_khr: transmute(
                    load(c"vkCreateWaylandSurfaceKHR").ok_or(LoadingError)?,
                ),
                get_physical_device_wayland_presentation_support_khr: transmute(
                    load(c"vkGetPhysicalDeviceWaylandPresentationSupportKHR")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_wayland_surface_khr(
        &self,
        instance: Instance,
        create_info: &WaylandSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<SurfaceKHR> {
        unsafe {
            let mut surface = core::mem::MaybeUninit::uninit();
            let result = (self.create_wayland_surface_khr)(
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
    pub unsafe fn get_physical_device_wayland_presentation_support_khr(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_wayland_presentation_support_khr)(
                physical_device,
                queue_family_index,
                display,
            )
        }
    }
}
