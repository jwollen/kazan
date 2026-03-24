//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_encode_queue.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_encode_queue";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeSessionParametersGetInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeSessionParametersGetInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub video_session_parameters: VideoSessionParametersKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeSessionParametersGetInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeSessionParametersGetInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("video_session_parameters", &self.video_session_parameters)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeSessionParametersGetInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR;
    }

    impl Default for VideoEncodeSessionParametersGetInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                video_session_parameters: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeSessionParametersGetInfoKHR<'a> {
        #[inline]
        pub fn video_session_parameters(
            mut self,
            video_session_parameters: VideoSessionParametersKHR,
        ) -> Self {
            self.video_session_parameters = video_session_parameters;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeSessionParametersFeedbackInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeSessionParametersFeedbackInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub has_overrides: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeSessionParametersFeedbackInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeSessionParametersFeedbackInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("has_overrides", &self.has_overrides)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeSessionParametersFeedbackInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR;
    }

    impl Default for VideoEncodeSessionParametersFeedbackInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                has_overrides: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeSessionParametersFeedbackInfoKHR<'a> {
        #[inline]
        pub fn has_overrides(mut self, has_overrides: bool) -> Self {
            self.has_overrides = has_overrides.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeUsageInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeUsageInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub video_usage_hints: VideoEncodeUsageFlagsKHR,
        pub video_content_hints: VideoEncodeContentFlagsKHR,
        pub tuning_mode: VideoEncodeTuningModeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeUsageInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeUsageInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("video_usage_hints", &self.video_usage_hints)
                .field("video_content_hints", &self.video_content_hints)
                .field("tuning_mode", &self.tuning_mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeUsageInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_USAGE_INFO_KHR;
    }

    unsafe impl Extends<VideoProfileInfoKHR<'_>> for VideoEncodeUsageInfoKHR<'_> {}
    unsafe impl Extends<QueryPoolCreateInfo<'_>> for VideoEncodeUsageInfoKHR<'_> {}

    impl Default for VideoEncodeUsageInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                video_usage_hints: Default::default(),
                video_content_hints: Default::default(),
                tuning_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeUsageInfoKHR<'a> {
        #[inline]
        pub fn video_usage_hints(mut self, video_usage_hints: VideoEncodeUsageFlagsKHR) -> Self {
            self.video_usage_hints = video_usage_hints;
            self
        }

        #[inline]
        pub fn video_content_hints(
            mut self,
            video_content_hints: VideoEncodeContentFlagsKHR,
        ) -> Self {
            self.video_content_hints = video_content_hints;
            self
        }

        #[inline]
        pub fn tuning_mode(mut self, tuning_mode: VideoEncodeTuningModeKHR) -> Self {
            self.tuning_mode = tuning_mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoEncodeFlagsKHR,
        pub dst_buffer: Buffer,
        pub dst_buffer_offset: DeviceSize,
        pub dst_buffer_range: DeviceSize,
        pub src_picture_resource: VideoPictureResourceInfoKHR<'a>,
        pub p_setup_reference_slot: *const VideoReferenceSlotInfoKHR<'a>,
        pub reference_slot_count: u32,
        pub p_reference_slots: *const VideoReferenceSlotInfoKHR<'a>,
        pub preceding_externally_encoded_bytes: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("dst_buffer", &self.dst_buffer)
                .field("dst_buffer_offset", &self.dst_buffer_offset)
                .field("dst_buffer_range", &self.dst_buffer_range)
                .field("src_picture_resource", &self.src_picture_resource)
                .field("p_setup_reference_slot", &self.p_setup_reference_slot)
                .field("reference_slot_count", &self.reference_slot_count)
                .field("p_reference_slots", &self.p_reference_slots)
                .field(
                    "preceding_externally_encoded_bytes",
                    &self.preceding_externally_encoded_bytes,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_INFO_KHR;
    }

    impl Default for VideoEncodeInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                dst_buffer: Default::default(),
                dst_buffer_offset: Default::default(),
                dst_buffer_range: Default::default(),
                src_picture_resource: Default::default(),
                p_setup_reference_slot: ptr::null(),
                reference_slot_count: Default::default(),
                p_reference_slots: ptr::null(),
                preceding_externally_encoded_bytes: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoEncodeFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn dst_buffer(mut self, dst_buffer: Buffer) -> Self {
            self.dst_buffer = dst_buffer;
            self
        }

        #[inline]
        pub fn dst_buffer_offset(mut self, dst_buffer_offset: DeviceSize) -> Self {
            self.dst_buffer_offset = dst_buffer_offset;
            self
        }

        #[inline]
        pub fn dst_buffer_range(mut self, dst_buffer_range: DeviceSize) -> Self {
            self.dst_buffer_range = dst_buffer_range;
            self
        }

        #[inline]
        pub fn src_picture_resource(
            mut self,
            src_picture_resource: VideoPictureResourceInfoKHR<'a>,
        ) -> Self {
            self.src_picture_resource = src_picture_resource;
            self
        }

        #[inline]
        pub fn setup_reference_slot(
            mut self,
            setup_reference_slot: &'a VideoReferenceSlotInfoKHR<'a>,
        ) -> Self {
            self.p_setup_reference_slot = setup_reference_slot;
            self
        }

        #[inline]
        pub fn reference_slots(
            mut self,
            reference_slots: &'a [VideoReferenceSlotInfoKHR<'_>],
        ) -> Self {
            self.reference_slot_count = reference_slots.len().try_into().unwrap();
            self.p_reference_slots = reference_slots.as_ptr() as _;
            self
        }

        #[inline]
        pub fn preceding_externally_encoded_bytes(
            mut self,
            preceding_externally_encoded_bytes: u32,
        ) -> Self {
            self.preceding_externally_encoded_bytes = preceding_externally_encoded_bytes;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryPoolVideoEncodeFeedbackCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueryPoolVideoEncodeFeedbackCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueryPoolVideoEncodeFeedbackCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueryPoolVideoEncodeFeedbackCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("encode_feedback_flags", &self.encode_feedback_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueryPoolVideoEncodeFeedbackCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR;
    }

    unsafe impl Extends<QueryPoolCreateInfo<'_>> for QueryPoolVideoEncodeFeedbackCreateInfoKHR<'_> {}

    impl Default for QueryPoolVideoEncodeFeedbackCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                encode_feedback_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueryPoolVideoEncodeFeedbackCreateInfoKHR<'a> {
        #[inline]
        pub fn encode_feedback_flags(
            mut self,
            encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
        ) -> Self {
            self.encode_feedback_flags = encode_feedback_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeQualityLevelInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeQualityLevelInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub quality_level: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeQualityLevelInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeQualityLevelInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("quality_level", &self.quality_level)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeQualityLevelInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR;
    }

    unsafe impl Extends<VideoCodingControlInfoKHR<'_>> for VideoEncodeQualityLevelInfoKHR<'_> {}
    unsafe impl Extends<VideoSessionParametersCreateInfoKHR<'_>>
        for VideoEncodeQualityLevelInfoKHR<'_>
    {
    }

    impl Default for VideoEncodeQualityLevelInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                quality_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeQualityLevelInfoKHR<'a> {
        #[inline]
        pub fn quality_level(mut self, quality_level: u32) -> Self {
            self.quality_level = quality_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_video_profile: *const VideoProfileInfoKHR<'a>,
        pub quality_level: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVideoEncodeQualityLevelInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_video_profile", &self.p_video_profile)
                .field("quality_level", &self.quality_level)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR;
    }

    impl Default for PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_video_profile: ptr::null(),
                quality_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'a> {
        #[inline]
        pub fn video_profile(mut self, video_profile: &'a VideoProfileInfoKHR<'a>) -> Self {
            self.p_video_profile = video_profile;
            self
        }

        #[inline]
        pub fn quality_level(mut self, quality_level: u32) -> Self {
            self.quality_level = quality_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeQualityLevelPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeQualityLevelPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub preferred_rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
        pub preferred_rate_control_layer_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeQualityLevelPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeQualityLevelPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "preferred_rate_control_mode",
                    &self.preferred_rate_control_mode,
                )
                .field(
                    "preferred_rate_control_layer_count",
                    &self.preferred_rate_control_layer_count,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeQualityLevelPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR;
    }

    impl Default for VideoEncodeQualityLevelPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                preferred_rate_control_mode: Default::default(),
                preferred_rate_control_layer_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeQualityLevelPropertiesKHR<'a> {
        #[inline]
        pub fn preferred_rate_control_mode(
            mut self,
            preferred_rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
        ) -> Self {
            self.preferred_rate_control_mode = preferred_rate_control_mode;
            self
        }

        #[inline]
        pub fn preferred_rate_control_layer_count(
            mut self,
            preferred_rate_control_layer_count: u32,
        ) -> Self {
            self.preferred_rate_control_layer_count = preferred_rate_control_layer_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRateControlInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeRateControlInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoEncodeRateControlFlagsKHR,
        pub rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
        pub layer_count: u32,
        pub p_layers: *const VideoEncodeRateControlLayerInfoKHR<'a>,
        pub virtual_buffer_size_in_ms: u32,
        pub initial_virtual_buffer_size_in_ms: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeRateControlInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeRateControlInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("rate_control_mode", &self.rate_control_mode)
                .field("layer_count", &self.layer_count)
                .field("p_layers", &self.p_layers)
                .field("virtual_buffer_size_in_ms", &self.virtual_buffer_size_in_ms)
                .field(
                    "initial_virtual_buffer_size_in_ms",
                    &self.initial_virtual_buffer_size_in_ms,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeRateControlInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR;
    }

    unsafe impl Extends<VideoCodingControlInfoKHR<'_>> for VideoEncodeRateControlInfoKHR<'_> {}
    unsafe impl Extends<VideoBeginCodingInfoKHR<'_>> for VideoEncodeRateControlInfoKHR<'_> {}

    impl Default for VideoEncodeRateControlInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                rate_control_mode: Default::default(),
                layer_count: Default::default(),
                p_layers: ptr::null(),
                virtual_buffer_size_in_ms: Default::default(),
                initial_virtual_buffer_size_in_ms: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeRateControlInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoEncodeRateControlFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn rate_control_mode(
            mut self,
            rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
        ) -> Self {
            self.rate_control_mode = rate_control_mode;
            self
        }

        #[inline]
        pub fn layers(mut self, layers: &'a [VideoEncodeRateControlLayerInfoKHR<'_>]) -> Self {
            self.layer_count = layers.len().try_into().unwrap();
            self.p_layers = layers.as_ptr() as _;
            self
        }

        #[inline]
        pub fn virtual_buffer_size_in_ms(mut self, virtual_buffer_size_in_ms: u32) -> Self {
            self.virtual_buffer_size_in_ms = virtual_buffer_size_in_ms;
            self
        }

        #[inline]
        pub fn initial_virtual_buffer_size_in_ms(
            mut self,
            initial_virtual_buffer_size_in_ms: u32,
        ) -> Self {
            self.initial_virtual_buffer_size_in_ms = initial_virtual_buffer_size_in_ms;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRateControlLayerInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeRateControlLayerInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub average_bitrate: u64,
        pub max_bitrate: u64,
        pub frame_rate_numerator: u32,
        pub frame_rate_denominator: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeRateControlLayerInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeRateControlLayerInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("average_bitrate", &self.average_bitrate)
                .field("max_bitrate", &self.max_bitrate)
                .field("frame_rate_numerator", &self.frame_rate_numerator)
                .field("frame_rate_denominator", &self.frame_rate_denominator)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeRateControlLayerInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR;
    }

    impl Default for VideoEncodeRateControlLayerInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                average_bitrate: Default::default(),
                max_bitrate: Default::default(),
                frame_rate_numerator: Default::default(),
                frame_rate_denominator: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeRateControlLayerInfoKHR<'a> {
        #[inline]
        pub fn average_bitrate(mut self, average_bitrate: u64) -> Self {
            self.average_bitrate = average_bitrate;
            self
        }

        #[inline]
        pub fn max_bitrate(mut self, max_bitrate: u64) -> Self {
            self.max_bitrate = max_bitrate;
            self
        }

        #[inline]
        pub fn frame_rate_numerator(mut self, frame_rate_numerator: u32) -> Self {
            self.frame_rate_numerator = frame_rate_numerator;
            self
        }

        #[inline]
        pub fn frame_rate_denominator(mut self, frame_rate_denominator: u32) -> Self {
            self.frame_rate_denominator = frame_rate_denominator;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: VideoEncodeCapabilityFlagsKHR,
        pub rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
        pub max_rate_control_layers: u32,
        pub max_bitrate: u64,
        pub max_quality_levels: u32,
        pub encode_input_picture_granularity: Extent2D,
        pub supported_encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("rate_control_modes", &self.rate_control_modes)
                .field("max_rate_control_layers", &self.max_rate_control_layers)
                .field("max_bitrate", &self.max_bitrate)
                .field("max_quality_levels", &self.max_quality_levels)
                .field(
                    "encode_input_picture_granularity",
                    &self.encode_input_picture_granularity,
                )
                .field(
                    "supported_encode_feedback_flags",
                    &self.supported_encode_feedback_flags,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_CAPABILITIES_KHR;
    }

    unsafe impl Extends<VideoCapabilitiesKHR<'_>> for VideoEncodeCapabilitiesKHR<'_> {}

    impl Default for VideoEncodeCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                rate_control_modes: Default::default(),
                max_rate_control_layers: Default::default(),
                max_bitrate: Default::default(),
                max_quality_levels: Default::default(),
                encode_input_picture_granularity: Default::default(),
                supported_encode_feedback_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeCapabilitiesKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoEncodeCapabilityFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn rate_control_modes(
            mut self,
            rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
        ) -> Self {
            self.rate_control_modes = rate_control_modes;
            self
        }

        #[inline]
        pub fn max_rate_control_layers(mut self, max_rate_control_layers: u32) -> Self {
            self.max_rate_control_layers = max_rate_control_layers;
            self
        }

        #[inline]
        pub fn max_bitrate(mut self, max_bitrate: u64) -> Self {
            self.max_bitrate = max_bitrate;
            self
        }

        #[inline]
        pub fn max_quality_levels(mut self, max_quality_levels: u32) -> Self {
            self.max_quality_levels = max_quality_levels;
            self
        }

        #[inline]
        pub fn encode_input_picture_granularity(
            mut self,
            encode_input_picture_granularity: Extent2D,
        ) -> Self {
            self.encode_input_picture_granularity = encode_input_picture_granularity;
            self
        }

        #[inline]
        pub fn supported_encode_feedback_flags(
            mut self,
            supported_encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
        ) -> Self {
            self.supported_encode_feedback_flags = supported_encode_feedback_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeTuningModeKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeTuningModeKHR(i32);

    impl VideoEncodeTuningModeKHR {
        pub const DEFAULT_KHR: Self = Self(0);
        pub const HIGH_QUALITY_KHR: Self = Self(1);
        pub const LOW_LATENCY_KHR: Self = Self(2);
        pub const ULTRA_LOW_LATENCY_KHR: Self = Self(3);
        pub const LOSSLESS_KHR: Self = Self(4);
    }

    impl fmt::Debug for VideoEncodeTuningModeKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_KHR => Some("DEFAULT_KHR"),
                Self::HIGH_QUALITY_KHR => Some("HIGH_QUALITY_KHR"),
                Self::LOW_LATENCY_KHR => Some("LOW_LATENCY_KHR"),
                Self::ULTRA_LOW_LATENCY_KHR => Some("ULTRA_LOW_LATENCY_KHR"),
                Self::LOSSLESS_KHR => Some("LOSSLESS_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeFlagsKHR, Flags, VideoEncodeFlagBitsKHR);

    impl fmt::Debug for VideoEncodeFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeFlagBitsKHR::INTRA_REFRESH_KHR.0,
                    "INTRA_REFRESH_KHR",
                ),
                (
                    VideoEncodeFlagBitsKHR::WITH_QUANTIZATION_DELTA_MAP_KHR.0,
                    "WITH_QUANTIZATION_DELTA_MAP_KHR",
                ),
                (
                    VideoEncodeFlagBitsKHR::WITH_EMPHASIS_MAP_KHR.0,
                    "WITH_EMPHASIS_MAP_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeFlagBitsKHR(u32);

    impl VideoEncodeFlagBitsKHR {
        // VK_KHR_video_encode_intra_refresh
        pub const INTRA_REFRESH_KHR: Self = Self(1 << 2);

        // VK_KHR_video_encode_quantization_map
        pub const WITH_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 0);
        pub const WITH_EMPHASIS_MAP_KHR: Self = Self(1 << 1);
    }

    impl fmt::Debug for VideoEncodeFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INTRA_REFRESH_KHR => Some("INTRA_REFRESH_KHR"),
                Self::WITH_QUANTIZATION_DELTA_MAP_KHR => Some("WITH_QUANTIZATION_DELTA_MAP_KHR"),
                Self::WITH_EMPHASIS_MAP_KHR => Some("WITH_EMPHASIS_MAP_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeUsageFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeUsageFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeUsageFlagsKHR, Flags, VideoEncodeUsageFlagBitsKHR);

    impl VideoEncodeUsageFlagsKHR {
        pub const DEFAULT: Self = Self(0);
    }

    impl fmt::Debug for VideoEncodeUsageFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeUsageFlagBitsKHR::TRANSCODING_KHR.0,
                    "TRANSCODING_KHR",
                ),
                (
                    VideoEncodeUsageFlagBitsKHR::STREAMING_KHR.0,
                    "STREAMING_KHR",
                ),
                (
                    VideoEncodeUsageFlagBitsKHR::RECORDING_KHR.0,
                    "RECORDING_KHR",
                ),
                (
                    VideoEncodeUsageFlagBitsKHR::CONFERENCING_KHR.0,
                    "CONFERENCING_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeUsageFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeUsageFlagBitsKHR(u32);

    impl VideoEncodeUsageFlagBitsKHR {
        pub const TRANSCODING_KHR: Self = Self(1 << 0);
        pub const STREAMING_KHR: Self = Self(1 << 1);
        pub const RECORDING_KHR: Self = Self(1 << 2);
        pub const CONFERENCING_KHR: Self = Self(1 << 3);
    }

    impl fmt::Debug for VideoEncodeUsageFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TRANSCODING_KHR => Some("TRANSCODING_KHR"),
                Self::STREAMING_KHR => Some("STREAMING_KHR"),
                Self::RECORDING_KHR => Some("RECORDING_KHR"),
                Self::CONFERENCING_KHR => Some("CONFERENCING_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeContentFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeContentFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoEncodeContentFlagsKHR,
        Flags,
        VideoEncodeContentFlagBitsKHR
    );

    impl VideoEncodeContentFlagsKHR {
        pub const DEFAULT: Self = Self(0);
    }

    impl fmt::Debug for VideoEncodeContentFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (VideoEncodeContentFlagBitsKHR::CAMERA_KHR.0, "CAMERA_KHR"),
                (VideoEncodeContentFlagBitsKHR::DESKTOP_KHR.0, "DESKTOP_KHR"),
                (
                    VideoEncodeContentFlagBitsKHR::RENDERED_KHR.0,
                    "RENDERED_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeContentFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeContentFlagBitsKHR(u32);

    impl VideoEncodeContentFlagBitsKHR {
        pub const CAMERA_KHR: Self = Self(1 << 0);
        pub const DESKTOP_KHR: Self = Self(1 << 1);
        pub const RENDERED_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for VideoEncodeContentFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::CAMERA_KHR => Some("CAMERA_KHR"),
                Self::DESKTOP_KHR => Some("DESKTOP_KHR"),
                Self::RENDERED_KHR => Some("RENDERED_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeCapabilityFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeCapabilityFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoEncodeCapabilityFlagsKHR,
        Flags,
        VideoEncodeCapabilityFlagBitsKHR
    );

    impl fmt::Debug for VideoEncodeCapabilityFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeCapabilityFlagBitsKHR::PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR.0,
                    "PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR",
                ),
                (
                    VideoEncodeCapabilityFlagBitsKHR::INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR
                        .0,
                    "INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR",
                ),
                (
                    VideoEncodeCapabilityFlagBitsKHR::QUANTIZATION_DELTA_MAP_KHR.0,
                    "QUANTIZATION_DELTA_MAP_KHR",
                ),
                (
                    VideoEncodeCapabilityFlagBitsKHR::EMPHASIS_MAP_KHR.0,
                    "EMPHASIS_MAP_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeCapabilityFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeCapabilityFlagBitsKHR(u32);

    impl VideoEncodeCapabilityFlagBitsKHR {
        pub const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR: Self = Self(1 << 0);
        pub const INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR: Self = Self(1 << 1);
        // VK_KHR_video_encode_quantization_map
        pub const QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 2);
        pub const EMPHASIS_MAP_KHR: Self = Self(1 << 3);
    }

    impl fmt::Debug for VideoEncodeCapabilityFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR => {
                    Some("PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR")
                }
                Self::INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR => {
                    Some("INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR")
                }
                Self::QUANTIZATION_DELTA_MAP_KHR => Some("QUANTIZATION_DELTA_MAP_KHR"),
                Self::EMPHASIS_MAP_KHR => Some("EMPHASIS_MAP_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeFeedbackFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeFeedbackFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoEncodeFeedbackFlagsKHR,
        Flags,
        VideoEncodeFeedbackFlagBitsKHR
    );

    impl fmt::Debug for VideoEncodeFeedbackFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_BUFFER_OFFSET_KHR.0,
                    "BITSTREAM_BUFFER_OFFSET_KHR",
                ),
                (
                    VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_BYTES_WRITTEN_KHR.0,
                    "BITSTREAM_BYTES_WRITTEN_KHR",
                ),
                (
                    VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_HAS_OVERRIDES_KHR.0,
                    "BITSTREAM_HAS_OVERRIDES_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeFeedbackFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeFeedbackFlagBitsKHR(u32);

    impl VideoEncodeFeedbackFlagBitsKHR {
        pub const BITSTREAM_BUFFER_OFFSET_KHR: Self = Self(1 << 0);
        pub const BITSTREAM_BYTES_WRITTEN_KHR: Self = Self(1 << 1);
        pub const BITSTREAM_HAS_OVERRIDES_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for VideoEncodeFeedbackFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BITSTREAM_BUFFER_OFFSET_KHR => Some("BITSTREAM_BUFFER_OFFSET_KHR"),
                Self::BITSTREAM_BYTES_WRITTEN_KHR => Some("BITSTREAM_BYTES_WRITTEN_KHR"),
                Self::BITSTREAM_HAS_OVERRIDES_KHR => Some("BITSTREAM_HAS_OVERRIDES_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRateControlFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRateControlFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeRateControlFlagsKHR, Flags);

    impl fmt::Debug for VideoEncodeRateControlFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRateControlModeFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRateControlModeFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoEncodeRateControlModeFlagsKHR,
        Flags,
        VideoEncodeRateControlModeFlagBitsKHR
    );

    impl VideoEncodeRateControlModeFlagsKHR {
        pub const DEFAULT: Self = Self(0);
    }

    impl fmt::Debug for VideoEncodeRateControlModeFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeRateControlModeFlagBitsKHR::DISABLED_KHR.0,
                    "DISABLED_KHR",
                ),
                (VideoEncodeRateControlModeFlagBitsKHR::CBR_KHR.0, "CBR_KHR"),
                (VideoEncodeRateControlModeFlagBitsKHR::VBR_KHR.0, "VBR_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeRateControlModeFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeRateControlModeFlagBitsKHR(u32);

    impl VideoEncodeRateControlModeFlagBitsKHR {
        pub const DISABLED_KHR: Self = Self(1 << 0);
        pub const CBR_KHR: Self = Self(1 << 1);
        pub const VBR_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for VideoEncodeRateControlModeFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DISABLED_KHR => Some("DISABLED_KHR"),
                Self::CBR_KHR => Some("CBR_KHR"),
                Self::VBR_KHR => Some("VBR_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html>
    pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
        unsafe extern "system" fn(
            physical_device: PhysicalDevice,
            p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
            p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR<'_>,
        ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetEncodedVideoSessionParametersKHR.html>
    pub type PFN_vkGetEncodedVideoSessionParametersKHR = unsafe extern "system" fn(
        device: Device,
        p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR<'_>,
        p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
        p_data_size: *mut usize,
        p_data: *mut c_void,
    )
        -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEncodeVideoKHR.html>
    pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_encode_info: *const VideoEncodeInfoKHR<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkVideoEncodeSessionParametersGetInfoKHR =
        VideoEncodeSessionParametersGetInfoKHR<'static>;
    pub type VkVideoEncodeSessionParametersFeedbackInfoKHR =
        VideoEncodeSessionParametersFeedbackInfoKHR<'static>;
    pub type VkVideoEncodeUsageInfoKHR = VideoEncodeUsageInfoKHR<'static>;
    pub type VkVideoEncodeInfoKHR = VideoEncodeInfoKHR<'static>;
    pub type VkQueryPoolVideoEncodeFeedbackCreateInfoKHR =
        QueryPoolVideoEncodeFeedbackCreateInfoKHR<'static>;
    pub type VkVideoEncodeQualityLevelInfoKHR = VideoEncodeQualityLevelInfoKHR<'static>;
    pub type VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR =
        PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'static>;
    pub type VkVideoEncodeQualityLevelPropertiesKHR = VideoEncodeQualityLevelPropertiesKHR<'static>;
    pub type VkVideoEncodeRateControlInfoKHR = VideoEncodeRateControlInfoKHR<'static>;
    pub type VkVideoEncodeRateControlLayerInfoKHR = VideoEncodeRateControlLayerInfoKHR<'static>;
    pub type VkVideoEncodeCapabilitiesKHR = VideoEncodeCapabilitiesKHR<'static>;
    pub type VkVideoEncodeTuningModeKHR = VideoEncodeTuningModeKHR;
    pub type VkVideoEncodeFlagsKHR = VideoEncodeFlagsKHR;
    pub type VkVideoEncodeFlagBitsKHR = VideoEncodeFlagBitsKHR;
    pub type VkVideoEncodeUsageFlagsKHR = VideoEncodeUsageFlagsKHR;
    pub type VkVideoEncodeUsageFlagBitsKHR = VideoEncodeUsageFlagBitsKHR;
    pub type VkVideoEncodeContentFlagsKHR = VideoEncodeContentFlagsKHR;
    pub type VkVideoEncodeContentFlagBitsKHR = VideoEncodeContentFlagBitsKHR;
    pub type VkVideoEncodeCapabilityFlagsKHR = VideoEncodeCapabilityFlagsKHR;
    pub type VkVideoEncodeCapabilityFlagBitsKHR = VideoEncodeCapabilityFlagBitsKHR;
    pub type VkVideoEncodeFeedbackFlagsKHR = VideoEncodeFeedbackFlagsKHR;
    pub type VkVideoEncodeFeedbackFlagBitsKHR = VideoEncodeFeedbackFlagBitsKHR;
    pub type VkVideoEncodeRateControlFlagsKHR = VideoEncodeRateControlFlagsKHR;
    pub type VkVideoEncodeRateControlModeFlagsKHR = VideoEncodeRateControlModeFlagsKHR;
    pub type VkVideoEncodeRateControlModeFlagBitsKHR = VideoEncodeRateControlModeFlagBitsKHR;
    impl VideoEncodeSessionParametersGetInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeSessionParametersGetInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeSessionParametersFeedbackInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeSessionParametersFeedbackInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeUsageInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeUsageInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl QueryPoolVideoEncodeFeedbackCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkQueryPoolVideoEncodeFeedbackCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeQualityLevelInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeQualityLevelInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeQualityLevelPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeQualityLevelPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeRateControlInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeRateControlInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeRateControlLayerInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeRateControlLayerInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct InstanceFn {
    get_physical_device_video_encode_quality_level_properties:
        PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
}

impl LoadInstanceFn for InstanceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_physical_device_video_encode_quality_level_properties: transmute(
                    load(c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl InstanceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_physical_device_video_encode_quality_level_properties(
        &self,
        physical_device: PhysicalDevice,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
        quality_level_properties: &mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.get_physical_device_video_encode_quality_level_properties)(
                physical_device,
                quality_level_info,
                quality_level_properties,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}

pub struct DeviceFn {
    get_encoded_video_session_parameters: PFN_vkGetEncodedVideoSessionParametersKHR,
    cmd_encode_video: PFN_vkCmdEncodeVideoKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_encoded_video_session_parameters: transmute(
                    load(c"vkGetEncodedVideoSessionParametersKHR").ok_or(MissingEntryPointError)?,
                ),
                cmd_encode_video: transmute(
                    load(c"vkCmdEncodeVideoKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetEncodedVideoSessionParametersKHR.html>
    #[inline]
    pub unsafe fn get_encoded_video_session_parameters<'a>(
        &self,
        device: Device,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR<'a>,
        feedback_info: Option<&mut VideoEncodeSessionParametersFeedbackInfoKHR<'a>>,
        mut data: impl EnumerateInto<u8>,
    ) -> crate::Result<()> {
        unsafe {
            let call = |data_size,
                        data,
                        feedback_info: Option<
                &mut VideoEncodeSessionParametersFeedbackInfoKHR<'a>,
            >| {
                let result = (self.get_encoded_video_session_parameters)(
                    device,
                    video_session_parameters_info,
                    feedback_info.to_raw_mut_ptr(),
                    data_size,
                    data as _,
                );

                match result {
                    VkResult::SUCCESS => Ok(()),
                    VkResult::INCOMPLETE => Ok(()),
                    err => Err(err),
                }
            };
            let mut len = 0;
            call(&mut len, std::ptr::null_mut(), None)?;
            let capacity = len.try_into().expect("failed to convert `N` to usize");
            let data_buf = data.reserve(capacity);
            len = data_buf.len().try_into().unwrap();
            let result = call(&mut len, data_buf.as_mut_ptr() as *mut _, feedback_info)?;
            data.set_len(len.try_into().unwrap());
            Ok(result)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdEncodeVideoKHR.html>
    #[inline]
    pub unsafe fn cmd_encode_video(
        &self,
        command_buffer: CommandBuffer,
        encode_info: &VideoEncodeInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_encode_video)(command_buffer, encode_info) }
    }
}
