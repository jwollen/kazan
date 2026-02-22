#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct InstanceFn {
    get_physical_device_surface_present_modes2_ext: PFN_vkGetPhysicalDeviceSurfacePresentModes2EXT,
}
impl InstanceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_physical_device_surface_present_modes2_ext: transmute(
                    load(c"vkGetPhysicalDeviceSurfacePresentModes2EXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl InstanceFn {
    pub unsafe fn get_physical_device_surface_present_modes2_ext(
        &self,
        physical_device: PhysicalDevice,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        present_modes: impl ExtendUninit<PresentModeKHR>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(present_modes, |present_mode_count, present_modes| {
                result((self.get_physical_device_surface_present_modes2_ext)(
                    physical_device,
                    surface_info,
                    present_mode_count,
                    present_modes as _,
                ))
            })
        }
    }
}
pub struct DeviceFn {
    acquire_full_screen_exclusive_mode_ext: PFN_vkAcquireFullScreenExclusiveModeEXT,
    release_full_screen_exclusive_mode_ext: PFN_vkReleaseFullScreenExclusiveModeEXT,
    get_device_group_surface_present_modes2_ext:
        Option<PFN_vkGetDeviceGroupSurfacePresentModes2EXT>,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                acquire_full_screen_exclusive_mode_ext: transmute(
                    load(c"vkAcquireFullScreenExclusiveModeEXT").ok_or(LoadingError)?,
                ),
                release_full_screen_exclusive_mode_ext: transmute(
                    load(c"vkReleaseFullScreenExclusiveModeEXT").ok_or(LoadingError)?,
                ),
                get_device_group_surface_present_modes2_ext: transmute(load(
                    c"vkGetDeviceGroupSurfacePresentModes2EXT",
                )),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn acquire_full_screen_exclusive_mode_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.acquire_full_screen_exclusive_mode_ext)(
                device, swapchain,
            ))
        }
    }
    pub unsafe fn release_full_screen_exclusive_mode_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.release_full_screen_exclusive_mode_ext)(
                device, swapchain,
            ))
        }
    }
    pub unsafe fn get_device_group_surface_present_modes2_ext(
        &self,
        device: Device,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        modes: &mut DeviceGroupPresentModeFlagsKHR,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_device_group_surface_present_modes2_ext.unwrap())(
                device,
                surface_info,
                modes,
            ))
        }
    }
}
