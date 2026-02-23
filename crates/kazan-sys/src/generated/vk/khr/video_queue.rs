#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VideoSessionKHR(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VideoSessionParametersKHR(u64);
#[repr(C)]
pub struct QueueFamilyVideoPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_codec_operations: VideoCodecOperationFlagsKHR,
}
#[repr(C)]
pub struct QueueFamilyQueryResultStatusPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub query_result_status_support: Bool32,
}
#[repr(C)]
pub struct VideoProfileListInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub profile_count: u32,
    pub p_profiles: *const VideoProfileInfoKHR,
}
#[repr(C)]
pub struct PhysicalDeviceVideoFormatInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_usage: ImageUsageFlags,
}
#[repr(C)]
pub struct VideoFormatPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub component_mapping: ComponentMapping,
    pub image_create_flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub image_tiling: ImageTiling,
    pub image_usage_flags: ImageUsageFlags,
}
#[repr(C)]
pub struct VideoProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_codec_operation: VideoCodecOperationFlagBitsKHR,
    pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
}
#[repr(C)]
pub struct VideoCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoCapabilityFlagsKHR,
    pub min_bitstream_buffer_offset_alignment: DeviceSize,
    pub min_bitstream_buffer_size_alignment: DeviceSize,
    pub picture_access_granularity: Extent2D,
    pub min_coded_extent: Extent2D,
    pub max_coded_extent: Extent2D,
    pub max_dpb_slots: u32,
    pub max_active_reference_pictures: u32,
    pub std_header_version: ExtensionProperties,
}
#[repr(C)]
pub struct VideoSessionMemoryRequirementsKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_bind_index: u32,
    pub memory_requirements: MemoryRequirements,
}
#[repr(C)]
pub struct BindVideoSessionMemoryInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_bind_index: u32,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub memory_size: DeviceSize,
}
#[repr(C)]
pub struct VideoPictureResourceInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub coded_offset: Offset2D,
    pub coded_extent: Extent2D,
    pub base_array_layer: u32,
    pub image_view_binding: ImageView,
}
#[repr(C)]
pub struct VideoReferenceSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub slot_index: i32,
    pub p_picture_resource: *const VideoPictureResourceInfoKHR,
}
#[repr(C)]
pub struct VideoSessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub flags: VideoSessionCreateFlagsKHR,
    pub p_video_profile: *const VideoProfileInfoKHR,
    pub picture_format: Format,
    pub max_coded_extent: Extent2D,
    pub reference_picture_format: Format,
    pub max_dpb_slots: u32,
    pub max_active_reference_pictures: u32,
    pub p_std_header_version: *const ExtensionProperties,
}
#[repr(C)]
pub struct VideoSessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoSessionParametersCreateFlagsKHR,
    pub video_session_parameters_template: VideoSessionParametersKHR,
    pub video_session: VideoSessionKHR,
}
#[repr(C)]
pub struct VideoSessionParametersUpdateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub update_sequence_count: u32,
}
#[repr(C)]
pub struct VideoBeginCodingInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoBeginCodingFlagsKHR,
    pub video_session: VideoSessionKHR,
    pub video_session_parameters: VideoSessionParametersKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR,
}
#[repr(C)]
pub struct VideoEndCodingInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEndCodingFlagsKHR,
}
#[repr(C)]
pub struct VideoCodingControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoCodingControlFlagsKHR,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QueryResultStatusKHR(i32);
impl QueryResultStatusKHR {
    pub const ERROR_KHR: Self = Self(-1);
    pub const NOT_READY_KHR: Self = Self(0);
    pub const COMPLETE_KHR: Self = Self(1);
    pub const INSUFFICIENT_BITSTREAM_BUFFER_RANGE_KHR: Self = Self(-1000299000);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoCodecOperationFlagsKHR: Flags {
        const DECODE_H264_KHR = VideoCodecOperationFlagBitsKHR::DECODE_H264_KHR.0;
        const DECODE_H265_KHR = VideoCodecOperationFlagBitsKHR::DECODE_H265_KHR.0;
        const DECODE_AV1_KHR = VideoCodecOperationFlagBitsKHR::DECODE_AV1_KHR.0;
        const DECODE_VP9_KHR = VideoCodecOperationFlagBitsKHR::DECODE_VP9_KHR.0;
        const ENCODE_H264_KHR = VideoCodecOperationFlagBitsKHR::ENCODE_H264_KHR.0;
        const ENCODE_H265_KHR = VideoCodecOperationFlagBitsKHR::ENCODE_H265_KHR.0;
        const ENCODE_AV1_KHR = VideoCodecOperationFlagBitsKHR::ENCODE_AV1_KHR.0;
        const NONE = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoCodecOperationFlagBitsKHR(u32);
impl VideoCodecOperationFlagBitsKHR {
    pub const DECODE_H264_KHR: Self = Self(1 << 0);
    pub const DECODE_H265_KHR: Self = Self(1 << 1);
    pub const DECODE_AV1_KHR: Self = Self(1 << 2);
    pub const DECODE_VP9_KHR: Self = Self(1 << 3);
    pub const ENCODE_H264_KHR: Self = Self(1 << 16);
    pub const ENCODE_H265_KHR: Self = Self(1 << 17);
    pub const ENCODE_AV1_KHR: Self = Self(1 << 18);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoCapabilityFlagsKHR: Flags {
        const PROTECTED_CONTENT_KHR = VideoCapabilityFlagBitsKHR::PROTECTED_CONTENT_KHR.0;
        const SEPARATE_REFERENCE_IMAGES_KHR = VideoCapabilityFlagBitsKHR::SEPARATE_REFERENCE_IMAGES_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoCapabilityFlagBitsKHR(u32);
impl VideoCapabilityFlagBitsKHR {
    pub const PROTECTED_CONTENT_KHR: Self = Self(1 << 0);
    pub const SEPARATE_REFERENCE_IMAGES_KHR: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoSessionCreateFlagsKHR: Flags {
        const PROTECTED_CONTENT_KHR = VideoSessionCreateFlagBitsKHR::PROTECTED_CONTENT_KHR.0;
        const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR = VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR.0;
        const INLINE_QUERIES_KHR = VideoSessionCreateFlagBitsKHR::INLINE_QUERIES_KHR.0;
        const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR = VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR.0;
        const ALLOW_ENCODE_EMPHASIS_MAP_KHR = VideoSessionCreateFlagBitsKHR::ALLOW_ENCODE_EMPHASIS_MAP_KHR.0;
        const INLINE_SESSION_PARAMETERS_KHR = VideoSessionCreateFlagBitsKHR::INLINE_SESSION_PARAMETERS_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionCreateFlagBitsKHR(u32);
impl VideoSessionCreateFlagBitsKHR {
    pub const PROTECTED_CONTENT_KHR: Self = Self(1 << 0);
    pub const ALLOW_ENCODE_PARAMETER_OPTIMIZATIONS_KHR: Self = Self(1 << 1);
    pub const INLINE_QUERIES_KHR: Self = Self(1 << 2);
    pub const ALLOW_ENCODE_QUANTIZATION_DELTA_MAP_KHR: Self = Self(1 << 3);
    pub const ALLOW_ENCODE_EMPHASIS_MAP_KHR: Self = Self(1 << 4);
    pub const INLINE_SESSION_PARAMETERS_KHR: Self = Self(1 << 5);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoSessionParametersCreateFlagsKHR: Flags {
        const QUANTIZATION_MAP_COMPATIBLE_KHR = VideoSessionParametersCreateFlagBitsKHR::QUANTIZATION_MAP_COMPATIBLE_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionParametersCreateFlagBitsKHR(u32);
impl VideoSessionParametersCreateFlagBitsKHR {
    pub const QUANTIZATION_MAP_COMPATIBLE_KHR: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoBeginCodingFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEndCodingFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoCodingControlFlagsKHR: Flags {
        const RESET_KHR = VideoCodingControlFlagBitsKHR::RESET_KHR.0;
        const ENCODE_RATE_CONTROL_KHR = VideoCodingControlFlagBitsKHR::ENCODE_RATE_CONTROL_KHR.0;
        const ENCODE_QUALITY_LEVEL_KHR = VideoCodingControlFlagBitsKHR::ENCODE_QUALITY_LEVEL_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoCodingControlFlagBitsKHR(u32);
impl VideoCodingControlFlagBitsKHR {
    pub const RESET_KHR: Self = Self(1 << 0);
    pub const ENCODE_RATE_CONTROL_KHR: Self = Self(1 << 1);
    pub const ENCODE_QUALITY_LEVEL_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoChromaSubsamplingFlagsKHR: Flags {
        const MONOCHROME_KHR = VideoChromaSubsamplingFlagBitsKHR::MONOCHROME_KHR.0;
        const _420_KHR = VideoChromaSubsamplingFlagBitsKHR::_420_KHR.0;
        const _422_KHR = VideoChromaSubsamplingFlagBitsKHR::_422_KHR.0;
        const _444_KHR = VideoChromaSubsamplingFlagBitsKHR::_444_KHR.0;
        const INVALID = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoChromaSubsamplingFlagBitsKHR(u32);
impl VideoChromaSubsamplingFlagBitsKHR {
    pub const MONOCHROME_KHR: Self = Self(1 << 0);
    pub const _420_KHR: Self = Self(1 << 1);
    pub const _422_KHR: Self = Self(1 << 2);
    pub const _444_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoComponentBitDepthFlagsKHR: Flags {
        const _8_KHR = VideoComponentBitDepthFlagBitsKHR::_8_KHR.0;
        const _10_KHR = VideoComponentBitDepthFlagBitsKHR::_10_KHR.0;
        const _12_KHR = VideoComponentBitDepthFlagBitsKHR::_12_KHR.0;
        const INVALID = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoComponentBitDepthFlagBitsKHR(u32);
impl VideoComponentBitDepthFlagBitsKHR {
    pub const _8_KHR: Self = Self(1 << 0);
    pub const _10_KHR: Self = Self(1 << 2);
    pub const _12_KHR: Self = Self(1 << 4);
}
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_profile: *const VideoProfileInfoKHR,
    p_capabilities: *mut VideoCapabilitiesKHR,
) -> Result;
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
    p_video_format_property_count: *mut u32,
    p_video_format_properties: *mut VideoFormatPropertiesKHR,
) -> Result;
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_video_session: *mut VideoSessionKHR,
) -> Result;
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionParametersCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_video_session_parameters: *mut VideoSessionParametersKHR,
) -> Result;
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_update_info: *const VideoSessionParametersUpdateInfoKHR,
) -> Result;
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_allocator: *const AllocationCallbacks,
);
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_memory_requirements_count: *mut u32,
    p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR,
) -> Result;
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    bind_session_memory_info_count: u32,
    p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR,
) -> Result;
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const VideoBeginCodingInfoKHR,
);
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_coding_control_info: *const VideoCodingControlInfoKHR,
);
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_end_coding_info: *const VideoEndCodingInfoKHR,
);
