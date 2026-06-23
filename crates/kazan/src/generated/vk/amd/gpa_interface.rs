//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_gpa_interface.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_gpa_interface";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    handle_nondispatchable!(
        GpaSessionAMD,
        GPA_SESSION_AMD,
        doc = "<https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaSessionAMD.html>"
    );

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaPerfBlockPropertiesAMD.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct GpaPerfBlockPropertiesAMD {
        pub block_type: GpaPerfBlockAMD,
        pub flags: GpaPerfBlockPropertiesFlagsAMD,
        pub instance_count: u32,
        pub max_event_id: u32,
        pub max_global_only_counters: u32,
        pub max_global_shared_counters: u32,
        pub max_streaming_counters: u32,
    }

    impl GpaPerfBlockPropertiesAMD {
        #[inline]
        pub fn block_type(mut self, block_type: GpaPerfBlockAMD) -> Self {
            self.block_type = block_type;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: GpaPerfBlockPropertiesFlagsAMD) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn instance_count(mut self, instance_count: u32) -> Self {
            self.instance_count = instance_count;
            self
        }

        #[inline]
        pub fn max_event_id(mut self, max_event_id: u32) -> Self {
            self.max_event_id = max_event_id;
            self
        }

        #[inline]
        pub fn max_global_only_counters(mut self, max_global_only_counters: u32) -> Self {
            self.max_global_only_counters = max_global_only_counters;
            self
        }

        #[inline]
        pub fn max_global_shared_counters(mut self, max_global_shared_counters: u32) -> Self {
            self.max_global_shared_counters = max_global_shared_counters;
            self
        }

        #[inline]
        pub fn max_streaming_counters(mut self, max_streaming_counters: u32) -> Self {
            self.max_streaming_counters = max_streaming_counters;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGpaFeaturesAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceGpaFeaturesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub perf_counters: Bool32,
        pub streaming_perf_counters: Bool32,
        pub sq_thread_tracing: Bool32,
        pub clock_modes: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceGpaFeaturesAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceGpaFeaturesAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("perf_counters", &self.perf_counters)
                .field("streaming_perf_counters", &self.streaming_perf_counters)
                .field("sq_thread_tracing", &self.sq_thread_tracing)
                .field("clock_modes", &self.clock_modes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceGpaFeaturesAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_GPA_FEATURES_AMD;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceGpaFeaturesAMD<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceGpaFeaturesAMD<'_> {}

    impl Default for PhysicalDeviceGpaFeaturesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                perf_counters: Default::default(),
                streaming_perf_counters: Default::default(),
                sq_thread_tracing: Default::default(),
                clock_modes: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceGpaFeaturesAMD<'a> {
        #[inline]
        pub fn perf_counters(mut self, perf_counters: bool) -> Self {
            self.perf_counters = perf_counters.into();
            self
        }

        #[inline]
        pub fn streaming_perf_counters(mut self, streaming_perf_counters: bool) -> Self {
            self.streaming_perf_counters = streaming_perf_counters.into();
            self
        }

        #[inline]
        pub fn sq_thread_tracing(mut self, sq_thread_tracing: bool) -> Self {
            self.sq_thread_tracing = sq_thread_tracing.into();
            self
        }

        #[inline]
        pub fn clock_modes(mut self, clock_modes: bool) -> Self {
            self.clock_modes = clock_modes.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGpaPropertiesAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceGpaPropertiesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: PhysicalDeviceGpaPropertiesFlagsAMD,
        pub max_sqtt_se_buffer_size: DeviceSize,
        pub shader_engine_count: u32,
        pub perf_block_count: u32,
        pub p_perf_blocks: *mut GpaPerfBlockPropertiesAMD,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceGpaPropertiesAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceGpaPropertiesAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("max_sqtt_se_buffer_size", &self.max_sqtt_se_buffer_size)
                .field("shader_engine_count", &self.shader_engine_count)
                .field("perf_block_count", &self.perf_block_count)
                .field("p_perf_blocks", &self.p_perf_blocks)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceGpaPropertiesAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_GPA_PROPERTIES_AMD;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceGpaPropertiesAMD<'_> {}

    impl Default for PhysicalDeviceGpaPropertiesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                max_sqtt_se_buffer_size: Default::default(),
                shader_engine_count: Default::default(),
                perf_block_count: Default::default(),
                p_perf_blocks: ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceGpaPropertiesAMD<'a> {
        #[inline]
        pub fn flags(mut self, flags: PhysicalDeviceGpaPropertiesFlagsAMD) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn max_sqtt_se_buffer_size(mut self, max_sqtt_se_buffer_size: DeviceSize) -> Self {
            self.max_sqtt_se_buffer_size = max_sqtt_se_buffer_size;
            self
        }

        #[inline]
        pub fn shader_engine_count(mut self, shader_engine_count: u32) -> Self {
            self.shader_engine_count = shader_engine_count;
            self
        }

        #[inline]
        pub fn perf_blocks(mut self, perf_blocks: &'a mut [GpaPerfBlockPropertiesAMD]) -> Self {
            self.perf_block_count = perf_blocks.len().try_into().unwrap();
            self.p_perf_blocks = perf_blocks.as_mut_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGpaProperties2AMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceGpaProperties2AMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub revision_id: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceGpaProperties2AMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceGpaProperties2AMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("revision_id", &self.revision_id)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceGpaProperties2AMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_GPA_PROPERTIES_2_AMD;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>> for PhysicalDeviceGpaProperties2AMD<'_> {}

    impl Default for PhysicalDeviceGpaProperties2AMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                revision_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceGpaProperties2AMD<'a> {
        #[inline]
        pub fn revision_id(mut self, revision_id: u32) -> Self {
            self.revision_id = revision_id;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaPerfCounterAMD.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct GpaPerfCounterAMD {
        pub block_type: GpaPerfBlockAMD,
        pub block_instance: u32,
        pub event_id: u32,
    }

    impl GpaPerfCounterAMD {
        #[inline]
        pub fn block_type(mut self, block_type: GpaPerfBlockAMD) -> Self {
            self.block_type = block_type;
            self
        }

        #[inline]
        pub fn block_instance(mut self, block_instance: u32) -> Self {
            self.block_instance = block_instance;
            self
        }

        #[inline]
        pub fn event_id(mut self, event_id: u32) -> Self {
            self.event_id = event_id;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaSampleBeginInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GpaSampleBeginInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub sample_type: GpaSampleTypeAMD,
        pub sample_internal_operations: Bool32,
        pub cache_flush_on_counter_collection: Bool32,
        pub sq_shader_mask_enable: Bool32,
        pub sq_shader_mask: GpaSqShaderStageFlagsAMD,
        pub perf_counter_count: u32,
        pub p_perf_counters: *const GpaPerfCounterAMD,
        pub streaming_perf_trace_sample_interval: u32,
        pub perf_counter_device_memory_limit: DeviceSize,
        pub sq_thread_trace_enable: Bool32,
        pub sq_thread_trace_suppress_instruction_tokens: Bool32,
        pub sq_thread_trace_device_memory_limit: DeviceSize,
        pub timing_pre_sample: PipelineStageFlags,
        pub timing_post_sample: PipelineStageFlags,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GpaSampleBeginInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GpaSampleBeginInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("sample_type", &self.sample_type)
                .field(
                    "sample_internal_operations",
                    &self.sample_internal_operations,
                )
                .field(
                    "cache_flush_on_counter_collection",
                    &self.cache_flush_on_counter_collection,
                )
                .field("sq_shader_mask_enable", &self.sq_shader_mask_enable)
                .field("sq_shader_mask", &self.sq_shader_mask)
                .field("perf_counter_count", &self.perf_counter_count)
                .field("p_perf_counters", &self.p_perf_counters)
                .field(
                    "streaming_perf_trace_sample_interval",
                    &self.streaming_perf_trace_sample_interval,
                )
                .field(
                    "perf_counter_device_memory_limit",
                    &self.perf_counter_device_memory_limit,
                )
                .field("sq_thread_trace_enable", &self.sq_thread_trace_enable)
                .field(
                    "sq_thread_trace_suppress_instruction_tokens",
                    &self.sq_thread_trace_suppress_instruction_tokens,
                )
                .field(
                    "sq_thread_trace_device_memory_limit",
                    &self.sq_thread_trace_device_memory_limit,
                )
                .field("timing_pre_sample", &self.timing_pre_sample)
                .field("timing_post_sample", &self.timing_post_sample)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GpaSampleBeginInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GPA_SAMPLE_BEGIN_INFO_AMD;
    }

    impl Default for GpaSampleBeginInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                sample_type: Default::default(),
                sample_internal_operations: Default::default(),
                cache_flush_on_counter_collection: Default::default(),
                sq_shader_mask_enable: Default::default(),
                sq_shader_mask: Default::default(),
                perf_counter_count: Default::default(),
                p_perf_counters: ptr::null(),
                streaming_perf_trace_sample_interval: Default::default(),
                perf_counter_device_memory_limit: Default::default(),
                sq_thread_trace_enable: Default::default(),
                sq_thread_trace_suppress_instruction_tokens: Default::default(),
                sq_thread_trace_device_memory_limit: Default::default(),
                timing_pre_sample: Default::default(),
                timing_post_sample: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GpaSampleBeginInfoAMD<'a> {
        #[inline]
        pub fn sample_type(mut self, sample_type: GpaSampleTypeAMD) -> Self {
            self.sample_type = sample_type;
            self
        }

        #[inline]
        pub fn sample_internal_operations(mut self, sample_internal_operations: bool) -> Self {
            self.sample_internal_operations = sample_internal_operations.into();
            self
        }

        #[inline]
        pub fn cache_flush_on_counter_collection(
            mut self,
            cache_flush_on_counter_collection: bool,
        ) -> Self {
            self.cache_flush_on_counter_collection = cache_flush_on_counter_collection.into();
            self
        }

        #[inline]
        pub fn sq_shader_mask_enable(mut self, sq_shader_mask_enable: bool) -> Self {
            self.sq_shader_mask_enable = sq_shader_mask_enable.into();
            self
        }

        #[inline]
        pub fn sq_shader_mask(mut self, sq_shader_mask: GpaSqShaderStageFlagsAMD) -> Self {
            self.sq_shader_mask = sq_shader_mask;
            self
        }

        #[inline]
        pub fn perf_counters(mut self, perf_counters: &'a [GpaPerfCounterAMD]) -> Self {
            self.perf_counter_count = perf_counters.len().try_into().unwrap();
            self.p_perf_counters = perf_counters.as_ptr() as _;
            self
        }

        #[inline]
        pub fn streaming_perf_trace_sample_interval(
            mut self,
            streaming_perf_trace_sample_interval: u32,
        ) -> Self {
            self.streaming_perf_trace_sample_interval = streaming_perf_trace_sample_interval;
            self
        }

        #[inline]
        pub fn perf_counter_device_memory_limit(
            mut self,
            perf_counter_device_memory_limit: DeviceSize,
        ) -> Self {
            self.perf_counter_device_memory_limit = perf_counter_device_memory_limit;
            self
        }

        #[inline]
        pub fn sq_thread_trace_enable(mut self, sq_thread_trace_enable: bool) -> Self {
            self.sq_thread_trace_enable = sq_thread_trace_enable.into();
            self
        }

        #[inline]
        pub fn sq_thread_trace_suppress_instruction_tokens(
            mut self,
            sq_thread_trace_suppress_instruction_tokens: bool,
        ) -> Self {
            self.sq_thread_trace_suppress_instruction_tokens =
                sq_thread_trace_suppress_instruction_tokens.into();
            self
        }

        #[inline]
        pub fn sq_thread_trace_device_memory_limit(
            mut self,
            sq_thread_trace_device_memory_limit: DeviceSize,
        ) -> Self {
            self.sq_thread_trace_device_memory_limit = sq_thread_trace_device_memory_limit;
            self
        }

        #[inline]
        pub fn timing_pre_sample(mut self, timing_pre_sample: PipelineStageFlags) -> Self {
            self.timing_pre_sample = timing_pre_sample;
            self
        }

        #[inline]
        pub fn timing_post_sample(mut self, timing_post_sample: PipelineStageFlags) -> Self {
            self.timing_post_sample = timing_post_sample;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaDeviceClockModeInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GpaDeviceClockModeInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub clock_mode: GpaDeviceClockModeAMD,
        pub memory_clock_ratio_to_peak: f32,
        pub engine_clock_ratio_to_peak: f32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GpaDeviceClockModeInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GpaDeviceClockModeInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("clock_mode", &self.clock_mode)
                .field(
                    "memory_clock_ratio_to_peak",
                    &self.memory_clock_ratio_to_peak,
                )
                .field(
                    "engine_clock_ratio_to_peak",
                    &self.engine_clock_ratio_to_peak,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GpaDeviceClockModeInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GPA_DEVICE_CLOCK_MODE_INFO_AMD;
    }

    impl Default for GpaDeviceClockModeInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                clock_mode: Default::default(),
                memory_clock_ratio_to_peak: Default::default(),
                engine_clock_ratio_to_peak: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GpaDeviceClockModeInfoAMD<'a> {
        #[inline]
        pub fn clock_mode(mut self, clock_mode: GpaDeviceClockModeAMD) -> Self {
            self.clock_mode = clock_mode;
            self
        }

        #[inline]
        pub fn memory_clock_ratio_to_peak(mut self, memory_clock_ratio_to_peak: f32) -> Self {
            self.memory_clock_ratio_to_peak = memory_clock_ratio_to_peak;
            self
        }

        #[inline]
        pub fn engine_clock_ratio_to_peak(mut self, engine_clock_ratio_to_peak: f32) -> Self {
            self.engine_clock_ratio_to_peak = engine_clock_ratio_to_peak;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaDeviceGetClockInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GpaDeviceGetClockInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_clock_ratio_to_peak: f32,
        pub engine_clock_ratio_to_peak: f32,
        pub memory_clock_frequency: u32,
        pub engine_clock_frequency: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GpaDeviceGetClockInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GpaDeviceGetClockInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "memory_clock_ratio_to_peak",
                    &self.memory_clock_ratio_to_peak,
                )
                .field(
                    "engine_clock_ratio_to_peak",
                    &self.engine_clock_ratio_to_peak,
                )
                .field("memory_clock_frequency", &self.memory_clock_frequency)
                .field("engine_clock_frequency", &self.engine_clock_frequency)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GpaDeviceGetClockInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GPA_DEVICE_GET_CLOCK_INFO_AMD;
    }

    impl Default for GpaDeviceGetClockInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_clock_ratio_to_peak: Default::default(),
                engine_clock_ratio_to_peak: Default::default(),
                memory_clock_frequency: Default::default(),
                engine_clock_frequency: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GpaDeviceGetClockInfoAMD<'a> {
        #[inline]
        pub fn memory_clock_ratio_to_peak(mut self, memory_clock_ratio_to_peak: f32) -> Self {
            self.memory_clock_ratio_to_peak = memory_clock_ratio_to_peak;
            self
        }

        #[inline]
        pub fn engine_clock_ratio_to_peak(mut self, engine_clock_ratio_to_peak: f32) -> Self {
            self.engine_clock_ratio_to_peak = engine_clock_ratio_to_peak;
            self
        }

        #[inline]
        pub fn memory_clock_frequency(mut self, memory_clock_frequency: u32) -> Self {
            self.memory_clock_frequency = memory_clock_frequency;
            self
        }

        #[inline]
        pub fn engine_clock_frequency(mut self, engine_clock_frequency: u32) -> Self {
            self.engine_clock_frequency = engine_clock_frequency;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaSessionCreateInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GpaSessionCreateInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub secondary_copy_source: GpaSessionAMD,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GpaSessionCreateInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GpaSessionCreateInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("secondary_copy_source", &self.secondary_copy_source)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GpaSessionCreateInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::GPA_SESSION_CREATE_INFO_AMD;
    }

    impl Default for GpaSessionCreateInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                secondary_copy_source: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GpaSessionCreateInfoAMD<'a> {
        #[inline]
        pub fn secondary_copy_source(mut self, secondary_copy_source: GpaSessionAMD) -> Self {
            self.secondary_copy_source = secondary_copy_source;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaPerfBlockAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GpaPerfBlockAMD(i32);

    impl GpaPerfBlockAMD {
        #[inline]
        pub const fn from_raw(x: i32) -> Self {
            Self(x)
        }
        #[inline]
        pub const fn as_raw(self) -> i32 {
            self.0
        }

        pub const CPF_AMD: Self = Self(0);
        pub const IA_AMD: Self = Self(1);
        pub const VGT_AMD: Self = Self(2);
        pub const PA_AMD: Self = Self(3);
        pub const SC_AMD: Self = Self(4);
        pub const SPI_AMD: Self = Self(5);
        pub const SQ_AMD: Self = Self(6);
        pub const SX_AMD: Self = Self(7);
        pub const TA_AMD: Self = Self(8);
        pub const TD_AMD: Self = Self(9);
        pub const TCP_AMD: Self = Self(10);
        pub const TCC_AMD: Self = Self(11);
        pub const TCA_AMD: Self = Self(12);
        pub const DB_AMD: Self = Self(13);
        pub const CB_AMD: Self = Self(14);
        pub const GDS_AMD: Self = Self(15);
        pub const SRBM_AMD: Self = Self(16);
        pub const GRBM_AMD: Self = Self(17);
        pub const GRBM_SE_AMD: Self = Self(18);
        pub const RLC_AMD: Self = Self(19);
        pub const DMA_AMD: Self = Self(20);
        pub const MC_AMD: Self = Self(21);
        pub const CPG_AMD: Self = Self(22);
        pub const CPC_AMD: Self = Self(23);
        pub const WD_AMD: Self = Self(24);
        pub const TCS_AMD: Self = Self(25);
        pub const ATC_AMD: Self = Self(26);
        pub const ATC_L2_AMD: Self = Self(27);
        pub const MC_VM_L2_AMD: Self = Self(28);
        pub const EA_AMD: Self = Self(29);
        pub const RPB_AMD: Self = Self(30);
        pub const RMI_AMD: Self = Self(31);
        pub const UMCCH_AMD: Self = Self(32);
        pub const GE_AMD: Self = Self(33);
        pub const GL1A_AMD: Self = Self(34);
        pub const GL1C_AMD: Self = Self(35);
        pub const GL1CG_AMD: Self = Self(36);
        pub const GL2A_AMD: Self = Self(37);
        pub const GL2C_AMD: Self = Self(38);
        pub const CHA_AMD: Self = Self(39);
        pub const CHC_AMD: Self = Self(40);
        pub const CHCG_AMD: Self = Self(41);
        pub const GUS_AMD: Self = Self(42);
        pub const GCR_AMD: Self = Self(43);
        pub const PH_AMD: Self = Self(44);
        pub const UTCL1_AMD: Self = Self(45);
        pub const GE_DIST_AMD: Self = Self(46);
        pub const GE_SE_AMD: Self = Self(47);
        pub const DF_MALL_AMD: Self = Self(48);
        pub const SQ_WGP_AMD: Self = Self(49);
        pub const PC_AMD: Self = Self(50);
        pub const GL1XA_AMD: Self = Self(51);
        pub const GL1XC_AMD: Self = Self(52);
        pub const WGS_AMD: Self = Self(53);
        pub const EACPWD_AMD: Self = Self(54);
        pub const EASE_AMD: Self = Self(55);
        pub const RLCUSER_AMD: Self = Self(56);
    }

    impl fmt::Debug for GpaPerfBlockAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CPF_AMD => Some("CPF_AMD"),
                Self::IA_AMD => Some("IA_AMD"),
                Self::VGT_AMD => Some("VGT_AMD"),
                Self::PA_AMD => Some("PA_AMD"),
                Self::SC_AMD => Some("SC_AMD"),
                Self::SPI_AMD => Some("SPI_AMD"),
                Self::SQ_AMD => Some("SQ_AMD"),
                Self::SX_AMD => Some("SX_AMD"),
                Self::TA_AMD => Some("TA_AMD"),
                Self::TD_AMD => Some("TD_AMD"),
                Self::TCP_AMD => Some("TCP_AMD"),
                Self::TCC_AMD => Some("TCC_AMD"),
                Self::TCA_AMD => Some("TCA_AMD"),
                Self::DB_AMD => Some("DB_AMD"),
                Self::CB_AMD => Some("CB_AMD"),
                Self::GDS_AMD => Some("GDS_AMD"),
                Self::SRBM_AMD => Some("SRBM_AMD"),
                Self::GRBM_AMD => Some("GRBM_AMD"),
                Self::GRBM_SE_AMD => Some("GRBM_SE_AMD"),
                Self::RLC_AMD => Some("RLC_AMD"),
                Self::DMA_AMD => Some("DMA_AMD"),
                Self::MC_AMD => Some("MC_AMD"),
                Self::CPG_AMD => Some("CPG_AMD"),
                Self::CPC_AMD => Some("CPC_AMD"),
                Self::WD_AMD => Some("WD_AMD"),
                Self::TCS_AMD => Some("TCS_AMD"),
                Self::ATC_AMD => Some("ATC_AMD"),
                Self::ATC_L2_AMD => Some("ATC_L2_AMD"),
                Self::MC_VM_L2_AMD => Some("MC_VM_L2_AMD"),
                Self::EA_AMD => Some("EA_AMD"),
                Self::RPB_AMD => Some("RPB_AMD"),
                Self::RMI_AMD => Some("RMI_AMD"),
                Self::UMCCH_AMD => Some("UMCCH_AMD"),
                Self::GE_AMD => Some("GE_AMD"),
                Self::GL1A_AMD => Some("GL1A_AMD"),
                Self::GL1C_AMD => Some("GL1C_AMD"),
                Self::GL1CG_AMD => Some("GL1CG_AMD"),
                Self::GL2A_AMD => Some("GL2A_AMD"),
                Self::GL2C_AMD => Some("GL2C_AMD"),
                Self::CHA_AMD => Some("CHA_AMD"),
                Self::CHC_AMD => Some("CHC_AMD"),
                Self::CHCG_AMD => Some("CHCG_AMD"),
                Self::GUS_AMD => Some("GUS_AMD"),
                Self::GCR_AMD => Some("GCR_AMD"),
                Self::PH_AMD => Some("PH_AMD"),
                Self::UTCL1_AMD => Some("UTCL1_AMD"),
                Self::GE_DIST_AMD => Some("GE_DIST_AMD"),
                Self::GE_SE_AMD => Some("GE_SE_AMD"),
                Self::DF_MALL_AMD => Some("DF_MALL_AMD"),
                Self::SQ_WGP_AMD => Some("SQ_WGP_AMD"),
                Self::PC_AMD => Some("PC_AMD"),
                Self::GL1XA_AMD => Some("GL1XA_AMD"),
                Self::GL1XC_AMD => Some("GL1XC_AMD"),
                Self::WGS_AMD => Some("WGS_AMD"),
                Self::EACPWD_AMD => Some("EACPWD_AMD"),
                Self::EASE_AMD => Some("EASE_AMD"),
                Self::RLCUSER_AMD => Some("RLCUSER_AMD"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaSampleTypeAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GpaSampleTypeAMD(i32);

    impl GpaSampleTypeAMD {
        #[inline]
        pub const fn from_raw(x: i32) -> Self {
            Self(x)
        }
        #[inline]
        pub const fn as_raw(self) -> i32 {
            self.0
        }

        pub const CUMULATIVE_AMD: Self = Self(0);
        pub const TRACE_AMD: Self = Self(1);
        pub const TIMING_AMD: Self = Self(2);
    }

    impl fmt::Debug for GpaSampleTypeAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CUMULATIVE_AMD => Some("CUMULATIVE_AMD"),
                Self::TRACE_AMD => Some("TRACE_AMD"),
                Self::TIMING_AMD => Some("TIMING_AMD"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaDeviceClockModeAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GpaDeviceClockModeAMD(i32);

    impl GpaDeviceClockModeAMD {
        #[inline]
        pub const fn from_raw(x: i32) -> Self {
            Self(x)
        }
        #[inline]
        pub const fn as_raw(self) -> i32 {
            self.0
        }

        pub const DEFAULT_AMD: Self = Self(0);
        pub const QUERY_AMD: Self = Self(1);
        pub const PROFILING_AMD: Self = Self(2);
        pub const MIN_MEMORY_AMD: Self = Self(3);
        pub const MIN_ENGINE_AMD: Self = Self(4);
        pub const PEAK_AMD: Self = Self(5);
    }

    impl fmt::Debug for GpaDeviceClockModeAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_AMD => Some("DEFAULT_AMD"),
                Self::QUERY_AMD => Some("QUERY_AMD"),
                Self::PROFILING_AMD => Some("PROFILING_AMD"),
                Self::MIN_MEMORY_AMD => Some("MIN_MEMORY_AMD"),
                Self::MIN_ENGINE_AMD => Some("MIN_ENGINE_AMD"),
                Self::PEAK_AMD => Some("PEAK_AMD"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaSqShaderStageFlagsAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct GpaSqShaderStageFlagsAMD(Flags);
    vk_bitflags_wrapped!(GpaSqShaderStageFlagsAMD, Flags, GpaSqShaderStageFlagBitsAMD);

    impl fmt::Debug for GpaSqShaderStageFlagsAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (GpaSqShaderStageFlagBitsAMD::PS_AMD.0, "PS_AMD"),
                (GpaSqShaderStageFlagBitsAMD::VS_AMD.0, "VS_AMD"),
                (GpaSqShaderStageFlagBitsAMD::GS_AMD.0, "GS_AMD"),
                (GpaSqShaderStageFlagBitsAMD::ES_AMD.0, "ES_AMD"),
                (GpaSqShaderStageFlagBitsAMD::HS_AMD.0, "HS_AMD"),
                (GpaSqShaderStageFlagBitsAMD::LS_AMD.0, "LS_AMD"),
                (GpaSqShaderStageFlagBitsAMD::CS_AMD.0, "CS_AMD"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaSqShaderStageFlagBitsAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct GpaSqShaderStageFlagBitsAMD(u32);

    impl GpaSqShaderStageFlagBitsAMD {
        pub const PS_AMD: Self = Self(1 << 0);
        pub const VS_AMD: Self = Self(1 << 1);
        pub const GS_AMD: Self = Self(1 << 2);
        pub const ES_AMD: Self = Self(1 << 3);
        pub const HS_AMD: Self = Self(1 << 4);
        pub const LS_AMD: Self = Self(1 << 5);
        pub const CS_AMD: Self = Self(1 << 6);
    }

    impl fmt::Debug for GpaSqShaderStageFlagBitsAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PS_AMD => Some("PS_AMD"),
                Self::VS_AMD => Some("VS_AMD"),
                Self::GS_AMD => Some("GS_AMD"),
                Self::ES_AMD => Some("ES_AMD"),
                Self::HS_AMD => Some("HS_AMD"),
                Self::LS_AMD => Some("LS_AMD"),
                Self::CS_AMD => Some("CS_AMD"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGpaPerfBlockPropertiesFlagsAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct GpaPerfBlockPropertiesFlagsAMD(Flags);
    vk_bitflags_wrapped!(GpaPerfBlockPropertiesFlagsAMD, Flags);

    impl fmt::Debug for GpaPerfBlockPropertiesFlagsAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGpaPropertiesFlagsAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct PhysicalDeviceGpaPropertiesFlagsAMD(Flags);
    vk_bitflags_wrapped!(PhysicalDeviceGpaPropertiesFlagsAMD, Flags);

    impl fmt::Debug for PhysicalDeviceGpaPropertiesFlagsAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateGpaSessionAMD.html>
    pub type PFN_vkCreateGpaSessionAMD = unsafe extern "system" fn(
        device: Device,
        p_create_info: *const GpaSessionCreateInfoAMD<'_>,
        p_allocator: *const AllocationCallbacks<'_>,
        p_gpa_session: *mut GpaSessionAMD,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyGpaSessionAMD.html>
    pub type PFN_vkDestroyGpaSessionAMD = unsafe extern "system" fn(
        device: Device,
        gpa_session: GpaSessionAMD,
        p_allocator: *const AllocationCallbacks<'_>,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetGpaDeviceClockModeAMD.html>
    pub type PFN_vkSetGpaDeviceClockModeAMD = unsafe extern "system" fn(
        device: Device,
        p_info: *mut GpaDeviceClockModeInfoAMD<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGpaDeviceClockInfoAMD.html>
    pub type PFN_vkGetGpaDeviceClockInfoAMD = unsafe extern "system" fn(
        device: Device,
        p_info: *mut GpaDeviceGetClockInfoAMD<'_>,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginGpaSessionAMD.html>
    pub type PFN_vkCmdBeginGpaSessionAMD = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndGpaSessionAMD.html>
    pub type PFN_vkCmdEndGpaSessionAMD = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginGpaSampleAMD.html>
    pub type PFN_vkCmdBeginGpaSampleAMD = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
        p_gpa_sample_begin_info: *const GpaSampleBeginInfoAMD<'_>,
        p_sample_id: *mut u32,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndGpaSampleAMD.html>
    pub type PFN_vkCmdEndGpaSampleAMD = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
    );
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGpaSessionStatusAMD.html>
    pub type PFN_vkGetGpaSessionStatusAMD =
        unsafe extern "system" fn(device: Device, gpa_session: GpaSessionAMD) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGpaSessionResultsAMD.html>
    pub type PFN_vkGetGpaSessionResultsAMD = unsafe extern "system" fn(
        device: Device,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
        p_size_in_bytes: *mut usize,
        p_data: *mut c_void,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkResetGpaSessionAMD.html>
    pub type PFN_vkResetGpaSessionAMD =
        unsafe extern "system" fn(device: Device, gpa_session: GpaSessionAMD) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyGpaSessionResultsAMD.html>
    pub type PFN_vkCmdCopyGpaSessionResultsAMD =
        unsafe extern "system" fn(command_buffer: CommandBuffer, gpa_session: GpaSessionAMD);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkGpaSessionAMD = GpaSessionAMD;
    pub type VkGpaPerfBlockPropertiesAMD = GpaPerfBlockPropertiesAMD;
    pub type VkPhysicalDeviceGpaFeaturesAMD = PhysicalDeviceGpaFeaturesAMD<'static>;
    pub type VkPhysicalDeviceGpaPropertiesAMD = PhysicalDeviceGpaPropertiesAMD<'static>;
    pub type VkPhysicalDeviceGpaProperties2AMD = PhysicalDeviceGpaProperties2AMD<'static>;
    pub type VkGpaPerfCounterAMD = GpaPerfCounterAMD;
    pub type VkGpaSampleBeginInfoAMD = GpaSampleBeginInfoAMD<'static>;
    pub type VkGpaDeviceClockModeInfoAMD = GpaDeviceClockModeInfoAMD<'static>;
    pub type VkGpaDeviceGetClockInfoAMD = GpaDeviceGetClockInfoAMD<'static>;
    pub type VkGpaSessionCreateInfoAMD = GpaSessionCreateInfoAMD<'static>;
    pub type VkGpaPerfBlockAMD = GpaPerfBlockAMD;
    pub type VkGpaSampleTypeAMD = GpaSampleTypeAMD;
    pub type VkGpaDeviceClockModeAMD = GpaDeviceClockModeAMD;
    pub type VkGpaSqShaderStageFlagsAMD = GpaSqShaderStageFlagsAMD;
    pub type VkGpaSqShaderStageFlagBitsAMD = GpaSqShaderStageFlagBitsAMD;
    pub type VkGpaPerfBlockPropertiesFlagsAMD = GpaPerfBlockPropertiesFlagsAMD;
    pub type VkPhysicalDeviceGpaPropertiesFlagsAMD = PhysicalDeviceGpaPropertiesFlagsAMD;
    impl PhysicalDeviceGpaFeaturesAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceGpaFeaturesAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceGpaPropertiesAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceGpaPropertiesAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceGpaProperties2AMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceGpaProperties2AMD {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GpaSampleBeginInfoAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGpaSampleBeginInfoAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GpaDeviceClockModeInfoAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGpaDeviceClockModeInfoAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GpaDeviceGetClockInfoAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGpaDeviceGetClockInfoAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl GpaSessionCreateInfoAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkGpaSessionCreateInfoAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    create_gpa_session: PFN_vkCreateGpaSessionAMD,
    destroy_gpa_session: PFN_vkDestroyGpaSessionAMD,
    set_gpa_device_clock_mode: PFN_vkSetGpaDeviceClockModeAMD,
    get_gpa_device_clock_info: PFN_vkGetGpaDeviceClockInfoAMD,
    cmd_begin_gpa_session: PFN_vkCmdBeginGpaSessionAMD,
    cmd_end_gpa_session: PFN_vkCmdEndGpaSessionAMD,
    cmd_begin_gpa_sample: PFN_vkCmdBeginGpaSampleAMD,
    cmd_end_gpa_sample: PFN_vkCmdEndGpaSampleAMD,
    get_gpa_session_status: PFN_vkGetGpaSessionStatusAMD,
    get_gpa_session_results: PFN_vkGetGpaSessionResultsAMD,
    reset_gpa_session: PFN_vkResetGpaSessionAMD,
    cmd_copy_gpa_session_results: PFN_vkCmdCopyGpaSessionResultsAMD,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                create_gpa_session: transmute(
                    load(c"vkCreateGpaSessionAMD").ok_or(MissingEntryPointError)?,
                ),
                destroy_gpa_session: transmute(
                    load(c"vkDestroyGpaSessionAMD").ok_or(MissingEntryPointError)?,
                ),
                set_gpa_device_clock_mode: transmute(
                    load(c"vkSetGpaDeviceClockModeAMD").ok_or(MissingEntryPointError)?,
                ),
                get_gpa_device_clock_info: transmute(
                    load(c"vkGetGpaDeviceClockInfoAMD").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_gpa_session: transmute(
                    load(c"vkCmdBeginGpaSessionAMD").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_gpa_session: transmute(
                    load(c"vkCmdEndGpaSessionAMD").ok_or(MissingEntryPointError)?,
                ),
                cmd_begin_gpa_sample: transmute(
                    load(c"vkCmdBeginGpaSampleAMD").ok_or(MissingEntryPointError)?,
                ),
                cmd_end_gpa_sample: transmute(
                    load(c"vkCmdEndGpaSampleAMD").ok_or(MissingEntryPointError)?,
                ),
                get_gpa_session_status: transmute(
                    load(c"vkGetGpaSessionStatusAMD").ok_or(MissingEntryPointError)?,
                ),
                get_gpa_session_results: transmute(
                    load(c"vkGetGpaSessionResultsAMD").ok_or(MissingEntryPointError)?,
                ),
                reset_gpa_session: transmute(
                    load(c"vkResetGpaSessionAMD").ok_or(MissingEntryPointError)?,
                ),
                cmd_copy_gpa_session_results: transmute(
                    load(c"vkCmdCopyGpaSessionResultsAMD").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCreateGpaSessionAMD.html>
    #[inline]
    pub unsafe fn create_gpa_session(
        &self,
        device: Device,
        create_info: &GpaSessionCreateInfoAMD<'_>,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) -> crate::Result<GpaSessionAMD> {
        unsafe {
            let mut gpa_session = core::mem::MaybeUninit::uninit();
            let result = (self.create_gpa_session)(
                device,
                create_info,
                allocator.to_raw_ptr(),
                gpa_session.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(gpa_session.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkDestroyGpaSessionAMD.html>
    #[inline]
    pub unsafe fn destroy_gpa_session(
        &self,
        device: Device,
        gpa_session: GpaSessionAMD,
        allocator: Option<&AllocationCallbacks<'_>>,
    ) {
        unsafe { (self.destroy_gpa_session)(device, gpa_session, allocator.to_raw_ptr()) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetGpaDeviceClockModeAMD.html>
    #[inline]
    pub unsafe fn set_gpa_device_clock_mode(
        &self,
        device: Device,
        info: &mut GpaDeviceClockModeInfoAMD<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.set_gpa_device_clock_mode)(device, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGpaDeviceClockInfoAMD.html>
    #[inline]
    pub unsafe fn get_gpa_device_clock_info(
        &self,
        device: Device,
        info: &mut GpaDeviceGetClockInfoAMD<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_gpa_device_clock_info)(device, info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginGpaSessionAMD.html>
    #[inline]
    pub unsafe fn cmd_begin_gpa_session(
        &self,
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.cmd_begin_gpa_session)(command_buffer, gpa_session);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndGpaSessionAMD.html>
    #[inline]
    pub unsafe fn cmd_end_gpa_session(
        &self,
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.cmd_end_gpa_session)(command_buffer, gpa_session);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdBeginGpaSampleAMD.html>
    #[inline]
    pub unsafe fn cmd_begin_gpa_sample(
        &self,
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
        gpa_sample_begin_info: &GpaSampleBeginInfoAMD<'_>,
    ) -> crate::Result<u32> {
        unsafe {
            let mut sample_id = core::mem::MaybeUninit::uninit();
            let result = (self.cmd_begin_gpa_sample)(
                command_buffer,
                gpa_session,
                gpa_sample_begin_info,
                sample_id.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(sample_id.assume_init()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEndGpaSampleAMD.html>
    #[inline]
    pub unsafe fn cmd_end_gpa_sample(
        &self,
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
    ) {
        unsafe { (self.cmd_end_gpa_sample)(command_buffer, gpa_session, sample_id) }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGpaSessionStatusAMD.html>
    #[inline]
    pub unsafe fn get_gpa_session_status(
        &self,
        device: Device,
        gpa_session: GpaSessionAMD,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_gpa_session_status)(device, gpa_session);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetGpaSessionResultsAMD.html>
    #[inline]
    pub unsafe fn get_gpa_session_results(
        &self,
        device: Device,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
        mut data: impl EnumerateInto<u8>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |size_in_bytes, data| {
                let result = (self.get_gpa_session_results)(
                    device,
                    gpa_session,
                    sample_id,
                    size_in_bytes,
                    data as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut())?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let data_buf = data.reserve(capacity);
            len = data_buf.len().try_into().unwrap();
            let result = call(&mut len, data_buf.as_mut_ptr() as *mut _)?;
            data.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkResetGpaSessionAMD.html>
    #[inline]
    pub unsafe fn reset_gpa_session(
        &self,
        device: Device,
        gpa_session: GpaSessionAMD,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.reset_gpa_session)(device, gpa_session);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdCopyGpaSessionResultsAMD.html>
    #[inline]
    pub unsafe fn cmd_copy_gpa_session_results(
        &self,
        command_buffer: CommandBuffer,
        gpa_session: GpaSessionAMD,
    ) {
        unsafe { (self.cmd_copy_gpa_session_results)(command_buffer, gpa_session) }
    }
}
