//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_decode_h265.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_decode_h265";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH265ProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH265ProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_profile_idc: StdVideoH265ProfileIdc,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH265ProfileInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH265ProfileInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("std_profile_idc", &self.std_profile_idc)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH265ProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H265_PROFILE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoDecodeH265ProfileInfoKHR<'a> {}
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoDecodeH265ProfileInfoKHR<'a> {}

    impl Default for VideoDecodeH265ProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                std_profile_idc: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH265ProfileInfoKHR<'a> {
        #[inline]
        pub fn std_profile_idc(mut self, std_profile_idc: StdVideoH265ProfileIdc) -> Self {
            self.std_profile_idc = std_profile_idc;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH265CapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH265CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_level_idc: StdVideoH265LevelIdc,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH265CapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH265CapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_level_idc", &self.max_level_idc)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH265CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H265_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoDecodeH265CapabilitiesKHR<'a> {}

    impl Default for VideoDecodeH265CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_level_idc: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH265CapabilitiesKHR<'a> {
        #[inline]
        pub fn max_level_idc(mut self, max_level_idc: StdVideoH265LevelIdc) -> Self {
            self.max_level_idc = max_level_idc;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH265SessionParametersAddInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH265SessionParametersAddInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_vps_count: u32,
        pub p_std_vp_ss: *const StdVideoH265VideoParameterSet<'a>,
        pub std_sps_count: u32,
        pub p_std_sp_ss: *const StdVideoH265SequenceParameterSet<'a>,
        pub std_pps_count: u32,
        pub p_std_pp_ss: *const StdVideoH265PictureParameterSet<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH265SessionParametersAddInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH265SessionParametersAddInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("std_vps_count", &self.std_vps_count)
                .field("p_std_vp_ss", &self.p_std_vp_ss)
                .field("std_sps_count", &self.std_sps_count)
                .field("p_std_sp_ss", &self.p_std_sp_ss)
                .field("std_pps_count", &self.std_pps_count)
                .field("p_std_pp_ss", &self.p_std_pp_ss)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH265SessionParametersAddInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersUpdateInfoKHR<'a>>
        for VideoDecodeH265SessionParametersAddInfoKHR<'a>
    {
    }

    impl Default for VideoDecodeH265SessionParametersAddInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                std_vps_count: Default::default(),
                p_std_vp_ss: core::ptr::null(),
                std_sps_count: Default::default(),
                p_std_sp_ss: core::ptr::null(),
                std_pps_count: Default::default(),
                p_std_pp_ss: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH265SessionParametersAddInfoKHR<'a> {
        #[inline]
        pub fn std_vp_ss(mut self, std_vp_ss: &'a [StdVideoH265VideoParameterSet<'a>]) -> Self {
            self.std_vps_count = std_vp_ss.len().try_into().unwrap();
            self.p_std_vp_ss = std_vp_ss.as_ptr();
            self
        }

        #[inline]
        pub fn std_sp_ss(mut self, std_sp_ss: &'a [StdVideoH265SequenceParameterSet<'a>]) -> Self {
            self.std_sps_count = std_sp_ss.len().try_into().unwrap();
            self.p_std_sp_ss = std_sp_ss.as_ptr();
            self
        }

        #[inline]
        pub fn std_pp_ss(mut self, std_pp_ss: &'a [StdVideoH265PictureParameterSet<'a>]) -> Self {
            self.std_pps_count = std_pp_ss.len().try_into().unwrap();
            self.p_std_pp_ss = std_pp_ss.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH265SessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH265SessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_std_vps_count: u32,
        pub max_std_sps_count: u32,
        pub max_std_pps_count: u32,
        pub p_parameters_add_info: *const VideoDecodeH265SessionParametersAddInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH265SessionParametersCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH265SessionParametersCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_std_vps_count", &self.max_std_vps_count)
                .field("max_std_sps_count", &self.max_std_sps_count)
                .field("max_std_pps_count", &self.max_std_pps_count)
                .field("p_parameters_add_info", &self.p_parameters_add_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH265SessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersCreateInfoKHR<'a>>
        for VideoDecodeH265SessionParametersCreateInfoKHR<'a>
    {
    }

    impl Default for VideoDecodeH265SessionParametersCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                max_std_vps_count: Default::default(),
                max_std_sps_count: Default::default(),
                max_std_pps_count: Default::default(),
                p_parameters_add_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH265SessionParametersCreateInfoKHR<'a> {
        #[inline]
        pub fn max_std_vps_count(mut self, max_std_vps_count: u32) -> Self {
            self.max_std_vps_count = max_std_vps_count;
            self
        }

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
            parameters_add_info: &'a VideoDecodeH265SessionParametersAddInfoKHR<'a>,
        ) -> Self {
            self.p_parameters_add_info = parameters_add_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH265PictureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH265PictureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_picture_info: *const StdVideoDecodeH265PictureInfo,
        pub slice_segment_count: u32,
        pub p_slice_segment_offsets: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH265PictureInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH265PictureInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_picture_info", &self.p_std_picture_info)
                .field("slice_segment_count", &self.slice_segment_count)
                .field("p_slice_segment_offsets", &self.p_slice_segment_offsets)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH265PictureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H265_PICTURE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoDecodeInfoKHR<'a>> for VideoDecodeH265PictureInfoKHR<'a> {}

    impl Default for VideoDecodeH265PictureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_picture_info: core::ptr::null(),
                slice_segment_count: Default::default(),
                p_slice_segment_offsets: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH265PictureInfoKHR<'a> {
        #[inline]
        pub fn std_picture_info(
            mut self,
            std_picture_info: &'a StdVideoDecodeH265PictureInfo,
        ) -> Self {
            self.p_std_picture_info = std_picture_info;
            self
        }

        #[inline]
        pub fn slice_segment_offsets(mut self, slice_segment_offsets: &'a [u32]) -> Self {
            self.slice_segment_count = slice_segment_offsets.len().try_into().unwrap();
            self.p_slice_segment_offsets = slice_segment_offsets.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH265DpbSlotInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoDecodeH265DpbSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_reference_info: *const StdVideoDecodeH265ReferenceInfo,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoDecodeH265DpbSlotInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoDecodeH265DpbSlotInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_reference_info", &self.p_std_reference_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH265DpbSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H265_DPB_SLOT_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoReferenceSlotInfoKHR<'a>> for VideoDecodeH265DpbSlotInfoKHR<'a> {}

    impl Default for VideoDecodeH265DpbSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_reference_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH265DpbSlotInfoKHR<'a> {
        #[inline]
        pub fn std_reference_info(
            mut self,
            std_reference_info: &'a StdVideoDecodeH265ReferenceInfo,
        ) -> Self {
            self.p_std_reference_info = std_reference_info;
            self
        }
    }
}
