//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_decode_queue.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_decode_queue";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: VideoDecodeCapabilityFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_CAPABILITIES_KHR;
    }

    unsafe impl Extends<VideoCapabilitiesKHR<'_>> for VideoDecodeCapabilitiesKHR<'_> {}

    impl Default for VideoDecodeCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeCapabilitiesKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoDecodeCapabilityFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeUsageInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeUsageInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub video_usage_hints: VideoDecodeUsageFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeUsageInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeUsageInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("video_usage_hints", &self.video_usage_hints)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeUsageInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_USAGE_INFO_KHR;
    }

    unsafe impl Extends<VideoProfileInfoKHR<'_>> for VideoDecodeUsageInfoKHR<'_> {}
    unsafe impl Extends<QueryPoolCreateInfo<'_>> for VideoDecodeUsageInfoKHR<'_> {}

    impl Default for VideoDecodeUsageInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                video_usage_hints: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeUsageInfoKHR<'a> {
        #[inline]
        pub fn video_usage_hints(mut self, video_usage_hints: VideoDecodeUsageFlagsKHR) -> Self {
            self.video_usage_hints = video_usage_hints;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
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

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("src_buffer", &self.src_buffer)
                .field("src_buffer_offset", &self.src_buffer_offset)
                .field("src_buffer_range", &self.src_buffer_range)
                .field("dst_picture_resource", &self.dst_picture_resource)
                .field("p_setup_reference_slot", &self.p_setup_reference_slot)
                .field("reference_slot_count", &self.reference_slot_count)
                .field("p_reference_slots", &self.p_reference_slots)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_INFO_KHR;
    }

    impl Default for VideoDecodeInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                src_buffer: Default::default(),
                src_buffer_offset: Default::default(),
                src_buffer_range: Default::default(),
                dst_picture_resource: Default::default(),
                p_setup_reference_slot: ptr::null(),
                reference_slot_count: Default::default(),
                p_reference_slots: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoDecodeFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn src_buffer(mut self, src_buffer: Buffer) -> Self {
            self.src_buffer = src_buffer;
            self
        }

        #[inline]
        pub fn src_buffer_offset(mut self, src_buffer_offset: DeviceSize) -> Self {
            self.src_buffer_offset = src_buffer_offset;
            self
        }

        #[inline]
        pub fn src_buffer_range(mut self, src_buffer_range: DeviceSize) -> Self {
            self.src_buffer_range = src_buffer_range;
            self
        }

        #[inline]
        pub fn dst_picture_resource(
            mut self,
            dst_picture_resource: VideoPictureResourceInfoKHR<'a>,
        ) -> Self {
            self.dst_picture_resource = dst_picture_resource;
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
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeUsageFlagsKHR.html>
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

    impl fmt::Debug for VideoDecodeUsageFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoDecodeUsageFlagsKHR::TRANSCODING_KHR.0,
                    "TRANSCODING_KHR",
                ),
                (VideoDecodeUsageFlagsKHR::OFFLINE_KHR.0, "OFFLINE_KHR"),
                (VideoDecodeUsageFlagsKHR::STREAMING_KHR.0, "STREAMING_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeUsageFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoDecodeUsageFlagBitsKHR(u32);

    impl VideoDecodeUsageFlagBitsKHR {
        pub const TRANSCODING_KHR: Self = Self(1 << 0);
        pub const OFFLINE_KHR: Self = Self(1 << 1);
        pub const STREAMING_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for VideoDecodeUsageFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TRANSCODING_KHR => Some("TRANSCODING_KHR"),
                Self::OFFLINE_KHR => Some("OFFLINE_KHR"),
                Self::STREAMING_KHR => Some("STREAMING_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeCapabilityFlagsKHR.html>
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

    impl fmt::Debug for VideoDecodeCapabilityFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoDecodeCapabilityFlagsKHR::DPB_AND_OUTPUT_COINCIDE_KHR.0,
                    "DPB_AND_OUTPUT_COINCIDE_KHR",
                ),
                (
                    VideoDecodeCapabilityFlagsKHR::DPB_AND_OUTPUT_DISTINCT_KHR.0,
                    "DPB_AND_OUTPUT_DISTINCT_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeCapabilityFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoDecodeCapabilityFlagBitsKHR(u32);

    impl VideoDecodeCapabilityFlagBitsKHR {
        pub const DPB_AND_OUTPUT_COINCIDE_KHR: Self = Self(1 << 0);
        pub const DPB_AND_OUTPUT_DISTINCT_KHR: Self = Self(1 << 1);
    }

    impl fmt::Debug for VideoDecodeCapabilityFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DPB_AND_OUTPUT_COINCIDE_KHR => Some("DPB_AND_OUTPUT_COINCIDE_KHR"),
                Self::DPB_AND_OUTPUT_DISTINCT_KHR => Some("DPB_AND_OUTPUT_DISTINCT_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoDecodeFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoDecodeFlagsKHR, Flags);

    impl fmt::Debug for VideoDecodeFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            debug_flags(f, &[], self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDecodeVideoKHR.html>
    pub type PFN_vkCmdDecodeVideoKHR = unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        p_decode_info: *const VideoDecodeInfoKHR<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkVideoDecodeCapabilitiesKHR = VideoDecodeCapabilitiesKHR<'static>;
    pub type VkVideoDecodeUsageInfoKHR = VideoDecodeUsageInfoKHR<'static>;
    pub type VkVideoDecodeInfoKHR = VideoDecodeInfoKHR<'static>;
    pub type VkVideoDecodeUsageFlagsKHR = VideoDecodeUsageFlagsKHR;
    pub type VkVideoDecodeUsageFlagBitsKHR = VideoDecodeUsageFlagBitsKHR;
    pub type VkVideoDecodeCapabilityFlagsKHR = VideoDecodeCapabilityFlagsKHR;
    pub type VkVideoDecodeCapabilityFlagBitsKHR = VideoDecodeCapabilityFlagBitsKHR;
    pub type VkVideoDecodeFlagsKHR = VideoDecodeFlagsKHR;
    impl VideoDecodeCapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeCapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeUsageInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeUsageInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_decode_video_khr: PFN_vkCmdDecodeVideoKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdDecodeVideoKHR.html>
    #[inline]
    pub unsafe fn cmd_decode_video_khr(
        &self,
        command_buffer: CommandBuffer,
        decode_info: &VideoDecodeInfoKHR<'_>,
    ) {
        unsafe { (self.cmd_decode_video_khr)(command_buffer, decode_info) }
    }
}
