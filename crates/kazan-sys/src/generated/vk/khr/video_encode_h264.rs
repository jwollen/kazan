#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264CapabilitiesKHR {
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
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264QualityLevelPropertiesKHR {
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
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level_idc: Bool32,
    pub max_level_idc: StdVideoH264LevelIdc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const StdVideoH264SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const StdVideoH264PictureParameterSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoEncodeH264SessionParametersAddInfoKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionParametersGetInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub write_std_sps: Bool32,
    pub write_std_pps: Bool32,
    pub std_sps_id: u32,
    pub std_pps_id: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264SessionParametersFeedbackInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_std_sps_overrides: Bool32,
    pub has_std_pps_overrides: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoEncodeH264ReferenceInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub nalu_slice_entry_count: u32,
    pub p_nalu_slice_entries: *const VideoEncodeH264NaluSliceInfoKHR,
    pub p_std_picture_info: *const StdVideoEncodeH264PictureInfo,
    pub generate_prefix_nalu: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH264ProfileIdc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264NaluSliceInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub constant_qp: i32,
    pub p_std_slice_header: *const StdVideoEncodeH264SliceHeader,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264RateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeH264RateControlFlagsKHR,
    pub gop_frame_count: u32,
    pub idr_period: u32,
    pub consecutive_b_frame_count: u32,
    pub temporal_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264QpKHR {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264FrameSizeKHR {
    pub frame_i_size: u32,
    pub frame_p_size: u32,
    pub frame_b_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264GopRemainingFrameInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_i: u32,
    pub gop_remaining_p: u32,
    pub gop_remaining_b: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH264RateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_min_qp: Bool32,
    pub min_qp: VideoEncodeH264QpKHR,
    pub use_max_qp: Bool32,
    pub max_qp: VideoEncodeH264QpKHR,
    pub use_max_frame_size: Bool32,
    pub max_frame_size: VideoEncodeH264FrameSizeKHR,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
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
        const B_PICTURE_INTRA_REFRESH_KHR = VideoEncodeH264CapabilityFlagBitsKHR::B_PICTURE_INTRA_REFRESH_KHR.0;
        const MB_QP_DIFF_WRAPAROUND_KHR = VideoEncodeH264CapabilityFlagBitsKHR::MB_QP_DIFF_WRAPAROUND_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(1 << 10);
    pub const MB_QP_DIFF_WRAPAROUND_KHR: Self = Self(1 << 9);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
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
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH264RateControlFlagsKHR: Flags {
        const ATTEMPT_HRD_COMPLIANCE_KHR = VideoEncodeH264RateControlFlagBitsKHR::ATTEMPT_HRD_COMPLIANCE_KHR.0;
        const REGULAR_GOP_KHR = VideoEncodeH264RateControlFlagBitsKHR::REGULAR_GOP_KHR.0;
        const REFERENCE_PATTERN_FLAT_KHR = VideoEncodeH264RateControlFlagBitsKHR::REFERENCE_PATTERN_FLAT_KHR.0;
        const REFERENCE_PATTERN_DYADIC_KHR = VideoEncodeH264RateControlFlagBitsKHR::REFERENCE_PATTERN_DYADIC_KHR.0;
        const TEMPORAL_LAYER_PATTERN_DYADIC_KHR = VideoEncodeH264RateControlFlagBitsKHR::TEMPORAL_LAYER_PATTERN_DYADIC_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH264RateControlFlagBitsKHR(u32);
impl VideoEncodeH264RateControlFlagBitsKHR {
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1 << 0);
    pub const REGULAR_GOP_KHR: Self = Self(1 << 1);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(1 << 2);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(1 << 3);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(1 << 4);
}
