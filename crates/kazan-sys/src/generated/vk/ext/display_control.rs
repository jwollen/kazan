#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayPowerInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub power_state: DisplayPowerStateEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayPowerInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_POWER_INFO_EXT,
            p_next: core::ptr::null(),
            power_state: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DisplayPowerInfoEXT<'a> {
    pub fn power_state(mut self, power_state: DisplayPowerStateEXT) -> Self {
        self.power_state = power_state;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceEventInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_event: DeviceEventTypeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DeviceEventInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEVICE_EVENT_INFO_EXT,
            p_next: core::ptr::null(),
            device_event: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DeviceEventInfoEXT<'a> {
    pub fn device_event(mut self, device_event: DeviceEventTypeEXT) -> Self {
        self.device_event = device_event;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DisplayEventInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub display_event: DisplayEventTypeEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DisplayEventInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DISPLAY_EVENT_INFO_EXT,
            p_next: core::ptr::null(),
            display_event: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> DisplayEventInfoEXT<'a> {
    pub fn display_event(mut self, display_event: DisplayEventTypeEXT) -> Self {
        self.display_event = display_event;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SwapchainCounterCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub surface_counters: SurfaceCounterFlagsEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for SwapchainCounterCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SWAPCHAIN_COUNTER_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            surface_counters: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> SwapchainCounterCreateInfoEXT<'a> {
    pub fn surface_counters(mut self, surface_counters: SurfaceCounterFlagsEXT) -> Self {
        self.surface_counters = surface_counters;
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayPowerStateEXT(i32);
impl DisplayPowerStateEXT {
    pub const OFF_EXT: Self = Self(0);
    pub const SUSPEND_EXT: Self = Self(1);
    pub const ON_EXT: Self = Self(2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceEventTypeEXT(i32);
impl DeviceEventTypeEXT {
    pub const DISPLAY_HOTPLUG_EXT: Self = Self(0);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayEventTypeEXT(i32);
impl DisplayEventTypeEXT {
    pub const FIRST_PIXEL_OUT_EXT: Self = Self(0);
}
pub type PFN_vkDisplayPowerControlEXT = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_power_info: *const DisplayPowerInfoEXT<'_>,
) -> Result;
pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
    device: Device,
    p_device_event_info: *const DeviceEventInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_fence: *mut Fence,
) -> Result;
pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
    device: Device,
    display: DisplayKHR,
    p_display_event_info: *const DisplayEventInfoEXT<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_fence: *mut Fence,
) -> Result;
pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagBitsEXT,
    p_counter_value: *mut u64,
) -> Result;
