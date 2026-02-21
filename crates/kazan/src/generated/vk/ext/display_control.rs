#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    display_power_control_ext: PFN_vkDisplayPowerControlEXT,
    register_device_event_ext: PFN_vkRegisterDeviceEventEXT,
    register_display_event_ext: PFN_vkRegisterDisplayEventEXT,
    get_swapchain_counter_ext: PFN_vkGetSwapchainCounterEXT,
}
impl DeviceFn {
    pub unsafe fn display_power_control_ext(
        &self,
        device: Device,
        display: DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> Result {
        unsafe { (self.display_power_control_ext)(device, display, display_power_info) }
    }
    pub unsafe fn register_device_event_ext(
        &self,
        device: Device,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        fence: &mut Fence,
    ) -> Result {
        unsafe {
            (self.register_device_event_ext)(
                device,
                device_event_info,
                allocator.to_raw_ptr(),
                fence,
            )
        }
    }
    pub unsafe fn register_display_event_ext(
        &self,
        device: Device,
        display: DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
        fence: &mut Fence,
    ) -> Result {
        unsafe {
            (self.register_display_event_ext)(
                device,
                display,
                display_event_info,
                allocator.to_raw_ptr(),
                fence,
            )
        }
    }
    pub unsafe fn get_swapchain_counter_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
        counter_value: &mut u64,
    ) -> Result {
        unsafe { (self.get_swapchain_counter_ext)(device, swapchain, counter, counter_value) }
    }
}
