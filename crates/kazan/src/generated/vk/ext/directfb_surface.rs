#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    create_direct_fb_surface_ext: PFN_vkCreateDirectFBSurfaceEXT,
    get_physical_device_direct_fb_presentation_support_ext:
        PFN_vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                create_direct_fb_surface_ext: transmute(
                    load(c"vkCreateDirectFBSurfaceEXT").ok_or(LoadingError)?,
                ),
                get_physical_device_direct_fb_presentation_support_ext: transmute(
                    load(c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT")
                        .ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn create_direct_fb_surface_ext(
        &self,
        instance: Instance,
        create_info: &DirectFBSurfaceCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        surface: &mut SurfaceKHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.create_direct_fb_surface_ext)(
                instance,
                create_info,
                allocator.to_raw_ptr(),
                surface,
            ))
        }
    }
    pub unsafe fn get_physical_device_direct_fb_presentation_support_ext(
        &self,
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        dfb: &mut IDirectFB,
    ) -> Bool32 {
        unsafe {
            (self.get_physical_device_direct_fb_presentation_support_ext)(
                physical_device,
                queue_family_index,
                dfb,
            )
        }
    }
}
