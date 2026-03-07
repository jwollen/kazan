#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_low_latency2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLatencySleepModeInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LatencySleepModeInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub low_latency_mode: Bool32,
        pub low_latency_boost: Bool32,
        pub minimum_interval_us: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for LatencySleepModeInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("LatencySleepModeInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("low_latency_mode", &self.low_latency_mode)
                .field("low_latency_boost", &self.low_latency_boost)
                .field("minimum_interval_us", &self.minimum_interval_us)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for LatencySleepModeInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::LATENCY_SLEEP_MODE_INFO_NV;
    }

    impl Default for LatencySleepModeInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                low_latency_mode: Default::default(),
                low_latency_boost: Default::default(),
                minimum_interval_us: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> LatencySleepModeInfoNV<'a> {
        pub fn low_latency_mode(mut self, low_latency_mode: bool) -> Self {
            self.low_latency_mode = low_latency_mode.into();
            self
        }

        pub fn low_latency_boost(mut self, low_latency_boost: bool) -> Self {
            self.low_latency_boost = low_latency_boost.into();
            self
        }

        pub fn minimum_interval_us(mut self, minimum_interval_us: u32) -> Self {
            self.minimum_interval_us = minimum_interval_us;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLatencySleepInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LatencySleepInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub signal_semaphore: Semaphore,
        pub value: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for LatencySleepInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("LatencySleepInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("signal_semaphore", &self.signal_semaphore)
                .field("value", &self.value)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for LatencySleepInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::LATENCY_SLEEP_INFO_NV;
    }

    impl Default for LatencySleepInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                signal_semaphore: Default::default(),
                value: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> LatencySleepInfoNV<'a> {
        pub fn signal_semaphore(mut self, signal_semaphore: Semaphore) -> Self {
            self.signal_semaphore = signal_semaphore;
            self
        }

        pub fn value(mut self, value: u64) -> Self {
            self.value = value;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSetLatencyMarkerInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SetLatencyMarkerInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub present_id: u64,
        pub marker: LatencyMarkerNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SetLatencyMarkerInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SetLatencyMarkerInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_id", &self.present_id)
                .field("marker", &self.marker)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SetLatencyMarkerInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SET_LATENCY_MARKER_INFO_NV;
    }

    impl Default for SetLatencyMarkerInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                present_id: Default::default(),
                marker: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SetLatencyMarkerInfoNV<'a> {
        pub fn present_id(mut self, present_id: u64) -> Self {
            self.present_id = present_id;
            self
        }

        pub fn marker(mut self, marker: LatencyMarkerNV) -> Self {
            self.marker = marker;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGetLatencyMarkerInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GetLatencyMarkerInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub timing_count: u32,
        pub p_timings: *mut LatencyTimingsFrameReportNV<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GetLatencyMarkerInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GetLatencyMarkerInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("timing_count", &self.timing_count)
                .field("p_timings", &self.p_timings)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GetLatencyMarkerInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GET_LATENCY_MARKER_INFO_NV;
    }

    impl Default for GetLatencyMarkerInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                timing_count: Default::default(),
                p_timings: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GetLatencyMarkerInfoNV<'a> {
        pub fn timings(mut self, timings: &'a mut [LatencyTimingsFrameReportNV<'a>]) -> Self {
            self.timing_count = timings.len().try_into().unwrap();
            self.p_timings = timings.as_mut_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLatencyTimingsFrameReportNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LatencyTimingsFrameReportNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_id: u64,
        pub input_sample_time_us: u64,
        pub sim_start_time_us: u64,
        pub sim_end_time_us: u64,
        pub render_submit_start_time_us: u64,
        pub render_submit_end_time_us: u64,
        pub present_start_time_us: u64,
        pub present_end_time_us: u64,
        pub driver_start_time_us: u64,
        pub driver_end_time_us: u64,
        pub os_render_queue_start_time_us: u64,
        pub os_render_queue_end_time_us: u64,
        pub gpu_render_start_time_us: u64,
        pub gpu_render_end_time_us: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for LatencyTimingsFrameReportNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("LatencyTimingsFrameReportNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_id", &self.present_id)
                .field("input_sample_time_us", &self.input_sample_time_us)
                .field("sim_start_time_us", &self.sim_start_time_us)
                .field("sim_end_time_us", &self.sim_end_time_us)
                .field(
                    "render_submit_start_time_us",
                    &self.render_submit_start_time_us,
                )
                .field("render_submit_end_time_us", &self.render_submit_end_time_us)
                .field("present_start_time_us", &self.present_start_time_us)
                .field("present_end_time_us", &self.present_end_time_us)
                .field("driver_start_time_us", &self.driver_start_time_us)
                .field("driver_end_time_us", &self.driver_end_time_us)
                .field(
                    "os_render_queue_start_time_us",
                    &self.os_render_queue_start_time_us,
                )
                .field(
                    "os_render_queue_end_time_us",
                    &self.os_render_queue_end_time_us,
                )
                .field("gpu_render_start_time_us", &self.gpu_render_start_time_us)
                .field("gpu_render_end_time_us", &self.gpu_render_end_time_us)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for LatencyTimingsFrameReportNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::LATENCY_TIMINGS_FRAME_REPORT_NV;
    }

    impl Default for LatencyTimingsFrameReportNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_id: Default::default(),
                input_sample_time_us: Default::default(),
                sim_start_time_us: Default::default(),
                sim_end_time_us: Default::default(),
                render_submit_start_time_us: Default::default(),
                render_submit_end_time_us: Default::default(),
                present_start_time_us: Default::default(),
                present_end_time_us: Default::default(),
                driver_start_time_us: Default::default(),
                driver_end_time_us: Default::default(),
                os_render_queue_start_time_us: Default::default(),
                os_render_queue_end_time_us: Default::default(),
                gpu_render_start_time_us: Default::default(),
                gpu_render_end_time_us: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> LatencyTimingsFrameReportNV<'a> {
        pub fn present_id(mut self, present_id: u64) -> Self {
            self.present_id = present_id;
            self
        }

        pub fn input_sample_time_us(mut self, input_sample_time_us: u64) -> Self {
            self.input_sample_time_us = input_sample_time_us;
            self
        }

        pub fn sim_start_time_us(mut self, sim_start_time_us: u64) -> Self {
            self.sim_start_time_us = sim_start_time_us;
            self
        }

        pub fn sim_end_time_us(mut self, sim_end_time_us: u64) -> Self {
            self.sim_end_time_us = sim_end_time_us;
            self
        }

        pub fn render_submit_start_time_us(mut self, render_submit_start_time_us: u64) -> Self {
            self.render_submit_start_time_us = render_submit_start_time_us;
            self
        }

        pub fn render_submit_end_time_us(mut self, render_submit_end_time_us: u64) -> Self {
            self.render_submit_end_time_us = render_submit_end_time_us;
            self
        }

        pub fn present_start_time_us(mut self, present_start_time_us: u64) -> Self {
            self.present_start_time_us = present_start_time_us;
            self
        }

        pub fn present_end_time_us(mut self, present_end_time_us: u64) -> Self {
            self.present_end_time_us = present_end_time_us;
            self
        }

        pub fn driver_start_time_us(mut self, driver_start_time_us: u64) -> Self {
            self.driver_start_time_us = driver_start_time_us;
            self
        }

        pub fn driver_end_time_us(mut self, driver_end_time_us: u64) -> Self {
            self.driver_end_time_us = driver_end_time_us;
            self
        }

        pub fn os_render_queue_start_time_us(mut self, os_render_queue_start_time_us: u64) -> Self {
            self.os_render_queue_start_time_us = os_render_queue_start_time_us;
            self
        }

        pub fn os_render_queue_end_time_us(mut self, os_render_queue_end_time_us: u64) -> Self {
            self.os_render_queue_end_time_us = os_render_queue_end_time_us;
            self
        }

        pub fn gpu_render_start_time_us(mut self, gpu_render_start_time_us: u64) -> Self {
            self.gpu_render_start_time_us = gpu_render_start_time_us;
            self
        }

        pub fn gpu_render_end_time_us(mut self, gpu_render_end_time_us: u64) -> Self {
            self.gpu_render_end_time_us = gpu_render_end_time_us;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOutOfBandQueueTypeInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct OutOfBandQueueTypeInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub queue_type: OutOfBandQueueTypeNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for OutOfBandQueueTypeInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OutOfBandQueueTypeInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("queue_type", &self.queue_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for OutOfBandQueueTypeInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::OUT_OF_BAND_QUEUE_TYPE_INFO_NV;
    }

    impl Default for OutOfBandQueueTypeInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                queue_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> OutOfBandQueueTypeInfoNV<'a> {
        pub fn queue_type(mut self, queue_type: OutOfBandQueueTypeNV) -> Self {
            self.queue_type = queue_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLatencySubmissionPresentIdNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LatencySubmissionPresentIdNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub present_id: u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for LatencySubmissionPresentIdNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("LatencySubmissionPresentIdNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_id", &self.present_id)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for LatencySubmissionPresentIdNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::LATENCY_SUBMISSION_PRESENT_ID_NV;
    }

    unsafe impl<'a> Extends<SubmitInfo<'a>> for LatencySubmissionPresentIdNV<'a> {}
    unsafe impl<'a> Extends<SubmitInfo2<'a>> for LatencySubmissionPresentIdNV<'a> {}

    impl Default for LatencySubmissionPresentIdNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                present_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> LatencySubmissionPresentIdNV<'a> {
        pub fn present_id(mut self, present_id: u64) -> Self {
            self.present_id = present_id;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainLatencyCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SwapchainLatencyCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub latency_mode_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainLatencyCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainLatencyCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("latency_mode_enable", &self.latency_mode_enable)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainLatencyCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SWAPCHAIN_LATENCY_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<SwapchainCreateInfoKHR<'a>> for SwapchainLatencyCreateInfoNV<'a> {}

    impl Default for SwapchainLatencyCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                latency_mode_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainLatencyCreateInfoNV<'a> {
        pub fn latency_mode_enable(mut self, latency_mode_enable: bool) -> Self {
            self.latency_mode_enable = latency_mode_enable.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLatencySurfaceCapabilitiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct LatencySurfaceCapabilitiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub present_mode_count: u32,
        pub p_present_modes: *mut PresentModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for LatencySurfaceCapabilitiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("LatencySurfaceCapabilitiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_mode_count", &self.present_mode_count)
                .field("p_present_modes", &self.p_present_modes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for LatencySurfaceCapabilitiesNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::LATENCY_SURFACE_CAPABILITIES_NV;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for LatencySurfaceCapabilitiesNV<'a> {}

    impl Default for LatencySurfaceCapabilitiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                present_mode_count: Default::default(),
                p_present_modes: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> LatencySurfaceCapabilitiesNV<'a> {
        pub fn present_modes(mut self, present_modes: &'a mut [PresentModeKHR]) -> Self {
            self.present_mode_count = present_modes.len().try_into().unwrap();
            self.p_present_modes = present_modes.as_mut_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkLatencyMarkerNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct LatencyMarkerNV(i32);

    impl LatencyMarkerNV {
        pub const SIMULATION_START_NV: Self = Self(0);
        pub const SIMULATION_END_NV: Self = Self(1);
        pub const RENDERSUBMIT_START_NV: Self = Self(2);
        pub const RENDERSUBMIT_END_NV: Self = Self(3);
        pub const PRESENT_START_NV: Self = Self(4);
        pub const PRESENT_END_NV: Self = Self(5);
        pub const INPUT_SAMPLE_NV: Self = Self(6);
        pub const TRIGGER_FLASH_NV: Self = Self(7);
        pub const OUT_OF_BAND_RENDERSUBMIT_START_NV: Self = Self(8);
        pub const OUT_OF_BAND_RENDERSUBMIT_END_NV: Self = Self(9);
        pub const OUT_OF_BAND_PRESENT_START_NV: Self = Self(10);
        pub const OUT_OF_BAND_PRESENT_END_NV: Self = Self(11);
    }

    impl fmt::Debug for LatencyMarkerNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SIMULATION_START_NV => Some("SIMULATION_START_NV"),
                Self::SIMULATION_END_NV => Some("SIMULATION_END_NV"),
                Self::RENDERSUBMIT_START_NV => Some("RENDERSUBMIT_START_NV"),
                Self::RENDERSUBMIT_END_NV => Some("RENDERSUBMIT_END_NV"),
                Self::PRESENT_START_NV => Some("PRESENT_START_NV"),
                Self::PRESENT_END_NV => Some("PRESENT_END_NV"),
                Self::INPUT_SAMPLE_NV => Some("INPUT_SAMPLE_NV"),
                Self::TRIGGER_FLASH_NV => Some("TRIGGER_FLASH_NV"),
                Self::OUT_OF_BAND_RENDERSUBMIT_START_NV => {
                    Some("OUT_OF_BAND_RENDERSUBMIT_START_NV")
                }
                Self::OUT_OF_BAND_RENDERSUBMIT_END_NV => Some("OUT_OF_BAND_RENDERSUBMIT_END_NV"),
                Self::OUT_OF_BAND_PRESENT_START_NV => Some("OUT_OF_BAND_PRESENT_START_NV"),
                Self::OUT_OF_BAND_PRESENT_END_NV => Some("OUT_OF_BAND_PRESENT_END_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkOutOfBandQueueTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct OutOfBandQueueTypeNV(i32);

    impl OutOfBandQueueTypeNV {
        pub const RENDER_NV: Self = Self(0);
        pub const PRESENT_NV: Self = Self(1);
    }

    impl fmt::Debug for OutOfBandQueueTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::RENDER_NV => Some("RENDER_NV"),
                Self::PRESENT_NV => Some("PRESENT_NV"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLatencySleepModeNV.html>
    pub type PFN_vkSetLatencySleepModeNV = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_sleep_mode_info: *const LatencySleepModeInfoNV<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkLatencySleepNV.html>
    pub type PFN_vkLatencySleepNV = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_sleep_info: *const LatencySleepInfoNV<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLatencyMarkerNV.html>
    pub type PFN_vkSetLatencyMarkerNV = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_latency_marker_info: *const SetLatencyMarkerInfoNV<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetLatencyTimingsNV.html>
    pub type PFN_vkGetLatencyTimingsNV = unsafe extern "system" fn(
        device: Device,
        swapchain: SwapchainKHR,
        p_latency_marker_info: *mut GetLatencyMarkerInfoNV<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueNotifyOutOfBandNV.html>
    pub type PFN_vkQueueNotifyOutOfBandNV = unsafe extern "system" fn(
        queue: Queue,
        p_queue_type_info: *const OutOfBandQueueTypeInfoNV<'_>,
    );
}

pub struct DeviceFn {
    set_latency_sleep_mode_nv: PFN_vkSetLatencySleepModeNV,
    latency_sleep_nv: PFN_vkLatencySleepNV,
    set_latency_marker_nv: PFN_vkSetLatencyMarkerNV,
    get_latency_timings_nv: PFN_vkGetLatencyTimingsNV,
    queue_notify_out_of_band_nv: PFN_vkQueueNotifyOutOfBandNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                set_latency_sleep_mode_nv: transmute(
                    load(c"vkSetLatencySleepModeNV").ok_or(MissingEntryPointError)?,
                ),
                latency_sleep_nv: transmute(
                    load(c"vkLatencySleepNV").ok_or(MissingEntryPointError)?,
                ),
                set_latency_marker_nv: transmute(
                    load(c"vkSetLatencyMarkerNV").ok_or(MissingEntryPointError)?,
                ),
                get_latency_timings_nv: transmute(
                    load(c"vkGetLatencyTimingsNV").ok_or(MissingEntryPointError)?,
                ),
                queue_notify_out_of_band_nv: transmute(
                    load(c"vkQueueNotifyOutOfBandNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLatencySleepModeNV.html>
    pub unsafe fn set_latency_sleep_mode_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        sleep_mode_info: &LatencySleepModeInfoNV<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_latency_sleep_mode_nv)(device, swapchain, sleep_mode_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkLatencySleepNV.html>
    pub unsafe fn latency_sleep_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        sleep_info: &LatencySleepInfoNV<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.latency_sleep_nv)(device, swapchain, sleep_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetLatencyMarkerNV.html>
    pub unsafe fn set_latency_marker_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        latency_marker_info: &SetLatencyMarkerInfoNV<'_>,
    ) {
        unsafe { (self.set_latency_marker_nv)(device, swapchain, latency_marker_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetLatencyTimingsNV.html>
    pub unsafe fn get_latency_timings_nv(
        &self,
        device: Device,
        swapchain: SwapchainKHR,
        latency_marker_info: &mut GetLatencyMarkerInfoNV<'_>,
    ) {
        unsafe { (self.get_latency_timings_nv)(device, swapchain, latency_marker_info) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkQueueNotifyOutOfBandNV.html>
    pub unsafe fn queue_notify_out_of_band_nv(
        &self,
        queue: Queue,
        queue_type_info: &OutOfBandQueueTypeInfoNV<'_>,
    ) {
        unsafe { (self.queue_notify_out_of_band_nv)(queue, queue_type_info) }
    }
}
