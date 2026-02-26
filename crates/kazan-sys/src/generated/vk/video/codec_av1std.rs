#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const STD_VIDEO_AV1_NUM_REF_FRAMES: u32 = 8;
pub const STD_VIDEO_AV1_REFS_PER_FRAME: u32 = 7;
pub const STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME: u32 = 8;
pub const STD_VIDEO_AV1_MAX_TILE_COLS: u32 = 64;
pub const STD_VIDEO_AV1_MAX_TILE_ROWS: u32 = 64;
pub const STD_VIDEO_AV1_MAX_SEGMENTS: u32 = 8;
pub const STD_VIDEO_AV1_SEG_LVL_MAX: u32 = 8;
pub const STD_VIDEO_AV1_PRIMARY_REF_NONE: u8 = 7;
pub const STD_VIDEO_AV1_SELECT_INTEGER_MV: u8 = 2;
pub const STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS: u32 = 2;
pub const STD_VIDEO_AV1_SKIP_MODE_FRAMES: u32 = 2;
pub const STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS: u32 = 4;
pub const STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
pub const STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS: u32 = 8;
pub const STD_VIDEO_AV1_MAX_NUM_PLANES: u32 = 3;
pub const STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS: u32 = 6;
pub const STD_VIDEO_AV1_MAX_NUM_Y_POINTS: u32 = 14;
pub const STD_VIDEO_AV1_MAX_NUM_CB_POINTS: u32 = 10;
pub const STD_VIDEO_AV1_MAX_NUM_CR_POINTS: u32 = 10;
pub const STD_VIDEO_AV1_MAX_NUM_POS_LUMA: u32 = 24;
pub const STD_VIDEO_AV1_MAX_NUM_POS_CHROMA: u32 = 25;
#[repr(C)]
pub struct StdVideoAV1ColorConfigFlags {
    pub mono_chrome: u32,
    pub color_range: u32,
    pub separate_uv_delta_q: u32,
    pub color_description_present_flag: u32,
    pub reserved: u32,
}
#[repr(C)]
pub struct StdVideoAV1ColorConfig {
    pub flags: StdVideoAV1ColorConfigFlags,
    pub bit_depth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub reserved1: u8,
    pub color_primaries: StdVideoAV1ColorPrimaries,
    pub transfer_characteristics: StdVideoAV1TransferCharacteristics,
    pub matrix_coefficients: StdVideoAV1MatrixCoefficients,
    pub chroma_sample_position: StdVideoAV1ChromaSamplePosition,
}
#[repr(C)]
pub struct StdVideoAV1TimingInfoFlags {
    pub equal_picture_interval: u32,
    pub reserved: u32,
}
#[repr(C)]
pub struct StdVideoAV1TimingInfo {
    pub flags: StdVideoAV1TimingInfoFlags,
    pub num_units_in_display_tick: u32,
    pub time_scale: u32,
    pub num_ticks_per_picture_minus_1: u32,
}
#[repr(C)]
pub struct StdVideoAV1SequenceHeaderFlags {
    pub still_picture: u32,
    pub reduced_still_picture_header: u32,
    pub use_128x128_superblock: u32,
    pub enable_filter_intra: u32,
    pub enable_intra_edge_filter: u32,
    pub enable_interintra_compound: u32,
    pub enable_masked_compound: u32,
    pub enable_warped_motion: u32,
    pub enable_dual_filter: u32,
    pub enable_order_hint: u32,
    pub enable_jnt_comp: u32,
    pub enable_ref_frame_mvs: u32,
    pub frame_id_numbers_present_flag: u32,
    pub enable_superres: u32,
    pub enable_cdef: u32,
    pub enable_restoration: u32,
    pub film_grain_params_present: u32,
    pub timing_info_present_flag: u32,
    pub initial_display_delay_present_flag: u32,
    pub reserved: u32,
}
#[repr(C)]
pub struct StdVideoAV1SequenceHeader<'a> {
    pub flags: StdVideoAV1SequenceHeaderFlags,
    pub seq_profile: StdVideoAV1Profile,
    pub frame_width_bits_minus_1: u8,
    pub frame_height_bits_minus_1: u8,
    pub max_frame_width_minus_1: u16,
    pub max_frame_height_minus_1: u16,
    pub delta_frame_id_length_minus_2: u8,
    pub additional_frame_id_length_minus_1: u8,
    pub order_hint_bits_minus_1: u8,
    pub seq_force_integer_mv: u8,
    pub seq_force_screen_content_tools: u8,
    pub reserved1: [u8; 5],
    pub p_color_config: *const StdVideoAV1ColorConfig,
    pub p_timing_info: *const StdVideoAV1TimingInfo,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct StdVideoAV1LoopFilterFlags {
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
}
#[repr(C)]
pub struct StdVideoAV1LoopFilter {
    pub flags: StdVideoAV1LoopFilterFlags,
    pub loop_filter_level: [u8; STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS as usize],
    pub loop_filter_sharpness: u8,
    pub update_ref_delta: u8,
    pub loop_filter_ref_deltas: [i8; STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME as usize],
    pub update_mode_delta: u8,
    pub loop_filter_mode_deltas: [i8; STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS as usize],
}
#[repr(C)]
pub struct StdVideoAV1QuantizationFlags {
    pub using_qmatrix: u32,
    pub diff_uv_delta: u32,
    pub reserved: u32,
}
#[repr(C)]
pub struct StdVideoAV1Quantization {
    pub flags: StdVideoAV1QuantizationFlags,
    pub base_q_idx: u8,
    pub delta_qy_dc: i8,
    pub delta_qu_dc: i8,
    pub delta_qu_ac: i8,
    pub delta_qv_dc: i8,
    pub delta_qv_ac: i8,
    pub qm_y: u8,
    pub qm_u: u8,
    pub qm_v: u8,
}
#[repr(C)]
pub struct StdVideoAV1Segmentation {
    pub feature_enabled: [u8; STD_VIDEO_AV1_MAX_SEGMENTS as usize],
    pub feature_data:
        [[i16; STD_VIDEO_AV1_SEG_LVL_MAX as usize]; STD_VIDEO_AV1_MAX_SEGMENTS as usize],
}
#[repr(C)]
pub struct StdVideoAV1TileInfoFlags {
    pub uniform_tile_spacing_flag: u32,
    pub reserved: u32,
}
#[repr(C)]
pub struct StdVideoAV1TileInfo<'a> {
    pub flags: StdVideoAV1TileInfoFlags,
    pub tile_cols: u8,
    pub tile_rows: u8,
    pub context_update_tile_id: u16,
    pub tile_size_bytes_minus_1: u8,
    pub reserved1: [u8; 7],
    pub p_mi_col_starts: *const u16,
    pub p_mi_row_starts: *const u16,
    pub p_width_in_sbs_minus1: *const u16,
    pub p_height_in_sbs_minus1: *const u16,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct StdVideoAV1CDEF {
    pub cdef_damping_minus_3: u8,
    pub cdef_bits: u8,
    pub cdef_y_pri_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_y_sec_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_uv_pri_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    pub cdef_uv_sec_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
}
#[repr(C)]
pub struct StdVideoAV1LoopRestoration {
    pub frame_restoration_type:
        [StdVideoAV1FrameRestorationType; STD_VIDEO_AV1_MAX_NUM_PLANES as usize],
    pub loop_restoration_size: [u16; STD_VIDEO_AV1_MAX_NUM_PLANES as usize],
}
#[repr(C)]
pub struct StdVideoAV1GlobalMotion {
    pub gm_type: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
    pub gm_params:
        [[i32; STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS as usize]; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
}
#[repr(C)]
pub struct StdVideoAV1FilmGrainFlags {
    pub chroma_scaling_from_luma: u32,
    pub overlap_flag: u32,
    pub clip_to_restricted_range: u32,
    pub update_grain: u32,
    pub reserved: u32,
}
#[repr(C)]
pub struct StdVideoAV1FilmGrain {
    pub flags: StdVideoAV1FilmGrainFlags,
    pub grain_scaling_minus_8: u8,
    pub ar_coeff_lag: u8,
    pub ar_coeff_shift_minus_6: u8,
    pub grain_scale_shift: u8,
    pub grain_seed: u16,
    pub film_grain_params_ref_idx: u8,
    pub num_y_points: u8,
    pub point_y_value: [u8; STD_VIDEO_AV1_MAX_NUM_Y_POINTS as usize],
    pub point_y_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_Y_POINTS as usize],
    pub num_cb_points: u8,
    pub point_cb_value: [u8; STD_VIDEO_AV1_MAX_NUM_CB_POINTS as usize],
    pub point_cb_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_CB_POINTS as usize],
    pub num_cr_points: u8,
    pub point_cr_value: [u8; STD_VIDEO_AV1_MAX_NUM_CR_POINTS as usize],
    pub point_cr_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_CR_POINTS as usize],
    pub ar_coeffs_y_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_LUMA as usize],
    pub ar_coeffs_cb_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_CHROMA as usize],
    pub ar_coeffs_cr_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_CHROMA as usize],
    pub cb_mult: u8,
    pub cb_luma_mult: u8,
    pub cb_offset: u16,
    pub cr_mult: u8,
    pub cr_luma_mult: u8,
    pub cr_offset: u16,
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1Profile(i32);
impl StdVideoAV1Profile {
    pub const MAIN: Self = Self(0);
    pub const HIGH: Self = Self(1);
    pub const PROFESSIONAL: Self = Self(2);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1Level(i32);
impl StdVideoAV1Level {
    pub const _2_0: Self = Self(0);
    pub const _2_1: Self = Self(1);
    pub const _2_2: Self = Self(2);
    pub const _2_3: Self = Self(3);
    pub const _3_0: Self = Self(4);
    pub const _3_1: Self = Self(5);
    pub const _3_2: Self = Self(6);
    pub const _3_3: Self = Self(7);
    pub const _4_0: Self = Self(8);
    pub const _4_1: Self = Self(9);
    pub const _4_2: Self = Self(10);
    pub const _4_3: Self = Self(11);
    pub const _5_0: Self = Self(12);
    pub const _5_1: Self = Self(13);
    pub const _5_2: Self = Self(14);
    pub const _5_3: Self = Self(15);
    pub const _6_0: Self = Self(16);
    pub const _6_1: Self = Self(17);
    pub const _6_2: Self = Self(18);
    pub const _6_3: Self = Self(19);
    pub const _7_0: Self = Self(20);
    pub const _7_1: Self = Self(21);
    pub const _7_2: Self = Self(22);
    pub const _7_3: Self = Self(23);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1FrameType(i32);
impl StdVideoAV1FrameType {
    pub const KEY: Self = Self(0);
    pub const INTER: Self = Self(1);
    pub const INTRA_ONLY: Self = Self(2);
    pub const SWITCH: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1ReferenceName(i32);
impl StdVideoAV1ReferenceName {
    pub const INTRA_FRAME: Self = Self(0);
    pub const LAST_FRAME: Self = Self(1);
    pub const LAST2_FRAME: Self = Self(2);
    pub const LAST3_FRAME: Self = Self(3);
    pub const GOLDEN_FRAME: Self = Self(4);
    pub const BWDREF_FRAME: Self = Self(5);
    pub const ALTREF2_FRAME: Self = Self(6);
    pub const ALTREF_FRAME: Self = Self(7);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1InterpolationFilter(i32);
impl StdVideoAV1InterpolationFilter {
    pub const EIGHTTAP: Self = Self(0);
    pub const EIGHTTAP_SMOOTH: Self = Self(1);
    pub const EIGHTTAP_SHARP: Self = Self(2);
    pub const BILINEAR: Self = Self(3);
    pub const SWITCHABLE: Self = Self(4);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1TxMode(i32);
impl StdVideoAV1TxMode {
    pub const ONLY_4X4: Self = Self(0);
    pub const LARGEST: Self = Self(1);
    pub const SELECT: Self = Self(2);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1FrameRestorationType(i32);
impl StdVideoAV1FrameRestorationType {
    pub const NONE: Self = Self(0);
    pub const WIENER: Self = Self(1);
    pub const SGRPROJ: Self = Self(2);
    pub const SWITCHABLE: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1ColorPrimaries(i32);
impl StdVideoAV1ColorPrimaries {
    pub const BT_709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    pub const BT_470_M: Self = Self(4);
    pub const BT_470_B_G: Self = Self(5);
    pub const BT_601: Self = Self(6);
    pub const SMPTE_240: Self = Self(7);
    pub const GENERIC_FILM: Self = Self(8);
    pub const BT_2020: Self = Self(9);
    pub const XYZ: Self = Self(10);
    pub const SMPTE_431: Self = Self(11);
    pub const SMPTE_432: Self = Self(12);
    pub const EBU_3213: Self = Self(22);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1TransferCharacteristics(i32);
impl StdVideoAV1TransferCharacteristics {
    pub const RESERVED_0: Self = Self(0);
    pub const BT_709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    pub const RESERVED_3: Self = Self(3);
    pub const BT_470_M: Self = Self(4);
    pub const BT_470_B_G: Self = Self(5);
    pub const BT_601: Self = Self(6);
    pub const SMPTE_240: Self = Self(7);
    pub const LINEAR: Self = Self(8);
    pub const LOG_100: Self = Self(9);
    pub const LOG_100_SQRT10: Self = Self(10);
    pub const IEC_61966: Self = Self(11);
    pub const BT_1361: Self = Self(12);
    pub const SRGB: Self = Self(13);
    pub const BT_2020_10_BIT: Self = Self(14);
    pub const BT_2020_12_BIT: Self = Self(15);
    pub const SMPTE_2084: Self = Self(16);
    pub const SMPTE_428: Self = Self(17);
    pub const HLG: Self = Self(18);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1MatrixCoefficients(i32);
impl StdVideoAV1MatrixCoefficients {
    pub const IDENTITY: Self = Self(0);
    pub const BT_709: Self = Self(1);
    pub const UNSPECIFIED: Self = Self(2);
    pub const RESERVED_3: Self = Self(3);
    pub const FCC: Self = Self(4);
    pub const BT_470_B_G: Self = Self(5);
    pub const BT_601: Self = Self(6);
    pub const SMPTE_240: Self = Self(7);
    pub const SMPTE_YCGCO: Self = Self(8);
    pub const BT_2020_NCL: Self = Self(9);
    pub const BT_2020_CL: Self = Self(10);
    pub const SMPTE_2085: Self = Self(11);
    pub const CHROMAT_NCL: Self = Self(12);
    pub const CHROMAT_CL: Self = Self(13);
    pub const ICTCP: Self = Self(14);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoAV1ChromaSamplePosition(i32);
impl StdVideoAV1ChromaSamplePosition {
    pub const UNKNOWN: Self = Self(0);
    pub const VERTICAL: Self = Self(1);
    pub const COLOCATED: Self = Self(2);
    pub const RESERVED: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
