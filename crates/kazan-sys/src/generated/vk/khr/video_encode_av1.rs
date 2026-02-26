#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1CapabilitiesKHR<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1CapabilitiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_CAPABILITIES_KHR,
            p_next: core::ptr::null_mut(),
            flags: Default::default(),
            max_level: Default::default(),
            coded_picture_alignment: Default::default(),
            max_tiles: Default::default(),
            min_tile_size: Default::default(),
            max_tile_size: Default::default(),
            superblock_sizes: Default::default(),
            max_single_reference_count: Default::default(),
            single_reference_name_mask: Default::default(),
            max_unidirectional_compound_reference_count: Default::default(),
            max_unidirectional_compound_group1_reference_count: Default::default(),
            unidirectional_compound_reference_name_mask: Default::default(),
            max_bidirectional_compound_reference_count: Default::default(),
            max_bidirectional_compound_group1_reference_count: Default::default(),
            max_bidirectional_compound_group2_reference_count: Default::default(),
            bidirectional_compound_reference_name_mask: Default::default(),
            max_temporal_layer_count: Default::default(),
            max_spatial_layer_count: Default::default(),
            max_operating_points: Default::default(),
            min_q_index: Default::default(),
            max_q_index: Default::default(),
            prefers_gop_remaining_frames: Default::default(),
            requires_gop_remaining_frames: Default::default(),
            std_syntax_flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1QualityLevelPropertiesKHR<'a> {
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
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1QualityLevelPropertiesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_QUALITY_LEVEL_PROPERTIES_KHR,
            p_next: core::ptr::null_mut(),
            preferred_rate_control_flags: Default::default(),
            preferred_gop_frame_count: Default::default(),
            preferred_key_frame_period: Default::default(),
            preferred_consecutive_bipredictive_frame_count: Default::default(),
            preferred_temporal_layer_count: Default::default(),
            preferred_constant_q_index: Default::default(),
            preferred_max_single_reference_count: Default::default(),
            preferred_single_reference_name_mask: Default::default(),
            preferred_max_unidirectional_compound_reference_count: Default::default(),
            preferred_max_unidirectional_compound_group1_reference_count: Default::default(),
            preferred_unidirectional_compound_reference_name_mask: Default::default(),
            preferred_max_bidirectional_compound_reference_count: Default::default(),
            preferred_max_bidirectional_compound_group1_reference_count: Default::default(),
            preferred_max_bidirectional_compound_group2_reference_count: Default::default(),
            preferred_bidirectional_compound_reference_name_mask: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVideoEncodeAV1FeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub video_encode_av1: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceVideoEncodeAV1FeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_VIDEO_ENCODE_AV1_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            video_encode_av1: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1SessionCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level: Bool32,
    pub max_level: StdVideoAV1Level,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1SessionCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_SESSION_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            use_max_level: Default::default(),
            max_level: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1SessionParametersCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_sequence_header: *const StdVideoAV1SequenceHeader<'a>,
    pub p_std_decoder_model_info: *const StdVideoEncodeAV1DecoderModelInfo,
    pub std_operating_point_count: u32,
    pub p_std_operating_points: *const StdVideoEncodeAV1OperatingPointInfo,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1SessionParametersCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_SESSION_PARAMETERS_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_sequence_header: core::ptr::null(),
            p_std_decoder_model_info: core::ptr::null(),
            std_operating_point_count: Default::default(),
            p_std_operating_points: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1DpbSlotInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoEncodeAV1ReferenceInfo<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1DpbSlotInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_DPB_SLOT_INFO_KHR,
            p_next: core::ptr::null(),
            p_std_reference_info: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1PictureInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub prediction_mode: VideoEncodeAV1PredictionModeKHR,
    pub rate_control_group: VideoEncodeAV1RateControlGroupKHR,
    pub constant_q_index: u32,
    pub p_std_picture_info: *const StdVideoEncodeAV1PictureInfo<'a>,
    pub reference_name_slot_indices: [i32; MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
    pub primary_reference_cdf_only: Bool32,
    pub generate_obu_extension_header: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1PictureInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_PICTURE_INFO_KHR,
            p_next: core::ptr::null(),
            prediction_mode: Default::default(),
            rate_control_group: Default::default(),
            constant_q_index: Default::default(),
            p_std_picture_info: core::ptr::null(),
            reference_name_slot_indices: [Default::default(); _],
            primary_reference_cdf_only: Default::default(),
            generate_obu_extension_header: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1ProfileInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile: StdVideoAV1Profile,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1ProfileInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_PROFILE_INFO_KHR,
            p_next: core::ptr::null(),
            std_profile: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1RateControlInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeAV1RateControlFlagsKHR,
    pub gop_frame_count: u32,
    pub key_frame_period: u32,
    pub consecutive_bipredictive_frame_count: u32,
    pub temporal_layer_count: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1RateControlInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_RATE_CONTROL_INFO_KHR,
            p_next: core::ptr::null(),
            flags: Default::default(),
            gop_frame_count: Default::default(),
            key_frame_period: Default::default(),
            consecutive_bipredictive_frame_count: Default::default(),
            temporal_layer_count: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VideoEncodeAV1QIndexKHR {
    pub intra_q_index: u32,
    pub predictive_q_index: u32,
    pub bipredictive_q_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VideoEncodeAV1FrameSizeKHR {
    pub intra_frame_size: u32,
    pub predictive_frame_size: u32,
    pub bipredictive_frame_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1GopRemainingFrameInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_intra: u32,
    pub gop_remaining_predictive: u32,
    pub gop_remaining_bipredictive: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1GopRemainingFrameInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_GOP_REMAINING_FRAME_INFO_KHR,
            p_next: core::ptr::null(),
            use_gop_remaining_frames: Default::default(),
            gop_remaining_intra: Default::default(),
            gop_remaining_predictive: Default::default(),
            gop_remaining_bipredictive: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeAV1RateControlLayerInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_min_q_index: Bool32,
    pub min_q_index: VideoEncodeAV1QIndexKHR,
    pub use_max_q_index: Bool32,
    pub max_q_index: VideoEncodeAV1QIndexKHR,
    pub use_max_frame_size: Bool32,
    pub max_frame_size: VideoEncodeAV1FrameSizeKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for VideoEncodeAV1RateControlLayerInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::VIDEO_ENCODE_AV1_RATE_CONTROL_LAYER_INFO_KHR,
            p_next: core::ptr::null(),
            use_min_q_index: Default::default(),
            min_q_index: Default::default(),
            use_max_q_index: Default::default(),
            max_q_index: Default::default(),
            use_max_frame_size: Default::default(),
            max_frame_size: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1PredictionModeKHR(i32);
impl VideoEncodeAV1PredictionModeKHR {
    pub const INTRA_ONLY_KHR: Self = Self(0);
    pub const SINGLE_REFERENCE_KHR: Self = Self(1);
    pub const UNIDIRECTIONAL_COMPOUND_KHR: Self = Self(2);
    pub const BIDIRECTIONAL_COMPOUND_KHR: Self = Self(3);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1RateControlGroupKHR(i32);
impl VideoEncodeAV1RateControlGroupKHR {
    pub const INTRA_KHR: Self = Self(0);
    pub const PREDICTIVE_KHR: Self = Self(1);
    pub const BIPREDICTIVE_KHR: Self = Self(2);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeAV1CapabilityFlagsKHR: Flags {
        const PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR = VideoEncodeAV1CapabilityFlagBitsKHR::PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR.0;
        const GENERATE_OBU_EXTENSION_HEADER_KHR = VideoEncodeAV1CapabilityFlagBitsKHR::GENERATE_OBU_EXTENSION_HEADER_KHR.0;
        const PRIMARY_REFERENCE_CDF_ONLY_KHR = VideoEncodeAV1CapabilityFlagBitsKHR::PRIMARY_REFERENCE_CDF_ONLY_KHR.0;
        const FRAME_SIZE_OVERRIDE_KHR = VideoEncodeAV1CapabilityFlagBitsKHR::FRAME_SIZE_OVERRIDE_KHR.0;
        const MOTION_VECTOR_SCALING_KHR = VideoEncodeAV1CapabilityFlagBitsKHR::MOTION_VECTOR_SCALING_KHR.0;
        const COMPOUND_PREDICTION_INTRA_REFRESH_KHR = VideoEncodeAV1CapabilityFlagBitsKHR::COMPOUND_PREDICTION_INTRA_REFRESH_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1CapabilityFlagBitsKHR(u32);
impl VideoEncodeAV1CapabilityFlagBitsKHR {
    pub const PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_KHR: Self = Self(1 << 0);
    pub const GENERATE_OBU_EXTENSION_HEADER_KHR: Self = Self(1 << 1);
    pub const PRIMARY_REFERENCE_CDF_ONLY_KHR: Self = Self(1 << 2);
    pub const FRAME_SIZE_OVERRIDE_KHR: Self = Self(1 << 3);
    pub const MOTION_VECTOR_SCALING_KHR: Self = Self(1 << 4);
    pub const COMPOUND_PREDICTION_INTRA_REFRESH_KHR: Self = Self(1 << 5);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeAV1StdFlagsKHR: Flags {
        const UNIFORM_TILE_SPACING_FLAG_SET_KHR = VideoEncodeAV1StdFlagBitsKHR::UNIFORM_TILE_SPACING_FLAG_SET_KHR.0;
        const SKIP_MODE_PRESENT_UNSET_KHR = VideoEncodeAV1StdFlagBitsKHR::SKIP_MODE_PRESENT_UNSET_KHR.0;
        const PRIMARY_REF_FRAME_KHR = VideoEncodeAV1StdFlagBitsKHR::PRIMARY_REF_FRAME_KHR.0;
        const DELTA_Q_KHR = VideoEncodeAV1StdFlagBitsKHR::DELTA_Q_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1StdFlagBitsKHR(u32);
impl VideoEncodeAV1StdFlagBitsKHR {
    pub const UNIFORM_TILE_SPACING_FLAG_SET_KHR: Self = Self(1 << 0);
    pub const SKIP_MODE_PRESENT_UNSET_KHR: Self = Self(1 << 1);
    pub const PRIMARY_REF_FRAME_KHR: Self = Self(1 << 2);
    pub const DELTA_Q_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeAV1RateControlFlagsKHR: Flags {
        const REGULAR_GOP_KHR = VideoEncodeAV1RateControlFlagBitsKHR::REGULAR_GOP_KHR.0;
        const TEMPORAL_LAYER_PATTERN_DYADIC_KHR = VideoEncodeAV1RateControlFlagBitsKHR::TEMPORAL_LAYER_PATTERN_DYADIC_KHR.0;
        const REFERENCE_PATTERN_FLAT_KHR = VideoEncodeAV1RateControlFlagBitsKHR::REFERENCE_PATTERN_FLAT_KHR.0;
        const REFERENCE_PATTERN_DYADIC_KHR = VideoEncodeAV1RateControlFlagBitsKHR::REFERENCE_PATTERN_DYADIC_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1RateControlFlagBitsKHR(u32);
impl VideoEncodeAV1RateControlFlagBitsKHR {
    pub const REGULAR_GOP_KHR: Self = Self(1 << 0);
    pub const TEMPORAL_LAYER_PATTERN_DYADIC_KHR: Self = Self(1 << 1);
    pub const REFERENCE_PATTERN_FLAT_KHR: Self = Self(1 << 2);
    pub const REFERENCE_PATTERN_DYADIC_KHR: Self = Self(1 << 3);
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VideoEncodeAV1SuperblockSizeFlagsKHR: Flags {
        const _64_KHR = VideoEncodeAV1SuperblockSizeFlagBitsKHR::_64_KHR.0;
        const _128_KHR = VideoEncodeAV1SuperblockSizeFlagBitsKHR::_128_KHR.0;
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VideoEncodeAV1SuperblockSizeFlagBitsKHR(u32);
impl VideoEncodeAV1SuperblockSizeFlagBitsKHR {
    pub const _64_KHR: Self = Self(1 << 0);
    pub const _128_KHR: Self = Self(1 << 1);
}
