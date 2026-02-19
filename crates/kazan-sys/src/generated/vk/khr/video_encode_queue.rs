#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
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
    pub preferred_rate_control_mode: VideoEncodeRateControlModeFlagsKHR,
    pub preferred_rate_control_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeRateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeRateControlFlagsKHR,
    pub rate_control_mode: VideoEncodeRateControlModeFlagsKHR,
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
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeUsageFlagsKHR: Flags {
        const TRANSCODING_KHR = 1 << 0;
        const STREAMING_KHR = 1 << 1;
        const RECORDING_KHR = 1 << 2;
        const CONFERENCING_KHR = 1 << 3;
        const DEFAULT = 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeContentFlagsKHR: Flags {
        const CAMERA_KHR = 1 << 0;
        const DESKTOP_KHR = 1 << 1;
        const RENDERED_KHR = 1 << 2;
        const DEFAULT = 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeCapabilityFlagsKHR: Flags {
        const PRECEDING_EXTERNALLY_ENCODED_BYTES_KHR = 1 << 0;
        const INSUFFICIENTSTREAM_BUFFER_RANGE_DETECTION_KHR = 1 << 1;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeFeedbackFlagsKHR: Flags {
        const BITSTREAM_BUFFER_OFFSET_KHR = 1 << 0;
        const BITSTREAM_BYTES_WRITTEN_KHR = 1 << 1;
        const BITSTREAM_HAS_OVERRIDES_KHR = 1 << 2;
    }
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
        const DISABLED_KHR = 1 << 0;
        const CBR_KHR = 1 << 1;
        const VBR_KHR = 1 << 2;
        const DEFAULT = 0;
    }
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
