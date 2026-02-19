#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeCapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoDecodeCapabilityFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeUsageInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_usage_hints: VideoDecodeUsageFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
        const TRANSCODING_KHR = 1 << 0;
        const OFFLINE_KHR = 1 << 1;
        const STREAMING_KHR = 1 << 2;
        const DEFAULT = 0;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoDecodeCapabilityFlagsKHR: Flags {
        const DPB_AND_OUTPUT_COINCIDE_KHR = 1 << 0;
        const DPB_AND_OUTPUT_DISTINCT_KHR = 1 << 1;
    }
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
