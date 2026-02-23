#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct VideoEncodeH265CapabilitiesKHR {
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
}
#[repr(C)]
pub struct VideoEncodeH265QualityLevelPropertiesKHR {
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
}
#[repr(C)]
pub struct VideoEncodeH265SessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level_idc: Bool32,
    pub max_level_idc: StdVideoH265LevelIdc,
}
#[repr(C)]
pub struct VideoEncodeH265SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_vps_count: u32,
    pub p_std_vp_ss: *const StdVideoH265VideoParameterSet,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const StdVideoH265SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const StdVideoH265PictureParameterSet,
}
#[repr(C)]
pub struct VideoEncodeH265SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_vps_count: u32,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoEncodeH265SessionParametersAddInfoKHR,
}
#[repr(C)]
pub struct VideoEncodeH265SessionParametersGetInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub write_std_vps: Bool32,
    pub write_std_sps: Bool32,
    pub write_std_pps: Bool32,
    pub std_vps_id: u32,
    pub std_sps_id: u32,
    pub std_pps_id: u32,
}
#[repr(C)]
pub struct VideoEncodeH265SessionParametersFeedbackInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_std_vps_overrides: Bool32,
    pub has_std_sps_overrides: Bool32,
    pub has_std_pps_overrides: Bool32,
}
#[repr(C)]
pub struct VideoEncodeH265PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub nalu_slice_segment_entry_count: u32,
    pub p_nalu_slice_segment_entries: *const VideoEncodeH265NaluSliceSegmentInfoKHR,
    pub p_std_picture_info: *const StdVideoEncodeH265PictureInfo,
}
#[repr(C)]
pub struct VideoEncodeH265NaluSliceSegmentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub constant_qp: i32,
    pub p_std_slice_segment_header: *const StdVideoEncodeH265SliceSegmentHeader,
}
#[repr(C)]
pub struct VideoEncodeH265RateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeH265RateControlFlagsKHR,
    pub gop_frame_count: u32,
    pub idr_period: u32,
    pub consecutive_b_frame_count: u32,
    pub sub_layer_count: u32,
}
#[repr(C)]
pub struct VideoEncodeH265QpKHR {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}
#[repr(C)]
pub struct VideoEncodeH265FrameSizeKHR {
    pub frame_i_size: u32,
    pub frame_p_size: u32,
    pub frame_b_size: u32,
}
#[repr(C)]
pub struct VideoEncodeH265GopRemainingFrameInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_i: u32,
    pub gop_remaining_p: u32,
    pub gop_remaining_b: u32,
}
#[repr(C)]
pub struct VideoEncodeH265RateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_min_qp: Bool32,
    pub min_qp: VideoEncodeH265QpKHR,
    pub use_max_qp: Bool32,
    pub max_qp: VideoEncodeH265QpKHR,
    pub use_max_frame_size: Bool32,
    pub max_frame_size: VideoEncodeH265FrameSizeKHR,
}
#[repr(C)]
pub struct VideoEncodeH265ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH265ProfileIdc,
}
#[repr(C)]
pub struct VideoEncodeH265DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoEncodeH265ReferenceInfo,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265CapabilityFlagsKHR: Flags {
        const HRD_COMPLIANCE_KHR = VideoEncodeH265CapabilityFlagBitsKHR::HRD_COMPLIANCE_KHR.0;
        const PREDICTION_WEIGHT_TABLE_GENERATED_KHR = VideoEncodeH265CapabilityFlagBitsKHR::PREDICTION_WEIGHT_TABLE_GENERATED_KHR.0;
        const ROW_UNALIGNED_SLICE_SEGMENT_KHR = VideoEncodeH265CapabilityFlagBitsKHR::ROW_UNALIGNED_SLICE_SEGMENT_KHR.0;
        const DIFFERENT_SLICE_SEGMENT_TYPE_KHR = VideoEncodeH265CapabilityFlagBitsKHR::DIFFERENT_SLICE_SEGMENT_TYPE_KHR.0;
        const B_FRAME_IN_L0_LIST_KHR = VideoEncodeH265CapabilityFlagBitsKHR::B_FRAME_IN_L0_LIST_KHR.0;
        const B_FRAME_IN_L1_LIST_KHR = VideoEncodeH265CapabilityFlagBitsKHR::B_FRAME_IN_L1_LIST_KHR.0;
        const PER_PICTURE_TYPE_MIN_MAX_QP_KHR = VideoEncodeH265CapabilityFlagBitsKHR::PER_PICTURE_TYPE_MIN_MAX_QP_KHR.0;
        const PER_SLICE_SEGMENT_CONSTANT_QP_KHR = VideoEncodeH265CapabilityFlagBitsKHR::PER_SLICE_SEGMENT_CONSTANT_QP_KHR.0;
        const MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR = VideoEncodeH265CapabilityFlagBitsKHR::MULTIPLE_TILES_PER_SLICE_SEGMENT_KHR.0;
        const MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR = VideoEncodeH265CapabilityFlagBitsKHR::MULTIPLE_SLICE_SEGMENTS_PER_TILE_KHR.0;
        const CU_QP_DIFF_WRAPAROUND_KHR = VideoEncodeH265CapabilityFlagBitsKHR::CU_QP_DIFF_WRAPAROUND_KHR.0;
        const B_PICTURE_INTRA_REFRESH_KHR = VideoEncodeH265CapabilityFlagBitsKHR::B_PICTURE_INTRA_REFRESH_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    pub const CU_QP_DIFF_WRAPAROUND_KHR: Self = Self(1 << 10);
    pub const B_PICTURE_INTRA_REFRESH_KHR: Self = Self(1 << 11);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265StdFlagsKHR: Flags {
        const SEPARATE_COLOR_PLANE_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::SEPARATE_COLOR_PLANE_FLAG_SET_KHR.0;
        const SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_KHR.0;
        const SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::SCALING_LIST_DATA_PRESENT_FLAG_SET_KHR.0;
        const PCM_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::PCM_ENABLED_FLAG_SET_KHR.0;
        const SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_KHR.0;
        const INIT_QP_MINUS26_KHR = VideoEncodeH265StdFlagBitsKHR::INIT_QP_MINUS26_KHR.0;
        const WEIGHTED_PRED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::WEIGHTED_PRED_FLAG_SET_KHR.0;
        const WEIGHTED_BIPRED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::WEIGHTED_BIPRED_FLAG_SET_KHR.0;
        const LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR = VideoEncodeH265StdFlagBitsKHR::LOG2_PARALLEL_MERGE_LEVEL_MINUS2_KHR.0;
        const SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::SIGN_DATA_HIDING_ENABLED_FLAG_SET_KHR.0;
        const TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::TRANSFORM_SKIP_ENABLED_FLAG_SET_KHR.0;
        const TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR = VideoEncodeH265StdFlagBitsKHR::TRANSFORM_SKIP_ENABLED_FLAG_UNSET_KHR.0;
        const PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_KHR.0;
        const TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::TRANSQUANT_BYPASS_ENABLED_FLAG_SET_KHR.0;
        const CONSTRAINED_INTRA_PRED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::CONSTRAINED_INTRA_PRED_FLAG_SET_KHR.0;
        const ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_KHR.0;
        const DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_KHR.0;
        const DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_KHR.0;
        const DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR = VideoEncodeH265StdFlagBitsKHR::DEPENDENT_SLICE_SEGMENT_FLAG_SET_KHR.0;
        const SLICE_QP_DELTA_KHR = VideoEncodeH265StdFlagBitsKHR::SLICE_QP_DELTA_KHR.0;
        const DIFFERENT_SLICE_QP_DELTA_KHR = VideoEncodeH265StdFlagBitsKHR::DIFFERENT_SLICE_QP_DELTA_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265RateControlFlagsKHR: Flags {
        const ATTEMPT_HRD_COMPLIANCE_KHR = VideoEncodeH265RateControlFlagBitsKHR::ATTEMPT_HRD_COMPLIANCE_KHR.0;
        const REGULAR_GOP_KHR = VideoEncodeH265RateControlFlagBitsKHR::REGULAR_GOP_KHR.0;
        const REFERENCE_PATTERN_FLAT_KHR = VideoEncodeH265RateControlFlagBitsKHR::REFERENCE_PATTERN_FLAT_KHR.0;
        const REFERENCE_PATTERN_DYADIC_KHR = VideoEncodeH265RateControlFlagBitsKHR::REFERENCE_PATTERN_DYADIC_KHR.0;
        const TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR = VideoEncodeH265RateControlFlagBitsKHR::TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH265RateControlFlagBitsKHR(u32);
impl VideoEncodeH265RateControlFlagBitsKHR {
    pub const ATTEMPT_HRD_COMPLIANCE_KHR: Self = Self(1 << 0);
    pub const REGULAR_GOP_KHR: Self = Self(1 << 1);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(1 << 2);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(1 << 3);
    pub const TEMPORAL_SUB_LAYER_PATTERN_DYADIC_KHR: Self = Self(1 << 4);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265CtbSizeFlagsKHR: Flags {
        const _16_KHR = VideoEncodeH265CtbSizeFlagBitsKHR::_16_KHR.0;
        const _32_KHR = VideoEncodeH265CtbSizeFlagBitsKHR::_32_KHR.0;
        const _64_KHR = VideoEncodeH265CtbSizeFlagBitsKHR::_64_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH265CtbSizeFlagBitsKHR(u32);
impl VideoEncodeH265CtbSizeFlagBitsKHR {
    pub const _16_KHR: Self = Self(1 << 0);
    pub const _32_KHR: Self = Self(1 << 1);
    pub const _64_KHR: Self = Self(1 << 2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265TransformBlockSizeFlagsKHR: Flags {
        const _4_KHR = VideoEncodeH265TransformBlockSizeFlagBitsKHR::_4_KHR.0;
        const _8_KHR = VideoEncodeH265TransformBlockSizeFlagBitsKHR::_8_KHR.0;
        const _16_KHR = VideoEncodeH265TransformBlockSizeFlagBitsKHR::_16_KHR.0;
        const _32_KHR = VideoEncodeH265TransformBlockSizeFlagBitsKHR::_32_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeH265TransformBlockSizeFlagBitsKHR(u32);
impl VideoEncodeH265TransformBlockSizeFlagBitsKHR {
    pub const _4_KHR: Self = Self(1 << 0);
    pub const _8_KHR: Self = Self(1 << 1);
    pub const _16_KHR: Self = Self(1 << 2);
    pub const _32_KHR: Self = Self(1 << 3);
}
