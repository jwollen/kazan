#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_encode_intra_refresh";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeIntraRefreshCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeIntraRefreshCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub intra_refresh_modes: VideoEncodeIntraRefreshModeFlagsKHR,
        pub max_intra_refresh_cycle_duration: u32,
        pub max_intra_refresh_active_reference_pictures: u32,
        pub partition_independent_intra_refresh_regions: Bool32,
        pub non_rectangular_intra_refresh_regions: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for VideoEncodeIntraRefreshCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeIntraRefreshCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("intra_refresh_modes", &self.intra_refresh_modes)
                .field(
                    "max_intra_refresh_cycle_duration",
                    &self.max_intra_refresh_cycle_duration,
                )
                .field(
                    "max_intra_refresh_active_reference_pictures",
                    &self.max_intra_refresh_active_reference_pictures,
                )
                .field(
                    "partition_independent_intra_refresh_regions",
                    &self.partition_independent_intra_refresh_regions,
                )
                .field(
                    "non_rectangular_intra_refresh_regions",
                    &self.non_rectangular_intra_refresh_regions,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeIntraRefreshCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_INTRA_REFRESH_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoEncodeIntraRefreshCapabilitiesKHR<'a> {}

    impl Default for VideoEncodeIntraRefreshCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                intra_refresh_modes: Default::default(),
                max_intra_refresh_cycle_duration: Default::default(),
                max_intra_refresh_active_reference_pictures: Default::default(),
                partition_independent_intra_refresh_regions: Default::default(),
                non_rectangular_intra_refresh_regions: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeIntraRefreshCapabilitiesKHR<'a> {
        pub fn intra_refresh_modes(
            mut self,
            intra_refresh_modes: VideoEncodeIntraRefreshModeFlagsKHR,
        ) -> Self {
            self.intra_refresh_modes = intra_refresh_modes;
            self
        }

        pub fn max_intra_refresh_cycle_duration(
            mut self,
            max_intra_refresh_cycle_duration: u32,
        ) -> Self {
            self.max_intra_refresh_cycle_duration = max_intra_refresh_cycle_duration;
            self
        }

        pub fn max_intra_refresh_active_reference_pictures(
            mut self,
            max_intra_refresh_active_reference_pictures: u32,
        ) -> Self {
            self.max_intra_refresh_active_reference_pictures =
                max_intra_refresh_active_reference_pictures;
            self
        }

        pub fn partition_independent_intra_refresh_regions(
            mut self,
            partition_independent_intra_refresh_regions: bool,
        ) -> Self {
            self.partition_independent_intra_refresh_regions =
                partition_independent_intra_refresh_regions.into();
            self
        }

        pub fn non_rectangular_intra_refresh_regions(
            mut self,
            non_rectangular_intra_refresh_regions: bool,
        ) -> Self {
            self.non_rectangular_intra_refresh_regions =
                non_rectangular_intra_refresh_regions.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeSessionIntraRefreshCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeSessionIntraRefreshCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub intra_refresh_mode: VideoEncodeIntraRefreshModeFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for VideoEncodeSessionIntraRefreshCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeSessionIntraRefreshCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("intra_refresh_mode", &self.intra_refresh_mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeSessionIntraRefreshCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_SESSION_INTRA_REFRESH_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionCreateInfoKHR<'a>>
        for VideoEncodeSessionIntraRefreshCreateInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeSessionIntraRefreshCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                intra_refresh_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeSessionIntraRefreshCreateInfoKHR<'a> {
        pub fn intra_refresh_mode(
            mut self,
            intra_refresh_mode: VideoEncodeIntraRefreshModeFlagBitsKHR,
        ) -> Self {
            self.intra_refresh_mode = intra_refresh_mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeIntraRefreshInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeIntraRefreshInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub intra_refresh_cycle_duration: u32,
        pub intra_refresh_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for VideoEncodeIntraRefreshInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeIntraRefreshInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "intra_refresh_cycle_duration",
                    &self.intra_refresh_cycle_duration,
                )
                .field("intra_refresh_index", &self.intra_refresh_index)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeIntraRefreshInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_INTRA_REFRESH_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeInfoKHR<'a>> for VideoEncodeIntraRefreshInfoKHR<'a> {}

    impl Default for VideoEncodeIntraRefreshInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                intra_refresh_cycle_duration: Default::default(),
                intra_refresh_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeIntraRefreshInfoKHR<'a> {
        pub fn intra_refresh_cycle_duration(mut self, intra_refresh_cycle_duration: u32) -> Self {
            self.intra_refresh_cycle_duration = intra_refresh_cycle_duration;
            self
        }

        pub fn intra_refresh_index(mut self, intra_refresh_index: u32) -> Self {
            self.intra_refresh_index = intra_refresh_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoReferenceIntraRefreshInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoReferenceIntraRefreshInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dirty_intra_refresh_regions: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for VideoReferenceIntraRefreshInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoReferenceIntraRefreshInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "dirty_intra_refresh_regions",
                    &self.dirty_intra_refresh_regions,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoReferenceIntraRefreshInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_REFERENCE_INTRA_REFRESH_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoReferenceSlotInfoKHR<'a>> for VideoReferenceIntraRefreshInfoKHR<'a> {}

    impl Default for VideoReferenceIntraRefreshInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                dirty_intra_refresh_regions: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoReferenceIntraRefreshInfoKHR<'a> {
        pub fn dirty_intra_refresh_regions(mut self, dirty_intra_refresh_regions: u32) -> Self {
            self.dirty_intra_refresh_regions = dirty_intra_refresh_regions;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_encode_intra_refresh: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "video_encode_intra_refresh",
                    &self.video_encode_intra_refresh,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_INTRA_REFRESH_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                video_encode_intra_refresh: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'a> {
        pub fn video_encode_intra_refresh(mut self, video_encode_intra_refresh: bool) -> Self {
            self.video_encode_intra_refresh = video_encode_intra_refresh.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeIntraRefreshModeFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeIntraRefreshModeFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeIntraRefreshModeFlagsKHR, Flags);

    impl VideoEncodeIntraRefreshModeFlagsKHR {
        pub const PER_PICTURE_PARTITION_KHR: Self =
            Self(VideoEncodeIntraRefreshModeFlagBitsKHR::PER_PICTURE_PARTITION_KHR.0);
        pub const BLOCK_BASED_KHR: Self =
            Self(VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_BASED_KHR.0);
        pub const BLOCK_ROW_BASED_KHR: Self =
            Self(VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_ROW_BASED_KHR.0);
        pub const BLOCK_COLUMN_BASED_KHR: Self =
            Self(VideoEncodeIntraRefreshModeFlagBitsKHR::BLOCK_COLUMN_BASED_KHR.0);
        pub const NONE: Self = Self(0);
    }

    impl fmt::Debug for VideoEncodeIntraRefreshModeFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeIntraRefreshModeFlagsKHR::PER_PICTURE_PARTITION_KHR.0,
                    "PER_PICTURE_PARTITION_KHR",
                ),
                (
                    VideoEncodeIntraRefreshModeFlagsKHR::BLOCK_BASED_KHR.0,
                    "BLOCK_BASED_KHR",
                ),
                (
                    VideoEncodeIntraRefreshModeFlagsKHR::BLOCK_ROW_BASED_KHR.0,
                    "BLOCK_ROW_BASED_KHR",
                ),
                (
                    VideoEncodeIntraRefreshModeFlagsKHR::BLOCK_COLUMN_BASED_KHR.0,
                    "BLOCK_COLUMN_BASED_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeIntraRefreshModeFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeIntraRefreshModeFlagBitsKHR(u32);

    impl VideoEncodeIntraRefreshModeFlagBitsKHR {
        pub const PER_PICTURE_PARTITION_KHR: Self = Self(1 << 0);
        pub const BLOCK_BASED_KHR: Self = Self(1 << 1);
        pub const BLOCK_ROW_BASED_KHR: Self = Self(1 << 2);
        pub const BLOCK_COLUMN_BASED_KHR: Self = Self(1 << 3);
    }

    impl fmt::Debug for VideoEncodeIntraRefreshModeFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::PER_PICTURE_PARTITION_KHR => Some("PER_PICTURE_PARTITION_KHR"),
                Self::BLOCK_BASED_KHR => Some("BLOCK_BASED_KHR"),
                Self::BLOCK_ROW_BASED_KHR => Some("BLOCK_ROW_BASED_KHR"),
                Self::BLOCK_COLUMN_BASED_KHR => Some("BLOCK_COLUMN_BASED_KHR"),
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
