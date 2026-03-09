//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_decode_av1.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_decode_av1";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    pub const MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeAV1ProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeAV1ProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_profile: StdVideoAV1Profile,
        pub film_grain_support: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeAV1ProfileInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeAV1ProfileInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("std_profile", &self.std_profile)
                .field("film_grain_support", &self.film_grain_support)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeAV1ProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_AV1_PROFILE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoDecodeAV1ProfileInfoKHR<'a> {}
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoDecodeAV1ProfileInfoKHR<'a> {}

    impl Default for VideoDecodeAV1ProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                std_profile: Default::default(),
                film_grain_support: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeAV1ProfileInfoKHR<'a> {
        #[inline]
        pub fn std_profile(mut self, std_profile: StdVideoAV1Profile) -> Self {
            self.std_profile = std_profile;
            self
        }

        #[inline]
        pub fn film_grain_support(mut self, film_grain_support: bool) -> Self {
            self.film_grain_support = film_grain_support.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeAV1CapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeAV1CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_level: StdVideoAV1Level,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeAV1CapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeAV1CapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_level", &self.max_level)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeAV1CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_AV1_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoDecodeAV1CapabilitiesKHR<'a> {}

    impl Default for VideoDecodeAV1CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeAV1CapabilitiesKHR<'a> {
        #[inline]
        pub fn max_level(mut self, max_level: StdVideoAV1Level) -> Self {
            self.max_level = max_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeAV1SessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeAV1SessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_sequence_header: *const StdVideoAV1SequenceHeader<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeAV1SessionParametersCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeAV1SessionParametersCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_sequence_header", &self.p_std_sequence_header)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeAV1SessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersCreateInfoKHR<'a>>
        for VideoDecodeAV1SessionParametersCreateInfoKHR<'a>
    {
    }

    impl Default for VideoDecodeAV1SessionParametersCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_std_sequence_header: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeAV1SessionParametersCreateInfoKHR<'a> {
        #[inline]
        pub fn std_sequence_header(
            mut self,
            std_sequence_header: &'a StdVideoAV1SequenceHeader<'a>,
        ) -> Self {
            self.p_std_sequence_header = std_sequence_header;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeAV1PictureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeAV1PictureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_picture_info: *const StdVideoDecodeAV1PictureInfo<'a>,
        pub reference_name_slot_indices: [i32; MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
        pub frame_header_offset: u32,
        pub tile_count: u32,
        pub p_tile_offsets: *const u32,
        pub p_tile_sizes: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeAV1PictureInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeAV1PictureInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_picture_info", &self.p_std_picture_info)
                .field(
                    "reference_name_slot_indices",
                    &self.reference_name_slot_indices,
                )
                .field("frame_header_offset", &self.frame_header_offset)
                .field("tile_count", &self.tile_count)
                .field("p_tile_offsets", &self.p_tile_offsets)
                .field("p_tile_sizes", &self.p_tile_sizes)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeAV1PictureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_AV1_PICTURE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoDecodeInfoKHR<'a>> for VideoDecodeAV1PictureInfoKHR<'a> {}

    impl Default for VideoDecodeAV1PictureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_std_picture_info: ptr::null(),
                reference_name_slot_indices: [Default::default(); _],
                frame_header_offset: Default::default(),
                tile_count: Default::default(),
                p_tile_offsets: ptr::null(),
                p_tile_sizes: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeAV1PictureInfoKHR<'a> {
        #[inline]
        pub fn std_picture_info(
            mut self,
            std_picture_info: &'a StdVideoDecodeAV1PictureInfo<'a>,
        ) -> Self {
            self.p_std_picture_info = std_picture_info;
            self
        }

        #[inline]
        pub fn reference_name_slot_indices(
            mut self,
            reference_name_slot_indices: [i32; MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
        ) -> Self {
            self.reference_name_slot_indices = reference_name_slot_indices;
            self
        }

        #[inline]
        pub fn frame_header_offset(mut self, frame_header_offset: u32) -> Self {
            self.frame_header_offset = frame_header_offset;
            self
        }

        #[inline]
        pub fn tiles(mut self, tile_offsets: &'a [u32], tile_sizes: &'a [u32]) -> Self {
            self.tile_count = tile_offsets.len().try_into().unwrap();
            assert_eq!(tile_sizes.len(), self.tile_count as usize);
            self.p_tile_offsets = tile_offsets.as_ptr();
            self.p_tile_sizes = tile_sizes.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeAV1DpbSlotInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeAV1DpbSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_reference_info: *const StdVideoDecodeAV1ReferenceInfo,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeAV1DpbSlotInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeAV1DpbSlotInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_reference_info", &self.p_std_reference_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeAV1DpbSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_AV1_DPB_SLOT_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoReferenceSlotInfoKHR<'a>> for VideoDecodeAV1DpbSlotInfoKHR<'a> {}

    impl Default for VideoDecodeAV1DpbSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_std_reference_info: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeAV1DpbSlotInfoKHR<'a> {
        #[inline]
        pub fn std_reference_info(
            mut self,
            std_reference_info: &'a StdVideoDecodeAV1ReferenceInfo,
        ) -> Self {
            self.p_std_reference_info = std_reference_info;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkVideoDecodeAV1ProfileInfoKHR = VideoDecodeAV1ProfileInfoKHR<'static>;
    pub type VkVideoDecodeAV1CapabilitiesKHR = VideoDecodeAV1CapabilitiesKHR<'static>;
    pub type VkVideoDecodeAV1SessionParametersCreateInfoKHR =
        VideoDecodeAV1SessionParametersCreateInfoKHR<'static>;
    pub type VkVideoDecodeAV1PictureInfoKHR = VideoDecodeAV1PictureInfoKHR<'static>;
    pub type VkVideoDecodeAV1DpbSlotInfoKHR = VideoDecodeAV1DpbSlotInfoKHR<'static>;
    impl VideoDecodeAV1ProfileInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeAV1ProfileInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeAV1CapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeAV1CapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeAV1SessionParametersCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoDecodeAV1SessionParametersCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeAV1PictureInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeAV1PictureInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeAV1DpbSlotInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeAV1DpbSlotInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
