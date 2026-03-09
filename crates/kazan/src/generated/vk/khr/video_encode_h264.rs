//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_video_encode_h264.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_video_encode_h264";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264CapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: VideoEncodeH264CapabilityFlagsKHR,
        pub max_level_idc: StdVideoH264LevelIdc,
        pub max_slice_count: u32,
        pub max_p_picture_l0_reference_count: u32,
        pub max_b_picture_l0_reference_count: u32,
        pub max_l1_reference_count: u32,
        pub max_temporal_layer_count: u32,
        pub expect_dyadic_temporal_layer_pattern: Bool32,
        pub min_qp: i32,
        pub max_qp: i32,
        pub prefers_gop_remaining_frames: Bool32,
        pub requires_gop_remaining_frames: Bool32,
        pub std_syntax_flags: VideoEncodeH264StdFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264CapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264CapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("max_level_idc", &self.max_level_idc)
                .field("max_slice_count", &self.max_slice_count)
                .field(
                    "max_p_picture_l0_reference_count",
                    &self.max_p_picture_l0_reference_count,
                )
                .field(
                    "max_b_picture_l0_reference_count",
                    &self.max_b_picture_l0_reference_count,
                )
                .field("max_l1_reference_count", &self.max_l1_reference_count)
                .field("max_temporal_layer_count", &self.max_temporal_layer_count)
                .field(
                    "expect_dyadic_temporal_layer_pattern",
                    &self.expect_dyadic_temporal_layer_pattern,
                )
                .field("min_qp", &self.min_qp)
                .field("max_qp", &self.max_qp)
                .field(
                    "prefers_gop_remaining_frames",
                    &self.prefers_gop_remaining_frames,
                )
                .field(
                    "requires_gop_remaining_frames",
                    &self.requires_gop_remaining_frames,
                )
                .field("std_syntax_flags", &self.std_syntax_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H264_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoEncodeH264CapabilitiesKHR<'a> {}

    impl Default for VideoEncodeH264CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                flags: Default::default(),
                max_level_idc: Default::default(),
                max_slice_count: Default::default(),
                max_p_picture_l0_reference_count: Default::default(),
                max_b_picture_l0_reference_count: Default::default(),
                max_l1_reference_count: Default::default(),
                max_temporal_layer_count: Default::default(),
                expect_dyadic_temporal_layer_pattern: Default::default(),
                min_qp: Default::default(),
                max_qp: Default::default(),
                prefers_gop_remaining_frames: Default::default(),
                requires_gop_remaining_frames: Default::default(),
                std_syntax_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264CapabilitiesKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoEncodeH264CapabilityFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn max_level_idc(mut self, max_level_idc: StdVideoH264LevelIdc) -> Self {
            self.max_level_idc = max_level_idc;
            self
        }

        #[inline]
        pub fn max_slice_count(mut self, max_slice_count: u32) -> Self {
            self.max_slice_count = max_slice_count;
            self
        }

        #[inline]
        pub fn max_p_picture_l0_reference_count(
            mut self,
            max_p_picture_l0_reference_count: u32,
        ) -> Self {
            self.max_p_picture_l0_reference_count = max_p_picture_l0_reference_count;
            self
        }

        #[inline]
        pub fn max_b_picture_l0_reference_count(
            mut self,
            max_b_picture_l0_reference_count: u32,
        ) -> Self {
            self.max_b_picture_l0_reference_count = max_b_picture_l0_reference_count;
            self
        }

        #[inline]
        pub fn max_l1_reference_count(mut self, max_l1_reference_count: u32) -> Self {
            self.max_l1_reference_count = max_l1_reference_count;
            self
        }

        #[inline]
        pub fn max_temporal_layer_count(mut self, max_temporal_layer_count: u32) -> Self {
            self.max_temporal_layer_count = max_temporal_layer_count;
            self
        }

        #[inline]
        pub fn expect_dyadic_temporal_layer_pattern(
            mut self,
            expect_dyadic_temporal_layer_pattern: bool,
        ) -> Self {
            self.expect_dyadic_temporal_layer_pattern = expect_dyadic_temporal_layer_pattern.into();
            self
        }

        #[inline]
        pub fn min_qp(mut self, min_qp: i32) -> Self {
            self.min_qp = min_qp;
            self
        }

        #[inline]
        pub fn max_qp(mut self, max_qp: i32) -> Self {
            self.max_qp = max_qp;
            self
        }

        #[inline]
        pub fn prefers_gop_remaining_frames(mut self, prefers_gop_remaining_frames: bool) -> Self {
            self.prefers_gop_remaining_frames = prefers_gop_remaining_frames.into();
            self
        }

        #[inline]
        pub fn requires_gop_remaining_frames(
            mut self,
            requires_gop_remaining_frames: bool,
        ) -> Self {
            self.requires_gop_remaining_frames = requires_gop_remaining_frames.into();
            self
        }

        #[inline]
        pub fn std_syntax_flags(mut self, std_syntax_flags: VideoEncodeH264StdFlagsKHR) -> Self {
            self.std_syntax_flags = std_syntax_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264QualityLevelPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264QualityLevelPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub preferred_rate_control_flags: VideoEncodeH264RateControlFlagsKHR,
        pub preferred_gop_frame_count: u32,
        pub preferred_idr_period: u32,
        pub preferred_consecutive_b_frame_count: u32,
        pub preferred_temporal_layer_count: u32,
        pub preferred_constant_qp: VideoEncodeH264QpKHR,
        pub preferred_max_l0_reference_count: u32,
        pub preferred_max_l1_reference_count: u32,
        pub preferred_std_entropy_coding_mode_flag: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264QualityLevelPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264QualityLevelPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "preferred_rate_control_flags",
                    &self.preferred_rate_control_flags,
                )
                .field("preferred_gop_frame_count", &self.preferred_gop_frame_count)
                .field("preferred_idr_period", &self.preferred_idr_period)
                .field(
                    "preferred_consecutive_b_frame_count",
                    &self.preferred_consecutive_b_frame_count,
                )
                .field(
                    "preferred_temporal_layer_count",
                    &self.preferred_temporal_layer_count,
                )
                .field("preferred_constant_qp", &self.preferred_constant_qp)
                .field(
                    "preferred_max_l0_reference_count",
                    &self.preferred_max_l0_reference_count,
                )
                .field(
                    "preferred_max_l1_reference_count",
                    &self.preferred_max_l1_reference_count,
                )
                .field(
                    "preferred_std_entropy_coding_mode_flag",
                    &self.preferred_std_entropy_coding_mode_flag,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264QualityLevelPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeQualityLevelPropertiesKHR<'a>>
        for VideoEncodeH264QualityLevelPropertiesKHR<'a>
    {
    }

    impl Default for VideoEncodeH264QualityLevelPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                preferred_rate_control_flags: Default::default(),
                preferred_gop_frame_count: Default::default(),
                preferred_idr_period: Default::default(),
                preferred_consecutive_b_frame_count: Default::default(),
                preferred_temporal_layer_count: Default::default(),
                preferred_constant_qp: Default::default(),
                preferred_max_l0_reference_count: Default::default(),
                preferred_max_l1_reference_count: Default::default(),
                preferred_std_entropy_coding_mode_flag: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264QualityLevelPropertiesKHR<'a> {
        #[inline]
        pub fn preferred_rate_control_flags(
            mut self,
            preferred_rate_control_flags: VideoEncodeH264RateControlFlagsKHR,
        ) -> Self {
            self.preferred_rate_control_flags = preferred_rate_control_flags;
            self
        }

        #[inline]
        pub fn preferred_gop_frame_count(mut self, preferred_gop_frame_count: u32) -> Self {
            self.preferred_gop_frame_count = preferred_gop_frame_count;
            self
        }

        #[inline]
        pub fn preferred_idr_period(mut self, preferred_idr_period: u32) -> Self {
            self.preferred_idr_period = preferred_idr_period;
            self
        }

        #[inline]
        pub fn preferred_consecutive_b_frame_count(
            mut self,
            preferred_consecutive_b_frame_count: u32,
        ) -> Self {
            self.preferred_consecutive_b_frame_count = preferred_consecutive_b_frame_count;
            self
        }

        #[inline]
        pub fn preferred_temporal_layer_count(
            mut self,
            preferred_temporal_layer_count: u32,
        ) -> Self {
            self.preferred_temporal_layer_count = preferred_temporal_layer_count;
            self
        }

        #[inline]
        pub fn preferred_constant_qp(
            mut self,
            preferred_constant_qp: VideoEncodeH264QpKHR,
        ) -> Self {
            self.preferred_constant_qp = preferred_constant_qp;
            self
        }

        #[inline]
        pub fn preferred_max_l0_reference_count(
            mut self,
            preferred_max_l0_reference_count: u32,
        ) -> Self {
            self.preferred_max_l0_reference_count = preferred_max_l0_reference_count;
            self
        }

        #[inline]
        pub fn preferred_max_l1_reference_count(
            mut self,
            preferred_max_l1_reference_count: u32,
        ) -> Self {
            self.preferred_max_l1_reference_count = preferred_max_l1_reference_count;
            self
        }

        #[inline]
        pub fn preferred_std_entropy_coding_mode_flag(
            mut self,
            preferred_std_entropy_coding_mode_flag: bool,
        ) -> Self {
            self.preferred_std_entropy_coding_mode_flag =
                preferred_std_entropy_coding_mode_flag.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264SessionCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264SessionCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_max_level_idc: Bool32,
        pub max_level_idc: StdVideoH264LevelIdc,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264SessionCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264SessionCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("use_max_level_idc", &self.use_max_level_idc)
                .field("max_level_idc", &self.max_level_idc)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264SessionCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionCreateInfoKHR<'a>> for VideoEncodeH264SessionCreateInfoKHR<'a> {}

    impl Default for VideoEncodeH264SessionCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                use_max_level_idc: Default::default(),
                max_level_idc: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264SessionCreateInfoKHR<'a> {
        #[inline]
        pub fn use_max_level_idc(mut self, use_max_level_idc: bool) -> Self {
            self.use_max_level_idc = use_max_level_idc.into();
            self
        }

        #[inline]
        pub fn max_level_idc(mut self, max_level_idc: StdVideoH264LevelIdc) -> Self {
            self.max_level_idc = max_level_idc;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264SessionParametersAddInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264SessionParametersAddInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_sps_count: u32,
        pub p_std_sp_ss: *const StdVideoH264SequenceParameterSet<'a>,
        pub std_pps_count: u32,
        pub p_std_pp_ss: *const StdVideoH264PictureParameterSet<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264SessionParametersAddInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264SessionParametersAddInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("std_sps_count", &self.std_sps_count)
                .field("p_std_sp_ss", &self.p_std_sp_ss)
                .field("std_pps_count", &self.std_pps_count)
                .field("p_std_pp_ss", &self.p_std_pp_ss)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264SessionParametersAddInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersUpdateInfoKHR<'a>>
        for VideoEncodeH264SessionParametersAddInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeH264SessionParametersAddInfoKHR<'_> {
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

    impl<'a> VideoEncodeH264SessionParametersAddInfoKHR<'a> {
        #[inline]
        pub fn std_sp_ss(mut self, std_sp_ss: &'a [StdVideoH264SequenceParameterSet<'a>]) -> Self {
            self.std_sps_count = std_sp_ss.len().try_into().unwrap();
            self.p_std_sp_ss = std_sp_ss.as_ptr();
            self
        }

        #[inline]
        pub fn std_pp_ss(mut self, std_pp_ss: &'a [StdVideoH264PictureParameterSet<'a>]) -> Self {
            self.std_pps_count = std_pp_ss.len().try_into().unwrap();
            self.p_std_pp_ss = std_pp_ss.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264SessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264SessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_std_sps_count: u32,
        pub max_std_pps_count: u32,
        pub p_parameters_add_info: *const VideoEncodeH264SessionParametersAddInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264SessionParametersCreateInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264SessionParametersCreateInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("max_std_sps_count", &self.max_std_sps_count)
                .field("max_std_pps_count", &self.max_std_pps_count)
                .field("p_parameters_add_info", &self.p_parameters_add_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264SessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoSessionParametersCreateInfoKHR<'a>>
        for VideoEncodeH264SessionParametersCreateInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeH264SessionParametersCreateInfoKHR<'_> {
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

    impl<'a> VideoEncodeH264SessionParametersCreateInfoKHR<'a> {
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
            parameters_add_info: &'a VideoEncodeH264SessionParametersAddInfoKHR<'a>,
        ) -> Self {
            self.p_parameters_add_info = parameters_add_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264SessionParametersGetInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264SessionParametersGetInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub write_std_sps: Bool32,
        pub write_std_pps: Bool32,
        pub std_sps_id: u32,
        pub std_pps_id: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264SessionParametersGetInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264SessionParametersGetInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("write_std_sps", &self.write_std_sps)
                .field("write_std_pps", &self.write_std_pps)
                .field("std_sps_id", &self.std_sps_id)
                .field("std_pps_id", &self.std_pps_id)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264SessionParametersGetInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeSessionParametersGetInfoKHR<'a>>
        for VideoEncodeH264SessionParametersGetInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeH264SessionParametersGetInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                write_std_sps: Default::default(),
                write_std_pps: Default::default(),
                std_sps_id: Default::default(),
                std_pps_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264SessionParametersGetInfoKHR<'a> {
        #[inline]
        pub fn write_std_sps(mut self, write_std_sps: bool) -> Self {
            self.write_std_sps = write_std_sps.into();
            self
        }

        #[inline]
        pub fn write_std_pps(mut self, write_std_pps: bool) -> Self {
            self.write_std_pps = write_std_pps.into();
            self
        }

        #[inline]
        pub fn std_sps_id(mut self, std_sps_id: u32) -> Self {
            self.std_sps_id = std_sps_id;
            self
        }

        #[inline]
        pub fn std_pps_id(mut self, std_pps_id: u32) -> Self {
            self.std_pps_id = std_pps_id;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264SessionParametersFeedbackInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264SessionParametersFeedbackInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub has_std_sps_overrides: Bool32,
        pub has_std_pps_overrides: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264SessionParametersFeedbackInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264SessionParametersFeedbackInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("has_std_sps_overrides", &self.has_std_sps_overrides)
                .field("has_std_pps_overrides", &self.has_std_pps_overrides)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264SessionParametersFeedbackInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeSessionParametersFeedbackInfoKHR<'a>>
        for VideoEncodeH264SessionParametersFeedbackInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeH264SessionParametersFeedbackInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                has_std_sps_overrides: Default::default(),
                has_std_pps_overrides: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264SessionParametersFeedbackInfoKHR<'a> {
        #[inline]
        pub fn has_std_sps_overrides(mut self, has_std_sps_overrides: bool) -> Self {
            self.has_std_sps_overrides = has_std_sps_overrides.into();
            self
        }

        #[inline]
        pub fn has_std_pps_overrides(mut self, has_std_pps_overrides: bool) -> Self {
            self.has_std_pps_overrides = has_std_pps_overrides.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264DpbSlotInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264DpbSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_reference_info: *const StdVideoEncodeH264ReferenceInfo,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264DpbSlotInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264DpbSlotInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_std_reference_info", &self.p_std_reference_info)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264DpbSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoReferenceSlotInfoKHR<'a>> for VideoEncodeH264DpbSlotInfoKHR<'a> {}

    impl Default for VideoEncodeH264DpbSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                p_std_reference_info: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264DpbSlotInfoKHR<'a> {
        #[inline]
        pub fn std_reference_info(
            mut self,
            std_reference_info: &'a StdVideoEncodeH264ReferenceInfo,
        ) -> Self {
            self.p_std_reference_info = std_reference_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264PictureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264PictureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub nalu_slice_entry_count: u32,
        pub p_nalu_slice_entries: *const VideoEncodeH264NaluSliceInfoKHR<'a>,
        pub p_std_picture_info: *const StdVideoEncodeH264PictureInfo<'a>,
        pub generate_prefix_nalu: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264PictureInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264PictureInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("nalu_slice_entry_count", &self.nalu_slice_entry_count)
                .field("p_nalu_slice_entries", &self.p_nalu_slice_entries)
                .field("p_std_picture_info", &self.p_std_picture_info)
                .field("generate_prefix_nalu", &self.generate_prefix_nalu)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264PictureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H264_PICTURE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeInfoKHR<'a>> for VideoEncodeH264PictureInfoKHR<'a> {}

    impl Default for VideoEncodeH264PictureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                nalu_slice_entry_count: Default::default(),
                p_nalu_slice_entries: ptr::null(),
                p_std_picture_info: ptr::null(),
                generate_prefix_nalu: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264PictureInfoKHR<'a> {
        #[inline]
        pub fn nalu_slice_entries(
            mut self,
            nalu_slice_entries: &'a [VideoEncodeH264NaluSliceInfoKHR<'a>],
        ) -> Self {
            self.nalu_slice_entry_count = nalu_slice_entries.len().try_into().unwrap();
            self.p_nalu_slice_entries = nalu_slice_entries.as_ptr();
            self
        }

        #[inline]
        pub fn std_picture_info(
            mut self,
            std_picture_info: &'a StdVideoEncodeH264PictureInfo<'a>,
        ) -> Self {
            self.p_std_picture_info = std_picture_info;
            self
        }

        #[inline]
        pub fn generate_prefix_nalu(mut self, generate_prefix_nalu: bool) -> Self {
            self.generate_prefix_nalu = generate_prefix_nalu.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264ProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264ProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_profile_idc: StdVideoH264ProfileIdc,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264ProfileInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264ProfileInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("std_profile_idc", &self.std_profile_idc)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264ProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H264_PROFILE_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoEncodeH264ProfileInfoKHR<'a> {}
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoEncodeH264ProfileInfoKHR<'a> {}

    impl Default for VideoEncodeH264ProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                std_profile_idc: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264ProfileInfoKHR<'a> {
        #[inline]
        pub fn std_profile_idc(mut self, std_profile_idc: StdVideoH264ProfileIdc) -> Self {
            self.std_profile_idc = std_profile_idc;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264NaluSliceInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264NaluSliceInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub constant_qp: i32,
        pub p_std_slice_header: *const StdVideoEncodeH264SliceHeader<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264NaluSliceInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264NaluSliceInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("constant_qp", &self.constant_qp)
                .field("p_std_slice_header", &self.p_std_slice_header)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264NaluSliceInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR;
    }

    impl Default for VideoEncodeH264NaluSliceInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                constant_qp: Default::default(),
                p_std_slice_header: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264NaluSliceInfoKHR<'a> {
        #[inline]
        pub fn constant_qp(mut self, constant_qp: i32) -> Self {
            self.constant_qp = constant_qp;
            self
        }

        #[inline]
        pub fn std_slice_header(
            mut self,
            std_slice_header: &'a StdVideoEncodeH264SliceHeader<'a>,
        ) -> Self {
            self.p_std_slice_header = std_slice_header;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264RateControlInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264RateControlInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoEncodeH264RateControlFlagsKHR,
        pub gop_frame_count: u32,
        pub idr_period: u32,
        pub consecutive_b_frame_count: u32,
        pub temporal_layer_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264RateControlInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264RateControlInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .field("gop_frame_count", &self.gop_frame_count)
                .field("idr_period", &self.idr_period)
                .field("consecutive_b_frame_count", &self.consecutive_b_frame_count)
                .field("temporal_layer_count", &self.temporal_layer_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264RateControlInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoCodingControlInfoKHR<'a>> for VideoEncodeH264RateControlInfoKHR<'a> {}
    unsafe impl<'a> Extends<VideoBeginCodingInfoKHR<'a>> for VideoEncodeH264RateControlInfoKHR<'a> {}

    impl Default for VideoEncodeH264RateControlInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                flags: Default::default(),
                gop_frame_count: Default::default(),
                idr_period: Default::default(),
                consecutive_b_frame_count: Default::default(),
                temporal_layer_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264RateControlInfoKHR<'a> {
        #[inline]
        pub fn flags(mut self, flags: VideoEncodeH264RateControlFlagsKHR) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn gop_frame_count(mut self, gop_frame_count: u32) -> Self {
            self.gop_frame_count = gop_frame_count;
            self
        }

        #[inline]
        pub fn idr_period(mut self, idr_period: u32) -> Self {
            self.idr_period = idr_period;
            self
        }

        #[inline]
        pub fn consecutive_b_frame_count(mut self, consecutive_b_frame_count: u32) -> Self {
            self.consecutive_b_frame_count = consecutive_b_frame_count;
            self
        }

        #[inline]
        pub fn temporal_layer_count(mut self, temporal_layer_count: u32) -> Self {
            self.temporal_layer_count = temporal_layer_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264QpKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct VideoEncodeH264QpKHR {
        pub qp_i: i32,
        pub qp_p: i32,
        pub qp_b: i32,
    }

    impl VideoEncodeH264QpKHR {
        #[inline]
        pub fn qp_i(mut self, qp_i: i32) -> Self {
            self.qp_i = qp_i;
            self
        }

        #[inline]
        pub fn qp_p(mut self, qp_p: i32) -> Self {
            self.qp_p = qp_p;
            self
        }

        #[inline]
        pub fn qp_b(mut self, qp_b: i32) -> Self {
            self.qp_b = qp_b;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264FrameSizeKHR.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct VideoEncodeH264FrameSizeKHR {
        pub frame_i_size: u32,
        pub frame_p_size: u32,
        pub frame_b_size: u32,
    }

    impl VideoEncodeH264FrameSizeKHR {
        #[inline]
        pub fn frame_i_size(mut self, frame_i_size: u32) -> Self {
            self.frame_i_size = frame_i_size;
            self
        }

        #[inline]
        pub fn frame_p_size(mut self, frame_p_size: u32) -> Self {
            self.frame_p_size = frame_p_size;
            self
        }

        #[inline]
        pub fn frame_b_size(mut self, frame_b_size: u32) -> Self {
            self.frame_b_size = frame_b_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264GopRemainingFrameInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264GopRemainingFrameInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_gop_remaining_frames: Bool32,
        pub gop_remaining_i: u32,
        pub gop_remaining_p: u32,
        pub gop_remaining_b: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264GopRemainingFrameInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264GopRemainingFrameInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("use_gop_remaining_frames", &self.use_gop_remaining_frames)
                .field("gop_remaining_i", &self.gop_remaining_i)
                .field("gop_remaining_p", &self.gop_remaining_p)
                .field("gop_remaining_b", &self.gop_remaining_b)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264GopRemainingFrameInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoBeginCodingInfoKHR<'a>>
        for VideoEncodeH264GopRemainingFrameInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeH264GopRemainingFrameInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                use_gop_remaining_frames: Default::default(),
                gop_remaining_i: Default::default(),
                gop_remaining_p: Default::default(),
                gop_remaining_b: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264GopRemainingFrameInfoKHR<'a> {
        #[inline]
        pub fn use_gop_remaining_frames(mut self, use_gop_remaining_frames: bool) -> Self {
            self.use_gop_remaining_frames = use_gop_remaining_frames.into();
            self
        }

        #[inline]
        pub fn gop_remaining_i(mut self, gop_remaining_i: u32) -> Self {
            self.gop_remaining_i = gop_remaining_i;
            self
        }

        #[inline]
        pub fn gop_remaining_p(mut self, gop_remaining_p: u32) -> Self {
            self.gop_remaining_p = gop_remaining_p;
            self
        }

        #[inline]
        pub fn gop_remaining_b(mut self, gop_remaining_b: u32) -> Self {
            self.gop_remaining_b = gop_remaining_b;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264RateControlLayerInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct VideoEncodeH264RateControlLayerInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_min_qp: Bool32,
        pub min_qp: VideoEncodeH264QpKHR,
        pub use_max_qp: Bool32,
        pub max_qp: VideoEncodeH264QpKHR,
        pub use_max_frame_size: Bool32,
        pub max_frame_size: VideoEncodeH264FrameSizeKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for VideoEncodeH264RateControlLayerInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("VideoEncodeH264RateControlLayerInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("use_min_qp", &self.use_min_qp)
                .field("min_qp", &self.min_qp)
                .field("use_max_qp", &self.use_max_qp)
                .field("max_qp", &self.max_qp)
                .field("use_max_frame_size", &self.use_max_frame_size)
                .field("max_frame_size", &self.max_frame_size)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH264RateControlLayerInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR;
    }

    unsafe impl<'a> Extends<VideoEncodeRateControlLayerInfoKHR<'a>>
        for VideoEncodeH264RateControlLayerInfoKHR<'a>
    {
    }

    impl Default for VideoEncodeH264RateControlLayerInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                use_min_qp: Default::default(),
                min_qp: Default::default(),
                use_max_qp: Default::default(),
                max_qp: Default::default(),
                use_max_frame_size: Default::default(),
                max_frame_size: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> VideoEncodeH264RateControlLayerInfoKHR<'a> {
        #[inline]
        pub fn use_min_qp(mut self, use_min_qp: bool) -> Self {
            self.use_min_qp = use_min_qp.into();
            self
        }

        #[inline]
        pub fn min_qp(mut self, min_qp: VideoEncodeH264QpKHR) -> Self {
            self.min_qp = min_qp;
            self
        }

        #[inline]
        pub fn use_max_qp(mut self, use_max_qp: bool) -> Self {
            self.use_max_qp = use_max_qp.into();
            self
        }

        #[inline]
        pub fn max_qp(mut self, max_qp: VideoEncodeH264QpKHR) -> Self {
            self.max_qp = max_qp;
            self
        }

        #[inline]
        pub fn use_max_frame_size(mut self, use_max_frame_size: bool) -> Self {
            self.use_max_frame_size = use_max_frame_size.into();
            self
        }

        #[inline]
        pub fn max_frame_size(mut self, max_frame_size: VideoEncodeH264FrameSizeKHR) -> Self {
            self.max_frame_size = max_frame_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264CapabilityFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH264CapabilityFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeH264CapabilityFlagsKHR, Flags);

    impl VideoEncodeH264CapabilityFlagsKHR {
        pub const HRD_COMPLIANCE_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::HRD_COMPLIANCE_KHR.0);
        pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::PREDICTION_WEIGHT_TABLE_GENERATED_KHR.0);
        pub const ROW_UNALIGNED_SLICE_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::ROW_UNALIGNED_SLICE_KHR.0);
        pub const DIFFERENT_SLICE_TYPE_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::DIFFERENT_SLICE_TYPE_KHR.0);
        pub const B_FRAME_IN_L0_LIST_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::B_FRAME_IN_L0_LIST_KHR.0);
        pub const B_FRAME_IN_L1_LIST_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::B_FRAME_IN_L1_LIST_KHR.0);
        pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::PER_PICTURE_TYPE_MIN_MAX_QP_KHR.0);
        pub const PER_SLICE_CONSTANT_QP_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::PER_SLICE_CONSTANT_QP_KHR.0);
        pub const GENERATE_PREFIX_NALU_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::GENERATE_PREFIX_NALU_KHR.0);

        // VK_KHR_video_encode_intra_refresh
        pub const B_PICTURE_INTRA_REFRESH_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::B_PICTURE_INTRA_REFRESH_KHR.0);

        // VK_KHR_video_encode_quantization_map
        pub const MB_QP_DIFF_WRAPAROUND_KHR: Self =
            Self(VideoEncodeH264CapabilityFlagBitsKHR::MB_QP_DIFF_WRAPAROUND_KHR.0);
    }

    impl fmt::Debug for VideoEncodeH264CapabilityFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeH264CapabilityFlagsKHR::HRD_COMPLIANCE_KHR.0,
                    "HRD_COMPLIANCE_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::PREDICTION_WEIGHT_TABLE_GENERATED_KHR.0,
                    "PREDICTION_WEIGHT_TABLE_GENERATED_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::ROW_UNALIGNED_SLICE_KHR.0,
                    "ROW_UNALIGNED_SLICE_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::DIFFERENT_SLICE_TYPE_KHR.0,
                    "DIFFERENT_SLICE_TYPE_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::B_FRAME_IN_L0_LIST_KHR.0,
                    "B_FRAME_IN_L0_LIST_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::B_FRAME_IN_L1_LIST_KHR.0,
                    "B_FRAME_IN_L1_LIST_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::PER_PICTURE_TYPE_MIN_MAX_QP_KHR.0,
                    "PER_PICTURE_TYPE_MIN_MAX_QP_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::PER_SLICE_CONSTANT_QP_KHR.0,
                    "PER_SLICE_CONSTANT_QP_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::GENERATE_PREFIX_NALU_KHR.0,
                    "GENERATE_PREFIX_NALU_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::B_PICTURE_INTRA_REFRESH_KHR.0,
                    "B_PICTURE_INTRA_REFRESH_KHR",
                ),
                (
                    VideoEncodeH264CapabilityFlagsKHR::MB_QP_DIFF_WRAPAROUND_KHR.0,
                    "MB_QP_DIFF_WRAPAROUND_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264CapabilityFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH264CapabilityFlagBitsKHR(u32);

    impl VideoEncodeH264CapabilityFlagBitsKHR {
        pub const HRD_COMPLIANCE_KHR: Self = Self(1 << 0);
        pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self = Self(1 << 1);
        pub const ROW_UNALIGNED_SLICE_KHR: Self = Self(1 << 2);
        pub const DIFFERENT_SLICE_TYPE_KHR: Self = Self(1 << 3);
        pub const B_FRAME_IN_L0_LIST_KHR: Self = Self(1 << 4);
        pub const B_FRAME_IN_L1_LIST_KHR: Self = Self(1 << 5);
        pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self = Self(1 << 6);
        pub const PER_SLICE_CONSTANT_QP_KHR: Self = Self(1 << 7);
        pub const GENERATE_PREFIX_NALU_KHR: Self = Self(1 << 8);
        // VK_KHR_video_encode_intra_refresh
        pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(1 << 10);

        // VK_KHR_video_encode_quantization_map
        pub const MB_QP_DIFF_WRAPAROUND_KHR: Self = Self(1 << 9);
    }

    impl fmt::Debug for VideoEncodeH264CapabilityFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::HRD_COMPLIANCE_KHR => Some("HRD_COMPLIANCE_KHR"),
                Self::PREDICTION_WEIGHT_TABLE_GENERATED_KHR => {
                    Some("PREDICTION_WEIGHT_TABLE_GENERATED_KHR")
                }
                Self::ROW_UNALIGNED_SLICE_KHR => Some("ROW_UNALIGNED_SLICE_KHR"),
                Self::DIFFERENT_SLICE_TYPE_KHR => Some("DIFFERENT_SLICE_TYPE_KHR"),
                Self::B_FRAME_IN_L0_LIST_KHR => Some("B_FRAME_IN_L0_LIST_KHR"),
                Self::B_FRAME_IN_L1_LIST_KHR => Some("B_FRAME_IN_L1_LIST_KHR"),
                Self::PER_PICTURE_TYPE_MIN_MAX_QP_KHR => Some("PER_PICTURE_TYPE_MIN_MAX_QP_KHR"),
                Self::PER_SLICE_CONSTANT_QP_KHR => Some("PER_SLICE_CONSTANT_QP_KHR"),
                Self::GENERATE_PREFIX_NALU_KHR => Some("GENERATE_PREFIX_NALU_KHR"),
                Self::B_PICTURE_INTRA_REFRESH_KHR => Some("B_PICTURE_INTRA_REFRESH_KHR"),
                Self::MB_QP_DIFF_WRAPAROUND_KHR => Some("MB_QP_DIFF_WRAPAROUND_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264StdFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH264StdFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeH264StdFlagsKHR, Flags);

    impl VideoEncodeH264StdFlagsKHR {
        pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::SEPARATE_COLOR_PLANE_FLAG_SET_KHR.0);
        pub const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR.0);
        pub const SCALING_MATRIX_PRESENT_FLAG_SET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::SCALING_MATRIX_PRESENT_FLAG_SET_KHR.0);
        pub const CHROMA_QP_INDEX_OFFSET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::CHROMA_QP_INDEX_OFFSET_KHR.0);
        pub const SECOND_CHROMA_QP_INDEX_OFFSET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::SECOND_CHROMA_QP_INDEX_OFFSET_KHR.0);
        pub const PIC_INIT_QP_MINUS26_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::PIC_INIT_QP_MINUS26_KHR.0);
        pub const WEIGHTED_PRED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::WEIGHTED_PRED_FLAG_SET_KHR.0);
        pub const WEIGHTED_BIPRED_IDC_EXPLICIT_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::WEIGHTED_BIPRED_IDC_EXPLICIT_KHR.0);
        pub const WEIGHTED_BIPRED_IDC_IMPLICIT_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::WEIGHTED_BIPRED_IDC_IMPLICIT_KHR.0);
        pub const TRANSFORM_8X8_MODE_FLAG_SET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::TRANSFORM_8X8_MODE_FLAG_SET_KHR.0);
        pub const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR.0);
        pub const ENTROPY_CODING_MODE_FLAG_UNSET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::ENTROPY_CODING_MODE_FLAG_UNSET_KHR.0);
        pub const ENTROPY_CODING_MODE_FLAG_SET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::ENTROPY_CODING_MODE_FLAG_SET_KHR.0);
        pub const DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR.0);
        pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::CONSTRAINED_INTRA_PRED_FLAG_SET_KHR.0);
        pub const DEBLOCKING_FILTER_DISABLED_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::DEBLOCKING_FILTER_DISABLED_KHR.0);
        pub const DEBLOCKING_FILTER_ENABLED_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::DEBLOCKING_FILTER_ENABLED_KHR.0);
        pub const DEBLOCKING_FILTER_PARTIAL_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::DEBLOCKING_FILTER_PARTIAL_KHR.0);
        pub const SLICE_QP_DELTA_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::SLICE_QP_DELTA_KHR.0);
        pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self =
            Self(VideoEncodeH264StdFlagBitsKHR::DIFFERENT_SLICE_QP_DELTA_KHR.0);
    }

    impl fmt::Debug for VideoEncodeH264StdFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeH264StdFlagsKHR::SEPARATE_COLOR_PLANE_FLAG_SET_KHR.0,
                    "SEPARATE_COLOR_PLANE_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR.0,
                    "QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::SCALING_MATRIX_PRESENT_FLAG_SET_KHR.0,
                    "SCALING_MATRIX_PRESENT_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::CHROMA_QP_INDEX_OFFSET_KHR.0,
                    "CHROMA_QP_INDEX_OFFSET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::SECOND_CHROMA_QP_INDEX_OFFSET_KHR.0,
                    "SECOND_CHROMA_QP_INDEX_OFFSET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::PIC_INIT_QP_MINUS26_KHR.0,
                    "PIC_INIT_QP_MINUS26_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::WEIGHTED_PRED_FLAG_SET_KHR.0,
                    "WEIGHTED_PRED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::WEIGHTED_BIPRED_IDC_EXPLICIT_KHR.0,
                    "WEIGHTED_BIPRED_IDC_EXPLICIT_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::WEIGHTED_BIPRED_IDC_IMPLICIT_KHR.0,
                    "WEIGHTED_BIPRED_IDC_IMPLICIT_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::TRANSFORM_8X8_MODE_FLAG_SET_KHR.0,
                    "TRANSFORM_8X8_MODE_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR.0,
                    "DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::ENTROPY_CODING_MODE_FLAG_UNSET_KHR.0,
                    "ENTROPY_CODING_MODE_FLAG_UNSET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::ENTROPY_CODING_MODE_FLAG_SET_KHR.0,
                    "ENTROPY_CODING_MODE_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR.0,
                    "DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::CONSTRAINED_INTRA_PRED_FLAG_SET_KHR.0,
                    "CONSTRAINED_INTRA_PRED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::DEBLOCKING_FILTER_DISABLED_KHR.0,
                    "DEBLOCKING_FILTER_DISABLED_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::DEBLOCKING_FILTER_ENABLED_KHR.0,
                    "DEBLOCKING_FILTER_ENABLED_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::DEBLOCKING_FILTER_PARTIAL_KHR.0,
                    "DEBLOCKING_FILTER_PARTIAL_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::SLICE_QP_DELTA_KHR.0,
                    "SLICE_QP_DELTA_KHR",
                ),
                (
                    VideoEncodeH264StdFlagsKHR::DIFFERENT_SLICE_QP_DELTA_KHR.0,
                    "DIFFERENT_SLICE_QP_DELTA_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264StdFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH264StdFlagBitsKHR(u32);

    impl VideoEncodeH264StdFlagBitsKHR {
        pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self = Self(1 << 0);
        pub const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR: Self = Self(1 << 1);
        pub const SCALING_MATRIX_PRESENT_FLAG_SET_KHR: Self = Self(1 << 2);
        pub const CHROMA_QP_INDEX_OFFSET_KHR: Self = Self(1 << 3);
        pub const SECOND_CHROMA_QP_INDEX_OFFSET_KHR: Self = Self(1 << 4);
        pub const PIC_INIT_QP_MINUS26_KHR: Self = Self(1 << 5);
        pub const WEIGHTED_PRED_FLAG_SET_KHR: Self = Self(1 << 6);
        pub const WEIGHTED_BIPRED_IDC_EXPLICIT_KHR: Self = Self(1 << 7);
        pub const WEIGHTED_BIPRED_IDC_IMPLICIT_KHR: Self = Self(1 << 8);
        pub const TRANSFORM_8X8_MODE_FLAG_SET_KHR: Self = Self(1 << 9);
        pub const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR: Self = Self(1 << 10);
        pub const ENTROPY_CODING_MODE_FLAG_UNSET_KHR: Self = Self(1 << 11);
        pub const ENTROPY_CODING_MODE_FLAG_SET_KHR: Self = Self(1 << 12);
        pub const DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR: Self = Self(1 << 13);
        pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self = Self(1 << 14);
        pub const DEBLOCKING_FILTER_DISABLED_KHR: Self = Self(1 << 15);
        pub const DEBLOCKING_FILTER_ENABLED_KHR: Self = Self(1 << 16);
        pub const DEBLOCKING_FILTER_PARTIAL_KHR: Self = Self(1 << 17);
        pub const SLICE_QP_DELTA_KHR: Self = Self(1 << 19);
        pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self = Self(1 << 20);
    }

    impl fmt::Debug for VideoEncodeH264StdFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SEPARATE_COLOR_PLANE_FLAG_SET_KHR => {
                    Some("SEPARATE_COLOR_PLANE_FLAG_SET_KHR")
                }
                Self::QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR => {
                    Some("QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR")
                }
                Self::SCALING_MATRIX_PRESENT_FLAG_SET_KHR => {
                    Some("SCALING_MATRIX_PRESENT_FLAG_SET_KHR")
                }
                Self::CHROMA_QP_INDEX_OFFSET_KHR => Some("CHROMA_QP_INDEX_OFFSET_KHR"),
                Self::SECOND_CHROMA_QP_INDEX_OFFSET_KHR => {
                    Some("SECOND_CHROMA_QP_INDEX_OFFSET_KHR")
                }
                Self::PIC_INIT_QP_MINUS26_KHR => Some("PIC_INIT_QP_MINUS26_KHR"),
                Self::WEIGHTED_PRED_FLAG_SET_KHR => Some("WEIGHTED_PRED_FLAG_SET_KHR"),
                Self::WEIGHTED_BIPRED_IDC_EXPLICIT_KHR => Some("WEIGHTED_BIPRED_IDC_EXPLICIT_KHR"),
                Self::WEIGHTED_BIPRED_IDC_IMPLICIT_KHR => Some("WEIGHTED_BIPRED_IDC_IMPLICIT_KHR"),
                Self::TRANSFORM_8X8_MODE_FLAG_SET_KHR => Some("TRANSFORM_8X8_MODE_FLAG_SET_KHR"),
                Self::DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR => {
                    Some("DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR")
                }
                Self::ENTROPY_CODING_MODE_FLAG_UNSET_KHR => {
                    Some("ENTROPY_CODING_MODE_FLAG_UNSET_KHR")
                }
                Self::ENTROPY_CODING_MODE_FLAG_SET_KHR => Some("ENTROPY_CODING_MODE_FLAG_SET_KHR"),
                Self::DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR => {
                    Some("DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR")
                }
                Self::CONSTRAINED_INTRA_PRED_FLAG_SET_KHR => {
                    Some("CONSTRAINED_INTRA_PRED_FLAG_SET_KHR")
                }
                Self::DEBLOCKING_FILTER_DISABLED_KHR => Some("DEBLOCKING_FILTER_DISABLED_KHR"),
                Self::DEBLOCKING_FILTER_ENABLED_KHR => Some("DEBLOCKING_FILTER_ENABLED_KHR"),
                Self::DEBLOCKING_FILTER_PARTIAL_KHR => Some("DEBLOCKING_FILTER_PARTIAL_KHR"),
                Self::SLICE_QP_DELTA_KHR => Some("SLICE_QP_DELTA_KHR"),
                Self::DIFFERENT_SLICE_QP_DELTA_KHR => Some("DIFFERENT_SLICE_QP_DELTA_KHR"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264RateControlFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH264RateControlFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeH264RateControlFlagsKHR, Flags);

    impl VideoEncodeH264RateControlFlagsKHR {
        pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self =
            Self(VideoEncodeH264RateControlFlagBitsKHR::ATTEMPT_HRD_COMPLIANCE_KHR.0);
        pub const REGULAR_GOP_KHR: Self =
            Self(VideoEncodeH264RateControlFlagBitsKHR::REGULAR_GOP_KHR.0);
        pub const REFERENCE_PATTERN_FLAT_KHR: Self =
            Self(VideoEncodeH264RateControlFlagBitsKHR::REFERENCE_PATTERN_FLAT_KHR.0);
        pub const REFERENCE_PATTERN_DYADIC_KHR: Self =
            Self(VideoEncodeH264RateControlFlagBitsKHR::REFERENCE_PATTERN_DYADIC_KHR.0);
        pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self =
            Self(VideoEncodeH264RateControlFlagBitsKHR::TEMPORAL_LAYER_PATTERN_DYADIC_KHR.0);
    }

    impl fmt::Debug for VideoEncodeH264RateControlFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeH264RateControlFlagsKHR::ATTEMPT_HRD_COMPLIANCE_KHR.0,
                    "ATTEMPT_HRD_COMPLIANCE_KHR",
                ),
                (
                    VideoEncodeH264RateControlFlagsKHR::REGULAR_GOP_KHR.0,
                    "REGULAR_GOP_KHR",
                ),
                (
                    VideoEncodeH264RateControlFlagsKHR::REFERENCE_PATTERN_FLAT_KHR.0,
                    "REFERENCE_PATTERN_FLAT_KHR",
                ),
                (
                    VideoEncodeH264RateControlFlagsKHR::REFERENCE_PATTERN_DYADIC_KHR.0,
                    "REFERENCE_PATTERN_DYADIC_KHR",
                ),
                (
                    VideoEncodeH264RateControlFlagsKHR::TEMPORAL_LAYER_PATTERN_DYADIC_KHR.0,
                    "TEMPORAL_LAYER_PATTERN_DYADIC_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH264RateControlFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH264RateControlFlagBitsKHR(u32);

    impl VideoEncodeH264RateControlFlagBitsKHR {
        pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1 << 0);
        pub const REGULAR_GOP_KHR: Self = Self(1 << 1);
        pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(1 << 2);
        pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(1 << 3);
        pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(1 << 4);
    }

    impl fmt::Debug for VideoEncodeH264RateControlFlagBitsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ATTEMPT_HRD_COMPLIANCE_KHR => Some("ATTEMPT_HRD_COMPLIANCE_KHR"),
                Self::REGULAR_GOP_KHR => Some("REGULAR_GOP_KHR"),
                Self::REFERENCE_PATTERN_FLAT_KHR => Some("REFERENCE_PATTERN_FLAT_KHR"),
                Self::REFERENCE_PATTERN_DYADIC_KHR => Some("REFERENCE_PATTERN_DYADIC_KHR"),
                Self::TEMPORAL_LAYER_PATTERN_DYADIC_KHR => {
                    Some("TEMPORAL_LAYER_PATTERN_DYADIC_KHR")
                }
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

    pub type VkVideoEncodeH264CapabilitiesKHR = VideoEncodeH264CapabilitiesKHR<'static>;
    pub type VkVideoEncodeH264QualityLevelPropertiesKHR =
        VideoEncodeH264QualityLevelPropertiesKHR<'static>;
    pub type VkVideoEncodeH264SessionCreateInfoKHR = VideoEncodeH264SessionCreateInfoKHR<'static>;
    pub type VkVideoEncodeH264SessionParametersAddInfoKHR =
        VideoEncodeH264SessionParametersAddInfoKHR<'static>;
    pub type VkVideoEncodeH264SessionParametersCreateInfoKHR =
        VideoEncodeH264SessionParametersCreateInfoKHR<'static>;
    pub type VkVideoEncodeH264SessionParametersGetInfoKHR =
        VideoEncodeH264SessionParametersGetInfoKHR<'static>;
    pub type VkVideoEncodeH264SessionParametersFeedbackInfoKHR =
        VideoEncodeH264SessionParametersFeedbackInfoKHR<'static>;
    pub type VkVideoEncodeH264DpbSlotInfoKHR = VideoEncodeH264DpbSlotInfoKHR<'static>;
    pub type VkVideoEncodeH264PictureInfoKHR = VideoEncodeH264PictureInfoKHR<'static>;
    pub type VkVideoEncodeH264ProfileInfoKHR = VideoEncodeH264ProfileInfoKHR<'static>;
    pub type VkVideoEncodeH264NaluSliceInfoKHR = VideoEncodeH264NaluSliceInfoKHR<'static>;
    pub type VkVideoEncodeH264RateControlInfoKHR = VideoEncodeH264RateControlInfoKHR<'static>;
    pub type VkVideoEncodeH264QpKHR = VideoEncodeH264QpKHR;
    pub type VkVideoEncodeH264FrameSizeKHR = VideoEncodeH264FrameSizeKHR;
    pub type VkVideoEncodeH264GopRemainingFrameInfoKHR =
        VideoEncodeH264GopRemainingFrameInfoKHR<'static>;
    pub type VkVideoEncodeH264RateControlLayerInfoKHR =
        VideoEncodeH264RateControlLayerInfoKHR<'static>;
    pub type VkVideoEncodeH264CapabilityFlagsKHR = VideoEncodeH264CapabilityFlagsKHR;
    pub type VkVideoEncodeH264CapabilityFlagBitsKHR = VideoEncodeH264CapabilityFlagBitsKHR;
    pub type VkVideoEncodeH264StdFlagsKHR = VideoEncodeH264StdFlagsKHR;
    pub type VkVideoEncodeH264StdFlagBitsKHR = VideoEncodeH264StdFlagBitsKHR;
    pub type VkVideoEncodeH264RateControlFlagsKHR = VideoEncodeH264RateControlFlagsKHR;
    pub type VkVideoEncodeH264RateControlFlagBitsKHR = VideoEncodeH264RateControlFlagBitsKHR;
    impl VideoEncodeH264CapabilitiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264CapabilitiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264QualityLevelPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264QualityLevelPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264SessionCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264SessionCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264SessionParametersAddInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeH264SessionParametersAddInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264SessionParametersCreateInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeH264SessionParametersCreateInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264SessionParametersGetInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeH264SessionParametersGetInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264SessionParametersFeedbackInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkVideoEncodeH264SessionParametersFeedbackInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264DpbSlotInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264DpbSlotInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264PictureInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264PictureInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264ProfileInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264ProfileInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264NaluSliceInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264NaluSliceInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264RateControlInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264RateControlInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264GopRemainingFrameInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264GopRemainingFrameInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl VideoEncodeH264RateControlLayerInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkVideoEncodeH264RateControlLayerInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
