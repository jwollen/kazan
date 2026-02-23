#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct VideoDecodeCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoDecodeCapabilityFlagsKHR,
}
#[repr(C)]
pub struct VideoDecodeUsageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_usage_hints: VideoDecodeUsageFlagsKHR,
}
#[repr(C)]
pub struct VideoDecodeInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoDecodeFlagsKHR,
    pub src_buffer: Buffer,
    pub src_buffer_offset: DeviceSize,
    pub src_buffer_range: DeviceSize,
    pub dst_picture_resource: VideoPictureResourceInfoKHR,
    pub p_setup_reference_slot: *const VideoReferenceSlotInfoKHR,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoDecodeUsageFlagsKHR: Flags {
        const TRANSCODING_KHR = VideoDecodeUsageFlagBitsKHR::TRANSCODING_KHR.0;
        const OFFLINE_KHR = VideoDecodeUsageFlagBitsKHR::OFFLINE_KHR.0;
        const STREAMING_KHR = VideoDecodeUsageFlagBitsKHR::STREAMING_KHR.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeUsageFlagBitsKHR(u32);
impl VideoDecodeUsageFlagBitsKHR {
    pub const TRANSCODING_KHR: Self = Self(1 << 0);
    pub const OFFLINE_KHR: Self = Self(1 << 1);
    pub const STREAMING_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoDecodeCapabilityFlagsKHR: Flags {
        const DPB_AND_OUTPUT_COINCIDE_KHR = VideoDecodeCapabilityFlagBitsKHR::DPB_AND_OUTPUT_COINCIDE_KHR.0;
        const DPB_AND_OUTPUT_DISTINCT_KHR = VideoDecodeCapabilityFlagBitsKHR::DPB_AND_OUTPUT_DISTINCT_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeCapabilityFlagBitsKHR(u32);
impl VideoDecodeCapabilityFlagBitsKHR {
    pub const DPB_AND_OUTPUT_COINCIDE_KHR: Self = Self(1 << 0);
    pub const DPB_AND_OUTPUT_DISTINCT_KHR: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoDecodeFlagsKHR: Flags {
    }
}
pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_decode_info: *const VideoDecodeInfoKHR,
);
