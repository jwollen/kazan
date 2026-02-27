#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeSessionParametersGetInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_session_parameters: VideoSessionParametersKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeSessionParametersGetInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_SESSION_PARAMETERS_GET_INFO_KHR,
            p_next: core::ptr::null(),
            video_session_parameters: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeSessionParametersGetInfoKHR<'a> {
    pub fn video_session_parameters(
        mut self,
        video_session_parameters: VideoSessionParametersKHR,
    ) -> Self {
        self.video_session_parameters = video_session_parameters;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeSessionParametersFeedbackInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_overrides: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeSessionParametersFeedbackInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_SESSION_PARAMETERS_FEEDBACK_INFO_KHR,
            p_next: core::ptr::null_mut(),
            has_overrides: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeSessionParametersFeedbackInfoKHR<'a> {
    pub fn has_overrides(mut self, has_overrides: Bool32) -> Self {
        self.has_overrides = has_overrides;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeUsageInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_usage_hints: VideoEncodeUsageFlagsKHR,
    pub video_content_hints: VideoEncodeContentFlagsKHR,
    pub tuning_mode: VideoEncodeTuningModeKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeUsageInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_USAGE_INFO_KHR,
            p_next: core::ptr::null(),
            video_usage_hints: Default::default(),
            video_content_hints: Default::default(),
            tuning_mode: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeUsageInfoKHR<'a> {
    pub fn video_usage_hints(mut self, video_usage_hints: VideoEncodeUsageFlagsKHR) -> Self {
        self.video_usage_hints = video_usage_hints;
        self
    }
    pub fn video_content_hints(mut self, video_content_hints: VideoEncodeContentFlagsKHR) -> Self {
        self.video_content_hints = video_content_hints;
        self
    }
    pub fn tuning_mode(mut self, tuning_mode: VideoEncodeTuningModeKHR) -> Self {
        self.tuning_mode = tuning_mode;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoEncodeInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            dst_buffer: Default::default(),
            dst_buffer_offset: Default::default(),
            dst_buffer_range: Default::default(),
            src_picture_resource: Default::default(),
            p_setup_reference_slot: core::ptr::null(),
            reference_slot_count: Default::default(),
            p_reference_slots: core::ptr::null(),
            preceding_externally_encoded_bytes: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeInfoKHR<'a> {
    pub fn flags(mut self, flags: VideoEncodeFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn dst_buffer(mut self, dst_buffer: Buffer) -> Self {
        self.dst_buffer = dst_buffer;
        self
    }
    pub fn dst_buffer_offset(mut self, dst_buffer_offset: DeviceSize) -> Self {
        self.dst_buffer_offset = dst_buffer_offset;
        self
    }
    pub fn dst_buffer_range(mut self, dst_buffer_range: DeviceSize) -> Self {
        self.dst_buffer_range = dst_buffer_range;
        self
    }
    pub fn src_picture_resource(
        mut self,
        src_picture_resource: VideoPictureResourceInfoKHR<'a>,
    ) -> Self {
        self.src_picture_resource = src_picture_resource;
        self
    }
    pub fn setup_reference_slot(
        mut self,
        setup_reference_slot: &'a VideoReferenceSlotInfoKHR<'a>,
    ) -> Self {
        self.p_setup_reference_slot = setup_reference_slot;
        self
    }
    pub fn reference_slots(mut self, reference_slots: &'a [VideoReferenceSlotInfoKHR<'a>]) -> Self {
        self.reference_slot_count = reference_slots.len().try_into().unwrap();
        self.p_reference_slots = reference_slots.as_ptr();
        self
    }
    pub fn preceding_externally_encoded_bytes(
        mut self,
        preceding_externally_encoded_bytes: u32,
    ) -> Self {
        self.preceding_externally_encoded_bytes = preceding_externally_encoded_bytes;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolVideoEncodeFeedbackCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueryPoolVideoEncodeFeedbackCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUERY_POOL_VIDEO_ENCODE_FEEDBACK_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            encode_feedback_flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> QueryPoolVideoEncodeFeedbackCreateInfoKHR<'a> {
    pub fn encode_feedback_flags(
        mut self,
        encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
    ) -> Self {
        self.encode_feedback_flags = encode_feedback_flags;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQualityLevelInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quality_level: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeQualityLevelInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR,
            p_next: core::ptr::null(),
            quality_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeQualityLevelInfoKHR<'a> {
    pub fn quality_level(mut self, quality_level: u32) -> Self {
        self.quality_level = quality_level;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_video_profile: *const VideoProfileInfoKHR<'a>,
    pub quality_level: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_QUALITY_LEVEL_INFO_KHR,
            p_next: core::ptr::null(),
            p_video_profile: core::ptr::null(),
            quality_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'a> {
    pub fn video_profile(mut self, video_profile: &'a VideoProfileInfoKHR<'a>) -> Self {
        self.p_video_profile = video_profile;
        self
    }
    pub fn quality_level(mut self, quality_level: u32) -> Self {
        self.quality_level = quality_level;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQualityLevelPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub preferred_rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    pub preferred_rate_control_layer_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeQualityLevelPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_QUALITY_LEVEL_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            preferred_rate_control_mode: Default::default(),
            preferred_rate_control_layer_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeQualityLevelPropertiesKHR<'a> {
    pub fn preferred_rate_control_mode(
        mut self,
        preferred_rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    ) -> Self {
        self.preferred_rate_control_mode = preferred_rate_control_mode;
        self
    }
    pub fn preferred_rate_control_layer_count(
        mut self,
        preferred_rate_control_layer_count: u32,
    ) -> Self {
        self.preferred_rate_control_layer_count = preferred_rate_control_layer_count;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoEncodeRateControlInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_RATE_CONTROL_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            rate_control_mode: Default::default(),
            layer_count: Default::default(),
            p_layers: core::ptr::null(),
            virtual_buffer_size_in_ms: Default::default(),
            initial_virtual_buffer_size_in_ms: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeRateControlInfoKHR<'a> {
    pub fn flags(mut self, flags: VideoEncodeRateControlFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn rate_control_mode(
        mut self,
        rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    ) -> Self {
        self.rate_control_mode = rate_control_mode;
        self
    }
    pub fn layers(mut self, layers: &'a [VideoEncodeRateControlLayerInfoKHR<'a>]) -> Self {
        self.layer_count = layers.len().try_into().unwrap();
        self.p_layers = layers.as_ptr();
        self
    }
    pub fn virtual_buffer_size_in_ms(mut self, virtual_buffer_size_in_ms: u32) -> Self {
        self.virtual_buffer_size_in_ms = virtual_buffer_size_in_ms;
        self
    }
    pub fn initial_virtual_buffer_size_in_ms(
        mut self,
        initial_virtual_buffer_size_in_ms: u32,
    ) -> Self {
        self.initial_virtual_buffer_size_in_ms = initial_virtual_buffer_size_in_ms;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeRateControlLayerInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub average_bitrate: u64,
    pub max_bitrate: u64,
    pub frame_rate_numerator: u32,
    pub frame_rate_denominator: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeRateControlLayerInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_RATE_CONTROL_LAYER_INFO_KHR,
            p_next: core::ptr::null(),
            average_bitrate: Default::default(),
            max_bitrate: Default::default(),
            frame_rate_numerator: Default::default(),
            frame_rate_denominator: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeRateControlLayerInfoKHR<'a> {
    pub fn average_bitrate(mut self, average_bitrate: u64) -> Self {
        self.average_bitrate = average_bitrate;
        self
    }
    pub fn max_bitrate(mut self, max_bitrate: u64) -> Self {
        self.max_bitrate = max_bitrate;
        self
    }
    pub fn frame_rate_numerator(mut self, frame_rate_numerator: u32) -> Self {
        self.frame_rate_numerator = frame_rate_numerator;
        self
    }
    pub fn frame_rate_denominator(mut self, frame_rate_denominator: u32) -> Self {
        self.frame_rate_denominator = frame_rate_denominator;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoEncodeCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
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
    pub fn flags(mut self, flags: VideoEncodeCapabilityFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn rate_control_modes(
        mut self,
        rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
    ) -> Self {
        self.rate_control_modes = rate_control_modes;
        self
    }
    pub fn max_rate_control_layers(mut self, max_rate_control_layers: u32) -> Self {
        self.max_rate_control_layers = max_rate_control_layers;
        self
    }
    pub fn max_bitrate(mut self, max_bitrate: u64) -> Self {
        self.max_bitrate = max_bitrate;
        self
    }
    pub fn max_quality_levels(mut self, max_quality_levels: u32) -> Self {
        self.max_quality_levels = max_quality_levels;
        self
    }
    pub fn encode_input_picture_granularity(
        mut self,
        encode_input_picture_granularity: Extent2D,
    ) -> Self {
        self.encode_input_picture_granularity = encode_input_picture_granularity;
        self
    }
    pub fn supported_encode_feedback_flags(
        mut self,
        supported_encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
    ) -> Self {
        self.supported_encode_feedback_flags = supported_encode_feedback_flags;
        self
    }
}
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeFlagsKHR: Flags {
        const WITH_QUANTIZATION_DELTA_MAP_KHR = VideoEncodeFlagBitsKHR::WITH_QUANTIZATION_DELTA_MAP_KHR.0;
        const WITH_EMPHASIS_MAP_KHR = VideoEncodeFlagBitsKHR::WITH_EMPHASIS_MAP_KHR.0;
        const INTRA_REFRESH_KHR = VideoEncodeFlagBitsKHR::INTRA_REFRESH_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeFlagBitsKHR(u32);
impl VideoEncodeFlagBitsKHR {
    pub const WITH_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 0);
    pub const WITH_EMPHASIS_MAP_KHR: Self = Self(1 << 1);
    pub const INTRA_REFRESH_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeUsageFlagsKHR: Flags {
        const TRANSCODING_KHR = VideoEncodeUsageFlagBitsKHR::TRANSCODING_KHR.0;
        const STREAMING_KHR = VideoEncodeUsageFlagBitsKHR::STREAMING_KHR.0;
        const RECORDING_KHR = VideoEncodeUsageFlagBitsKHR::RECORDING_KHR.0;
        const CONFERENCING_KHR = VideoEncodeUsageFlagBitsKHR::CONFERENCING_KHR.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeUsageFlagBitsKHR(u32);
impl VideoEncodeUsageFlagBitsKHR {
    pub const TRANSCODING_KHR: Self = Self(1 << 0);
    pub const STREAMING_KHR: Self = Self(1 << 1);
    pub const RECORDING_KHR: Self = Self(1 << 2);
    pub const CONFERENCING_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeContentFlagsKHR: Flags {
        const CAMERA_KHR = VideoEncodeContentFlagBitsKHR::CAMERA_KHR.0;
        const DESKTOP_KHR = VideoEncodeContentFlagBitsKHR::DESKTOP_KHR.0;
        const RENDERED_KHR = VideoEncodeContentFlagBitsKHR::RENDERED_KHR.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeContentFlagBitsKHR(u32);
impl VideoEncodeContentFlagBitsKHR {
    pub const CAMERA_KHR: Self = Self(1 << 0);
    pub const DESKTOP_KHR: Self = Self(1 << 1);
    pub const RENDERED_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeCapabilityFlagsKHR: Flags {
        const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR = VideoEncodeCapabilityFlagBitsKHR::PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR.0;
        const INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR = VideoEncodeCapabilityFlagBitsKHR::INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR.0;
        const QUANTIZATION_DELTA_MAP_KHR = VideoEncodeCapabilityFlagBitsKHR::QUANTIZATION_DELTA_MAP_KHR.0;
        const EMPHASIS_MAP_KHR = VideoEncodeCapabilityFlagBitsKHR::EMPHASIS_MAP_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeCapabilityFlagBitsKHR(u32);
impl VideoEncodeCapabilityFlagBitsKHR {
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR: Self = Self(1 << 0);
    pub const INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR: Self = Self(1 << 1);
    pub const QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 2);
    pub const EMPHASIS_MAP_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeFeedbackFlagsKHR: Flags {
        const BITSTREAM_BUFFER_OFFSET_KHR = VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_BUFFER_OFFSET_KHR.0;
        const BITSTREAM_BYTES_WRITTEN_KHR = VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_BYTES_WRITTEN_KHR.0;
        const BITSTREAM_HAS_OVERRIDES_KHR = VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_HAS_OVERRIDES_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeFeedbackFlagBitsKHR(u32);
impl VideoEncodeFeedbackFlagBitsKHR {
    pub const BITSTREAM_BUFFER_OFFSET_KHR: Self = Self(1 << 0);
    pub const BITSTREAM_BYTES_WRITTEN_KHR: Self = Self(1 << 1);
    pub const BITSTREAM_HAS_OVERRIDES_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeRateControlFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeRateControlModeFlagsKHR: Flags {
        const DISABLED_KHR = VideoEncodeRateControlModeFlagBitsKHR::DISABLED_KHR.0;
        const CBR_KHR = VideoEncodeRateControlModeFlagBitsKHR::CBR_KHR.0;
        const VBR_KHR = VideoEncodeRateControlModeFlagBitsKHR::VBR_KHR.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRateControlModeFlagBitsKHR(u32);
impl VideoEncodeRateControlModeFlagBitsKHR {
    pub const DISABLED_KHR: Self = Self(1 << 0);
    pub const CBR_KHR: Self = Self(1 << 1);
    pub const VBR_KHR: Self = Self(1 << 2);
}
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR<'_>,
        p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> Result;
pub type PFN_vkGetEncodedVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR<'_>,
    p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR<'_>,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_encode_info: *const VideoEncodeInfoKHR<'_>,
);
