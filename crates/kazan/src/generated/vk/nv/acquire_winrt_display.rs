#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    pub type PFN_vkAcquireWinrtDisplayNV = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> vk::Result;
    pub type PFN_vkGetWinrtDisplayNV = unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        device_relative_id: u32,
        p_display: *mut DisplayKHR,
    ) -> vk::Result;
}
pub struct InstanceFn {
    acquire_winrt_display_nv: PFN_vkAcquireWinrtDisplayNV,
    get_winrt_display_nv: PFN_vkGetWinrtDisplayNV,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                acquire_winrt_display_nv: transmute(
                    load(c"vkAcquireWinrtDisplayNV").ok_or(MissingEntryPointError)?,
                ),
                get_winrt_display_nv: transmute(
                    load(c"vkGetWinrtDisplayNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn acquire_winrt_display_nv(
        &self,
        physical_device: PhysicalDevice,
        display: DisplayKHR,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.acquire_winrt_display_nv)(physical_device, display);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_winrt_display_nv(
        &self,
        physical_device: PhysicalDevice,
        device_relative_id: u32,
    ) -> crate::Result<DisplayKHR> {
        unsafe {
            let mut display = core::mem::MaybeUninit::uninit();
            let result = (self.get_winrt_display_nv)(
                physical_device,
                device_relative_id,
                display.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(display.assume_init()),
                err => Err(err),
            }
        }
    }
}
