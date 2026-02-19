#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
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
        const HRD_COMPLIANCE_KHR = 1 << 0;
        const PREDICTION_WEIGHT_TABLE_GENERATED_KHR = 1 << 1;
        const ROW_UNALIGNED_SLICE_KHR = 1 << 2;
        const DIFFERENT_SLICE_TYPE_KHR = 1 << 3;
        const B_FRAME_IN_L0_LIST_KHR = 1 << 4;
        const B_FRAME_IN_L1_LIST_KHR = 1 << 5;
        const PER_PICTURE_TYPE_MIN_MAX_QP_KHR = 1 << 6;
        const PER_SLICE_CONSTANT_QP_KHR = 1 << 7;
        const GENERATE_PREFIX_NALU_KHR = 1 << 8;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH264StdFlagsKHR: Flags {
        const SEPARATE_COLOR_PLANE_FLAG_SET_KHR = 1 << 0;
        const QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_KHR = 1 << 1;
        const SCALING_MATRIX_PRESENT_FLAG_SET_KHR = 1 << 2;
        const CHROMA_QP_INDEX_OFFSET_KHR = 1 << 3;
        const SECOND_CHROMA_QP_INDEX_OFFSET_KHR = 1 << 4;
        const PIC_INIT_QP_MINUS26_KHR = 1 << 5;
        const WEIGHTED_PRED_FLAG_SET_KHR = 1 << 6;
        const WEIGHTED_BIPRED_IDC_EXPLICIT_KHR = 1 << 7;
        const WEIGHTED_BIPRED_IDC_IMPLICIT_KHR = 1 << 8;
        const TRANSFORM_8X8_MODE_FLAG_SET_KHR = 1 << 9;
        const DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_KHR = 1 << 10;
        const ENTROPY_CODING_MODE_FLAG_UNSET_KHR = 1 << 11;
        const ENTROPY_CODING_MODE_FLAG_SET_KHR = 1 << 12;
        const DIRECT_8X8_INFERENCE_FLAG_UNSET_KHR = 1 << 13;
        const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR = 1 << 14;
        const DEBLOCKING_FILTER_DISABLED_KHR = 1 << 15;
        const DEBLOCKING_FILTER_ENABLED_KHR = 1 << 16;
        const DEBLOCKING_FILTER_PARTIAL_KHR = 1 << 17;
        const SLICE_QP_DELTA_KHR = 1 << 19;
        const DIFFERENT_SLICE_QP_DELTA_KHR = 1 << 20;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH264RateControlFlagsKHR: Flags {
        const ATTEMPT_HRD_COMPLIANCE_KHR = 1 << 0;
        const REGULAR_GOP_KHR = 1 << 1;
        const REFERENCE_PATTERN_FLAT_KHR = 1 << 2;
        const REFERENCE_PATTERN_DYADIC_KHR = 1 << 3;
        const TEMPORAL_LAYER_PATTERN_DYADIC_KHR = 1 << 4;
    }
}
