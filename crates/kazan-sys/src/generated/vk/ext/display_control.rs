#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPowerInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub power_state: DisplayPowerStateEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_event: DeviceEventTypeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayEventInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_event: DisplayEventTypeEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainCounterCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_counters: SurfaceCounterFlagsEXT,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayPowerStateEXT(i32);
impl DisplayPowerStateEXT {
    pub const OFF_EXT: Self = Self(0);
    pub const SUSPEND_EXT: Self = Self(1);
    pub const ON_EXT: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceEventTypeEXT(i32);
impl DeviceEventTypeEXT {
    pub const DISPLAY_HOTPLUG_EXT: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayEventTypeEXT(i32);
impl DisplayEventTypeEXT {
    pub const FIRST_PIXEL_OUT_EXT: Self = Self(0);
}
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_power_info: *const DisplayPowerInfoEXT,
) -> Result;
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
    device: Device,
    p_device_event_info: *const DeviceEventInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> Result;
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_event_info: *const DisplayEventInfoEXT,
    p_allocator: *const AllocationCallbacks,
    p_fence: *mut Fence,
) -> Result;
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagBitsEXT,
    p_counter_value: *mut u64,
) -> Result;
