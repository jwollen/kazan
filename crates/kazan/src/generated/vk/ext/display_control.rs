#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    display_power_control_ext: PFN_vkDisplayPowerControlEXT,
    register_device_event_ext: PFN_vkRegisterDeviceEventEXT,
    register_display_event_ext: PFN_vkRegisterDisplayEventEXT,
    get_swapchain_counter_ext: PFN_vkGetSwapchainCounterEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                display_power_control_ext: transmute(
                    load(c"vkDisplayPowerControlEXT").ok_or(LoadingError)?,
                ),
                register_device_event_ext: transmute(
                    load(c"vkRegisterDeviceEventEXT").ok_or(LoadingError)?,
                ),
                register_display_event_ext: transmute(
                    load(c"vkRegisterDisplayEventEXT").ok_or(LoadingError)?,
                ),
                get_swapchain_counter_ext: transmute(
                    load(c"vkGetSwapchainCounterEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn display_power_control_ext(
        &self,
        device: Device,
        display: DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.display_power_control_ext)(device, display, display_power_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn register_device_event_ext(
        &self,
        device: Device,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Fence> {
        unsafe {
            let mut fence = core::mem::MaybeUninit::uninit();
            let result = (self.register_device_event_ext)(
                device,
                device_event_info,
                allocator.to_raw_ptr(),
                fence.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(fence.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn register_display_event_ext(
        &self,
        device: Device,
        display: DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> crate::Result<Fence> {
        unsafe {
            let mut fence = core::mem::MaybeUninit::uninit();
            let result = (self.register_display_event_ext)(
                device,
                display,
                display_event_info,
                allocator.to_raw_ptr(),
                fence.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(fence.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagBitsEXT,
    ) -> crate::Result<u64> {
        unsafe {
            let mut counter_value = core::mem::MaybeUninit::uninit();
            let result = (self.get_swapchain_counter_ext)(
                device,
                swapchain,
                counter,
                counter_value.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(counter_value.assume_init()),
                err => Err(err),
            }
        }
    }
}
