#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_direct_mode_display";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseDisplayEXT.html>
    pub type PFN_vkReleaseDisplayEXT = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> vk::Result;
}

pub struct InstanceFn {
    release_display_ext: PFN_vkReleaseDisplayEXT,
}

impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                release_display_ext: transmute(
                    load(c"vkReleaseDisplayEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkReleaseDisplayEXT.html>
    #[inline]
    pub unsafe fn release_display_ext(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.release_display_ext)(physical_device, display);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
