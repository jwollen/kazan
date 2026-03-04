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
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePresentTimingFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_timing: Bool32,
        pub present_at_absolute_time: Bool32,
        pub present_at_relative_time: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentTimingFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_TIMING_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePresentTimingFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePresentTimingFeaturesEXT<'a> {}
    impl Default for PhysicalDevicePresentTimingFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_timing: Default::default(),
                present_at_absolute_time: Default::default(),
                present_at_relative_time: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePresentTimingFeaturesEXT<'a> {
        pub fn present_timing(mut self, present_timing: Bool32) -> Self {
            self.present_timing = present_timing;
            self
        }
        pub fn present_at_absolute_time(mut self, present_at_absolute_time: Bool32) -> Self {
            self.present_at_absolute_time = present_at_absolute_time;
            self
        }
        pub fn present_at_relative_time(mut self, present_at_relative_time: Bool32) -> Self {
            self.present_at_relative_time = present_at_relative_time;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PresentTimingSurfaceCapabilitiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_timing_supported: Bool32,
        pub present_at_absolute_time_supported: Bool32,
        pub present_at_relative_time_supported: Bool32,
        pub present_stage_queries: PresentStageFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PresentTimingSurfaceCapabilitiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PRESENT_TIMING_SURFACE_CAPABILITIES_EXT;
    }
    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for PresentTimingSurfaceCapabilitiesEXT<'a> {}
    impl Default for PresentTimingSurfaceCapabilitiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_timing_supported: Default::default(),
                present_at_absolute_time_supported: Default::default(),
                present_at_relative_time_supported: Default::default(),
                present_stage_queries: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PresentTimingSurfaceCapabilitiesEXT<'a> {
        pub fn present_timing_supported(mut self, present_timing_supported: Bool32) -> Self {
            self.present_timing_supported = present_timing_supported;
            self
        }
        pub fn present_at_absolute_time_supported(
            mut self,
            present_at_absolute_time_supported: Bool32,
        ) -> Self {
            self.present_at_absolute_time_supported = present_at_absolute_time_supported;
            self
        }
        pub fn present_at_relative_time_supported(
            mut self,
            present_at_relative_time_supported: Bool32,
        ) -> Self {
            self.present_at_relative_time_supported = present_at_relative_time_supported;
            self
        }
        pub fn present_stage_queries(
            mut self,
            present_stage_queries: PresentStageFlagsEXT,
        ) -> Self {
            self.present_stage_queries = present_stage_queries;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SwapchainTimingPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub refresh_duration: u64,
        pub refresh_interval: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SwapchainTimingPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_TIMING_PROPERTIES_EXT;
    }
    impl Default for SwapchainTimingPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                refresh_duration: Default::default(),
                refresh_interval: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SwapchainTimingPropertiesEXT<'a> {
        pub fn refresh_duration(mut self, refresh_duration: u64) -> Self {
            self.refresh_duration = refresh_duration;
            self
        }
        pub fn refresh_interval(mut self, refresh_interval: u64) -> Self {
            self.refresh_interval = refresh_interval;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SwapchainTimeDomainPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub time_domain_count: u32,
        pub p_time_domains: *mut TimeDomainKHR,
        pub p_time_domain_ids: *mut u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SwapchainTimeDomainPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_TIME_DOMAIN_PROPERTIES_EXT;
    }
    impl Default for SwapchainTimeDomainPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                time_domain_count: Default::default(),
                p_time_domains: core::ptr::null_mut(),
                p_time_domain_ids: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SwapchainTimeDomainPropertiesEXT<'a> {
        pub fn time_domains(mut self, time_domains: &'a mut [TimeDomainKHR]) -> Self {
            self.time_domain_count = time_domains.len().try_into().unwrap();
            self.p_time_domains = time_domains.as_mut_ptr();
            self
        }
        pub fn time_domain_ids(mut self, time_domain_ids: &'a mut [u64]) -> Self {
            self.time_domain_count = time_domain_ids.len().try_into().unwrap();
            self.p_time_domain_ids = time_domain_ids.as_mut_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct PresentStageTimeEXT {
        pub stage: PresentStageFlagsEXT,
        pub time: u64,
    }
    impl PresentStageTimeEXT {
        pub fn stage(mut self, stage: PresentStageFlagsEXT) -> Self {
            self.stage = stage;
            self
        }
        pub fn time(mut self, time: u64) -> Self {
            self.time = time;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PastPresentationTimingInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: PastPresentationTimingFlagsEXT,
        pub swapchain: SwapchainKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PastPresentationTimingInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PAST_PRESENTATION_TIMING_INFO_EXT;
    }
    impl Default for PastPresentationTimingInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                swapchain: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PastPresentationTimingInfoEXT<'a> {
        pub fn flags(mut self, flags: PastPresentationTimingFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
        pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
            self.swapchain = swapchain;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PastPresentationTimingPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub timing_properties_counter: u64,
        pub time_domains_counter: u64,
        pub presentation_timing_count: u32,
        pub p_presentation_timings: *mut PastPresentationTimingEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PastPresentationTimingPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PAST_PRESENTATION_TIMING_PROPERTIES_EXT;
    }
    impl Default for PastPresentationTimingPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                timing_properties_counter: Default::default(),
                time_domains_counter: Default::default(),
                presentation_timing_count: Default::default(),
                p_presentation_timings: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PastPresentationTimingPropertiesEXT<'a> {
        pub fn timing_properties_counter(mut self, timing_properties_counter: u64) -> Self {
            self.timing_properties_counter = timing_properties_counter;
            self
        }
        pub fn time_domains_counter(mut self, time_domains_counter: u64) -> Self {
            self.time_domains_counter = time_domains_counter;
            self
        }
        pub fn presentation_timings(
            mut self,
            presentation_timings: &'a mut [PastPresentationTimingEXT<'a>],
        ) -> Self {
            self.presentation_timing_count = presentation_timings.len().try_into().unwrap();
            self.p_presentation_timings = presentation_timings.as_mut_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for PastPresentationTimingEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PAST_PRESENTATION_TIMING_EXT;
    }
    impl Default for PastPresentationTimingEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_id: Default::default(),
                target_time: Default::default(),
                present_stage_count: Default::default(),
                p_present_stages: core::ptr::null_mut(),
                time_domain: Default::default(),
                time_domain_id: Default::default(),
                report_complete: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PastPresentationTimingEXT<'a> {
        pub fn present_id(mut self, present_id: u64) -> Self {
            self.present_id = present_id;
            self
        }
        pub fn target_time(mut self, target_time: u64) -> Self {
            self.target_time = target_time;
            self
        }
        pub fn present_stages(mut self, present_stages: &'a mut [PresentStageTimeEXT]) -> Self {
            self.present_stage_count = present_stages.len().try_into().unwrap();
            self.p_present_stages = present_stages.as_mut_ptr();
            self
        }
        pub fn time_domain(mut self, time_domain: TimeDomainKHR) -> Self {
            self.time_domain = time_domain;
            self
        }
        pub fn time_domain_id(mut self, time_domain_id: u64) -> Self {
            self.time_domain_id = time_domain_id;
            self
        }
        pub fn report_complete(mut self, report_complete: Bool32) -> Self {
            self.report_complete = report_complete;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PresentTimingsInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_timing_infos: *const PresentTimingInfoEXT<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PresentTimingsInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_TIMINGS_INFO_EXT;
    }
    unsafe impl<'a> Extends<PresentInfoKHR<'a>> for PresentTimingsInfoEXT<'a> {}
    impl Default for PresentTimingsInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                swapchain_count: Default::default(),
                p_timing_infos: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PresentTimingsInfoEXT<'a> {
        pub fn timing_infos(mut self, timing_infos: &'a [PresentTimingInfoEXT<'a>]) -> Self {
            self.swapchain_count = timing_infos.len().try_into().unwrap();
            self.p_timing_infos = timing_infos.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for PresentTimingInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_TIMING_INFO_EXT;
    }
    impl Default for PresentTimingInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
        pub fn flags(mut self, flags: PresentTimingInfoFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
        pub fn target_time(mut self, target_time: u64) -> Self {
            self.target_time = target_time;
            self
        }
        pub fn time_domain_id(mut self, time_domain_id: u64) -> Self {
            self.time_domain_id = time_domain_id;
            self
        }
        pub fn present_stage_queries(
            mut self,
            present_stage_queries: PresentStageFlagsEXT,
        ) -> Self {
            self.present_stage_queries = present_stage_queries;
            self
        }
        pub fn target_time_domain_present_stage(
            mut self,
            target_time_domain_present_stage: PresentStageFlagsEXT,
        ) -> Self {
            self.target_time_domain_present_stage = target_time_domain_present_stage;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SwapchainCalibratedTimestampInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain: SwapchainKHR,
        pub present_stage: PresentStageFlagsEXT,
        pub time_domain_id: u64,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SwapchainCalibratedTimestampInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SWAPCHAIN_CALIBRATED_TIMESTAMP_INFO_EXT;
    }
    unsafe impl<'a> Extends<CalibratedTimestampInfoKHR<'a>>
        for SwapchainCalibratedTimestampInfoEXT<'a>
    {
    }
    impl Default for SwapchainCalibratedTimestampInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                swapchain: Default::default(),
                present_stage: Default::default(),
                time_domain_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SwapchainCalibratedTimestampInfoEXT<'a> {
        pub fn swapchain(mut self, swapchain: SwapchainKHR) -> Self {
            self.swapchain = swapchain;
            self
        }
        pub fn present_stage(mut self, present_stage: PresentStageFlagsEXT) -> Self {
            self.present_stage = present_stage;
            self
        }
        pub fn time_domain_id(mut self, time_domain_id: u64) -> Self {
            self.time_domain_id = time_domain_id;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PresentStageFlagsEXT(Flags);
    vk_bitflags_wrapped!(PresentStageFlagsEXT, Flags);
    impl PresentStageFlagsEXT {
        pub const QUEUE_OPERATIONS_END_EXT: Self =
            Self(PresentStageFlagBitsEXT::QUEUE_OPERATIONS_END_EXT.0);
        pub const REQUEST_DEQUEUED_EXT: Self =
            Self(PresentStageFlagBitsEXT::REQUEST_DEQUEUED_EXT.0);
        pub const IMAGE_FIRST_PIXEL_OUT_EXT: Self =
            Self(PresentStageFlagBitsEXT::IMAGE_FIRST_PIXEL_OUT_EXT.0);
        pub const IMAGE_FIRST_PIXEL_VISIBLE_EXT: Self =
            Self(PresentStageFlagBitsEXT::IMAGE_FIRST_PIXEL_VISIBLE_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PresentStageFlagBitsEXT(u32);
    impl PresentStageFlagBitsEXT {
        pub const QUEUE_OPERATIONS_END_EXT: Self = Self(1 << 0);
        pub const REQUEST_DEQUEUED_EXT: Self = Self(1 << 1);
        pub const IMAGE_FIRST_PIXEL_OUT_EXT: Self = Self(1 << 2);
        pub const IMAGE_FIRST_PIXEL_VISIBLE_EXT: Self = Self(1 << 3);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PastPresentationTimingFlagsEXT(Flags);
    vk_bitflags_wrapped!(PastPresentationTimingFlagsEXT, Flags);
    impl PastPresentationTimingFlagsEXT {
        pub const ALLOW_PARTIAL_RESULTS_EXT: Self =
            Self(PastPresentationTimingFlagBitsEXT::ALLOW_PARTIAL_RESULTS_EXT.0);
        pub const ALLOW_OUT_OF_ORDER_RESULTS_EXT: Self =
            Self(PastPresentationTimingFlagBitsEXT::ALLOW_OUT_OF_ORDER_RESULTS_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PastPresentationTimingFlagBitsEXT(u32);
    impl PastPresentationTimingFlagBitsEXT {
        pub const ALLOW_PARTIAL_RESULTS_EXT: Self = Self(1 << 0);
        pub const ALLOW_OUT_OF_ORDER_RESULTS_EXT: Self = Self(1 << 1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PresentTimingInfoFlagsEXT(Flags);
    vk_bitflags_wrapped!(PresentTimingInfoFlagsEXT, Flags);
    impl PresentTimingInfoFlagsEXT {
        pub const PRESENT_AT_RELATIVE_TIME_EXT: Self =
            Self(PresentTimingInfoFlagBitsEXT::PRESENT_AT_RELATIVE_TIME_EXT.0);
        pub const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT: Self =
            Self(PresentTimingInfoFlagBitsEXT::PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct PresentTimingInfoFlagBitsEXT(u32);
    impl PresentTimingInfoFlagBitsEXT {
        pub const PRESENT_AT_RELATIVE_TIME_EXT: Self = Self(1 << 0);
        pub const PRESENT_AT_NEAREST_REFRESH_CYCLE_EXT: Self = Self(1 << 1);
    }
    pub type PFN_vkSetSwapchainPresentTimingQueueSizeEXT =
        unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR, size: u32) -> vk::Result;
    pub type PFN_vkGetSwapchainTimingPropertiesEXT = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT<'_>,
        p_swapchain_timing_properties_counter: *mut u64,
    ) -> vk::Result;
    pub type PFN_vkGetSwapchainTimeDomainPropertiesEXT = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT<'_>,
        p_time_domains_counter: *mut u64,
    )
        -> vk::Result;
    pub type PFN_vkGetPastPresentationTimingEXT = unsafe extern "system" fn(
        device: Device,
        p_past_presentation_timing_info: *const PastPresentationTimingInfoEXT<'_>,
        p_past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    set_swapchain_present_timing_queue_size_ext: PFN_vkSetSwapchainPresentTimingQueueSizeEXT,
    get_swapchain_timing_properties_ext: PFN_vkGetSwapchainTimingPropertiesEXT,
    get_swapchain_time_domain_properties_ext: PFN_vkGetSwapchainTimeDomainPropertiesEXT,
    get_past_presentation_timing_ext: PFN_vkGetPastPresentationTimingEXT,
}
impl DeviceFn {
    pub unsafe fn load(
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
                VkResult::NOT_READY => Ok(()),
                err => Err(err),
            }
        }
    }
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
                VkResult::NOT_READY => Ok(()),
                err => Err(err),
            }
        }
    }
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
                VkResult::INCOMPLETE => Ok(()),
                err => Err(err),
            }
        }
    }
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
                VkResult::INCOMPLETE => Ok(()),
                err => Err(err),
            }
        }
    }
}
