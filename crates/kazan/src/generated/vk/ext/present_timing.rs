//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_present_timing.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_present_timing";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePresentTimingFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePresentTimingFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_timing: Bool32,
        pub present_at_absolute_time: Bool32,
        pub present_at_relative_time: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePresentTimingFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePresentTimingFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_timing", &self.present_timing)
                .field("present_at_absolute_time", &self.present_at_absolute_time)
                .field("present_at_relative_time", &self.present_at_relative_time)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentTimingFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDevicePresentTimingFeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePresentTimingFeaturesEXT<'_> {}

    impl Default for PhysicalDevicePresentTimingFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_timing: Default::default(),
                present_at_absolute_time: Default::default(),
                present_at_relative_time: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePresentTimingFeaturesEXT<'a> {
        #[inline]
        pub fn present_timing(mut self, present_timing: bool) -> Self {
            self.present_timing = present_timing.into();
            self
        }

        #[inline]
        pub fn present_at_absolute_time(mut self, present_at_absolute_time: bool) -> Self {
            self.present_at_absolute_time = present_at_absolute_time.into();
            self
        }

        #[inline]
        pub fn present_at_relative_time(mut self, present_at_relative_time: bool) -> Self {
            self.present_at_relative_time = present_at_relative_time.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentTimingSurfaceCapabilitiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentTimingSurfaceCapabilitiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_timing_supported: Bool32,
        pub present_at_absolute_time_supported: Bool32,
        pub present_at_relative_time_supported: Bool32,
        pub present_stage_queries: PresentStageFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentTimingSurfaceCapabilitiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentTimingSurfaceCapabilitiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_timing_supported", &self.present_timing_supported)
                .field(
                    "present_at_absolute_time_supported",
                    &self.present_at_absolute_time_supported,
                )
                .field(
                    "present_at_relative_time_supported",
                    &self.present_at_relative_time_supported,
                )
                .field("present_stage_queries", &self.present_stage_queries)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentTimingSurfaceCapabilitiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PRESENT_TIMING_SURFACE_CAPABILITIES_EXT;
    }

    unsafe impl Extends<SurfaceCapabilities2KHR<'_>> for PresentTimingSurfaceCapabilitiesEXT<'_> {}

    impl Default for PresentTimingSurfaceCapabilitiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_timing_supported: Default::default(),
                present_at_absolute_time_supported: Default::default(),
                present_at_relative_time_supported: Default::default(),
                present_stage_queries: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentTimingSurfaceCapabilitiesEXT<'a> {
        #[inline]
        pub fn present_timing_supported(mut self, present_timing_supported: bool) -> Self {
            self.present_timing_supported = present_timing_supported.into();
            self
        }

        #[inline]
        pub fn present_at_absolute_time_supported(
            mut self,
            present_at_absolute_time_supported: bool,
        ) -> Self {
            self.present_at_absolute_time_supported = present_at_absolute_time_supported.into();
            self
        }

        #[inline]
        pub fn present_at_relative_time_supported(
            mut self,
            present_at_relative_time_supported: bool,
        ) -> Self {
            self.present_at_relative_time_supported = present_at_relative_time_supported.into();
            self
        }

        #[inline]
        pub fn present_stage_queries(
            mut self,
            present_stage_queries: PresentStageFlagsEXT,
        ) -> Self {
            self.present_stage_queries = present_stage_queries;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainTimingPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainTimingPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub refresh_duration: u64,
        pub refresh_interval: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainTimingPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainTimingPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("refresh_duration", &self.refresh_duration)
                .field("refresh_interval", &self.refresh_interval)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainTimingPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_TIMING_PROPERTIES_EXT;
    }

    impl Default for SwapchainTimingPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                refresh_duration: Default::default(),
                refresh_interval: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainTimingPropertiesEXT<'a> {
        #[inline]
        pub fn refresh_duration(mut self, refresh_duration: u64) -> Self {
            self.refresh_duration = refresh_duration;
            self
        }

        #[inline]
        pub fn refresh_interval(mut self, refresh_interval: u64) -> Self {
            self.refresh_interval = refresh_interval;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainTimeDomainPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainTimeDomainPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub time_domain_count: u32,
        pub p_time_domains: *mut TimeDomainKHR,
        pub p_time_domain_ids: *mut u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainTimeDomainPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainTimeDomainPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("time_domain_count", &self.time_domain_count)
                .field("p_time_domains", &self.p_time_domains)
                .field("p_time_domain_ids", &self.p_time_domain_ids)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainTimeDomainPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT;
    }

    impl Default for SwapchainTimeDomainPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                time_domain_count: Default::default(),
                p_time_domains: ptr::null_mut(),
                p_time_domain_ids: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainTimeDomainPropertiesEXT<'a> {
        #[inline]
        pub fn time_domains(
            mut self,
            time_domains: Option<&'a mut [TimeDomainKHR]>,
            time_domain_ids: Option<&'a mut [u64]>,
        ) -> Self {
            self.time_domain_count = None
                .or_else(|| time_domains.as_deref().map(|s| s.len()))
                .or_else(|| time_domain_ids.as_deref().map(|s| s.len()))
                .unwrap_or(0)
                .try_into()
                .unwrap();
            if let Some(s) = &time_domain_ids {
                assert_eq!(s.len(), self.time_domain_count as usize);
            }
            self.p_time_domains = time_domains.map_or(ptr::null_mut(), |s| s.as_mut_ptr() as _);
            self.p_time_domain_ids =
                time_domain_ids.map_or(ptr::null_mut(), |s| s.as_mut_ptr() as _);
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentStageTimeEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct PresentStageTimeEXT {
        pub stage: PresentStageFlagsEXT,
        pub time: u64,
    }

    impl PresentStageTimeEXT {
        #[inline]
        pub fn stage(mut self, stage: PresentStageFlagsEXT) -> Self {
            self.stage = stage;
            self
        }

        #[inline]
        pub fn time(mut self, time: u64) -> Self {
            self.time = time;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPastPresentationTimingInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PastPresentationTimingInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PastPresentationTimingFlagsEXT,
        pub swapchain: SwapchainKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PastPresentationTimingInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PastPresentationTimingInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("swapchain", &self.swapchain)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PastPresentationTimingInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PAST_PRESENTATION_TIMING_INFO_EXT;
    }

    impl Default for PastPresentationTimingInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                swapchain: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PastPresentationTimingInfoEXT<'a> {
        #[inline]
        pub fn flags(mut self, flags: PastPresentationTimingFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
            self.swapchain = swapchain;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPastPresentationTimingPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PastPresentationTimingPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub timing_properties_counter: u64,
        pub time_domains_counter: u64,
        pub presentation_timing_count: u32,
        pub p_presentation_timings: *mut PastPresentationTimingEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PastPresentationTimingPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PastPresentationTimingPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("timing_properties_counter", &self.timing_properties_counter)
                .field("time_domains_counter", &self.time_domains_counter)
                .field("presentation_timing_count", &self.presentation_timing_count)
                .field("p_presentation_timings", &self.p_presentation_timings)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PastPresentationTimingPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PAST_PRESENTATION_TIMING_PROPERTIES_EXT;
    }

    impl Default for PastPresentationTimingPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                timing_properties_counter: Default::default(),
                time_domains_counter: Default::default(),
                presentation_timing_count: Default::default(),
                p_presentation_timings: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PastPresentationTimingPropertiesEXT<'a> {
        #[inline]
        pub fn timing_properties_counter(mut self, timing_properties_counter: u64) -> Self {
            self.timing_properties_counter = timing_properties_counter;
            self
        }

        #[inline]
        pub fn time_domains_counter(mut self, time_domains_counter: u64) -> Self {
            self.time_domains_counter = time_domains_counter;
            self
        }

        #[inline]
        pub fn presentation_timings(
            mut self,
            presentation_timings: &'a mut [PastPresentationTimingEXT<'_>],
        ) -> Self {
            self.presentation_timing_count = presentation_timings.len().try_into().unwrap();
            self.p_presentation_timings = presentation_timings.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPastPresentationTimingEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PastPresentationTimingEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_id: u64,
        pub target_time: u64,
        pub present_stage_count: u32,
        pub p_present_stages: *mut PresentStageTimeEXT,
        pub time_domain: TimeDomainKHR,
        pub time_domain_id: u64,
        pub report_complete: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PastPresentationTimingEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PastPresentationTimingEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_id", &self.present_id)
                .field("target_time", &self.target_time)
                .field("present_stage_count", &self.present_stage_count)
                .field("p_present_stages", &self.p_present_stages)
                .field("time_domain", &self.time_domain)
                .field("time_domain_id", &self.time_domain_id)
                .field("report_complete", &self.report_complete)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PastPresentationTimingEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PAST_PRESENTATION_TIMING_EXT;
    }

    impl Default for PastPresentationTimingEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_id: Default::default(),
                target_time: Default::default(),
                present_stage_count: Default::default(),
                p_present_stages: ptr::null_mut(),
                time_domain: Default::default(),
                time_domain_id: Default::default(),
                report_complete: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PastPresentationTimingEXT<'a> {
        #[inline]
        pub fn present_id(mut self, present_id: u64) -> Self {
            self.present_id = present_id;
            self
        }

        #[inline]
        pub fn target_time(mut self, target_time: u64) -> Self {
            self.target_time = target_time;
            self
        }

        #[inline]
        pub fn present_stages(mut self, present_stages: &'a mut [PresentStageTimeEXT]) -> Self {
            self.present_stage_count = present_stages.len().try_into().unwrap();
            self.p_present_stages = present_stages.as_mut_ptr() as _;
            self
        }

        #[inline]
        pub fn time_domain(mut self, time_domain: TimeDomainKHR) -> Self {
            self.time_domain = time_domain;
            self
        }

        #[inline]
        pub fn time_domain_id(mut self, time_domain_id: u64) -> Self {
            self.time_domain_id = time_domain_id;
            self
        }

        #[inline]
        pub fn report_complete(mut self, report_complete: bool) -> Self {
            self.report_complete = report_complete.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentTimingsInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentTimingsInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_timing_infos: *const PresentTimingInfoEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentTimingsInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentTimingsInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_timing_infos", &self.p_timing_infos)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentTimingsInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_TIMINGS_INFO_EXT;
    }

    unsafe impl Extends<PresentInfoKHR<'_>> for PresentTimingsInfoEXT<'_> {}

    impl Default for PresentTimingsInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain_count: Default::default(),
                p_timing_infos: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentTimingsInfoEXT<'a> {
        #[inline]
        pub fn timing_infos(mut self, timing_infos: &'a [PresentTimingInfoEXT<'_>]) -> Self {
            self.swapchain_count = timing_infos.len().try_into().unwrap();
            self.p_timing_infos = timing_infos.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentTimingInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentTimingInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PresentTimingInfoFlagsEXT,
        pub target_time: u64,
        pub time_domain_id: u64,
        pub present_stage_queries: PresentStageFlagsEXT,
        pub target_time_domain_present_stage: PresentStageFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentTimingInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentTimingInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("target_time", &self.target_time)
                .field("time_domain_id", &self.time_domain_id)
                .field("present_stage_queries", &self.present_stage_queries)
                .field(
                    "target_time_domain_present_stage",
                    &self.target_time_domain_present_stage,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentTimingInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_TIMING_INFO_EXT;
    }

    impl Default for PresentTimingInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                target_time: Default::default(),
                time_domain_id: Default::default(),
                present_stage_queries: Default::default(),
                target_time_domain_present_stage: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentTimingInfoEXT<'a> {
        #[inline]
        pub fn flags(mut self, flags: PresentTimingInfoFlagsEXT) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn target_time(mut self, target_time: u64) -> Self {
            self.target_time = target_time;
            self
        }

        #[inline]
        pub fn time_domain_id(mut self, time_domain_id: u64) -> Self {
            self.time_domain_id = time_domain_id;
            self
        }

        #[inline]
        pub fn present_stage_queries(
            mut self,
            present_stage_queries: PresentStageFlagsEXT,
        ) -> Self {
            self.present_stage_queries = present_stage_queries;
            self
        }

        #[inline]
        pub fn target_time_domain_present_stage(
            mut self,
            target_time_domain_present_stage: PresentStageFlagsEXT,
        ) -> Self {
            self.target_time_domain_present_stage = target_time_domain_present_stage;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainCalibratedTimestampInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainCalibratedTimestampInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain: SwapchainKHR,
        pub present_stage: PresentStageFlagsEXT,
        pub time_domain_id: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainCalibratedTimestampInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainCalibratedTimestampInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain", &self.swapchain)
                .field("present_stage", &self.present_stage)
                .field("time_domain_id", &self.time_domain_id)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainCalibratedTimestampInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT;
    }

    unsafe impl Extends<CalibratedTimestampInfoKHR<'_>> for SwapchainCalibratedTimestampInfoEXT<'_> {}

    impl Default for SwapchainCalibratedTimestampInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain: Default::default(),
                present_stage: Default::default(),
                time_domain_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainCalibratedTimestampInfoEXT<'a> {
        #[inline]
        pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
            self.swapchain = swapchain;
            self
        }

        #[inline]
        pub fn present_stage(mut self, present_stage: PresentStageFlagsEXT) -> Self {
            self.present_stage = present_stage;
            self
        }

        #[inline]
        pub fn time_domain_id(mut self, time_domain_id: u64) -> Self {
            self.time_domain_id = time_domain_id;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentStageFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PresentStageFlagsEXT(Flags);
    vk_bitflags_wrapped!(PresentStageFlagsEXT, Flags, PresentStageFlagBitsEXT);

    impl fmt::Debug for PresentStageFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    PresentStageFlagBitsEXT::QUEUE_OPERATIONS_END_EXT.0,
                    "QUEUE_OPERATIONS_END_EXT",
                ),
                (
                    PresentStageFlagBitsEXT::REQUEST_DEQUEUED_EXT.0,
                    "REQUEST_DEQUEUED_EXT",
                ),
                (
                    PresentStageFlagBitsEXT::IMAGE_FIRST_PIXEL_OUT_EXT.0,
                    "IMAGE_FIRST_PIXEL_OUT_EXT",
                ),
                (
                    PresentStageFlagBitsEXT::IMAGE_FIRST_PIXEL_VISIBLE_EXT.0,
                    "IMAGE_FIRST_PIXEL_VISIBLE_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentStageFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PresentStageFlagBitsEXT(u32);

    impl PresentStageFlagBitsEXT {
        pub const QUEUE_OPERATIONS_END_EXT: Self = Self(1 << 0);
        pub const REQUEST_DEQUEUED_EXT: Self = Self(1 << 1);
        pub const IMAGE_FIRST_PIXEL_OUT_EXT: Self = Self(1 << 2);
        pub const IMAGE_FIRST_PIXEL_VISIBLE_EXT: Self = Self(1 << 3);
    }

    impl fmt::Debug for PresentStageFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::QUEUE_OPERATIONS_END_EXT => Some("QUEUE_OPERATIONS_END_EXT"),
                Self::REQUEST_DEQUEUED_EXT => Some("REQUEST_DEQUEUED_EXT"),
                Self::IMAGE_FIRST_PIXEL_OUT_EXT => Some("IMAGE_FIRST_PIXEL_OUT_EXT"),
                Self::IMAGE_FIRST_PIXEL_VISIBLE_EXT => Some("IMAGE_FIRST_PIXEL_VISIBLE_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPastPresentationTimingFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PastPresentationTimingFlagsEXT(Flags);
    vk_bitflags_wrapped!(
        PastPresentationTimingFlagsEXT,
        Flags,
        PastPresentationTimingFlagBitsEXT
    );

    impl fmt::Debug for PastPresentationTimingFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    PastPresentationTimingFlagBitsEXT::ALLOW_PARTIAL_RESULTS_EXT.0,
                    "ALLOW_PARTIAL_RESULTS_EXT",
                ),
                (
                    PastPresentationTimingFlagBitsEXT::ALLOW_OUT_OF_ORDER_RESULTS_EXT.0,
                    "ALLOW_OUT_OF_ORDER_RESULTS_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPastPresentationTimingFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PastPresentationTimingFlagBitsEXT(u32);

    impl PastPresentationTimingFlagBitsEXT {
        pub const ALLOW_PARTIAL_RESULTS_EXT: Self = Self(1 << 0);
        pub const ALLOW_OUT_OF_ORDER_RESULTS_EXT: Self = Self(1 << 1);
    }

    impl fmt::Debug for PastPresentationTimingFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ALLOW_PARTIAL_RESULTS_EXT => Some("ALLOW_PARTIAL_RESULTS_EXT"),
                Self::ALLOW_OUT_OF_ORDER_RESULTS_EXT => Some("ALLOW_OUT_OF_ORDER_RESULTS_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentTimingInfoFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PresentTimingInfoFlagsEXT(Flags);
    vk_bitflags_wrapped!(
        PresentTimingInfoFlagsEXT,
        Flags,
        PresentTimingInfoFlagBitsEXT
    );

    impl fmt::Debug for PresentTimingInfoFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    PresentTimingInfoFlagBitsEXT::PRESENT_AT_RELATIVE_TIME_EXT.0,
                    "PRESENT_AT_RELATIVE_TIME_EXT",
                ),
                (
                    PresentTimingInfoFlagBitsEXT::PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT.0,
                    "PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentTimingInfoFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PresentTimingInfoFlagBitsEXT(u32);

    impl PresentTimingInfoFlagBitsEXT {
        pub const PRESENT_AT_RELATIVE_TIME_EXT: Self = Self(1 << 0);
        pub const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT: Self = Self(1 << 1);
    }

    impl fmt::Debug for PresentTimingInfoFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PRESENT_AT_RELATIVE_TIME_EXT => Some("PRESENT_AT_RELATIVE_TIME_EXT"),
                Self::PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT => {
                    Some("PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT")
                }
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetSwapchainPresentTimingQueueSizeEXT.html>
    pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT =
        unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR, size: u32) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSwapchainTimingPropertiesEXT.html>
    pub type PFN_vkGetSwapchainTimingPropertiesEXT = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT<'_>,
        p_swapchain_timing_properties_counter: *mut u64,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSwapchainTimeDomainPropertiesEXT.html>
    pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT<'_>,
        p_time_domains_counter: *mut u64,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPastPresentationTimingEXT.html>
    pub type PFN_vkGetPastPresentationTimingEXT = unsafe extern "system" fn(
        device: Device,
        p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT<'_>,
        p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePresentTimingFeaturesEXT =
        PhysicalDevicePresentTimingFeaturesEXT<'static>;
    pub type VkPresentTimingSurfaceCapabilitiesEXT = PresentTimingSurfaceCapabilitiesEXT<'static>;
    pub type VkSwapchainTimingPropertiesEXT = SwapchainTimingPropertiesEXT<'static>;
    pub type VkSwapchainTimeDomainPropertiesEXT = SwapchainTimeDomainPropertiesEXT<'static>;
    pub type VkPresentStageTimeEXT = PresentStageTimeEXT;
    pub type VkPastPresentationTimingInfoEXT = PastPresentationTimingInfoEXT<'static>;
    pub type VkPastPresentationTimingPropertiesEXT = PastPresentationTimingPropertiesEXT<'static>;
    pub type VkPastPresentationTimingEXT = PastPresentationTimingEXT<'static>;
    pub type VkPresentTimingsInfoEXT = PresentTimingsInfoEXT<'static>;
    pub type VkPresentTimingInfoEXT = PresentTimingInfoEXT<'static>;
    pub type VkSwapchainCalibratedTimestampInfoEXT = SwapchainCalibratedTimestampInfoEXT<'static>;
    pub type VkPresentStageFlagsEXT = PresentStageFlagsEXT;
    pub type VkPresentStageFlagBitsEXT = PresentStageFlagBitsEXT;
    pub type VkPastPresentationTimingFlagsEXT = PastPresentationTimingFlagsEXT;
    pub type VkPastPresentationTimingFlagBitsEXT = PastPresentationTimingFlagBitsEXT;
    pub type VkPresentTimingInfoFlagsEXT = PresentTimingInfoFlagsEXT;
    pub type VkPresentTimingInfoFlagBitsEXT = PresentTimingInfoFlagBitsEXT;
    impl PhysicalDevicePresentTimingFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDevicePresentTimingFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PresentTimingSurfaceCapabilitiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentTimingSurfaceCapabilitiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SwapchainTimingPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainTimingPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SwapchainTimeDomainPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainTimeDomainPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PastPresentationTimingInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPastPresentationTimingInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PastPresentationTimingPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPastPresentationTimingPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PastPresentationTimingEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPastPresentationTimingEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PresentTimingsInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentTimingsInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PresentTimingInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentTimingInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SwapchainCalibratedTimestampInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainCalibratedTimestampInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    set_swapchain_present_timing_queue_size_ext: PFN_vkSetSwapchainPresentTimingQueueSizeEXT,
    get_swapchain_timing_properties_ext: PFN_vkGetSwapchainTimingPropertiesEXT,
    get_swapchain_time_domain_properties_ext: PFN_vkGetSwapchainTimeDomainPropertiesEXT,
    get_past_presentation_timing_ext: PFN_vkGetPastPresentationTimingEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                set_swapchain_present_timing_queue_size_ext: transmute(
                    load(c"vkSetSwapchainPresentTimingQueueSizeEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
                get_swapchain_timing_properties_ext: transmute(
                    load(c"vkGetSwapchainTimingPropertiesEXT").ok_or(MissingEntryPointError)?,
                ),
                get_swapchain_time_domain_properties_ext: transmute(
                    load(c"vkGetSwapchainTimeDomainPropertiesEXT").ok_or(MissingEntryPointError)?,
                ),
                get_past_presentation_timing_ext: transmute(
                    load(c"vkGetPastPresentationTimingEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetSwapchainPresentTimingQueueSizeEXT.html>
    #[inline]
    pub unsafe fn set_swapchain_present_timing_queue_size_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.set_swapchain_present_timing_queue_size_ext)(device, swapchain, size);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSwapchainTimingPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_swapchain_timing_properties_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        swapchain_timing_properties: &mut SwapchainTimingPropertiesEXT<'_>,
        swapchain_timing_properties_counter: Option<&mut u64>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_swapchain_timing_properties_ext)(
                device,
                swapchain,
                swapchain_timing_properties,
                swapchain_timing_properties_counter.to_raw_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSwapchainTimeDomainPropertiesEXT.html>
    #[inline]
    pub unsafe fn get_swapchain_time_domain_properties_ext(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        swapchain_time_domain_properties: &mut SwapchainTimeDomainPropertiesEXT<'_>,
        time_domains_counter: Option<&mut u64>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_swapchain_time_domain_properties_ext)(
                device,
                swapchain,
                swapchain_time_domain_properties,
                time_domains_counter.to_raw_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPastPresentationTimingEXT.html>
    #[inline]
    pub unsafe fn get_past_presentation_timing_ext(
        &self,
        device: Device,
        past_presentation_timing_info: &PastPresentationTimingInfoEXT<'_>,
        past_presentation_timing_properties: &mut PastPresentationTimingPropertiesEXT<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_past_presentation_timing_ext)(
                device,
                past_presentation_timing_info,
                past_presentation_timing_properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
