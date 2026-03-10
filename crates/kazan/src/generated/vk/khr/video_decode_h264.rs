//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_decode_h264.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_decode_h264";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264ProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH264ProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_profile_idc: StdVideoH264ProfileIdc,
        pub picture_layout: VideoDecodeH264PictureLayoutFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH264ProfileInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH264ProfileInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("std_profile_idc", &self.std_profile_idc)
                .field("picture_layout", &self.picture_layout)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264ProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H264_PROFILE_INFO_KHR;
    }

    unsafe impl Extends<VideoProfileInfoKHR<'_>> for VideoDecodeH264ProfileInfoKHR<'_> {}
    unsafe impl Extends<QueryPoolCreateInfo<'_>> for VideoDecodeH264ProfileInfoKHR<'_> {}

    impl Default for VideoDecodeH264ProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                std_profile_idc: Default::default(),
                picture_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264ProfileInfoKHR<'a> {
        #[inline]
        pub fn std_profile_idc(mut self, std_profile_idc: StdVideoH264ProfileIdc) -> Self {
            self.std_profile_idc = std_profile_idc;
            self
        }

        #[inline]
        pub fn picture_layout(
            mut self,
            picture_layout: VideoDecodeH264PictureLayoutFlagBitsKHR,
        ) -> Self {
            self.picture_layout = picture_layout;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264CapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH264CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_level_idc: StdVideoH264LevelIdc,
        pub field_offset_granularity: Offset2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH264CapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH264CapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_level_idc", &self.max_level_idc)
                .field("field_offset_granularity", &self.field_offset_granularity)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H264_CAPABILITIES_KHR;
    }

    unsafe impl Extends<VideoCapabilitiesKHR<'_>> for VideoDecodeH264CapabilitiesKHR<'_> {}

    impl Default for VideoDecodeH264CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                max_level_idc: Default::default(),
                field_offset_granularity: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264CapabilitiesKHR<'a> {
        #[inline]
        pub fn max_level_idc(mut self, max_level_idc: StdVideoH264LevelIdc) -> Self {
            self.max_level_idc = max_level_idc;
            self
        }

        #[inline]
        pub fn field_offset_granularity(mut self, field_offset_granularity: Offset2D) -> Self {
            self.field_offset_granularity = field_offset_granularity;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264SessionParametersAddInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH264SessionParametersAddInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_sps_count: u32,
        pub p_std_sp_ss: *const StdVideoH264SequenceParameterSet<'a>,
        pub std_pps_count: u32,
        pub p_std_pp_ss: *const StdVideoH264PictureParameterSet<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH264SessionParametersAddInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH264SessionParametersAddInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("std_sps_count", &self.std_sps_count)
                .field("p_std_sp_ss", &self.p_std_sp_ss)
                .field("std_pps_count", &self.std_pps_count)
                .field("p_std_pp_ss", &self.p_std_pp_ss)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264SessionParametersAddInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR;
    }

    unsafe impl Extends<VideoSessionParametersUpdateInfoKHR<'_>>
        for VideoDecodeH264SessionParametersAddInfoKHR<'_>
    {
    }

    impl Default for VideoDecodeH264SessionParametersAddInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                std_sps_count: Default::default(),
                p_std_sp_ss: ptr::null(),
                std_pps_count: Default::default(),
                p_std_pp_ss: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264SessionParametersAddInfoKHR<'a> {
        #[inline]
        pub fn std_sp_ss(mut self, std_sp_ss: &'a [StdVideoH264SequenceParameterSet<'_>]) -> Self {
            self.std_sps_count = std_sp_ss.len().try_into().unwrap();
            self.p_std_sp_ss = std_sp_ss.as_ptr() as _;
            self
        }

        #[inline]
        pub fn std_pp_ss(mut self, std_pp_ss: &'a [StdVideoH264PictureParameterSet<'_>]) -> Self {
            self.std_pps_count = std_pp_ss.len().try_into().unwrap();
            self.p_std_pp_ss = std_pp_ss.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264SessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH264SessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_std_sps_count: u32,
        pub max_std_pps_count: u32,
        pub p_parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH264SessionParametersCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH264SessionParametersCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_std_sps_count", &self.max_std_sps_count)
                .field("max_std_pps_count", &self.max_std_pps_count)
                .field("p_parameters_add_info", &self.p_parameters_add_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264SessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }

    unsafe impl Extends<VideoSessionParametersCreateInfoKHR<'_>>
        for VideoDecodeH264SessionParametersCreateInfoKHR<'_>
    {
    }

    impl Default for VideoDecodeH264SessionParametersCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                max_std_sps_count: Default::default(),
                max_std_pps_count: Default::default(),
                p_parameters_add_info: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264SessionParametersCreateInfoKHR<'a> {
        #[inline]
        pub fn max_std_sps_count(mut self, max_std_sps_count: u32) -> Self {
            self.max_std_sps_count = max_std_sps_count;
            self
        }

        #[inline]
        pub fn max_std_pps_count(mut self, max_std_pps_count: u32) -> Self {
            self.max_std_pps_count = max_std_pps_count;
            self
        }

        #[inline]
        pub fn parameters_add_info(
            mut self,
            parameters_add_info: &'a VideoDecodeH264SessionParametersAddInfoKHR<'a>,
        ) -> Self {
            self.p_parameters_add_info = parameters_add_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264PictureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH264PictureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_picture_info: *const StdVideoDecodeH264PictureInfo,
        pub slice_count: u32,
        pub p_slice_offsets: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH264PictureInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH264PictureInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_picture_info", &self.p_std_picture_info)
                .field("slice_count", &self.slice_count)
                .field("p_slice_offsets", &self.p_slice_offsets)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264PictureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H264_PICTURE_INFO_KHR;
    }

    unsafe impl Extends<VideoDecodeInfoKHR<'_>> for VideoDecodeH264PictureInfoKHR<'_> {}

    impl Default for VideoDecodeH264PictureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_std_picture_info: ptr::null(),
                slice_count: Default::default(),
                p_slice_offsets: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264PictureInfoKHR<'a> {
        #[inline]
        pub fn std_picture_info(
            mut self,
            std_picture_info: &'a StdVideoDecodeH264PictureInfo,
        ) -> Self {
            self.p_std_picture_info = std_picture_info;
            self
        }

        #[inline]
        pub fn slice_offsets(mut self, slice_offsets: &'a [u32]) -> Self {
            self.slice_count = slice_offsets.len().try_into().unwrap();
            self.p_slice_offsets = slice_offsets.as_ptr() as _;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264DpbSlotInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH264DpbSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_reference_info: *const StdVideoDecodeH264ReferenceInfo,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH264DpbSlotInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH264DpbSlotInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_reference_info", &self.p_std_reference_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264DpbSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR;
    }

    unsafe impl Extends<VideoReferenceSlotInfoKHR<'_>> for VideoDecodeH264DpbSlotInfoKHR<'_> {}

    impl Default for VideoDecodeH264DpbSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_std_reference_info: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264DpbSlotInfoKHR<'a> {
        #[inline]
        pub fn std_reference_info(
            mut self,
            std_reference_info: &'a StdVideoDecodeH264ReferenceInfo,
        ) -> Self {
            self.p_std_reference_info = std_reference_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264PictureLayoutFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoDecodeH264PictureLayoutFlagsKHR(Flags);
    vk_bitflags_wrapped!(
        VideoDecodeH264PictureLayoutFlagsKHR,
        Flags,
        VideoDecodeH264PictureLayoutFlagBitsKHR
    );

    impl VideoDecodeH264PictureLayoutFlagsKHR {
        pub const PROGRESSIVE: Self = Self(0);
    }

    impl fmt::Debug for VideoDecodeH264PictureLayoutFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoDecodeH264PictureLayoutFlagBitsKHR::INTERLACED_INTERLEAVED_LINES_KHR.0,
                    "INTERLACED_INTERLEAVED_LINES_KHR",
                ),
                (
                    VideoDecodeH264PictureLayoutFlagBitsKHR::INTERLACED_SEPARATE_PLANES_KHR.0,
                    "INTERLACED_SEPARATE_PLANES_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoDecodeH264PictureLayoutFlagBitsKHR(u32);

    impl VideoDecodeH264PictureLayoutFlagBitsKHR {
        pub const INTERLACED_INTERLEAVED_LINES_KHR: Self = Self(1 << 0);
        pub const INTERLACED_SEPARATE_PLANES_KHR: Self = Self(1 << 1);
    }

    impl fmt::Debug for VideoDecodeH264PictureLayoutFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INTERLACED_INTERLEAVED_LINES_KHR => Some("INTERLACED_INTERLEAVED_LINES_KHR"),
                Self::INTERLACED_SEPARATE_PLANES_KHR => Some("INTERLACED_SEPARATE_PLANES_KHR"),
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

    pub type VkVideoDecodeH264ProfileInfoKHR = VideoDecodeH264ProfileInfoKHR<'static>;
    pub type VkVideoDecodeH264CapabilitiesKHR = VideoDecodeH264CapabilitiesKHR<'static>;
    pub type VkVideoDecodeH264SessionParametersAddInfoKHR =
        VideoDecodeH264SessionParametersAddInfoKHR<'static>;
    pub type VkVideoDecodeH264SessionParametersCreateInfoKHR =
        VideoDecodeH264SessionParametersCreateInfoKHR<'static>;
    pub type VkVideoDecodeH264PictureInfoKHR = VideoDecodeH264PictureInfoKHR<'static>;
    pub type VkVideoDecodeH264DpbSlotInfoKHR = VideoDecodeH264DpbSlotInfoKHR<'static>;
    pub type VkVideoDecodeH264PictureLayoutFlagsKHR = VideoDecodeH264PictureLayoutFlagsKHR;
    pub type VkVideoDecodeH264PictureLayoutFlagBitsKHR = VideoDecodeH264PictureLayoutFlagBitsKHR;
    impl VideoDecodeH264ProfileInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeH264ProfileInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeH264CapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeH264CapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeH264SessionParametersAddInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoDecodeH264SessionParametersAddInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeH264SessionParametersCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoDecodeH264SessionParametersCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeH264PictureInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeH264PictureInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoDecodeH264DpbSlotInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoDecodeH264DpbSlotInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
