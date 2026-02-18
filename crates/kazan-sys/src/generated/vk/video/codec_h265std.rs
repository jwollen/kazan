#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub const STD_VIDEO_H265_CPB_CNT_LIST_SIZE: u32 = 32;
pub const STD_VIDEO_H265_SUBLAYERS_LIST_SIZE: u32 = 7;
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS: u32 = 2;
pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE: u32 = 6;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: u32 = 19;
pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: u32 = 21;
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: u32 = 3;
pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: u32 = 128;
pub const STD_VIDEO_H265_MAX_NUM_LIST_REF: u32 = 15;
pub const STD_VIDEO_H265_MAX_CHROMA_PLANES: u32 = 2;
pub const STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = 64;
pub const STD_VIDEO_H265_MAX_DPB_SIZE: u32 = 16;
pub const STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS: u32 = 32;
pub const STD_VIDEO_H265_MAX_LONG_TERM_PICS: u32 = 16;
pub const STD_VIDEO_H265_MAX_DELTA_POC: u32 = 48;
pub const STD_VIDEO_H265_NO_REFERENCE_PICTURE: u8 = 0xF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265ProfileTierLevelFlags {
    pub general_tier_flag: u32,
    pub general_progressive_source_flag: u32,
    pub general_interlaced_source_flag: u32,
    pub general_non_packed_constraint_flag: u32,
    pub general_frame_only_constraint_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265ProfileTierLevel {
    pub flags: StdVideoH265ProfileTierLevelFlags,
    pub general_profile_idc: StdVideoH265ProfileIdc,
    pub general_level_idc: StdVideoH265LevelIdc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265DecPicBufMgr {
    pub max_latency_increase_plus1: u32,
    pub max_dec_pic_buffering_minus1: u8,
    pub max_num_reorder_pics: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265SubLayerHrdParameters {
    pub bit_rate_value_minus1: u32,
    pub cpb_size_value_minus1: u32,
    pub cpb_size_du_value_minus1: u32,
    pub bit_rate_du_value_minus1: u32,
    pub cbr_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265HrdFlags {
    pub nal_hrd_parameters_present_flag: u32,
    pub vcl_hrd_parameters_present_flag: u32,
    pub sub_pic_hrd_params_present_flag: u32,
    pub sub_pic_cpb_params_in_pic_timing_sei_flag: u32,
    pub fixed_pic_rate_general_flag: u32,
    pub fixed_pic_rate_within_cvs_flag: u32,
    pub low_delay_hrd_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265HrdParameters {
    pub flags: StdVideoH265HrdFlags,
    pub tick_divisor_minus2: u8,
    pub du_cpb_removal_delay_increment_length_minus1: u8,
    pub dpb_output_delay_du_length_minus1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub cpb_size_du_scale: u8,
    pub initial_cpb_removal_delay_length_minus1: u8,
    pub au_cpb_removal_delay_length_minus1: u8,
    pub dpb_output_delay_length_minus1: u8,
    pub cpb_cnt_minus1: u8,
    pub elemental_duration_in_tc_minus1: u16,
    pub reserved: u16,
    pub p_sub_layer_hrd_parameters_nal: *const StdVideoH265SubLayerHrdParameters,
    pub p_sub_layer_hrd_parameters_vcl: *const StdVideoH265SubLayerHrdParameters,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265VpsFlags {
    pub vps_temporal_id_nesting_flag: u32,
    pub vps_sub_layer_ordering_info_present_flag: u32,
    pub vps_timing_info_present_flag: u32,
    pub vps_poc_proportional_to_timing_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265VideoParameterSet {
    pub flags: StdVideoH265VpsFlags,
    pub vps_video_parameter_set_id: u8,
    pub vps_max_sub_layers_minus1: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub vps_num_units_in_tick: u32,
    pub vps_time_scale: u32,
    pub vps_num_ticks_poc_diff_one_minus1: u32,
    pub reserved3: u32,
    pub p_dec_pic_buf_mgr: *const StdVideoH265DecPicBufMgr,
    pub p_hrd_parameters: *const StdVideoH265HrdParameters,
    pub p_profile_tier_level: *const StdVideoH265ProfileTierLevel,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265ScalingLists {
    pub scaling_list4x4: u8,
    pub scaling_list8x8: u8,
    pub scaling_list16x16: u8,
    pub scaling_list32x32: u8,
    pub scaling_list_dc_coef16x16: u8,
    pub scaling_list_dc_coef32x32: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265ShortTermRefPicSetFlags {
    pub inter_ref_pic_set_prediction_flag: u32,
    pub delta_rps_sign: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265ShortTermRefPicSet {
    pub flags: StdVideoH265ShortTermRefPicSetFlags,
    pub delta_idx_minus1: u32,
    pub use_delta_flag: u16,
    pub abs_delta_rps_minus1: u16,
    pub used_by_curr_pic_flag: u16,
    pub used_by_curr_pic_s0_flag: u16,
    pub used_by_curr_pic_s1_flag: u16,
    pub reserved1: u16,
    pub reserved2: u8,
    pub reserved3: u8,
    pub num_negative_pics: u8,
    pub num_positive_pics: u8,
    pub delta_poc_s0_minus1: u16,
    pub delta_poc_s1_minus1: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265LongTermRefPicsSps {
    pub used_by_curr_pic_lt_sps_flag: u32,
    pub lt_ref_pic_poc_lsb_sps: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265SpsVuiFlags {
    pub aspect_ratio_info_present_flag: u32,
    pub overscan_info_present_flag: u32,
    pub overscan_appropriate_flag: u32,
    pub video_signal_type_present_flag: u32,
    pub video_full_range_flag: u32,
    pub colour_description_present_flag: u32,
    pub chroma_loc_info_present_flag: u32,
    pub neutral_chroma_indication_flag: u32,
    pub field_seq_flag: u32,
    pub frame_field_info_present_flag: u32,
    pub default_display_window_flag: u32,
    pub vui_timing_info_present_flag: u32,
    pub vui_poc_proportional_to_timing_flag: u32,
    pub vui_hrd_parameters_present_flag: u32,
    pub bitstream_restriction_flag: u32,
    pub tiles_fixed_structure_flag: u32,
    pub motion_vectors_over_pic_boundaries_flag: u32,
    pub restricted_ref_pic_lists_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265SequenceParameterSetVui {
    pub flags: StdVideoH265SpsVuiFlags,
    pub aspect_ratio_idc: StdVideoH265AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coeffs: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub def_disp_win_left_offset: u16,
    pub def_disp_win_right_offset: u16,
    pub def_disp_win_top_offset: u16,
    pub def_disp_win_bottom_offset: u16,
    pub vui_num_units_in_tick: u32,
    pub vui_time_scale: u32,
    pub vui_num_ticks_poc_diff_one_minus1: u32,
    pub min_spatial_segmentation_idc: u16,
    pub reserved3: u16,
    pub max_bytes_per_pic_denom: u8,
    pub max_bits_per_min_cu_denom: u8,
    pub log2_max_mv_length_horizontal: u8,
    pub log2_max_mv_length_vertical: u8,
    pub p_hrd_parameters: *const StdVideoH265HrdParameters,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265PredictorPaletteEntries {
    pub predictor_palette_entries: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265SpsFlags {
    pub sps_temporal_id_nesting_flag: u32,
    pub separate_colour_plane_flag: u32,
    pub conformance_window_flag: u32,
    pub sps_sub_layer_ordering_info_present_flag: u32,
    pub scaling_list_enabled_flag: u32,
    pub sps_scaling_list_data_present_flag: u32,
    pub amp_enabled_flag: u32,
    pub sample_adaptive_offset_enabled_flag: u32,
    pub pcm_enabled_flag: u32,
    pub pcm_loop_filter_disabled_flag: u32,
    pub long_term_ref_pics_present_flag: u32,
    pub sps_temporal_mvp_enabled_flag: u32,
    pub strong_intra_smoothing_enabled_flag: u32,
    pub vui_parameters_present_flag: u32,
    pub sps_extension_present_flag: u32,
    pub sps_range_extension_flag: u32,
    pub transform_skip_rotation_enabled_flag: u32,
    pub transform_skip_context_enabled_flag: u32,
    pub implicit_rdpcm_enabled_flag: u32,
    pub explicit_rdpcm_enabled_flag: u32,
    pub extended_precision_processing_flag: u32,
    pub intra_smoothing_disabled_flag: u32,
    pub high_precision_offsets_enabled_flag: u32,
    pub persistent_rice_adaptation_enabled_flag: u32,
    pub cabac_bypass_alignment_enabled_flag: u32,
    pub sps_scc_extension_flag: u32,
    pub sps_curr_pic_ref_enabled_flag: u32,
    pub palette_mode_enabled_flag: u32,
    pub sps_palette_predictor_initializers_present_flag: u32,
    pub intra_boundary_filtering_disabled_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265SequenceParameterSet {
    pub flags: StdVideoH265SpsFlags,
    pub chroma_format_idc: StdVideoH265ChromaFormatIdc,
    pub pic_width_in_luma_samples: u32,
    pub pic_height_in_luma_samples: u32,
    pub sps_video_parameter_set_id: u8,
    pub sps_max_sub_layers_minus1: u8,
    pub sps_seq_parameter_set_id: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub log2_min_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_luma_coding_block_size: u8,
    pub log2_min_luma_transform_block_size_minus2: u8,
    pub log2_diff_max_min_luma_transform_block_size: u8,
    pub max_transform_hierarchy_depth_inter: u8,
    pub max_transform_hierarchy_depth_intra: u8,
    pub num_short_term_ref_pic_sets: u8,
    pub num_long_term_ref_pics_sps: u8,
    pub pcm_sample_bit_depth_luma_minus1: u8,
    pub pcm_sample_bit_depth_chroma_minus1: u8,
    pub log2_min_pcm_luma_coding_block_size_minus3: u8,
    pub log2_diff_max_min_pcm_luma_coding_block_size: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub palette_max_size: u8,
    pub delta_palette_max_predictor_size: u8,
    pub motion_vector_resolution_control_idc: u8,
    pub sps_num_palette_predictor_initializers_minus1: u8,
    pub conf_win_left_offset: u32,
    pub conf_win_right_offset: u32,
    pub conf_win_top_offset: u32,
    pub conf_win_bottom_offset: u32,
    pub p_profile_tier_level: *const StdVideoH265ProfileTierLevel,
    pub p_dec_pic_buf_mgr: *const StdVideoH265DecPicBufMgr,
    pub p_scaling_lists: *const StdVideoH265ScalingLists,
    pub p_short_term_ref_pic_set: *const StdVideoH265ShortTermRefPicSet,
    pub p_long_term_ref_pics_sps: *const StdVideoH265LongTermRefPicsSps,
    pub p_sequence_parameter_set_vui: *const StdVideoH265SequenceParameterSetVui,
    pub p_predictor_palette_entries: *const StdVideoH265PredictorPaletteEntries,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265PpsFlags {
    pub dependent_slice_segments_enabled_flag: u32,
    pub output_flag_present_flag: u32,
    pub sign_data_hiding_enabled_flag: u32,
    pub cabac_init_present_flag: u32,
    pub constrained_intra_pred_flag: u32,
    pub transform_skip_enabled_flag: u32,
    pub cu_qp_delta_enabled_flag: u32,
    pub pps_slice_chroma_qp_offsets_present_flag: u32,
    pub weighted_pred_flag: u32,
    pub weighted_bipred_flag: u32,
    pub transquant_bypass_enabled_flag: u32,
    pub tiles_enabled_flag: u32,
    pub entropy_coding_sync_enabled_flag: u32,
    pub uniform_spacing_flag: u32,
    pub loop_filter_across_tiles_enabled_flag: u32,
    pub pps_loop_filter_across_slices_enabled_flag: u32,
    pub deblocking_filter_control_present_flag: u32,
    pub deblocking_filter_override_enabled_flag: u32,
    pub pps_deblocking_filter_disabled_flag: u32,
    pub pps_scaling_list_data_present_flag: u32,
    pub lists_modification_present_flag: u32,
    pub slice_segment_header_extension_present_flag: u32,
    pub pps_extension_present_flag: u32,
    pub cross_component_prediction_enabled_flag: u32,
    pub chroma_qp_offset_list_enabled_flag: u32,
    pub pps_curr_pic_ref_enabled_flag: u32,
    pub residual_adaptive_colour_transform_enabled_flag: u32,
    pub pps_slice_act_qp_offsets_present_flag: u32,
    pub pps_palette_predictor_initializers_present_flag: u32,
    pub monochrome_palette_flag: u32,
    pub pps_range_extension_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH265PictureParameterSet {
    pub flags: StdVideoH265PpsFlags,
    pub pps_pic_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub sps_video_parameter_set_id: u8,
    pub num_extra_slice_header_bits: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub init_qp_minus26: i8,
    pub diff_cu_qp_delta_depth: u8,
    pub pps_cb_qp_offset: i8,
    pub pps_cr_qp_offset: i8,
    pub pps_beta_offset_div2: i8,
    pub pps_tc_offset_div2: i8,
    pub log2_parallel_merge_level_minus2: u8,
    pub log2_max_transform_skip_block_size_minus2: u8,
    pub diff_cu_chroma_qp_offset_depth: u8,
    pub chroma_qp_offset_list_len_minus1: u8,
    pub cb_qp_offset_list: i8,
    pub cr_qp_offset_list: i8,
    pub log2_sao_offset_scale_luma: u8,
    pub log2_sao_offset_scale_chroma: u8,
    pub pps_act_y_qp_offset_plus5: i8,
    pub pps_act_cb_qp_offset_plus5: i8,
    pub pps_act_cr_qp_offset_plus3: i8,
    pub pps_num_palette_predictor_initializers: u8,
    pub luma_bit_depth_entry_minus8: u8,
    pub chroma_bit_depth_entry_minus8: u8,
    pub num_tile_columns_minus1: u8,
    pub num_tile_rows_minus1: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub column_width_minus1: u16,
    pub row_height_minus1: u16,
    pub reserved3: u32,
    pub p_scaling_lists: *const StdVideoH265ScalingLists,
    pub p_predictor_palette_entries: *const StdVideoH265PredictorPaletteEntries,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH265ChromaFormatIdc(i32);
impl StdVideoH265ChromaFormatIdc {
    pub const MONOCHROME: Self = Self(0);
    pub const _420: Self = Self(1);
    pub const _422: Self = Self(2);
    pub const _444: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH265ProfileIdc(i32);
impl StdVideoH265ProfileIdc {
    pub const MAIN: Self = Self(1);
    pub const MAIN_10: Self = Self(2);
    pub const MAIN_STILL_PICTURE: Self = Self(3);
    pub const FORMAT_RANGE_EXTENSIONS: Self = Self(4);
    pub const SCC_EXTENSIONS: Self = Self(9);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH265LevelIdc(i32);
impl StdVideoH265LevelIdc {
    pub const _1_0: Self = Self(0);
    pub const _2_0: Self = Self(1);
    pub const _2_1: Self = Self(2);
    pub const _3_0: Self = Self(3);
    pub const _3_1: Self = Self(4);
    pub const _4_0: Self = Self(5);
    pub const _4_1: Self = Self(6);
    pub const _5_0: Self = Self(7);
    pub const _5_1: Self = Self(8);
    pub const _5_2: Self = Self(9);
    pub const _6_0: Self = Self(10);
    pub const _6_1: Self = Self(11);
    pub const _6_2: Self = Self(12);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH265SliceType(i32);
impl StdVideoH265SliceType {
    pub const B: Self = Self(0);
    pub const P: Self = Self(1);
    pub const I: Self = Self(2);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH265PictureType(i32);
impl StdVideoH265PictureType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const IDR: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH265AspectRatioIdc(i32);
impl StdVideoH265AspectRatioIdc {
    pub const UNSPECIFIED: Self = Self(0);
    pub const SQUARE: Self = Self(1);
    pub const _12_11: Self = Self(2);
    pub const _10_11: Self = Self(3);
    pub const _16_11: Self = Self(4);
    pub const _40_33: Self = Self(5);
    pub const _24_11: Self = Self(6);
    pub const _20_11: Self = Self(7);
    pub const _32_11: Self = Self(8);
    pub const _80_33: Self = Self(9);
    pub const _18_11: Self = Self(10);
    pub const _15_11: Self = Self(11);
    pub const _64_33: Self = Self(12);
    pub const _160_99: Self = Self(13);
    pub const _4_3: Self = Self(14);
    pub const _3_2: Self = Self(15);
    pub const _2_1: Self = Self(16);
    pub const EXTENDED_SAR: Self = Self(255);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
