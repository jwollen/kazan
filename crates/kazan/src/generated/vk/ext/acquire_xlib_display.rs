//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_acquire_xlib_display.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_acquire_xlib_display";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireXlibDisplayEXT.html>
    pub type PFN_vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetRandROutputDisplayEXT.html>
    pub type PFN_vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
        p_display: *mut DisplayKHR,
    ) -> vk::Result;
}

pub struct InstanceFn {
    acquire_xlib_display: PFN_vkAcquireXlibDisplayEXT,
    get_rand_r_output_display: PFN_vkGetRandROutputDisplayEXT,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                acquire_xlib_display: transmute(
                    load(c"vkAcquireXlibDisplayEXT").ok_or(MissingEntryPointError)?,
                ),
                get_rand_r_output_display: transmute(
                    load(c"vkGetRandROutputDisplayEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkAcquireXlibDisplayEXT.html>
    #[inline]
    pub unsafe fn acquire_xlib_display(
        &self,
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        display: DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_xlib_display)(physical_device, dpy, display);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetRandROutputDisplayEXT.html>
    #[inline]
    pub unsafe fn get_rand_r_output_display(
        &self,
        physical_device: PhysicalDevice,
        dpy: *mut Display,
        rr_output: RROutput,
    ) -> crate::Result<DisplayKHR> {
        unsafe {
            let mut display = core::mem::MaybeUninit::uninit();
            let result = (self.get_rand_r_output_display)(
                physical_device,
                dpy,
                rr_output,
                display.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(display.assume_init()),
                err => Err(err),
            }
        }
    }
}
