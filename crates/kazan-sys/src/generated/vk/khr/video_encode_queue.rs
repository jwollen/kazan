#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeSessionParametersGetInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_session_parameters: VideoSessionParametersKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeSessionParametersFeedbackInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_overrides: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeUsageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_usage_hints: VideoEncodeUsageFlagsKHR,
    pub video_content_hints: VideoEncodeContentFlagsKHR,
    pub tuning_mode: VideoEncodeTuningModeKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeFlagsKHR,
    pub dst_buffer: Buffer,
    pub dst_buffer_offset: DeviceSize,
    pub dst_buffer_range: DeviceSize,
    pub src_picture_resource: VideoPictureResourceInfoKHR,
    pub p_setup_reference_slot: *const VideoReferenceSlotInfoKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR,
    pub preceding_externally_encoded_bytes: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueryPoolVideoEncodeFeedbackCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQualityLevelInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub quality_level: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeQualityLevelInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_video_profile: *const VideoProfileInfoKHR,
    pub quality_level: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeQualityLevelPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub preferred_rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    pub preferred_rate_control_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeRateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeRateControlFlagsKHR,
    pub rate_control_mode: VideoEncodeRateControlModeFlagBitsKHR,
    pub layer_count: u32,
    pub p_layers: *const VideoEncodeRateControlLayerInfoKHR,
    pub virtual_buffer_size_in_ms: u32,
    pub initial_virtual_buffer_size_in_ms: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeRateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub average_bitrate: u64,
    pub max_bitrate: u64,
    pub frame_rate_numerator: u32,
    pub frame_rate_denominator: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoEncodeCapabilityFlagsKHR,
    pub rate_control_modes: VideoEncodeRateControlModeFlagsKHR,
    pub max_rate_control_layers: u32,
    pub max_bitrate: u64,
    pub max_quality_levels: u32,
    pub encode_input_picture_granularity: Extent2D,
    pub supported_encode_feedback_flags: VideoEncodeFeedbackFlagsKHR,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeFlagsKHR: Flags {
        const WITH_QUANTIZATION_DELTA_MAP_KHR = VideoEncodeFlagBitsKHR::WITH_QUANTIZATION_DELTA_MAP_KHR.0;
        const WITH_EMPHASIS_MAP_KHR = VideoEncodeFlagBitsKHR::WITH_EMPHASIS_MAP_KHR.0;
        const INTRA_REFRESH_KHR = VideoEncodeFlagBitsKHR::INTRA_REFRESH_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeFlagBitsKHR(u32);
impl VideoEncodeFlagBitsKHR {
    pub const WITH_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 0);
    pub const WITH_EMPHASIS_MAP_KHR: Self = Self(1 << 1);
    pub const INTRA_REFRESH_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeUsageFlagsKHR: Flags {
        const TRANSCODING_KHR = VideoEncodeUsageFlagBitsKHR::TRANSCODING_KHR.0;
        const STREAMING_KHR = VideoEncodeUsageFlagBitsKHR::STREAMING_KHR.0;
        const RECORDING_KHR = VideoEncodeUsageFlagBitsKHR::RECORDING_KHR.0;
        const CONFERENCING_KHR = VideoEncodeUsageFlagBitsKHR::CONFERENCING_KHR.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeUsageFlagBitsKHR(u32);
impl VideoEncodeUsageFlagBitsKHR {
    pub const TRANSCODING_KHR: Self = Self(1 << 0);
    pub const STREAMING_KHR: Self = Self(1 << 1);
    pub const RECORDING_KHR: Self = Self(1 << 2);
    pub const CONFERENCING_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeContentFlagsKHR: Flags {
        const CAMERA_KHR = VideoEncodeContentFlagBitsKHR::CAMERA_KHR.0;
        const DESKTOP_KHR = VideoEncodeContentFlagBitsKHR::DESKTOP_KHR.0;
        const RENDERED_KHR = VideoEncodeContentFlagBitsKHR::RENDERED_KHR.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeContentFlagBitsKHR(u32);
impl VideoEncodeContentFlagBitsKHR {
    pub const CAMERA_KHR: Self = Self(1 << 0);
    pub const DESKTOP_KHR: Self = Self(1 << 1);
    pub const RENDERED_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeCapabilityFlagsKHR: Flags {
        const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR = VideoEncodeCapabilityFlagBitsKHR::PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR.0;
        const INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR = VideoEncodeCapabilityFlagBitsKHR::INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR.0;
        const QUANTIZATION_DELTA_MAP_KHR = VideoEncodeCapabilityFlagBitsKHR::QUANTIZATION_DELTA_MAP_KHR.0;
        const EMPHASIS_MAP_KHR = VideoEncodeCapabilityFlagBitsKHR::EMPHASIS_MAP_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeCapabilityFlagBitsKHR(u32);
impl VideoEncodeCapabilityFlagBitsKHR {
    pub const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR: Self = Self(1 << 0);
    pub const INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR: Self = Self(1 << 1);
    pub const QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 2);
    pub const EMPHASIS_MAP_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeFeedbackFlagsKHR: Flags {
        const BITSTREAM_BUFFER_OFFSET_KHR = VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_BUFFER_OFFSET_KHR.0;
        const BITSTREAM_BYTES_WRITTEN_KHR = VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_BYTES_WRITTEN_KHR.0;
        const BITSTREAM_HAS_OVERRIDES_KHR = VideoEncodeFeedbackFlagBitsKHR::BITSTREAM_HAS_OVERRIDES_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeFeedbackFlagBitsKHR(u32);
impl VideoEncodeFeedbackFlagBitsKHR {
    pub const BITSTREAM_BUFFER_OFFSET_KHR: Self = Self(1 << 0);
    pub const BITSTREAM_BYTES_WRITTEN_KHR: Self = Self(1 << 1);
    pub const BITSTREAM_HAS_OVERRIDES_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRateControlFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeRateControlModeFlagsKHR: Flags {
        const DISABLED_KHR = VideoEncodeRateControlModeFlagBitsKHR::DISABLED_KHR.0;
        const CBR_KHR = VideoEncodeRateControlModeFlagBitsKHR::CBR_KHR.0;
        const VBR_KHR = VideoEncodeRateControlModeFlagBitsKHR::VBR_KHR.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeRateControlModeFlagBitsKHR(u32);
impl VideoEncodeRateControlModeFlagBitsKHR {
    pub const DISABLED_KHR: Self = Self(1 << 0);
    pub const CBR_KHR: Self = Self(1 << 1);
    pub const VBR_KHR: Self = Self(1 << 2);
}
pub type PFN_vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        p_quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> Result;
pub type PFN_vkGetEncodedVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    p_video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR,
    p_feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
    p_data_size: *mut usize,
    p_data: *mut c_void,
) -> Result;
pub type PFN_vkCmdEncodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_encode_info: *const VideoEncodeInfoKHR,
);
