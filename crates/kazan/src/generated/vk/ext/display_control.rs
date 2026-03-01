#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
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
    unsafe impl<'a> TaggedStructure<'a> for DisplayPowerInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_POWER_INFO_EXT;
    }
    impl Default for DisplayPowerInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    unsafe impl<'a> TaggedStructure<'a> for DeviceEventInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DEVICE_EVENT_INFO_EXT;
    }
    impl Default for DeviceEventInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    unsafe impl<'a> TaggedStructure<'a> for DisplayEventInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_EVENT_INFO_EXT;
    }
    impl Default for DisplayEventInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    unsafe impl<'a> TaggedStructure<'a> for SwapchainCounterCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_COUNTER_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<SwapchainCreateInfoKHR<'a>> for SwapchainCounterCreateInfoEXT<'a> {}
    impl Default for SwapchainCounterCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    ) -> vk::Result;
    pub type PFN_vkRegisterDeviceEventEXT = unsafe extern "system" fn(
        device: Device,
        p_device_event_info: *const DeviceEventInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> vk::Result;
    pub type PFN_vkRegisterDisplayEventEXT = unsafe extern "system" fn(
        device: Device,
        display: DisplayKHR,
        p_display_event_info: *const DisplayEventInfoEXT<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_fence: *mut Fence,
    ) -> vk::Result;
    pub type PFN_vkGetSwapchainCounterEXT = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagBitsEXT,
        p_counter_value: *mut u64,
    ) -> vk::Result;
}
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
        display_power_info: &DisplayPowerInfoEXT<'_>,
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
        device_event_info: &DeviceEventInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
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
        display_event_info: &DisplayEventInfoEXT<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
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
