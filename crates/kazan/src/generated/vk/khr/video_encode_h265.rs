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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265CapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265CapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub flags: VideoEncodeH265CapabilityFlagsKHR,
        pub max_level_idc: StdVideoH265LevelIdc,
        pub max_slice_segment_count: u32,
        pub max_tiles: Extent2D,
        pub ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
        pub transform_block_sizes: VideoEncodeH265TransformBlockSizeFlagsKHR,
        pub max_p_picture_l0_reference_count: u32,
        pub max_b_picture_l0_reference_count: u32,
        pub max_l1_reference_count: u32,
        pub max_sub_layer_count: u32,
        pub expect_dyadic_temporal_sub_layer_pattern: Bool32,
        pub min_qp: i32,
        pub max_qp: i32,
        pub prefers_gop_remaining_frames: Bool32,
        pub requires_gop_remaining_frames: Bool32,
        pub std_syntax_flags: VideoEncodeH265StdFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265CapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H265_CAPABILITIES_KHR;
    }
    unsafe impl<'a> Extends<VideoCapabilitiesKHR<'a>> for VideoEncodeH265CapabilitiesKHR<'a> {}
    impl Default for VideoEncodeH265CapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                flags: Default::default(),
                max_level_idc: Default::default(),
                max_slice_segment_count: Default::default(),
                max_tiles: Default::default(),
                ctb_sizes: Default::default(),
                transform_block_sizes: Default::default(),
                max_p_picture_l0_reference_count: Default::default(),
                max_b_picture_l0_reference_count: Default::default(),
                max_l1_reference_count: Default::default(),
                max_sub_layer_count: Default::default(),
                expect_dyadic_temporal_sub_layer_pattern: Default::default(),
                min_qp: Default::default(),
                max_qp: Default::default(),
                prefers_gop_remaining_frames: Default::default(),
                requires_gop_remaining_frames: Default::default(),
                std_syntax_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265CapabilitiesKHR<'a> {
        pub fn flags(mut self, flags: VideoEncodeH265CapabilityFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn max_level_idc(mut self, max_level_idc: StdVideoH265LevelIdc) -> Self {
            self.max_level_idc = max_level_idc;
            self
        }
        pub fn max_slice_segment_count(mut self, max_slice_segment_count: u32) -> Self {
            self.max_slice_segment_count = max_slice_segment_count;
            self
        }
        pub fn max_tiles(mut self, max_tiles: Extent2D) -> Self {
            self.max_tiles = max_tiles;
            self
        }
        pub fn ctb_sizes(mut self, ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR) -> Self {
            self.ctb_sizes = ctb_sizes;
            self
        }
        pub fn transform_block_sizes(
            mut self,
            transform_block_sizes: VideoEncodeH265TransformBlockSizeFlagsKHR,
        ) -> Self {
            self.transform_block_sizes = transform_block_sizes;
            self
        }
        pub fn max_p_picture_l0_reference_count(
            mut self,
            max_p_picture_l0_reference_count: u32,
        ) -> Self {
            self.max_p_picture_l0_reference_count = max_p_picture_l0_reference_count;
            self
        }
        pub fn max_b_picture_l0_reference_count(
            mut self,
            max_b_picture_l0_reference_count: u32,
        ) -> Self {
            self.max_b_picture_l0_reference_count = max_b_picture_l0_reference_count;
            self
        }
        pub fn max_l1_reference_count(mut self, max_l1_reference_count: u32) -> Self {
            self.max_l1_reference_count = max_l1_reference_count;
            self
        }
        pub fn max_sub_layer_count(mut self, max_sub_layer_count: u32) -> Self {
            self.max_sub_layer_count = max_sub_layer_count;
            self
        }
        pub fn expect_dyadic_temporal_sub_layer_pattern(
            mut self,
            expect_dyadic_temporal_sub_layer_pattern: bool,
        ) -> Self {
            self.expect_dyadic_temporal_sub_layer_pattern =
                expect_dyadic_temporal_sub_layer_pattern.into();
            self
        }
        pub fn min_qp(mut self, min_qp: i32) -> Self {
            self.min_qp = min_qp;
            self
        }
        pub fn max_qp(mut self, max_qp: i32) -> Self {
            self.max_qp = max_qp;
            self
        }
        pub fn prefers_gop_remaining_frames(mut self, prefers_gop_remaining_frames: bool) -> Self {
            self.prefers_gop_remaining_frames = prefers_gop_remaining_frames.into();
            self
        }
        pub fn requires_gop_remaining_frames(
            mut self,
            requires_gop_remaining_frames: bool,
        ) -> Self {
            self.requires_gop_remaining_frames = requires_gop_remaining_frames.into();
            self
        }
        pub fn std_syntax_flags(mut self, std_syntax_flags: VideoEncodeH265StdFlagsKHR) -> Self {
            self.std_syntax_flags = std_syntax_flags;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265QualityLevelPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265QualityLevelPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub preferred_rate_control_flags: VideoEncodeH265RateControlFlagsKHR,
        pub preferred_gop_frame_count: u32,
        pub preferred_idr_period: u32,
        pub preferred_consecutive_b_frame_count: u32,
        pub preferred_sub_layer_count: u32,
        pub preferred_constant_qp: VideoEncodeH265QpKHR,
        pub preferred_max_l0_reference_count: u32,
        pub preferred_max_l1_reference_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265QualityLevelPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_QUALITY_LEVEL_PROPERTIES_KHR;
    }
    unsafe impl<'a> Extends<VideoEncodeQualityLevelPropertiesKHR<'a>>
        for VideoEncodeH265QualityLevelPropertiesKHR<'a>
    {
    }
    impl Default for VideoEncodeH265QualityLevelPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                preferred_rate_control_flags: Default::default(),
                preferred_gop_frame_count: Default::default(),
                preferred_idr_period: Default::default(),
                preferred_consecutive_b_frame_count: Default::default(),
                preferred_sub_layer_count: Default::default(),
                preferred_constant_qp: Default::default(),
                preferred_max_l0_reference_count: Default::default(),
                preferred_max_l1_reference_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265QualityLevelPropertiesKHR<'a> {
        pub fn preferred_rate_control_flags(
            mut self,
            preferred_rate_control_flags: VideoEncodeH265RateControlFlagsKHR,
        ) -> Self {
            self.preferred_rate_control_flags = preferred_rate_control_flags;
            self
        }
        pub fn preferred_gop_frame_count(mut self, preferred_gop_frame_count: u32) -> Self {
            self.preferred_gop_frame_count = preferred_gop_frame_count;
            self
        }
        pub fn preferred_idr_period(mut self, preferred_idr_period: u32) -> Self {
            self.preferred_idr_period = preferred_idr_period;
            self
        }
        pub fn preferred_consecutive_b_frame_count(
            mut self,
            preferred_consecutive_b_frame_count: u32,
        ) -> Self {
            self.preferred_consecutive_b_frame_count = preferred_consecutive_b_frame_count;
            self
        }
        pub fn preferred_sub_layer_count(mut self, preferred_sub_layer_count: u32) -> Self {
            self.preferred_sub_layer_count = preferred_sub_layer_count;
            self
        }
        pub fn preferred_constant_qp(
            mut self,
            preferred_constant_qp: VideoEncodeH265QpKHR,
        ) -> Self {
            self.preferred_constant_qp = preferred_constant_qp;
            self
        }
        pub fn preferred_max_l0_reference_count(
            mut self,
            preferred_max_l0_reference_count: u32,
        ) -> Self {
            self.preferred_max_l0_reference_count = preferred_max_l0_reference_count;
            self
        }
        pub fn preferred_max_l1_reference_count(
            mut self,
            preferred_max_l1_reference_count: u32,
        ) -> Self {
            self.preferred_max_l1_reference_count = preferred_max_l1_reference_count;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265SessionCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265SessionCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_max_level_idc: Bool32,
        pub max_level_idc: StdVideoH265LevelIdc,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265SessionCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_SESSION_CREATE_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoSessionCreateInfoKHR<'a>> for VideoEncodeH265SessionCreateInfoKHR<'a> {}
    impl Default for VideoEncodeH265SessionCreateInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                use_max_level_idc: Default::default(),
                max_level_idc: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265SessionCreateInfoKHR<'a> {
        pub fn use_max_level_idc(mut self, use_max_level_idc: bool) -> Self {
            self.use_max_level_idc = use_max_level_idc.into();
            self
        }
        pub fn max_level_idc(mut self, max_level_idc: StdVideoH265LevelIdc) -> Self {
            self.max_level_idc = max_level_idc;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265SessionParametersAddInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265SessionParametersAddInfoKHR<'a> {
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
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265SessionParametersAddInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_ADD_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoSessionParametersUpdateInfoKHR<'a>>
        for VideoEncodeH265SessionParametersAddInfoKHR<'a>
    {
    }
    impl Default for VideoEncodeH265SessionParametersAddInfoKHR<'_> {
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
    impl<'a> VideoEncodeH265SessionParametersAddInfoKHR<'a> {
        pub fn std_vp_ss(mut self, std_vp_ss: &'a [StdVideoH265VideoParameterSet<'a>]) -> Self {
            self.std_vps_count = std_vp_ss.len().try_into().unwrap();
            self.p_std_vp_ss = std_vp_ss.as_ptr();
            self
        }
        pub fn std_sp_ss(mut self, std_sp_ss: &'a [StdVideoH265SequenceParameterSet<'a>]) -> Self {
            self.std_sps_count = std_sp_ss.len().try_into().unwrap();
            self.p_std_sp_ss = std_sp_ss.as_ptr();
            self
        }
        pub fn std_pp_ss(mut self, std_pp_ss: &'a [StdVideoH265PictureParameterSet<'a>]) -> Self {
            self.std_pps_count = std_pp_ss.len().try_into().unwrap();
            self.p_std_pp_ss = std_pp_ss.as_ptr();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265SessionParametersCreateInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265SessionParametersCreateInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub max_std_vps_count: u32,
        pub max_std_sps_count: u32,
        pub max_std_pps_count: u32,
        pub p_parameters_add_info: *const VideoEncodeH265SessionParametersAddInfoKHR<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265SessionParametersCreateInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_CREATE_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoSessionParametersCreateInfoKHR<'a>>
        for VideoEncodeH265SessionParametersCreateInfoKHR<'a>
    {
    }
    impl Default for VideoEncodeH265SessionParametersCreateInfoKHR<'_> {
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
    impl<'a> VideoEncodeH265SessionParametersCreateInfoKHR<'a> {
        pub fn max_std_vps_count(mut self, max_std_vps_count: u32) -> Self {
            self.max_std_vps_count = max_std_vps_count;
            self
        }
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
            parameters_add_info: &'a VideoEncodeH265SessionParametersAddInfoKHR<'a>,
        ) -> Self {
            self.p_parameters_add_info = parameters_add_info;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265SessionParametersGetInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265SessionParametersGetInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub write_std_vps: Bool32,
        pub write_std_sps: Bool32,
        pub write_std_pps: Bool32,
        pub std_vps_id: u32,
        pub std_sps_id: u32,
        pub std_pps_id: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265SessionParametersGetInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_GET_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoEncodeSessionParametersGetInfoKHR<'a>>
        for VideoEncodeH265SessionParametersGetInfoKHR<'a>
    {
    }
    impl Default for VideoEncodeH265SessionParametersGetInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                write_std_vps: Default::default(),
                write_std_sps: Default::default(),
                write_std_pps: Default::default(),
                std_vps_id: Default::default(),
                std_sps_id: Default::default(),
                std_pps_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265SessionParametersGetInfoKHR<'a> {
        pub fn write_std_vps(mut self, write_std_vps: bool) -> Self {
            self.write_std_vps = write_std_vps.into();
            self
        }
        pub fn write_std_sps(mut self, write_std_sps: bool) -> Self {
            self.write_std_sps = write_std_sps.into();
            self
        }
        pub fn write_std_pps(mut self, write_std_pps: bool) -> Self {
            self.write_std_pps = write_std_pps.into();
            self
        }
        pub fn std_vps_id(mut self, std_vps_id: u32) -> Self {
            self.std_vps_id = std_vps_id;
            self
        }
        pub fn std_sps_id(mut self, std_sps_id: u32) -> Self {
            self.std_sps_id = std_sps_id;
            self
        }
        pub fn std_pps_id(mut self, std_pps_id: u32) -> Self {
            self.std_pps_id = std_pps_id;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265SessionParametersFeedbackInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265SessionParametersFeedbackInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub has_std_vps_overrides: Bool32,
        pub has_std_sps_overrides: Bool32,
        pub has_std_pps_overrides: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265SessionParametersFeedbackInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_SESSION_PARAMETERS_FEEDBACK_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoEncodeSessionParametersFeedbackInfoKHR<'a>>
        for VideoEncodeH265SessionParametersFeedbackInfoKHR<'a>
    {
    }
    impl Default for VideoEncodeH265SessionParametersFeedbackInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                has_std_vps_overrides: Default::default(),
                has_std_sps_overrides: Default::default(),
                has_std_pps_overrides: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265SessionParametersFeedbackInfoKHR<'a> {
        pub fn has_std_vps_overrides(mut self, has_std_vps_overrides: bool) -> Self {
            self.has_std_vps_overrides = has_std_vps_overrides.into();
            self
        }
        pub fn has_std_sps_overrides(mut self, has_std_sps_overrides: bool) -> Self {
            self.has_std_sps_overrides = has_std_sps_overrides.into();
            self
        }
        pub fn has_std_pps_overrides(mut self, has_std_pps_overrides: bool) -> Self {
            self.has_std_pps_overrides = has_std_pps_overrides.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265PictureInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265PictureInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub nalu_slice_segment_entry_count: u32,
        pub p_nalu_slice_segment_entries: *const VideoEncodeH265NaluSliceSegmentInfoKHR<'a>,
        pub p_std_picture_info: *const StdVideoEncodeH265PictureInfo<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265PictureInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H265_PICTURE_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoEncodeInfoKHR<'a>> for VideoEncodeH265PictureInfoKHR<'a> {}
    impl Default for VideoEncodeH265PictureInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                nalu_slice_segment_entry_count: Default::default(),
                p_nalu_slice_segment_entries: core::ptr::null(),
                p_std_picture_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265PictureInfoKHR<'a> {
        pub fn nalu_slice_segment_entries(
            mut self,
            nalu_slice_segment_entries: &'a [VideoEncodeH265NaluSliceSegmentInfoKHR<'a>],
        ) -> Self {
            self.nalu_slice_segment_entry_count =
                nalu_slice_segment_entries.len().try_into().unwrap();
            self.p_nalu_slice_segment_entries = nalu_slice_segment_entries.as_ptr();
            self
        }
        pub fn std_picture_info(
            mut self,
            std_picture_info: &'a StdVideoEncodeH265PictureInfo<'a>,
        ) -> Self {
            self.p_std_picture_info = std_picture_info;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265NaluSliceSegmentInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265NaluSliceSegmentInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub constant_qp: i32,
        pub p_std_slice_segment_header: *const StdVideoEncodeH265SliceSegmentHeader<'a>,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265NaluSliceSegmentInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_NALU_SLICE_SEGMENT_INFO_KHR;
    }
    impl Default for VideoEncodeH265NaluSliceSegmentInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                constant_qp: Default::default(),
                p_std_slice_segment_header: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265NaluSliceSegmentInfoKHR<'a> {
        pub fn constant_qp(mut self, constant_qp: i32) -> Self {
            self.constant_qp = constant_qp;
            self
        }
        pub fn std_slice_segment_header(
            mut self,
            std_slice_segment_header: &'a StdVideoEncodeH265SliceSegmentHeader<'a>,
        ) -> Self {
            self.p_std_slice_segment_header = std_slice_segment_header;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265RateControlInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265RateControlInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: VideoEncodeH265RateControlFlagsKHR,
        pub gop_frame_count: u32,
        pub idr_period: u32,
        pub consecutive_b_frame_count: u32,
        pub sub_layer_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265RateControlInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_RATE_CONTROL_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoCodingControlInfoKHR<'a>> for VideoEncodeH265RateControlInfoKHR<'a> {}
    unsafe impl<'a> Extends<VideoBeginCodingInfoKHR<'a>> for VideoEncodeH265RateControlInfoKHR<'a> {}
    impl Default for VideoEncodeH265RateControlInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                gop_frame_count: Default::default(),
                idr_period: Default::default(),
                consecutive_b_frame_count: Default::default(),
                sub_layer_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265RateControlInfoKHR<'a> {
        pub fn flags(mut self, flags: VideoEncodeH265RateControlFlagsKHR) -> Self {
            self.flags = flags;
            self
        }
        pub fn gop_frame_count(mut self, gop_frame_count: u32) -> Self {
            self.gop_frame_count = gop_frame_count;
            self
        }
        pub fn idr_period(mut self, idr_period: u32) -> Self {
            self.idr_period = idr_period;
            self
        }
        pub fn consecutive_b_frame_count(mut self, consecutive_b_frame_count: u32) -> Self {
            self.consecutive_b_frame_count = consecutive_b_frame_count;
            self
        }
        pub fn sub_layer_count(mut self, sub_layer_count: u32) -> Self {
            self.sub_layer_count = sub_layer_count;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265QpKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct VideoEncodeH265QpKHR {
        pub qp_i: i32,
        pub qp_p: i32,
        pub qp_b: i32,
    }
    impl VideoEncodeH265QpKHR {
        pub fn qp_i(mut self, qp_i: i32) -> Self {
            self.qp_i = qp_i;
            self
        }
        pub fn qp_p(mut self, qp_p: i32) -> Self {
            self.qp_p = qp_p;
            self
        }
        pub fn qp_b(mut self, qp_b: i32) -> Self {
            self.qp_b = qp_b;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265FrameSizeKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct VideoEncodeH265FrameSizeKHR {
        pub frame_i_size: u32,
        pub frame_p_size: u32,
        pub frame_b_size: u32,
    }
    impl VideoEncodeH265FrameSizeKHR {
        pub fn frame_i_size(mut self, frame_i_size: u32) -> Self {
            self.frame_i_size = frame_i_size;
            self
        }
        pub fn frame_p_size(mut self, frame_p_size: u32) -> Self {
            self.frame_p_size = frame_p_size;
            self
        }
        pub fn frame_b_size(mut self, frame_b_size: u32) -> Self {
            self.frame_b_size = frame_b_size;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265GopRemainingFrameInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265GopRemainingFrameInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_gop_remaining_frames: Bool32,
        pub gop_remaining_i: u32,
        pub gop_remaining_p: u32,
        pub gop_remaining_b: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265GopRemainingFrameInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_GOP_REMAINING_FRAME_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoBeginCodingInfoKHR<'a>>
        for VideoEncodeH265GopRemainingFrameInfoKHR<'a>
    {
    }
    impl Default for VideoEncodeH265GopRemainingFrameInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                use_gop_remaining_frames: Default::default(),
                gop_remaining_i: Default::default(),
                gop_remaining_p: Default::default(),
                gop_remaining_b: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265GopRemainingFrameInfoKHR<'a> {
        pub fn use_gop_remaining_frames(mut self, use_gop_remaining_frames: bool) -> Self {
            self.use_gop_remaining_frames = use_gop_remaining_frames.into();
            self
        }
        pub fn gop_remaining_i(mut self, gop_remaining_i: u32) -> Self {
            self.gop_remaining_i = gop_remaining_i;
            self
        }
        pub fn gop_remaining_p(mut self, gop_remaining_p: u32) -> Self {
            self.gop_remaining_p = gop_remaining_p;
            self
        }
        pub fn gop_remaining_b(mut self, gop_remaining_b: u32) -> Self {
            self.gop_remaining_b = gop_remaining_b;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265RateControlLayerInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265RateControlLayerInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub use_min_qp: Bool32,
        pub min_qp: VideoEncodeH265QpKHR,
        pub use_max_qp: Bool32,
        pub max_qp: VideoEncodeH265QpKHR,
        pub use_max_frame_size: Bool32,
        pub max_frame_size: VideoEncodeH265FrameSizeKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265RateControlLayerInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::VIDEO_ENCODE_H265_RATE_CONTROL_LAYER_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoEncodeRateControlLayerInfoKHR<'a>>
        for VideoEncodeH265RateControlLayerInfoKHR<'a>
    {
    }
    impl Default for VideoEncodeH265RateControlLayerInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
    impl<'a> VideoEncodeH265RateControlLayerInfoKHR<'a> {
        pub fn use_min_qp(mut self, use_min_qp: bool) -> Self {
            self.use_min_qp = use_min_qp.into();
            self
        }
        pub fn min_qp(mut self, min_qp: VideoEncodeH265QpKHR) -> Self {
            self.min_qp = min_qp;
            self
        }
        pub fn use_max_qp(mut self, use_max_qp: bool) -> Self {
            self.use_max_qp = use_max_qp.into();
            self
        }
        pub fn max_qp(mut self, max_qp: VideoEncodeH265QpKHR) -> Self {
            self.max_qp = max_qp;
            self
        }
        pub fn use_max_frame_size(mut self, use_max_frame_size: bool) -> Self {
            self.use_max_frame_size = use_max_frame_size.into();
            self
        }
        pub fn max_frame_size(mut self, max_frame_size: VideoEncodeH265FrameSizeKHR) -> Self {
            self.max_frame_size = max_frame_size;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265ProfileInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265ProfileInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub std_profile_idc: StdVideoH265ProfileIdc,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265ProfileInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H265_PROFILE_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoProfileInfoKHR<'a>> for VideoEncodeH265ProfileInfoKHR<'a> {}
    unsafe impl<'a> Extends<QueryPoolCreateInfo<'a>> for VideoEncodeH265ProfileInfoKHR<'a> {}
    impl Default for VideoEncodeH265ProfileInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                std_profile_idc: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265ProfileInfoKHR<'a> {
        pub fn std_profile_idc(mut self, std_profile_idc: StdVideoH265ProfileIdc) -> Self {
            self.std_profile_idc = std_profile_idc;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265DpbSlotInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoEncodeH265DpbSlotInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_std_reference_info: *const StdVideoEncodeH265ReferenceInfo,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for VideoEncodeH265DpbSlotInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::VIDEO_ENCODE_H265_DPB_SLOT_INFO_KHR;
    }
    unsafe impl<'a> Extends<VideoReferenceSlotInfoKHR<'a>> for VideoEncodeH265DpbSlotInfoKHR<'a> {}
    impl Default for VideoEncodeH265DpbSlotInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_std_reference_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoEncodeH265DpbSlotInfoKHR<'a> {
        pub fn std_reference_info(
            mut self,
            std_reference_info: &'a StdVideoEncodeH265ReferenceInfo,
        ) -> Self {
            self.p_std_reference_info = std_reference_info;
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265CapabilityFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265CapabilityFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeH265CapabilityFlagsKHR, Flags);
    impl VideoEncodeH265CapabilityFlagsKHR {
        pub const HRD_COMPLIANCE_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::HRD_COMPLIANCE_KHR.0);
        pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::PREDICTION_WEIGHT_TABLE_GENERATED_KHR.0);
        pub const ROW_UNALIGNED_SLICE_SEGMENT_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::ROW_UNALIGNED_SLICE_SEGMENT_KHR.0);
        pub const DIFFERENT_SLICE_SEGMENT_TYPE_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::DIFFERENT_SLICE_SEGMENT_TYPE_KHR.0);
        pub const B_FRAME_IN_L0_LIST_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::B_FRAME_IN_L0_LIST_KHR.0);
        pub const B_FRAME_IN_L1_LIST_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::B_FRAME_IN_L1_LIST_KHR.0);
        pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::PER_PICTURE_TYPE_MIN_MAX_QP_KHR.0);
        pub const PER_SLICE_SEGMENT_CONSTANT_QP_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::PER_SLICE_SEGMENT_CONSTANT_QP_KHR.0);
        pub const MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR.0);
        pub const MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR.0);
        // VK_KHR_video_encode_intra_refresh
        pub const B_PICTURE_INTRA_REFRESH_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::B_PICTURE_INTRA_REFRESH_KHR.0);
        // VK_KHR_video_encode_quantization_map
        pub const CU_QP_DIFF_WRAPAROUND_KHR: Self =
            Self(VideoEncodeH265CapabilityFlagBitsKHR::CU_QP_DIFF_WRAPAROUND_KHR.0);
    }
    impl fmt::Debug for VideoEncodeH265CapabilityFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeH265CapabilityFlagsKHR::HRD_COMPLIANCE_KHR.0,
                    "HRD_COMPLIANCE_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::PREDICTION_WEIGHT_TABLE_GENERATED_KHR.0,
                    "PREDICTION_WEIGHT_TABLE_GENERATED_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::ROW_UNALIGNED_SLICE_SEGMENT_KHR.0,
                    "ROW_UNALIGNED_SLICE_SEGMENT_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::DIFFERENT_SLICE_SEGMENT_TYPE_KHR.0,
                    "DIFFERENT_SLICE_SEGMENT_TYPE_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::B_FRAME_IN_L0_LIST_KHR.0,
                    "B_FRAME_IN_L0_LIST_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::B_FRAME_IN_L1_LIST_KHR.0,
                    "B_FRAME_IN_L1_LIST_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::PER_PICTURE_TYPE_MIN_MAX_QP_KHR.0,
                    "PER_PICTURE_TYPE_MIN_MAX_QP_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::PER_SLICE_SEGMENT_CONSTANT_QP_KHR.0,
                    "PER_SLICE_SEGMENT_CONSTANT_QP_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR.0,
                    "MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR.0,
                    "MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::B_PICTURE_INTRA_REFRESH_KHR.0,
                    "B_PICTURE_INTRA_REFRESH_KHR",
                ),
                (
                    VideoEncodeH265CapabilityFlagsKHR::CU_QP_DIFF_WRAPAROUND_KHR.0,
                    "CU_QP_DIFF_WRAPAROUND_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265CapabilityFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265CapabilityFlagBitsKHR(u32);
    impl VideoEncodeH265CapabilityFlagBitsKHR {
        pub const HRD_COMPLIANCE_KHR: Self = Self(1 << 0);
        pub const PREDICTION_WEIGHT_TABLE_GENERATED_KHR: Self = Self(1 << 1);
        pub const ROW_UNALIGNED_SLICE_SEGMENT_KHR: Self = Self(1 << 2);
        pub const DIFFERENT_SLICE_SEGMENT_TYPE_KHR: Self = Self(1 << 3);
        pub const B_FRAME_IN_L0_LIST_KHR: Self = Self(1 << 4);
        pub const B_FRAME_IN_L1_LIST_KHR: Self = Self(1 << 5);
        pub const PER_PICTURE_TYPE_MIN_MAX_QP_KHR: Self = Self(1 << 6);
        pub const PER_SLICE_SEGMENT_CONSTANT_QP_KHR: Self = Self(1 << 7);
        pub const MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR: Self = Self(1 << 8);
        pub const MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR: Self = Self(1 << 9);
        // VK_KHR_video_encode_intra_refresh
        pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(1 << 11);
        // VK_KHR_video_encode_quantization_map
        pub const CU_QP_DIFF_WRAPAROUND_KHR: Self = Self(1 << 10);
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265StdFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265StdFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeH265StdFlagsKHR, Flags);
    impl VideoEncodeH265StdFlagsKHR {
        pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::SEPARATE_COLOR_PLANE_FLAG_SET_KHR.0);
        pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR.0);
        pub const SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR.0);
        pub const PCM_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::PCM_ENABLED_FLAG_SET_KHR.0);
        pub const SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR.0);
        pub const INIT_QP_MINUS26_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::INIT_QP_MINUS26_KHR.0);
        pub const WEIGHTED_PRED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::WEIGHTED_PRED_FLAG_SET_KHR.0);
        pub const WEIGHTED_BIPRED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::WEIGHTED_BIPRED_FLAG_SET_KHR.0);
        pub const LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR.0);
        pub const SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR.0);
        pub const TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR.0);
        pub const TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR.0);
        pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR.0);
        pub const TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR.0);
        pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::CONSTRAINED_INTRA_PRED_FLAG_SET_KHR.0);
        pub const ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR.0);
        pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR.0);
        pub const DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR.0);
        pub const DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR.0);
        pub const SLICE_QP_DELTA_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::SLICE_QP_DELTA_KHR.0);
        pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self =
            Self(VideoEncodeH265StdFlagBitsKHR::DIFFERENT_SLICE_QP_DELTA_KHR.0);
    }
    impl fmt::Debug for VideoEncodeH265StdFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeH265StdFlagsKHR::SEPARATE_COLOR_PLANE_FLAG_SET_KHR.0,
                    "SEPARATE_COLOR_PLANE_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR.0,
                    "SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR.0,
                    "SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::PCM_ENABLED_FLAG_SET_KHR.0,
                    "PCM_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR.0,
                    "SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::INIT_QP_MINUS26_KHR.0,
                    "INIT_QP_MINUS26_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::WEIGHTED_PRED_FLAG_SET_KHR.0,
                    "WEIGHTED_PRED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::WEIGHTED_BIPRED_FLAG_SET_KHR.0,
                    "WEIGHTED_BIPRED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR.0,
                    "LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR.0,
                    "SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR.0,
                    "TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR.0,
                    "TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR.0,
                    "PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR.0,
                    "TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::CONSTRAINED_INTRA_PRED_FLAG_SET_KHR.0,
                    "CONSTRAINED_INTRA_PRED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR.0,
                    "ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR.0,
                    "DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR.0,
                    "DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR.0,
                    "DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::SLICE_QP_DELTA_KHR.0,
                    "SLICE_QP_DELTA_KHR",
                ),
                (
                    VideoEncodeH265StdFlagsKHR::DIFFERENT_SLICE_QP_DELTA_KHR.0,
                    "DIFFERENT_SLICE_QP_DELTA_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265StdFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265StdFlagBitsKHR(u32);
    impl VideoEncodeH265StdFlagBitsKHR {
        pub const SEPARATE_COLOR_PLANE_FLAG_SET_KHR: Self = Self(1 << 0);
        pub const SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR: Self = Self(1 << 1);
        pub const SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR: Self = Self(1 << 2);
        pub const PCM_ENABLED_FLAG_SET_KHR: Self = Self(1 << 3);
        pub const SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR: Self = Self(1 << 4);
        pub const INIT_QP_MINUS26_KHR: Self = Self(1 << 5);
        pub const WEIGHTED_PRED_FLAG_SET_KHR: Self = Self(1 << 6);
        pub const WEIGHTED_BIPRED_FLAG_SET_KHR: Self = Self(1 << 7);
        pub const LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR: Self = Self(1 << 8);
        pub const SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR: Self = Self(1 << 9);
        pub const TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR: Self = Self(1 << 10);
        pub const TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR: Self = Self(1 << 11);
        pub const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR: Self = Self(1 << 12);
        pub const TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR: Self = Self(1 << 13);
        pub const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR: Self = Self(1 << 14);
        pub const ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR: Self = Self(1 << 15);
        pub const DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR: Self = Self(1 << 16);
        pub const DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR: Self = Self(1 << 17);
        pub const DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR: Self = Self(1 << 18);
        pub const SLICE_QP_DELTA_KHR: Self = Self(1 << 19);
        pub const DIFFERENT_SLICE_QP_DELTA_KHR: Self = Self(1 << 20);
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265RateControlFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265RateControlFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeH265RateControlFlagsKHR, Flags);
    impl VideoEncodeH265RateControlFlagsKHR {
        pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self =
            Self(VideoEncodeH265RateControlFlagBitsKHR::ATTEMPT_HRD_COMPLIANCE_KHR.0);
        pub const REGULAR_GOP_KHR: Self =
            Self(VideoEncodeH265RateControlFlagBitsKHR::REGULAR_GOP_KHR.0);
        pub const REFERENCE_PATTERN_FLAT_KHR: Self =
            Self(VideoEncodeH265RateControlFlagBitsKHR::REFERENCE_PATTERN_FLAT_KHR.0);
        pub const REFERENCE_PATTERN_DYADIC_KHR: Self =
            Self(VideoEncodeH265RateControlFlagBitsKHR::REFERENCE_PATTERN_DYADIC_KHR.0);
        pub const TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR: Self =
            Self(VideoEncodeH265RateControlFlagBitsKHR::TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR.0);
    }
    impl fmt::Debug for VideoEncodeH265RateControlFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeH265RateControlFlagsKHR::ATTEMPT_HRD_COMPLIANCE_KHR.0,
                    "ATTEMPT_HRD_COMPLIANCE_KHR",
                ),
                (
                    VideoEncodeH265RateControlFlagsKHR::REGULAR_GOP_KHR.0,
                    "REGULAR_GOP_KHR",
                ),
                (
                    VideoEncodeH265RateControlFlagsKHR::REFERENCE_PATTERN_FLAT_KHR.0,
                    "REFERENCE_PATTERN_FLAT_KHR",
                ),
                (
                    VideoEncodeH265RateControlFlagsKHR::REFERENCE_PATTERN_DYADIC_KHR.0,
                    "REFERENCE_PATTERN_DYADIC_KHR",
                ),
                (
                    VideoEncodeH265RateControlFlagsKHR::TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR.0,
                    "TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265RateControlFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265RateControlFlagBitsKHR(u32);
    impl VideoEncodeH265RateControlFlagBitsKHR {
        pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1 << 0);
        pub const REGULAR_GOP_KHR: Self = Self(1 << 1);
        pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(1 << 2);
        pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(1 << 3);
        pub const TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR: Self = Self(1 << 4);
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265CtbSizeFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265CtbSizeFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeH265CtbSizeFlagsKHR, Flags);
    impl VideoEncodeH265CtbSizeFlagsKHR {
        pub const _16_KHR: Self = Self(VideoEncodeH265CtbSizeFlagBitsKHR::_16_KHR.0);
        pub const _32_KHR: Self = Self(VideoEncodeH265CtbSizeFlagBitsKHR::_32_KHR.0);
        pub const _64_KHR: Self = Self(VideoEncodeH265CtbSizeFlagBitsKHR::_64_KHR.0);
    }
    impl fmt::Debug for VideoEncodeH265CtbSizeFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (VideoEncodeH265CtbSizeFlagsKHR::_16_KHR.0, "_16_KHR"),
                (VideoEncodeH265CtbSizeFlagsKHR::_32_KHR.0, "_32_KHR"),
                (VideoEncodeH265CtbSizeFlagsKHR::_64_KHR.0, "_64_KHR"),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265CtbSizeFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265CtbSizeFlagBitsKHR(u32);
    impl VideoEncodeH265CtbSizeFlagBitsKHR {
        pub const _16_KHR: Self = Self(1 << 0);
        pub const _32_KHR: Self = Self(1 << 1);
        pub const _64_KHR: Self = Self(1 << 2);
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265TransformBlockSizeFlagsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265TransformBlockSizeFlagsKHR(Flags);
    vk_bitflags_wrapped!(VideoEncodeH265TransformBlockSizeFlagsKHR, Flags);
    impl VideoEncodeH265TransformBlockSizeFlagsKHR {
        pub const _4_KHR: Self = Self(VideoEncodeH265TransformBlockSizeFlagBitsKHR::_4_KHR.0);
        pub const _8_KHR: Self = Self(VideoEncodeH265TransformBlockSizeFlagBitsKHR::_8_KHR.0);
        pub const _16_KHR: Self = Self(VideoEncodeH265TransformBlockSizeFlagBitsKHR::_16_KHR.0);
        pub const _32_KHR: Self = Self(VideoEncodeH265TransformBlockSizeFlagBitsKHR::_32_KHR.0);
    }
    impl fmt::Debug for VideoEncodeH265TransformBlockSizeFlagsKHR {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    VideoEncodeH265TransformBlockSizeFlagsKHR::_4_KHR.0,
                    "_4_KHR",
                ),
                (
                    VideoEncodeH265TransformBlockSizeFlagsKHR::_8_KHR.0,
                    "_8_KHR",
                ),
                (
                    VideoEncodeH265TransformBlockSizeFlagsKHR::_16_KHR.0,
                    "_16_KHR",
                ),
                (
                    VideoEncodeH265TransformBlockSizeFlagsKHR::_32_KHR.0,
                    "_32_KHR",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkVideoEncodeH265TransformBlockSizeFlagBitsKHR.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct VideoEncodeH265TransformBlockSizeFlagBitsKHR(u32);
    impl VideoEncodeH265TransformBlockSizeFlagBitsKHR {
        pub const _4_KHR: Self = Self(1 << 0);
        pub const _8_KHR: Self = Self(1 << 1);
        pub const _16_KHR: Self = Self(1 << 2);
        pub const _32_KHR: Self = Self(1 << 3);
    }
}
