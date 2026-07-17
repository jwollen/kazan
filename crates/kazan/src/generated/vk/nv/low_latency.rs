//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_low_latency.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_low_latency";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryLowLatencySupportNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueryLowLatencySupportNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_queried_low_latency_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueryLowLatencySupportNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueryLowLatencySupportNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "p_queried_low_latency_data",
                    &self.p_queried_low_latency_data,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueryLowLatencySupportNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUERY_LOW_LATENCY_SUPPORT_NV;
    }

    unsafe impl Extends<SemaphoreCreateInfo<'_>> for QueryLowLatencySupportNV<'_> {}

    impl Default for QueryLowLatencySupportNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_queried_low_latency_data: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueryLowLatencySupportNV<'a> {
        #[inline]
        pub fn queried_low_latency_data(mut self, queried_low_latency_data: *mut c_void) -> Self {
            self.p_queried_low_latency_data = queried_low_latency_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLatencySleepModeLegacyNV.html>
    pub type PFN_vkSetLatencySleepModeLegacyNV = unsafe extern "system" fn(
        device: Device,
        low_latency_mode: Bool32,
        low_latency_boost: Bool32,
        minimum_interval_us: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkLatencySleepLegacyNV.html>
    pub type PFN_vkLatencySleepLegacyNV =
        unsafe extern "system" fn(device: Device, signal_semaphore: Semaphore, value: u64);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLatencyMarkerLegacyNV.html>
    pub type PFN_vkSetLatencyMarkerLegacyNV =
        unsafe extern "system" fn(device: Device, frame_id: u64, marker: u32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetLatencyTimingsLegacyNV.html>
    pub type PFN_vkGetLatencyTimingsLegacyNV =
        unsafe extern "system" fn(device: Device, p_timings: *mut c_void);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueNotifyOutOfBandLegacyNV.html>
    pub type PFN_vkQueueNotifyOutOfBandLegacyNV =
        unsafe extern "system" fn(queue: Queue, queue_type: u32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSleepStatusLegacyNV.html>
    pub type PFN_vkGetSleepStatusLegacyNV =
        unsafe extern "system" fn(device: Device, p_low_latency_mode: *mut Bool32);
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkShutdownLatencyDeviceLegacyNV.html>
    pub type PFN_vkShutdownLatencyDeviceLegacyNV = unsafe extern "system" fn(device: Device);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkQueryLowLatencySupportNV = QueryLowLatencySupportNV<'static>;
    impl QueryLowLatencySupportNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkQueryLowLatencySupportNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    set_latency_sleep_mode_legacy: PFN_vkSetLatencySleepModeLegacyNV,
    latency_sleep_legacy: PFN_vkLatencySleepLegacyNV,
    set_latency_marker_legacy: PFN_vkSetLatencyMarkerLegacyNV,
    get_latency_timings_legacy: PFN_vkGetLatencyTimingsLegacyNV,
    queue_notify_out_of_band_legacy: PFN_vkQueueNotifyOutOfBandLegacyNV,
    get_sleep_status_legacy: PFN_vkGetSleepStatusLegacyNV,
    shutdown_latency_device_legacy: PFN_vkShutdownLatencyDeviceLegacyNV,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                set_latency_sleep_mode_legacy: transmute(
                    load(c"vkSetLatencySleepModeLegacyNV").ok_or(MissingEntryPointError)?,
                ),
                latency_sleep_legacy: transmute(
                    load(c"vkLatencySleepLegacyNV").ok_or(MissingEntryPointError)?,
                ),
                set_latency_marker_legacy: transmute(
                    load(c"vkSetLatencyMarkerLegacyNV").ok_or(MissingEntryPointError)?,
                ),
                get_latency_timings_legacy: transmute(
                    load(c"vkGetLatencyTimingsLegacyNV").ok_or(MissingEntryPointError)?,
                ),
                queue_notify_out_of_band_legacy: transmute(
                    load(c"vkQueueNotifyOutOfBandLegacyNV").ok_or(MissingEntryPointError)?,
                ),
                get_sleep_status_legacy: transmute(
                    load(c"vkGetSleepStatusLegacyNV").ok_or(MissingEntryPointError)?,
                ),
                shutdown_latency_device_legacy: transmute(
                    load(c"vkShutdownLatencyDeviceLegacyNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLatencySleepModeLegacyNV.html>
    #[inline]
    pub unsafe fn set_latency_sleep_mode_legacy(
        &self,
        device: Device,
        low_latency_mode: bool,
        low_latency_boost: bool,
        minimum_interval_us: u32,
    ) {
        unsafe {
            (self.set_latency_sleep_mode_legacy)(
                device,
                low_latency_mode.into(),
                low_latency_boost.into(),
                minimum_interval_us,
            )
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkLatencySleepLegacyNV.html>
    #[inline]
    pub unsafe fn latency_sleep_legacy(
        &self,
        device: Device,
        signal_semaphore: Semaphore,
        value: u64,
    ) {
        unsafe { (self.latency_sleep_legacy)(device, signal_semaphore, value) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLatencyMarkerLegacyNV.html>
    #[inline]
    pub unsafe fn set_latency_marker_legacy(&self, device: Device, frame_id: u64, marker: u32) {
        unsafe { (self.set_latency_marker_legacy)(device, frame_id, marker) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetLatencyTimingsLegacyNV.html>
    #[inline]
    pub unsafe fn get_latency_timings_legacy(&self, device: Device, timings: *mut c_void) {
        unsafe { (self.get_latency_timings_legacy)(device, timings) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueNotifyOutOfBandLegacyNV.html>
    #[inline]
    pub unsafe fn queue_notify_out_of_band_legacy(&self, queue: Queue, queue_type: u32) {
        unsafe { (self.queue_notify_out_of_band_legacy)(queue, queue_type) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSleepStatusLegacyNV.html>
    #[inline]
    pub unsafe fn get_sleep_status_legacy(&self, device: Device) -> bool {
        unsafe {
            let mut low_latency_mode = core::mem::MaybeUninit::uninit();
            (self.get_sleep_status_legacy)(device, low_latency_mode.as_mut_ptr());
            low_latency_mode.assume_init() != 0
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkShutdownLatencyDeviceLegacyNV.html>
    #[inline]
    pub unsafe fn shutdown_latency_device_legacy(&self, device: Device) {
        unsafe { (self.shutdown_latency_device_legacy)(device) }
    }
}
