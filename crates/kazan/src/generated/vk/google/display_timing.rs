//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_GOOGLE_display_timing.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_GOOGLE_display_timing";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRefreshCycleDurationGOOGLE.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct RefreshCycleDurationGOOGLE {
        pub refresh_duration: u64,
    }

    impl RefreshCycleDurationGOOGLE {
        #[inline]
        pub fn refresh_duration(mut self, refresh_duration: u64) -> Self {
            self.refresh_duration = refresh_duration;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPastPresentationTimingGOOGLE.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct PastPresentationTimingGOOGLE {
        pub present_id: u32,
        pub desired_present_time: u64,
        pub actual_present_time: u64,
        pub earliest_present_time: u64,
        pub present_margin: u64,
    }

    impl PastPresentationTimingGOOGLE {
        #[inline]
        pub fn present_id(mut self, present_id: u32) -> Self {
            self.present_id = present_id;
            self
        }

        #[inline]
        pub fn desired_present_time(mut self, desired_present_time: u64) -> Self {
            self.desired_present_time = desired_present_time;
            self
        }

        #[inline]
        pub fn actual_present_time(mut self, actual_present_time: u64) -> Self {
            self.actual_present_time = actual_present_time;
            self
        }

        #[inline]
        pub fn earliest_present_time(mut self, earliest_present_time: u64) -> Self {
            self.earliest_present_time = earliest_present_time;
            self
        }

        #[inline]
        pub fn present_margin(mut self, present_margin: u64) -> Self {
            self.present_margin = present_margin;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentTimesInfoGOOGLE.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentTimesInfoGOOGLE<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_times: *const PresentTimeGOOGLE,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentTimesInfoGOOGLE<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentTimesInfoGOOGLE")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_times", &self.p_times)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentTimesInfoGOOGLE<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_TIMES_INFO_GOOGLE;
    }

    unsafe impl Extends<PresentInfoKHR<'_>> for PresentTimesInfoGOOGLE<'_> {}

    impl Default for PresentTimesInfoGOOGLE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain_count: Default::default(),
                p_times: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentTimesInfoGOOGLE<'a> {
        #[inline]
        pub fn times(mut self, times: &'a [PresentTimeGOOGLE]) -> Self {
            self.swapchain_count = times.len().try_into().unwrap();
            self.p_times = times.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentTimeGOOGLE.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct PresentTimeGOOGLE {
        pub present_id: u32,
        pub desired_present_time: u64,
    }

    impl PresentTimeGOOGLE {
        #[inline]
        pub fn present_id(mut self, present_id: u32) -> Self {
            self.present_id = present_id;
            self
        }

        #[inline]
        pub fn desired_present_time(mut self, desired_present_time: u64) -> Self {
            self.desired_present_time = desired_present_time;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetRefreshCycleDurationGOOGLE.html>
    pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPastPresentationTimingGOOGLE.html>
    pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut PastPresentationTimingGOOGLE,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkRefreshCycleDurationGOOGLE = RefreshCycleDurationGOOGLE;
    pub type VkPastPresentationTimingGOOGLE = PastPresentationTimingGOOGLE;
    pub type VkPresentTimesInfoGOOGLE = PresentTimesInfoGOOGLE<'static>;
    pub type VkPresentTimeGOOGLE = PresentTimeGOOGLE;
    impl PresentTimesInfoGOOGLE<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentTimesInfoGOOGLE {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_refresh_cycle_duration: PFN_vkGetRefreshCycleDurationGOOGLE,
    get_past_presentation_timing: PFN_vkGetPastPresentationTimingGOOGLE,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_refresh_cycle_duration: transmute(
                    load(c"vkGetRefreshCycleDurationGOOGLE").ok_or(MissingEntryPointError)?,
                ),
                get_past_presentation_timing: transmute(
                    load(c"vkGetPastPresentationTimingGOOGLE").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetRefreshCycleDurationGOOGLE.html>
    #[inline]
    pub unsafe fn get_refresh_cycle_duration(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<RefreshCycleDurationGOOGLE> {
        unsafe {
            let mut display_timing_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_refresh_cycle_duration)(
                device,
                swapchain,
                display_timing_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(display_timing_properties.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPastPresentationTimingGOOGLE.html>
    #[inline]
    pub unsafe fn get_past_presentation_timing(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        mut presentation_timings: impl EnumerateInto<PastPresentationTimingGOOGLE>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |presentation_timing_count, presentation_timings| {
                let result = (self.get_past_presentation_timing)(
                    device,
                    swapchain,
                    presentation_timing_count,
                    presentation_timings as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let presentation_timings_buf = presentation_timings.reserve(capacity);
            len = presentation_timings_buf.len().try_into().unwrap();
            let result = call(&mut len, presentation_timings_buf.as_mut_ptr() as *mut _)?;
            presentation_timings.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }
}
