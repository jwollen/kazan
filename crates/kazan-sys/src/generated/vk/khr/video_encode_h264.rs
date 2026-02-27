#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoEncodeH264CapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
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
    pub fn flags(mut self, flags: VideoEncodeH264CapabilityFlagsKHR) -> Self {
        self.flags = flags;
        self
    }
    pub fn max_level_idc(mut self, max_level_idc: StdVideoH264LevelIdc) -> Self {
        self.max_level_idc = max_level_idc;
        self
    }
    pub fn max_slice_count(mut self, max_slice_count: u32) -> Self {
        self.max_slice_count = max_slice_count;
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
    pub fn max_temporal_layer_count(mut self, max_temporal_layer_count: u32) -> Self {
        self.max_temporal_layer_count = max_temporal_layer_count;
        self
    }
    pub fn expect_dyadic_temporal_layer_pattern(
        mut self,
        expect_dyadic_temporal_layer_pattern: Bool32,
    ) -> Self {
        self.expect_dyadic_temporal_layer_pattern = expect_dyadic_temporal_layer_pattern;
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
    pub fn prefers_gop_remaining_frames(mut self, prefers_gop_remaining_frames: Bool32) -> Self {
        self.prefers_gop_remaining_frames = prefers_gop_remaining_frames;
        self
    }
    pub fn requires_gop_remaining_frames(mut self, requires_gop_remaining_frames: Bool32) -> Self {
        self.requires_gop_remaining_frames = requires_gop_remaining_frames;
        self
    }
    pub fn std_syntax_flags(mut self, std_syntax_flags: VideoEncodeH264StdFlagsKHR) -> Self {
        self.std_syntax_flags = std_syntax_flags;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoEncodeH264QualityLevelPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_QUALITY_LEVEL_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
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
    pub fn preferred_rate_control_flags(
        mut self,
        preferred_rate_control_flags: VideoEncodeH264RateControlFlagsKHR,
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
    pub fn preferred_temporal_layer_count(mut self, preferred_temporal_layer_count: u32) -> Self {
        self.preferred_temporal_layer_count = preferred_temporal_layer_count;
        self
    }
    pub fn preferred_constant_qp(mut self, preferred_constant_qp: VideoEncodeH264QpKHR) -> Self {
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
    pub fn preferred_std_entropy_coding_mode_flag(
        mut self,
        preferred_std_entropy_coding_mode_flag: Bool32,
    ) -> Self {
        self.preferred_std_entropy_coding_mode_flag = preferred_std_entropy_coding_mode_flag;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level_idc: Bool32,
    pub max_level_idc: StdVideoH264LevelIdc,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264SessionCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_SESSION_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            use_max_level_idc: Default::default(),
            max_level_idc: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264SessionCreateInfoKHR<'a> {
    pub fn use_max_level_idc(mut self, use_max_level_idc: Bool32) -> Self {
        self.use_max_level_idc = use_max_level_idc;
        self
    }
    pub fn max_level_idc(mut self, max_level_idc: StdVideoH264LevelIdc) -> Self {
        self.max_level_idc = max_level_idc;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionParametersAddInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const StdVideoH264SequenceParameterSet<'a>,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const StdVideoH264PictureParameterSet<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264SessionParametersAddInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_ADD_INFO_KHR,
            p_next: core::ptr::null(),
            std_sps_count: Default::default(),
            p_std_sp_ss: core::ptr::null(),
            std_pps_count: Default::default(),
            p_std_pp_ss: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264SessionParametersAddInfoKHR<'a> {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoEncodeH264SessionParametersAddInfoKHR<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264SessionParametersCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            max_std_sps_count: Default::default(),
            max_std_pps_count: Default::default(),
            p_parameters_add_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264SessionParametersCreateInfoKHR<'a> {
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
        parameters_add_info: &'a VideoEncodeH264SessionParametersAddInfoKHR<'a>,
    ) -> Self {
        self.p_parameters_add_info = parameters_add_info;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionParametersGetInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub write_std_sps: Bool32,
    pub write_std_pps: Bool32,
    pub std_sps_id: u32,
    pub std_pps_id: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264SessionParametersGetInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_GET_INFO_KHR,
            p_next: core::ptr::null(),
            write_std_sps: Default::default(),
            write_std_pps: Default::default(),
            std_sps_id: Default::default(),
            std_pps_id: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264SessionParametersGetInfoKHR<'a> {
    pub fn write_std_sps(mut self, write_std_sps: Bool32) -> Self {
        self.write_std_sps = write_std_sps;
        self
    }
    pub fn write_std_pps(mut self, write_std_pps: Bool32) -> Self {
        self.write_std_pps = write_std_pps;
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionParametersFeedbackInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_std_sps_overrides: Bool32,
    pub has_std_pps_overrides: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264SessionParametersFeedbackInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_SESSION_PARAMETERS_FEEDBACK_INFO_KHR,
            p_next: core::ptr::null_mut(),
            has_std_sps_overrides: Default::default(),
            has_std_pps_overrides: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264SessionParametersFeedbackInfoKHR<'a> {
    pub fn has_std_sps_overrides(mut self, has_std_sps_overrides: Bool32) -> Self {
        self.has_std_sps_overrides = has_std_sps_overrides;
        self
    }
    pub fn has_std_pps_overrides(mut self, has_std_pps_overrides: Bool32) -> Self {
        self.has_std_pps_overrides = has_std_pps_overrides;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264DpbSlotInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoEncodeH264ReferenceInfo,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264DpbSlotInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_DPB_SLOT_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_reference_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264DpbSlotInfoKHR<'a> {
    pub fn std_reference_info(
        mut self,
        std_reference_info: &'a StdVideoEncodeH264ReferenceInfo,
    ) -> Self {
        self.p_std_reference_info = std_reference_info;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264PictureInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub nalu_slice_entry_count: u32,
    pub p_nalu_slice_entries: *const VideoEncodeH264NaluSliceInfoKHR<'a>,
    pub p_std_picture_info: *const StdVideoEncodeH264PictureInfo<'a>,
    pub generate_prefix_nalu: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264PictureInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_PICTURE_INFO_KHR,
            p_next: core::ptr::null(),
            nalu_slice_entry_count: Default::default(),
            p_nalu_slice_entries: core::ptr::null(),
            p_std_picture_info: core::ptr::null(),
            generate_prefix_nalu: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264PictureInfoKHR<'a> {
    pub fn nalu_slice_entries(
        mut self,
        nalu_slice_entries: &'a [VideoEncodeH264NaluSliceInfoKHR<'a>],
    ) -> Self {
        self.nalu_slice_entry_count = nalu_slice_entries.len().try_into().unwrap();
        self.p_nalu_slice_entries = nalu_slice_entries.as_ptr();
        self
    }
    pub fn std_picture_info(
        mut self,
        std_picture_info: &'a StdVideoEncodeH264PictureInfo<'a>,
    ) -> Self {
        self.p_std_picture_info = std_picture_info;
        self
    }
    pub fn generate_prefix_nalu(mut self, generate_prefix_nalu: Bool32) -> Self {
        self.generate_prefix_nalu = generate_prefix_nalu;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH264ProfileIdc,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264ProfileInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_PROFILE_INFO_KHR,
            p_next: core::ptr::null(),
            std_profile_idc: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264ProfileInfoKHR<'a> {
    pub fn std_profile_idc(mut self, std_profile_idc: StdVideoH264ProfileIdc) -> Self {
        self.std_profile_idc = std_profile_idc;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264NaluSliceInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub constant_qp: i32,
    pub p_std_slice_header: *const StdVideoEncodeH264SliceHeader<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264NaluSliceInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_NALU_SLICE_INFO_KHR,
            p_next: core::ptr::null(),
            constant_qp: Default::default(),
            p_std_slice_header: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264NaluSliceInfoKHR<'a> {
    pub fn constant_qp(mut self, constant_qp: i32) -> Self {
        self.constant_qp = constant_qp;
        self
    }
    pub fn std_slice_header(
        mut self,
        std_slice_header: &'a StdVideoEncodeH264SliceHeader<'a>,
    ) -> Self {
        self.p_std_slice_header = std_slice_header;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoEncodeH264RateControlInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_INFO_KHR,
            p_next: core::ptr::null(),
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
    pub fn flags(mut self, flags: VideoEncodeH264RateControlFlagsKHR) -> Self {
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
    pub fn temporal_layer_count(mut self, temporal_layer_count: u32) -> Self {
        self.temporal_layer_count = temporal_layer_count;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VideoEncodeH264QpKHR {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}
impl VideoEncodeH264QpKHR {
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
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VideoEncodeH264FrameSizeKHR {
    pub frame_i_size: u32,
    pub frame_p_size: u32,
    pub frame_b_size: u32,
}
impl VideoEncodeH264FrameSizeKHR {
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264GopRemainingFrameInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_i: u32,
    pub gop_remaining_p: u32,
    pub gop_remaining_b: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeH264GopRemainingFrameInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_GOP_REMAINING_FRAME_INFO_KHR,
            p_next: core::ptr::null(),
            use_gop_remaining_frames: Default::default(),
            gop_remaining_i: Default::default(),
            gop_remaining_p: Default::default(),
            gop_remaining_b: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> VideoEncodeH264GopRemainingFrameInfoKHR<'a> {
    pub fn use_gop_remaining_frames(mut self, use_gop_remaining_frames: Bool32) -> Self {
        self.use_gop_remaining_frames = use_gop_remaining_frames;
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
#[repr(C)]
#[derive(Copy, Clone)]
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
impl Default for VideoEncodeH264RateControlLayerInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_H264_RATE_CONTROL_LAYER_INFO_KHR,
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
impl<'a> VideoEncodeH264RateControlLayerInfoKHR<'a> {
    pub fn use_min_qp(mut self, use_min_qp: Bool32) -> Self {
        self.use_min_qp = use_min_qp;
        self
    }
    pub fn min_qp(mut self, min_qp: VideoEncodeH264QpKHR) -> Self {
        self.min_qp = min_qp;
        self
    }
    pub fn use_max_qp(mut self, use_max_qp: Bool32) -> Self {
        self.use_max_qp = use_max_qp;
        self
    }
    pub fn max_qp(mut self, max_qp: VideoEncodeH264QpKHR) -> Self {
        self.max_qp = max_qp;
        self
    }
    pub fn use_max_frame_size(mut self, use_max_frame_size: Bool32) -> Self {
        self.use_max_frame_size = use_max_frame_size;
        self
    }
    pub fn max_frame_size(mut self, max_frame_size: VideoEncodeH264FrameSizeKHR) -> Self {
        self.max_frame_size = max_frame_size;
        self
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeH264CapabilityFlagsKHR: Flags {
        const HRD_COMPLIANCE_KHR = VideoEncodeH264CapabilityFlagBitsKHR::HRD_COMPLIANCE_KHR.0;
        const PREDICTION_WEIGHT_TABLE_GENERATED_KHR = VideoEncodeH264CapabilityFlagBitsKHR::PREDICTION_WEIGHT_TABLE_GENERATED_KHR.0;
        const ROW_UNALIGNED_SLICE_KHR = VideoEncodeH264CapabilityFlagBitsKHR::ROW_UNALIGNED_SLICE_KHR.0;
        const DIFFERENT_SLICE_TYPE_KHR = VideoEncodeH264CapabilityFlagBitsKHR::DIFFERENT_SLICE_TYPE_KHR.0;
        const B_FRAME_IN_L0_LIST_KHR = VideoEncodeH264CapabilityFlagBitsKHR::B_FRAME_IN_L0_LIST_KHR.0;
        const B_FRAME_IN_L1_LIST_KHR = VideoEncodeH264CapabilityFlagBitsKHR::B_FRAME_IN_L1_LIST_KHR.0;
        const PER_PICTURE_TYPE_MIN_MAX_QP_KHR = VideoEncodeH264CapabilityFlagBitsKHR::PER_PICTURE_TYPE_MIN_MAX_QP_KHR.0;
        const PER_SLICE_CONSTANT_QP_KHR = VideoEncodeH264CapabilityFlagBitsKHR::PER_SLICE_CONSTANT_QP_KHR.0;
        const GENERATE_PREFIX_NALU_KHR = VideoEncodeH264CapabilityFlagBitsKHR::GENERATE_PREFIX_NALU_KHR.0;
        const MB_QP_DIFF_WRAPAROUND_KHR = VideoEncodeH264CapabilityFlagBitsKHR::MB_QP_DIFF_WRAPAROUND_KHR.0;
        const B_PICTURE_INTRA_REFRESH_KHR = VideoEncodeH264CapabilityFlagBitsKHR::B_PICTURE_INTRA_REFRESH_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const MB_QP_DIFF_WRAPAROUND_KHR: Self = Self(1 << 9);
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(1 << 10);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeH264StdFlagsKHR: Flags {
        const SEPARATE_COLOR_PLANE_FLAG_SET_KHR = VideoEncodeH264StdFlagBitsKHR::SEPARATE_COLOR_PLANE_FLAG_SET_KHR.0;
        const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR = VideoEncodeH264StdFlagBitsKHR::QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR.0;
        const SCALING_MATRIX_PRESENT_FLAG_SET_KHR = VideoEncodeH264StdFlagBitsKHR::SCALING_MATRIX_PRESENT_FLAG_SET_KHR.0;
        const CHROMA_QP_INDEX_OFFSET_KHR = VideoEncodeH264StdFlagBitsKHR::CHROMA_QP_INDEX_OFFSET_KHR.0;
        const SECOND_CHROMA_QP_INDEX_OFFSET_KHR = VideoEncodeH264StdFlagBitsKHR::SECOND_CHROMA_QP_INDEX_OFFSET_KHR.0;
        const PIC_INIT_QP_MINUS26_KHR = VideoEncodeH264StdFlagBitsKHR::PIC_INIT_QP_MINUS26_KHR.0;
        const WEIGHTED_PRED_FLAG_SET_KHR = VideoEncodeH264StdFlagBitsKHR::WEIGHTED_PRED_FLAG_SET_KHR.0;
        const WEIGHTED_BIPRED_IDC_EXPLICIT_KHR = VideoEncodeH264StdFlagBitsKHR::WEIGHTED_BIPRED_IDC_EXPLICIT_KHR.0;
        const WEIGHTED_BIPRED_IDC_IMPLICIT_KHR = VideoEncodeH264StdFlagBitsKHR::WEIGHTED_BIPRED_IDC_IMPLICIT_KHR.0;
        const TRANSFORM_8X8_MODE_FLAG_SET_KHR = VideoEncodeH264StdFlagBitsKHR::TRANSFORM_8X8_MODE_FLAG_SET_KHR.0;
        const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR = VideoEncodeH264StdFlagBitsKHR::DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR.0;
        const ENTROPY_CODING_MODE_FLAG_UNSET_KHR = VideoEncodeH264StdFlagBitsKHR::ENTROPY_CODING_MODE_FLAG_UNSET_KHR.0;
        const ENTROPY_CODING_MODE_FLAG_SET_KHR = VideoEncodeH264StdFlagBitsKHR::ENTROPY_CODING_MODE_FLAG_SET_KHR.0;
        const DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR = VideoEncodeH264StdFlagBitsKHR::DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR.0;
        const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR = VideoEncodeH264StdFlagBitsKHR::CONSTRAINED_INTRA_PRED_FLAG_SET_KHR.0;
        const DEBLOCKING_FILTER_DISABLED_KHR = VideoEncodeH264StdFlagBitsKHR::DEBLOCKING_FILTER_DISABLED_KHR.0;
        const DEBLOCKING_FILTER_ENABLED_KHR = VideoEncodeH264StdFlagBitsKHR::DEBLOCKING_FILTER_ENABLED_KHR.0;
        const DEBLOCKING_FILTER_PARTIAL_KHR = VideoEncodeH264StdFlagBitsKHR::DEBLOCKING_FILTER_PARTIAL_KHR.0;
        const SLICE_QP_DELTA_KHR = VideoEncodeH264StdFlagBitsKHR::SLICE_QP_DELTA_KHR.0;
        const DIFFERENT_SLICE_QP_DELTA_KHR = VideoEncodeH264StdFlagBitsKHR::DIFFERENT_SLICE_QP_DELTA_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeH264RateControlFlagsKHR: Flags {
        const ATTEMPT_HRD_COMPLIANCE_KHR = VideoEncodeH264RateControlFlagBitsKHR::ATTEMPT_HRD_COMPLIANCE_KHR.0;
        const REGULAR_GOP_KHR = VideoEncodeH264RateControlFlagBitsKHR::REGULAR_GOP_KHR.0;
        const REFERENCE_PATTERN_FLAT_KHR = VideoEncodeH264RateControlFlagBitsKHR::REFERENCE_PATTERN_FLAT_KHR.0;
        const REFERENCE_PATTERN_DYADIC_KHR = VideoEncodeH264RateControlFlagBitsKHR::REFERENCE_PATTERN_DYADIC_KHR.0;
        const TEMPORAL_LAYER_PATTERN_DYADIC_KHR = VideoEncodeH264RateControlFlagBitsKHR::TEMPORAL_LAYER_PATTERN_DYADIC_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH264RateControlFlagBitsKHR(u32);
impl VideoEncodeH264RateControlFlagBitsKHR {
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1 << 0);
    pub const REGULAR_GOP_KHR: Self = Self(1 << 1);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(1 << 2);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(1 << 3);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(1 << 4);
}
