#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeCapabilitiesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoDecodeCapabilityFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeCapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeUsageInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub video_usage_hints: VideoDecodeUsageFlagsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeUsageInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_USAGE_INFO_KHR,
            p_next: core::ptr::null(),
            video_usage_hints: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoDecodeInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoDecodeFlagsKHR,
    pub src_buffer: Buffer,
    pub src_buffer_offset: DeviceSize,
    pub src_buffer_range: DeviceSize,
    pub dst_picture_resource: VideoPictureResourceInfoKHR<'a>,
    pub p_setup_reference_slot: *const VideoReferenceSlotInfoKHR<'a>,
    pub reference_slot_count: u32,
    pub p_reference_slots: *const VideoReferenceSlotInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoDecodeInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_DECODE_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            src_buffer: Default::default(),
            src_buffer_offset: Default::default(),
            src_buffer_range: Default::default(),
            dst_picture_resource: Default::default(),
            p_setup_reference_slot: core::ptr::null(),
            reference_slot_count: Default::default(),
            p_reference_slots: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoDecodeUsageFlagsKHR: Flags {
        const TRANSCODING_KHR = VideoDecodeUsageFlagBitsKHR::TRANSCODING_KHR.0;
        const OFFLINE_KHR = VideoDecodeUsageFlagBitsKHR::OFFLINE_KHR.0;
        const STREAMING_KHR = VideoDecodeUsageFlagBitsKHR::STREAMING_KHR.0;
        const DEFAULT = 0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeUsageFlagBitsKHR(u32);
impl VideoDecodeUsageFlagBitsKHR {
    pub const TRANSCODING_KHR: Self = Self(1 << 0);
    pub const OFFLINE_KHR: Self = Self(1 << 1);
    pub const STREAMING_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoDecodeCapabilityFlagsKHR: Flags {
        const DPB_AND_OUTPUT_COINCIDE_KHR = VideoDecodeCapabilityFlagBitsKHR::DPB_AND_OUTPUT_COINCIDE_KHR.0;
        const DPB_AND_OUTPUT_DISTINCT_KHR = VideoDecodeCapabilityFlagBitsKHR::DPB_AND_OUTPUT_DISTINCT_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoDecodeCapabilityFlagBitsKHR(u32);
impl VideoDecodeCapabilityFlagBitsKHR {
    pub const DPB_AND_OUTPUT_COINCIDE_KHR: Self = Self(1 << 0);
    pub const DPB_AND_OUTPUT_DISTINCT_KHR: Self = Self(1 << 1);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoDecodeFlagsKHR: Flags {
    }
}
pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_decode_info: *const VideoDecodeInfoKHR<'_>,
);
