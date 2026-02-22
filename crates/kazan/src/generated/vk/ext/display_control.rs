#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
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
            result((self.display_power_control_ext)(
                device,
                display,
                display_power_info,
            ))
        }
    }
    pub unsafe fn register_device_event_ext(
        &self,
        device: Device,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        fence: &mut Fence,
    ) -> crate::Result<()> {
        unsafe {
            result((self.register_device_event_ext)(
                device,
                device_event_info,
                allocator.to_raw_ptr(),
                fence,
            ))
        }
    }
    pub unsafe fn register_display_event_ext(
        &self,
        device: Device,
        display: DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        fence: &mut Fence,
    ) -> crate::Result<()> {
        unsafe {
            result((self.register_display_event_ext)(
                device,
                display,
                display_event_info,
                allocator.to_raw_ptr(),
                fence,
            ))
        }
    }
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagBitsEXT,
        counter_value: &mut u64,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_swapchain_counter_ext)(
                device,
                swapchain,
                counter,
                counter_value,
            ))
        }
    }
}
