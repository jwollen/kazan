#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
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
            partition_independent_intra_refresh_regions: Bool32,
        ) -> Self {
            self.partition_independent_intra_refresh_regions =
                partition_independent_intra_refresh_regions;
            self
        }
        pub fn non_rectangular_intra_refresh_regions(
            mut self,
            non_rectangular_intra_refresh_regions: Bool32,
        ) -> Self {
            self.non_rectangular_intra_refresh_regions = non_rectangular_intra_refresh_regions;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeSessionIntraRefreshCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub intra_refresh_mode: VideoEncodeIntraRefreshModeFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeIntraRefreshInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub intra_refresh_cycle_duration: u32,
        pub intra_refresh_index: u32,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoReferenceIntraRefreshInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub dirty_intra_refresh_regions: u32,
        pub _marker: PhantomData<&'a ()>,
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_encode_intra_refresh: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn video_encode_intra_refresh(mut self, video_encode_intra_refresh: Bool32) -> Self {
            self.video_encode_intra_refresh = video_encode_intra_refresh;
            self
        }
    }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeIntraRefreshModeFlagBitsKHR(u32);
    impl VideoEncodeIntraRefreshModeFlagBitsKHR {
        pub const PER_PICTURE_PARTITION_KHR: Self = Self(1 << 0);
        pub const BLOCK_BASED_KHR: Self = Self(1 << 1);
        pub const BLOCK_ROW_BASED_KHR: Self = Self(1 << 2);
        pub const BLOCK_COLUMN_BASED_KHR: Self = Self(1 << 3);
    }
}
