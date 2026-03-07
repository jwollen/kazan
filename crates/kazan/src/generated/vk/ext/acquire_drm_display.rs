//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_acquire_drm_display.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_acquire_drm_display";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireDrmDisplayEXT.html>
    pub type PFN_vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        drm_fd: i32,
        display: DisplayKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDrmDisplayEXT.html>
    pub type PFN_vkGetDrmDisplayEXT = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
        display: *mut DisplayKHR,
    ) -> vk::Result;
}

pub struct InstanceFn {
    acquire_drm_display_ext: PFN_vkAcquireDrmDisplayEXT,
    get_drm_display_ext: PFN_vkGetDrmDisplayEXT,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                acquire_drm_display_ext: transmute(
                    load(c"vkAcquireDrmDisplayEXT").ok_or(MissingEntryPointError)?,
                ),
                get_drm_display_ext: transmute(
                    load(c"vkGetDrmDisplayEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireDrmDisplayEXT.html>
    #[inline]
    pub unsafe fn acquire_drm_display_ext(
        &self,
        physical_device: PhysicalDevice,
        drm_fd: i32,
        display: DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_drm_display_ext)(physical_device, drm_fd, display);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetDrmDisplayEXT.html>
    #[inline]
    pub unsafe fn get_drm_display_ext(
        &self,
        physical_device: PhysicalDevice,
        drm_fd: i32,
        connector_id: u32,
    ) -> crate::Result<DisplayKHR> {
        unsafe {
            let mut display = core::mem::MaybeUninit::uninit();
            let result = (self.get_drm_display_ext)(
                physical_device,
                drm_fd,
                connector_id,
                display.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(display.assume_init()),
                err => Err(err),
            }
        }
    }
}
