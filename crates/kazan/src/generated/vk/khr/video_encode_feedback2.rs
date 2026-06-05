//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_encode_feedback2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_encode_feedback2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_encode_feedback2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVideoEncodeFeedback2FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("video_encode_feedback2", &self.video_encode_feedback2)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_FEEDBACK_2_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'_> {}

    impl Default for PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                video_encode_feedback2: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'a> {
        #[inline]
        pub fn video_encode_feedback2(mut self, video_encode_feedback2: bool) -> Self {
            self.video_encode_feedback2 = video_encode_feedback2.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeFeedback2CapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeFeedback2CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_per_partition_feedback_entries: u32,
        pub supported_per_partition_encode_feedback_flags: VideoEncodePerPartitionFeedbackFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeFeedback2CapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeFeedback2CapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_per_partition_feedback_entries",
                    &self.max_per_partition_feedback_entries,
                )
                .field(
                    "supported_per_partition_encode_feedback_flags",
                    &self.supported_per_partition_encode_feedback_flags,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeFeedback2CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_FEEDBACK_2_CAPABILITIES_KHR;
    }

    unsafe impl Extends<VideoCapabilitiesKHR<'_>> for VideoEncodeFeedback2CapabilitiesKHR<'_> {}

    impl Default for VideoEncodeFeedback2CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_per_partition_feedback_entries: Default::default(),
                supported_per_partition_encode_feedback_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeFeedback2CapabilitiesKHR<'a> {
        #[inline]
        pub fn max_per_partition_feedback_entries(
            mut self,
            max_per_partition_feedback_entries: u32,
        ) -> Self {
            self.max_per_partition_feedback_entries = max_per_partition_feedback_entries;
            self
        }

        #[inline]
        pub fn supported_per_partition_encode_feedback_flags(
            mut self,
            supported_per_partition_encode_feedback_flags: VideoEncodePerPartitionFeedbackFlagsKHR,
        ) -> Self {
            self.supported_per_partition_encode_feedback_flags =
                supported_per_partition_encode_feedback_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_per_partition_feedback_entries: u32,
        pub per_partition_encode_feedback_flags: VideoEncodePerPartitionFeedbackFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_per_partition_feedback_entries",
                    &self.max_per_partition_feedback_entries,
                )
                .field(
                    "per_partition_encode_feedback_flags",
                    &self.per_partition_encode_feedback_flags,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUERY_POOL_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_CREATE_INFO_KHR;
    }

    unsafe impl Extends<QueryPoolCreateInfo<'_>>
        for QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'_>
    {
    }

    impl Default for QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                max_per_partition_feedback_entries: Default::default(),
                per_partition_encode_feedback_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'a> {
        #[inline]
        pub fn max_per_partition_feedback_entries(
            mut self,
            max_per_partition_feedback_entries: u32,
        ) -> Self {
            self.max_per_partition_feedback_entries = max_per_partition_feedback_entries;
            self
        }

        #[inline]
        pub fn per_partition_encode_feedback_flags(
            mut self,
            per_partition_encode_feedback_flags: VideoEncodePerPartitionFeedbackFlagsKHR,
        ) -> Self {
            self.per_partition_encode_feedback_flags = per_partition_encode_feedback_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodePerPartitionFeedbackFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodePerPartitionFeedbackFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoEncodePerPartitionFeedbackFlagsKHR,
        Flags,
        VideoEncodePerPartitionFeedbackFlagBitsKHR
    );

    impl fmt::Debug for VideoEncodePerPartitionFeedbackFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodePerPartitionFeedbackFlagBitsKHR::STATUS_KHR.0,
                    "STATUS_KHR",
                ),
                (
                    VideoEncodePerPartitionFeedbackFlagBitsKHR::BITSTREAM_BUFFER_OFFSET_KHR.0,
                    "BITSTREAM_BUFFER_OFFSET_KHR",
                ),
                (
                    VideoEncodePerPartitionFeedbackFlagBitsKHR::BITSTREAM_BYTES_WRITTEN_KHR.0,
                    "BITSTREAM_BYTES_WRITTEN_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodePerPartitionFeedbackFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodePerPartitionFeedbackFlagBitsKHR(u32);

    impl VideoEncodePerPartitionFeedbackFlagBitsKHR {
        pub const STATUS_KHR: Self = Self(1 << 0);
        pub const BITSTREAM_BUFFER_OFFSET_KHR: Self = Self(1 << 1);
        pub const BITSTREAM_BYTES_WRITTEN_KHR: Self = Self(1 << 2);
    }

    impl fmt::Debug for VideoEncodePerPartitionFeedbackFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::STATUS_KHR => Some("STATUS_KHR"),
                Self::BITSTREAM_BUFFER_OFFSET_KHR => Some("BITSTREAM_BUFFER_OFFSET_KHR"),
                Self::BITSTREAM_BYTES_WRITTEN_KHR => Some("BITSTREAM_BYTES_WRITTEN_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR =
        PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'static>;
    pub type VkVideoEncodeFeedback2CapabilitiesKHR = VideoEncodeFeedback2CapabilitiesKHR<'static>;
    pub type VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR =
        QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'static>;
    pub type VkVideoEncodePerPartitionFeedbackFlagsKHR = VideoEncodePerPartitionFeedbackFlagsKHR;
    pub type VkVideoEncodePerPartitionFeedbackFlagBitsKHR =
        VideoEncodePerPartitionFeedbackFlagBitsKHR;
    impl PhysicalDeviceVideoEncodeFeedback2FeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeFeedback2CapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeFeedback2CapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl QueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
