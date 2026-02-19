#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoEncodeAV1CapabilityFlagsKHR,
    pub max_level: StdVideoAV1Level,
    pub coded_picture_alignment: Extent2D,
    pub max_tiles: Extent2D,
    pub min_tile_size: Extent2D,
    pub max_tile_size: Extent2D,
    pub superblock_sizes: VideoEncodeAV1SuperblockSizeFlagsKHR,
    pub max_single_reference_count: u32,
    pub single_reference_name_mask: u32,
    pub max_unidirectional_compound_reference_count: u32,
    pub max_unidirectional_compound_group1_reference_count: u32,
    pub unidirectional_compound_reference_name_mask: u32,
    pub max_bidirectional_compound_reference_count: u32,
    pub max_bidirectional_compound_group1_reference_count: u32,
    pub max_bidirectional_compound_group2_reference_count: u32,
    pub bidirectional_compound_reference_name_mask: u32,
    pub max_temporal_layer_count: u32,
    pub max_spatial_layer_count: u32,
    pub max_operating_points: u32,
    pub min_q_index: u32,
    pub max_q_index: u32,
    pub prefers_gop_remaining_frames: Bool32,
    pub requires_gop_remaining_frames: Bool32,
    pub std_syntax_flags: VideoEncodeAV1StdFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1QualityLevelPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub preferred_rate_control_flags: VideoEncodeAV1RateControlFlagsKHR,
    pub preferred_gop_frame_count: u32,
    pub preferred_key_frame_period: u32,
    pub preferred_consecutive_bipredictive_frame_count: u32,
    pub preferred_temporal_layer_count: u32,
    pub preferred_constant_q_index: VideoEncodeAV1QIndexKHR,
    pub preferred_max_single_reference_count: u32,
    pub preferred_single_reference_name_mask: u32,
    pub preferred_max_unidirectional_compound_reference_count: u32,
    pub preferred_max_unidirectional_compound_group1_reference_count: u32,
    pub preferred_unidirectional_compound_reference_name_mask: u32,
    pub preferred_max_bidirectional_compound_reference_count: u32,
    pub preferred_max_bidirectional_compound_group1_reference_count: u32,
    pub preferred_max_bidirectional_compound_group2_reference_count: u32,
    pub preferred_bidirectional_compound_reference_name_mask: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeAV1FeaturesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_av1: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1SessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level: Bool32,
    pub max_level: StdVideoAV1Level,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const StdVideoAV1SequenceHeader,
    pub p_std_decoder_model_info: *const StdVideoEncodeAV1DecoderModelInfo,
    pub std_operating_point_count: u32,
    pub p_std_operating_points: *const StdVideoEncodeAV1OperatingPointInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoEncodeAV1ReferenceInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub prediction_mode: VideoEncodeAV1PredictionModeKHR,
    pub rate_control_group: VideoEncodeAV1RateControlGroupKHR,
    pub constant_q_index: u32,
    pub p_std_picture_info: *const StdVideoEncodeAV1PictureInfo,
    pub reference_name_slot_indices: [i32; MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
    pub primary_reference_cdf_only: Bool32,
    pub generate_obu_extension_header: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: StdVideoAV1Profile,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1RateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeAV1RateControlFlagsKHR,
    pub gop_frame_count: u32,
    pub key_frame_period: u32,
    pub consecutive_bipredictive_frame_count: u32,
    pub temporal_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1QIndexKHR {
    pub intra_q_index: u32,
    pub predictive_q_index: u32,
    pub bipredictive_q_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1FrameSizeKHR {
    pub intra_frame_size: u32,
    pub predictive_frame_size: u32,
    pub bipredictive_frame_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1GopRemainingFrameInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_intra: u32,
    pub gop_remaining_predictive: u32,
    pub gop_remaining_bipredictive: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1RateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_min_q_index: Bool32,
    pub min_q_index: VideoEncodeAV1QIndexKHR,
    pub use_max_q_index: Bool32,
    pub max_q_index: VideoEncodeAV1QIndexKHR,
    pub use_max_frame_size: Bool32,
    pub max_frame_size: VideoEncodeAV1FrameSizeKHR,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1PredictionModeKHR(i32);
impl VideoEncodeAV1PredictionModeKHR {
    pub const INTRA_ONLY_KHR: Self = Self(0);
    pub const SINGLE_REFERENCE_KHR: Self = Self(1);
    pub const UNIDIRECTIONAL_COMPOUND_KHR: Self = Self(2);
    pub const BIDIRECTIONAL_COMPOUND_KHR: Self = Self(3);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1RateControlGroupKHR(i32);
impl VideoEncodeAV1RateControlGroupKHR {
    pub const INTRA_KHR: Self = Self(0);
    pub const PREDICTIVE_KHR: Self = Self(1);
    pub const BIPREDICTIVE_KHR: Self = Self(2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeAV1CapabilityFlagsKHR: Flags {
        const PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR = 1 << 0;
        const GENERATE_OBU_EXTENSION_HEADER_KHR = 1 << 1;
        const PRIMARY_REFERENCE_CDF_ONLY_KHR = 1 << 2;
        const FRAME_SIZE_OVERRIDE_KHR = 1 << 3;
        const MOTION_VECTOR_SCALING_KHR = 1 << 4;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeAV1StdFlagsKHR: Flags {
        const UNIFORM_TILE_SPACING_FLAG_SET_KHR = 1 << 0;
        const SKIP_MODE_PRESENT_UNSET_KHR = 1 << 1;
        const PRIMARY_REF_FRAME_KHR = 1 << 2;
        const DELTA_Q_KHR = 1 << 3;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeAV1RateControlFlagsKHR: Flags {
        const REGULAR_GOP_KHR = 1 << 0;
        const TEMPORAL_LAYER_PATTERN_DYADIC_KHR = 1 << 1;
        const REFERENCE_PATTERN_FLAT_KHR = 1 << 2;
        const REFERENCE_PATTERN_DYADIC_KHR = 1 << 3;
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeAV1SuperblockSizeFlagsKHR: Flags {
        const _64_KHR = 1 << 0;
        const _128_KHR = 1 << 1;
    }
}
