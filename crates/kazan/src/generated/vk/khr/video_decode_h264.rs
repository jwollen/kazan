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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264ProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeH264ProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_profile_idc: StdVideoH264ProfileIdc,
        pub picture_layout: VideoDecodeH264PictureLayoutFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264ProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H264_PROFILE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoDecodeH264ProfileInfoKHR<'a> {}
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoDecodeH264ProfileInfoKHR<'a> {}

    impl Default for VideoDecodeH264ProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                std_profile_idc: Default::default(),
                picture_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264ProfileInfoKHR<'a> {
        pub fn std_profile_idc(mut self, std_profile_idc: StdVideoH264ProfileIdc) -> Self {
            self.std_profile_idc = std_profile_idc;
            self
        }

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
    pub struct VideoDecodeH264CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_level_idc: StdVideoH264LevelIdc,
        pub field_offset_granularity: Offset2D,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H264_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoDecodeH264CapabilitiesKHR<'a> {}

    impl Default for VideoDecodeH264CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_level_idc: Default::default(),
                field_offset_granularity: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264CapabilitiesKHR<'a> {
        pub fn max_level_idc(mut self, max_level_idc: StdVideoH264LevelIdc) -> Self {
            self.max_level_idc = max_level_idc;
            self
        }

        pub fn field_offset_granularity(mut self, field_offset_granularity: Offset2D) -> Self {
            self.field_offset_granularity = field_offset_granularity;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264SessionParametersAddInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeH264SessionParametersAddInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_sps_count: u32,
        pub p_std_sp_ss: *const StdVideoH264SequenceParameterSet<'a>,
        pub std_pps_count: u32,
        pub p_std_pp_ss: *const StdVideoH264PictureParameterSet<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264SessionParametersAddInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersUpdateInfoKHR<'a>>
        for VideoDecodeH264SessionParametersAddInfoKHR<'a>
    {
    }

    impl Default for VideoDecodeH264SessionParametersAddInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                std_sps_count: Default::default(),
                p_std_sp_ss: core::ptr::null(),
                std_pps_count: Default::default(),
                p_std_pp_ss: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264SessionParametersAddInfoKHR<'a> {
        pub fn std_sp_ss(mut self, std_sp_ss: &'a [StdVideoH264SequenceParameterSet<'a>]) -> Self {
            self.std_sps_count = std_sp_ss.len().try_into().unwrap();
            self.p_std_sp_ss = std_sp_ss.as_ptr();
            self
        }

        pub fn std_pp_ss(mut self, std_pp_ss: &'a [StdVideoH264PictureParameterSet<'a>]) -> Self {
            self.std_pps_count = std_pp_ss.len().try_into().unwrap();
            self.p_std_pp_ss = std_pp_ss.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264SessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeH264SessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_std_sps_count: u32,
        pub max_std_pps_count: u32,
        pub p_parameters_add_info: *const VideoDecodeH264SessionParametersAddInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264SessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_DECODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersCreateInfoKHR<'a>>
        for VideoDecodeH264SessionParametersCreateInfoKHR<'a>
    {
    }

    impl Default for VideoDecodeH264SessionParametersCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                max_std_sps_count: Default::default(),
                max_std_pps_count: Default::default(),
                p_parameters_add_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264SessionParametersCreateInfoKHR<'a> {
        pub fn max_std_sps_count(mut self, max_std_sps_count: u32) -> Self {
            self.max_std_sps_count = max_std_sps_count;
            self
        }

        pub fn max_std_pps_count(mut self, max_std_pps_count: u32) -> Self {
            self.max_std_pps_count = max_std_pps_count;
            self
        }

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
    pub struct VideoDecodeH264PictureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_picture_info: *const StdVideoDecodeH264PictureInfo,
        pub slice_count: u32,
        pub p_slice_offsets: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264PictureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H264_PICTURE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoDecodeInfoKHR<'a>> for VideoDecodeH264PictureInfoKHR<'a> {}

    impl Default for VideoDecodeH264PictureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_picture_info: core::ptr::null(),
                slice_count: Default::default(),
                p_slice_offsets: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264PictureInfoKHR<'a> {
        pub fn std_picture_info(
            mut self,
            std_picture_info: &'a StdVideoDecodeH264PictureInfo,
        ) -> Self {
            self.p_std_picture_info = std_picture_info;
            self
        }

        pub fn slice_offsets(mut self, slice_offsets: &'a [u32]) -> Self {
            self.slice_count = slice_offsets.len().try_into().unwrap();
            self.p_slice_offsets = slice_offsets.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoDecodeH264DpbSlotInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoDecodeH264DpbSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_reference_info: *const StdVideoDecodeH264ReferenceInfo,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoDecodeH264DpbSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_DECODE_H264_DPB_SLOT_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoReferenceSlotInfoKHR<'a>> for VideoDecodeH264DpbSlotInfoKHR<'a> {}

    impl Default for VideoDecodeH264DpbSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_reference_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoDecodeH264DpbSlotInfoKHR<'a> {
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
    vk_bitflags_wrapped!(VideoDecodeH264PictureLayoutFlagsKHR, Flags);

    impl VideoDecodeH264PictureLayoutFlagsKHR {
        pub const INTERLACED_INTERLEAVED_LINES_KHR: Self =
            Self(VideoDecodeH264PictureLayoutFlagBitsKHR::INTERLACED_INTERLEAVED_LINES_KHR.0);
        pub const INTERLACED_SEPARATE_PLANES_KHR: Self =
            Self(VideoDecodeH264PictureLayoutFlagBitsKHR::INTERLACED_SEPARATE_PLANES_KHR.0);
        pub const PROGRESSIVE: Self = Self(0);
    }

    impl fmt::Debug for VideoDecodeH264PictureLayoutFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoDecodeH264PictureLayoutFlagsKHR::INTERLACED_INTERLEAVED_LINES_KHR.0,
                    "INTERLACED_INTERLEAVED_LINES_KHR",
                ),
                (
                    VideoDecodeH264PictureLayoutFlagsKHR::INTERLACED_SEPARATE_PLANES_KHR.0,
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
}
