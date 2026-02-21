#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_surface_present_modes2_ext: PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
}
impl InstanceFn {
    pub unsafe fn get_physical_device_surface_present_modes2_ext(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        present_modes: impl ExtendUninit<PresentModeKHR>,
    ) -> Result {
        unsafe {
            try_extend_uninit(present_modes, |present_mode_count, present_modes| {
                (self.get_physical_device_surface_present_modes2_ext)(
                    physical_device,
                    surface_info,
                    present_mode_count,
                    present_modes as _,
                )
            })
        }
    }
}
pub struct DeviceFn {
    acquire_full_screen_exclusive_mode_ext: PFN_vkAcquireFullScreenExclusiveModeEXT,
    release_full_screen_exclusive_mode_ext: PFN_vkReleaseFullScreenExclusiveModeEXT,
    get_device_group_surface_present_modes2_ext: PFN_vkGetDeviceGroupSurfacePresentModes2EXT,
}
impl DeviceFn {
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> Result {
        unsafe { (self.acquire_full_screen_exclusive_mode_ext)(device, swapchain) }
    }
    pub unsafe fn release_full_screen_exclusive_mode_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> Result {
        unsafe { (self.release_full_screen_exclusive_mode_ext)(device, swapchain) }
    }
    pub unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        device: Device,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        modes: &mut DeviceGroupPresentModeFlagsKHR,
    ) -> Result {
        unsafe { (self.get_device_group_surface_present_modes2_ext)(device, surface_info, modes) }
    }
}
