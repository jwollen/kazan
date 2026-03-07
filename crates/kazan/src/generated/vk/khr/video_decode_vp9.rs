#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_decode_vp9";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceVideoDecodeVP9FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceVideoDecodeVP9FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_decode_vp9: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceVideoDecodeVP9FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceVideoDecodeVP9FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("video_decode_vp9", &self.video_decode_vp9)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceVideoDecodeVP9FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_VIDEO_DECODE_VP9_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceVideoDecodeVP9FeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceVideoDecodeVP9FeaturesKHR<'a> {}

    impl Default for PhysicalDeviceVideoDecodeVP9FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                video_decode_vp9: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceVideoDecodeVP9FeaturesKHR<'a> {
        pub fn video_decode_vp9(mut self, video_decode_vp9: bool) -> Self {
            self.video_decode_vp9 = video_decode_vp9.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeVP9ProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeVP9ProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_profile: StdVideoVP9Profile,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeVP9ProfileInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeVP9ProfileInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("std_profile", &self.std_profile)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeVP9ProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_VP9_PROFILE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoDecodeVP9ProfileInfoKHR<'a> {}
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoDecodeVP9ProfileInfoKHR<'a> {}

    impl Default for VideoDecodeVP9ProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                std_profile: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeVP9ProfileInfoKHR<'a> {
        pub fn std_profile(mut self, std_profile: StdVideoVP9Profile) -> Self {
            self.std_profile = std_profile;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeVP9CapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeVP9CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_level: StdVideoVP9Level,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeVP9CapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeVP9CapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_level", &self.max_level)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeVP9CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_VP9_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoDecodeVP9CapabilitiesKHR<'a> {}

    impl Default for VideoDecodeVP9CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeVP9CapabilitiesKHR<'a> {
        pub fn max_level(mut self, max_level: StdVideoVP9Level) -> Self {
            self.max_level = max_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeVP9PictureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeVP9PictureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_picture_info: *const StdVideoDecodeVP9PictureInfo<'a>,
        pub reference_name_slot_indices: [i32; MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR as usize],
        pub uncompressed_header_offset: u32,
        pub compressed_header_offset: u32,
        pub tiles_offset: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeVP9PictureInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeVP9PictureInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_picture_info", &self.p_std_picture_info)
                .field(
                    "reference_name_slot_indices",
                    &self.reference_name_slot_indices,
                )
                .field(
                    "uncompressed_header_offset",
                    &self.uncompressed_header_offset,
                )
                .field("compressed_header_offset", &self.compressed_header_offset)
                .field("tiles_offset", &self.tiles_offset)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeVP9PictureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_VP9_PICTURE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoDecodeInfoKHR<'a>> for VideoDecodeVP9PictureInfoKHR<'a> {}

    impl Default for VideoDecodeVP9PictureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_picture_info: core::ptr::null(),
                reference_name_slot_indices: [Default::default(); _],
                uncompressed_header_offset: Default::default(),
                compressed_header_offset: Default::default(),
                tiles_offset: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeVP9PictureInfoKHR<'a> {
        pub fn std_picture_info(
            mut self,
            std_picture_info: &'a StdVideoDecodeVP9PictureInfo<'a>,
        ) -> Self {
            self.p_std_picture_info = std_picture_info;
            self
        }

        pub fn reference_name_slot_indices(
            mut self,
            reference_name_slot_indices: [i32; MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR as usize],
        ) -> Self {
            self.reference_name_slot_indices = reference_name_slot_indices;
            self
        }

        pub fn uncompressed_header_offset(mut self, uncompressed_header_offset: u32) -> Self {
            self.uncompressed_header_offset = uncompressed_header_offset;
            self
        }

        pub fn compressed_header_offset(mut self, compressed_header_offset: u32) -> Self {
            self.compressed_header_offset = compressed_header_offset;
            self
        }

        pub fn tiles_offset(mut self, tiles_offset: u32) -> Self {
            self.tiles_offset = tiles_offset;
            self
        }
    }
}
