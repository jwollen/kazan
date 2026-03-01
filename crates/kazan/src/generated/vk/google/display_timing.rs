#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct RefreshCycleDurationGOOGLE {
        pub refresh_duration: u64,
    }
    impl RefreshCycleDurationGOOGLE {
        pub fn refresh_duration(mut self, refresh_duration: u64) -> Self {
            self.refresh_duration = refresh_duration;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PastPresentationTimingGOOGLE {
        pub present_id: u32,
        pub desired_present_time: u64,
        pub actual_present_time: u64,
        pub earliest_present_time: u64,
        pub present_margin: u64,
    }
    impl PastPresentationTimingGOOGLE {
        pub fn present_id(mut self, present_id: u32) -> Self {
            self.present_id = present_id;
            self
        }
        pub fn desired_present_time(mut self, desired_present_time: u64) -> Self {
            self.desired_present_time = desired_present_time;
            self
        }
        pub fn actual_present_time(mut self, actual_present_time: u64) -> Self {
            self.actual_present_time = actual_present_time;
            self
        }
        pub fn earliest_present_time(mut self, earliest_present_time: u64) -> Self {
            self.earliest_present_time = earliest_present_time;
            self
        }
        pub fn present_margin(mut self, present_margin: u64) -> Self {
            self.present_margin = present_margin;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PresentTimesInfoGOOGLE<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_times: *const PresentTimeGOOGLE,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PresentTimesInfoGOOGLE<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_TIMES_INFO_GOOGLE;
    }
    unsafe impl<'a> Extends<PresentInfoKHR<'a>> for PresentTimesInfoGOOGLE<'a> {}
    impl Default for PresentTimesInfoGOOGLE<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                swapchain_count: Default::default(),
                p_times: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PresentTimesInfoGOOGLE<'a> {
        pub fn times(mut self, times: &'a [PresentTimeGOOGLE]) -> Self {
            self.swapchain_count = times.len().try_into().unwrap();
            self.p_times = times.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PresentTimeGOOGLE {
        pub present_id: u32,
        pub desired_present_time: u64,
    }
    impl PresentTimeGOOGLE {
        pub fn present_id(mut self, present_id: u32) -> Self {
            self.present_id = present_id;
            self
        }
        pub fn desired_present_time(mut self, desired_present_time: u64) -> Self {
            self.desired_present_time = desired_present_time;
            self
        }
    }
    pub type PFN_vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
    ) -> vk::Result;
    pub type PFN_vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_presentation_timing_count: *mut u32,
        p_presentation_timings: *mut PastPresentationTimingGOOGLE,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_refresh_cycle_duration_google: PFN_vkGetRefreshCycleDurationGOOGLE,
    get_past_presentation_timing_google: PFN_vkGetPastPresentationTimingGOOGLE,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_refresh_cycle_duration_google: transmute(
                    load(c"vkGetRefreshCycleDurationGOOGLE").ok_or(LoadingError)?,
                ),
                get_past_presentation_timing_google: transmute(
                    load(c"vkGetPastPresentationTimingGOOGLE").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_refresh_cycle_duration_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
    ) -> crate::Result<RefreshCycleDurationGOOGLE> {
        unsafe {
            let mut display_timing_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_refresh_cycle_duration_google)(
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
    pub unsafe fn get_past_presentation_timing_google(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        presentation_timings: impl ExtendUninit<PastPresentationTimingGOOGLE>,
    ) -> crate::Result<()> {
        unsafe {
            try_extend_uninit(
                presentation_timings,
                |presentation_timing_count, presentation_timings| {
                    let result = (self.get_past_presentation_timing_google)(
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
                },
            )
        }
    }
}
