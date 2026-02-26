#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VideoSessionKHR(u64);
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VideoSessionParametersKHR(u64);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyVideoPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_codec_operations: VideoCodecOperationFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueueFamilyVideoPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUEUE_FAMILY_VIDEO_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            video_codec_operations: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QueueFamilyQueryResultStatusPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub query_result_status_support: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for QueueFamilyQueryResultStatusPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::QUEUE_FAMILY_QUERY_RESULT_STATUS_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            query_result_status_support: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoProfileListInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub profile_count: u32,
    pub p_profiles: *const VideoProfileInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoProfileListInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_PROFILE_LIST_INFO_KHR,
            p_next: core::ptr::null(),
            profile_count: Default::default(),
            p_profiles: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoFormatInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image_usage: ImageUsageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVideoFormatInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VIDEO_FORMAT_INFO_KHR,
            p_next: core::ptr::null(),
            image_usage: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoFormatPropertiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub format: Format,
    pub component_mapping: ComponentMapping,
    pub image_create_flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub image_tiling: ImageTiling,
    pub image_usage_flags: ImageUsageFlags,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoFormatPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_FORMAT_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            format: Default::default(),
            component_mapping: Default::default(),
            image_create_flags: Default::default(),
            image_type: Default::default(),
            image_tiling: Default::default(),
            image_usage_flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_codec_operation: VideoCodecOperationFlagBitsKHR,
    pub chroma_subsampling: VideoChromaSubsamplingFlagsKHR,
    pub luma_bit_depth: VideoComponentBitDepthFlagsKHR,
    pub chroma_bit_depth: VideoComponentBitDepthFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoProfileInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_PROFILE_INFO_KHR,
            p_next: core::ptr::null(),
            video_codec_operation: Default::default(),
            chroma_subsampling: Default::default(),
            luma_bit_depth: Default::default(),
            chroma_bit_depth: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoCapabilitiesKHR<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            flags: Default::default(),
            min_bitstream_buffer_offset_alignment: Default::default(),
            min_bitstream_buffer_size_alignment: Default::default(),
            picture_access_granularity: Default::default(),
            min_coded_extent: Default::default(),
            max_coded_extent: Default::default(),
            max_dpb_slots: Default::default(),
            max_active_reference_pictures: Default::default(),
            std_header_version: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoSessionMemoryRequirementsKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub memory_bind_index: u32,
    pub memory_requirements: MemoryRequirements,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoSessionMemoryRequirementsKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_SESSION_MEMORY_REQUIREMENTS_KHR,
            p_next: core::ptr::null_mut(),
            memory_bind_index: Default::default(),
            memory_requirements: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BindVideoSessionMemoryInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub memory_bind_index: u32,
    pub memory: DeviceMemory,
    pub memory_offset: DeviceSize,
    pub memory_size: DeviceSize,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for BindVideoSessionMemoryInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::BIND_VIDEO_SESSION_MEMORY_INFO_KHR,
            p_next: core::ptr::null(),
            memory_bind_index: Default::default(),
            memory: Default::default(),
            memory_offset: Default::default(),
            memory_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoPictureResourceInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub coded_offset: Offset2D,
    pub coded_extent: Extent2D,
    pub base_array_layer: u32,
    pub image_view_binding: ImageView,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoPictureResourceInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_PICTURE_RESOURCE_INFO_KHR,
            p_next: core::ptr::null(),
            coded_offset: Default::default(),
            coded_extent: Default::default(),
            base_array_layer: Default::default(),
            image_view_binding: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoReferenceSlotInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub slot_index: i32,
    pub p_picture_resource: *const VideoPictureResourceInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoReferenceSlotInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_REFERENCE_SLOT_INFO_KHR,
            p_next: core::ptr::null(),
            slot_index: Default::default(),
            p_picture_resource: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoSessionCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub queue_family_index: u32,
    pub flags: VideoSessionCreateFlagsKHR,
    pub p_video_profile: *const VideoProfileInfoKHR<'a>,
    pub picture_format: Format,
    pub max_coded_extent: Extent2D,
    pub reference_picture_format: Format,
    pub max_dpb_slots: u32,
    pub max_active_reference_pictures: u32,
    pub p_std_header_version: *const ExtensionProperties,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoSessionCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_SESSION_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            queue_family_index: Default::default(),
            flags: Default::default(),
            p_video_profile: core::ptr::null(),
            picture_format: Default::default(),
            max_coded_extent: Default::default(),
            reference_picture_format: Default::default(),
            max_dpb_slots: Default::default(),
            max_active_reference_pictures: Default::default(),
            p_std_header_version: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoSessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoSessionParametersCreateFlagsKHR,
    pub video_session_parameters_template: VideoSessionParametersKHR,
    pub video_session: VideoSessionKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoSessionParametersCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            video_session_parameters_template: Default::default(),
            video_session: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoSessionParametersUpdateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub update_sequence_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoSessionParametersUpdateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_SESSION_PARAMETERS_UPDATE_INFO_KHR,
            p_next: core::ptr::null(),
            update_sequence_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoBeginCodingInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoBeginCodingFlagsKHR,
    pub video_session: VideoSessionKHR,
    pub video_session_parameters: VideoSessionParametersKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoBeginCodingInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_BEGIN_CODING_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            video_session: Default::default(),
            video_session_parameters: Default::default(),
            reference_slot_count: Default::default(),
            p_reference_slots: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEndCodingInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEndCodingFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEndCodingInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_END_CODING_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoCodingControlInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoCodingControlFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoCodingControlInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_CODING_CONTROL_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoCapabilityFlagsKHR: Flags {
        const PROTECTED_CONTENT_KHR = VideoCapabilityFlagBitsKHR::PROTECTED_CONTENT_KHR.0;
        const SEPARATE_REFERENCE_IMAGES_KHR = VideoCapabilityFlagBitsKHR::SEPARATE_REFERENCE_IMAGES_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoCapabilityFlagBitsKHR(u32);
impl VideoCapabilityFlagBitsKHR {
    pub const PROTECTED_CONTENT_KHR: Self = Self(1 << 0);
    pub const SEPARATE_REFERENCE_IMAGES_KHR: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoSessionParametersCreateFlagsKHR: Flags {
        const QUANTIZATION_MAP_COMPATIBLE_KHR = VideoSessionParametersCreateFlagBitsKHR::QUANTIZATION_MAP_COMPATIBLE_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoSessionParametersCreateFlagBitsKHR(u32);
impl VideoSessionParametersCreateFlagBitsKHR {
    pub const QUANTIZATION_MAP_COMPATIBLE_KHR: Self = Self(1 << 0);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoBeginCodingFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEndCodingFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoCodingControlFlagsKHR: Flags {
        const RESET_KHR = VideoCodingControlFlagBitsKHR::RESET_KHR.0;
        const ENCODE_RATE_CONTROL_KHR = VideoCodingControlFlagBitsKHR::ENCODE_RATE_CONTROL_KHR.0;
        const ENCODE_QUALITY_LEVEL_KHR = VideoCodingControlFlagBitsKHR::ENCODE_QUALITY_LEVEL_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoCodingControlFlagBitsKHR(u32);
impl VideoCodingControlFlagBitsKHR {
    pub const RESET_KHR: Self = Self(1 << 0);
    pub const ENCODE_RATE_CONTROL_KHR: Self = Self(1 << 1);
    pub const ENCODE_QUALITY_LEVEL_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoChromaSubsamplingFlagsKHR: Flags {
        const MONOCHROME_KHR = VideoChromaSubsamplingFlagBitsKHR::MONOCHROME_KHR.0;
        const _420_KHR = VideoChromaSubsamplingFlagBitsKHR::_420_KHR.0;
        const _422_KHR = VideoChromaSubsamplingFlagBitsKHR::_422_KHR.0;
        const _444_KHR = VideoChromaSubsamplingFlagBitsKHR::_444_KHR.0;
        const INVALID = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoChromaSubsamplingFlagBitsKHR(u32);
impl VideoChromaSubsamplingFlagBitsKHR {
    pub const MONOCHROME_KHR: Self = Self(1 << 0);
    pub const _420_KHR: Self = Self(1 << 1);
    pub const _422_KHR: Self = Self(1 << 2);
    pub const _444_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoComponentBitDepthFlagsKHR: Flags {
        const _8_KHR = VideoComponentBitDepthFlagBitsKHR::_8_KHR.0;
        const _10_KHR = VideoComponentBitDepthFlagBitsKHR::_10_KHR.0;
        const _12_KHR = VideoComponentBitDepthFlagBitsKHR::_12_KHR.0;
        const INVALID = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoComponentBitDepthFlagBitsKHR(u32);
impl VideoComponentBitDepthFlagBitsKHR {
    pub const _8_KHR: Self = Self(1 << 0);
    pub const _10_KHR: Self = Self(1 << 2);
    pub const _12_KHR: Self = Self(1 << 4);
}
pub type PFN_vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_profile: *const VideoProfileInfoKHR<'_>,
    p_capabilities: *mut VideoCapabilitiesKHR<'_>,
) -> Result;
pub type PFN_vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_video_format_info: *const PhysicalDeviceVideoFormatInfoKHR<'_>,
    p_video_format_property_count: *mut u32,
    p_video_format_properties: *mut VideoFormatPropertiesKHR<'_>,
) -> Result;
pub type PFN_vkCreateVideoSessionKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_video_session: *mut VideoSessionKHR,
) -> Result;
pub type PFN_vkDestroyVideoSessionKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const VideoSessionParametersCreateInfoKHR<'_>,
    p_allocator: *const AllocationCallbacks<'_>,
    p_video_session_parameters: *mut VideoSessionParametersKHR,
) -> Result;
pub type PFN_vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_update_info: *const VideoSessionParametersUpdateInfoKHR<'_>,
) -> Result;
pub type PFN_vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
    device: Device,
    video_session_parameters: VideoSessionParametersKHR,
    p_allocator: *const AllocationCallbacks<'_>,
);
pub type PFN_vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    p_memory_requirements_count: *mut u32,
    p_memory_requirements: *mut VideoSessionMemoryRequirementsKHR<'_>,
) -> Result;
pub type PFN_vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
    device: Device,
    video_session: VideoSessionKHR,
    bind_session_memory_info_count: u32,
    p_bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR<'_>,
) -> Result;
pub type PFN_vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_begin_info: *const VideoBeginCodingInfoKHR<'_>,
);
pub type PFN_vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_coding_control_info: *const VideoCodingControlInfoKHR<'_>,
);
pub type PFN_vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_end_coding_info: *const VideoEndCodingInfoKHR<'_>,
);
