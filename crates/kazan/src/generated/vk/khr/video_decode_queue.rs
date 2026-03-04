#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: VideoDecodeCapabilityFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_CAPABILITIES_KHR;
    }
    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoDecodeCapabilitiesKHR<'a> {}
    impl Default for VideoDecodeCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoDecodeCapabilitiesKHR<'a> {
        pub fn flags(mut self, flags: VideoDecodeCapabilityFlagsKHR) -> Self {
            self.flags = flags;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeUsageInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_USAGE_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoDecodeUsageInfoKHR<'a> {}
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoDecodeUsageInfoKHR<'a> {}
    impl Default for VideoDecodeUsageInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                video_usage_hints: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoDecodeUsageInfoKHR<'a> {
        pub fn video_usage_hints(mut self, video_usage_hints: VideoDecodeUsageFlagsKHR) -> Self {
            self.video_usage_hints = video_usage_hints;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_INFO_KHR;
    }
    impl Default for VideoDecodeInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
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
    impl<'a> VideoDecodeInfoKHR<'a> {
        pub fn flags(mut self, flags: VideoDecodeFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn src_buffer(mut self, src_buffer: Buffer) -> Self {
            self.src_buffer = src_buffer;
            self
        }
        pub fn src_buffer_offset(mut self, src_buffer_offset: DeviceSize) -> Self {
            self.src_buffer_offset = src_buffer_offset;
            self
        }
        pub fn src_buffer_range(mut self, src_buffer_range: DeviceSize) -> Self {
            self.src_buffer_range = src_buffer_range;
            self
        }
        pub fn dst_picture_resource(
            mut self,
            dst_picture_resource: VideoPictureResourceInfoKHR<'a>,
        ) -> Self {
            self.dst_picture_resource = dst_picture_resource;
            self
        }
        pub fn setup_reference_slot(
            mut self,
            setup_reference_slot: &'a VideoReferenceSlotInfoKHR<'a>,
        ) -> Self {
            self.p_setup_reference_slot = setup_reference_slot;
            self
        }
        pub fn reference_slots(
            mut self,
            reference_slots: &'a [VideoReferenceSlotInfoKHR<'a>],
        ) -> Self {
            self.reference_slot_count = reference_slots.len().try_into().unwrap();
            self.p_reference_slots = reference_slots.as_ptr();
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoDecodeUsageFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoDecodeUsageFlagsKHR, Flags);
    impl VideoDecodeUsageFlagsKHR {
        pub const TRANSCODING_KHR: Self = Self(VideoDecodeUsageFlagBitsKHR::TRANSCODING_KHR.0);
        pub const OFFLINE_KHR: Self = Self(VideoDecodeUsageFlagBitsKHR::OFFLINE_KHR.0);
        pub const STREAMING_KHR: Self = Self(VideoDecodeUsageFlagBitsKHR::STREAMING_KHR.0);
        pub const DEFAULT: Self = Self(0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoDecodeUsageFlagBitsKHR(u32);
    impl VideoDecodeUsageFlagBitsKHR {
        pub const TRANSCODING_KHR: Self = Self(1 << 0);
        pub const OFFLINE_KHR: Self = Self(1 << 1);
        pub const STREAMING_KHR: Self = Self(1 << 2);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoDecodeCapabilityFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoDecodeCapabilityFlagsKHR, Flags);
    impl VideoDecodeCapabilityFlagsKHR {
        pub const DPB_AND_OUTPUT_COINCIDE_KHR: Self =
            Self(VideoDecodeCapabilityFlagBitsKHR::DPB_AND_OUTPUT_COINCIDE_KHR.0);
        pub const DPB_AND_OUTPUT_DISTINCT_KHR: Self =
            Self(VideoDecodeCapabilityFlagBitsKHR::DPB_AND_OUTPUT_DISTINCT_KHR.0);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoDecodeCapabilityFlagBitsKHR(u32);
    impl VideoDecodeCapabilityFlagBitsKHR {
        pub const DPB_AND_OUTPUT_COINCIDE_KHR: Self = Self(1 << 0);
        pub const DPB_AND_OUTPUT_DISTINCT_KHR: Self = Self(1 << 1);
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoDecodeFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoDecodeFlagsKHR, Flags);
    impl VideoDecodeFlagsKHR {}
    pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_decode_info: *const VideoDecodeInfoKHR<'_>,
    );
}
pub struct DeviceFn {
    cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_decode_video_khr: transmute(
                    load(c"vkCmdDecodeVideoKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_decode_video_khr(
        &self,
        command_buffer: CommandBuffer,
        decode_info: &VideoDecodeInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_decode_video_khr)(command_buffer, decode_info) }
    }
}
