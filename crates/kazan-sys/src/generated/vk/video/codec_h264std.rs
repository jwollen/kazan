#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const STD_VIDEO_H264_CPB_CNT_LIST_SIZE: u32 = 32;
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
pub const STD_VIDEO_H264_MAX_NUM_LIST_REF: u32 = 32;
pub const STD_VIDEO_H264_MAX_CHROMA_PLANES: u32 = 2;
pub const STD_VIDEO_H264_NO_REFERENCE_PICTURE: u8 = 0xF;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoH264SpsVuiFlags {
    pub aspect_ratio_info_present_flag: u32,
    pub overscan_info_present_flag: u32,
    pub overscan_appropriate_flag: u32,
    pub video_signal_type_present_flag: u32,
    pub video_full_range_flag: u32,
    pub color_description_present_flag: u32,
    pub chroma_loc_info_present_flag: u32,
    pub timing_info_present_flag: u32,
    pub fixed_frame_rate_flag: u32,
    pub bitstream_restriction_flag: u32,
    pub nal_hrd_parameters_present_flag: u32,
    pub vcl_hrd_parameters_present_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH264HrdParameters {
    pub cpb_cnt_minus1: u8,
    pub bit_rate_scale: u8,
    pub cpb_size_scale: u8,
    pub reserved1: u8,
    pub bit_rate_value_minus1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub cpb_size_value_minus1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub cbr_flag: [u8; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
    pub initial_cpb_removal_delay_length_minus1: u32,
    pub cpb_removal_delay_length_minus1: u32,
    pub dpb_output_delay_length_minus1: u32,
    pub time_offset_length: u32,
}
impl Default for StdVideoH264HrdParameters {
    fn default() -> Self {
        Self {
            cpb_cnt_minus1: Default::default(),
            bit_rate_scale: Default::default(),
            cpb_size_scale: Default::default(),
            reserved1: Default::default(),
            bit_rate_value_minus1: [Default::default(); _],
            cpb_size_value_minus1: [Default::default(); _],
            cbr_flag: [Default::default(); _],
            initial_cpb_removal_delay_length_minus1: Default::default(),
            cpb_removal_delay_length_minus1: Default::default(),
            dpb_output_delay_length_minus1: Default::default(),
            time_offset_length: Default::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH264SequenceParameterSetVui<'a> {
    pub flags: StdVideoH264SpsVuiFlags,
    pub aspect_ratio_idc: StdVideoH264AspectRatioIdc,
    pub sar_width: u16,
    pub sar_height: u16,
    pub video_format: u8,
    pub colour_primaries: u8,
    pub transfer_characteristics: u8,
    pub matrix_coefficients: u8,
    pub num_units_in_tick: u32,
    pub time_scale: u32,
    pub max_num_reorder_frames: u8,
    pub max_dec_frame_buffering: u8,
    pub chroma_sample_loc_type_top_field: u8,
    pub chroma_sample_loc_type_bottom_field: u8,
    pub reserved1: u32,
    pub p_hrd_parameters: *const StdVideoH264HrdParameters,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for StdVideoH264SequenceParameterSetVui<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            aspect_ratio_idc: Default::default(),
            sar_width: Default::default(),
            sar_height: Default::default(),
            video_format: Default::default(),
            colour_primaries: Default::default(),
            transfer_characteristics: Default::default(),
            matrix_coefficients: Default::default(),
            num_units_in_tick: Default::default(),
            time_scale: Default::default(),
            max_num_reorder_frames: Default::default(),
            max_dec_frame_buffering: Default::default(),
            chroma_sample_loc_type_top_field: Default::default(),
            chroma_sample_loc_type_bottom_field: Default::default(),
            reserved1: Default::default(),
            p_hrd_parameters: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoH264SpsFlags {
    pub constraint_set0_flag: u32,
    pub constraint_set1_flag: u32,
    pub constraint_set2_flag: u32,
    pub constraint_set3_flag: u32,
    pub constraint_set4_flag: u32,
    pub constraint_set5_flag: u32,
    pub direct_8x8_inference_flag: u32,
    pub mb_adaptive_frame_field_flag: u32,
    pub frame_mbs_only_flag: u32,
    pub delta_pic_order_always_zero_flag: u32,
    pub separate_colour_plane_flag: u32,
    pub gaps_in_frame_num_value_allowed_flag: u32,
    pub qpprime_y_zero_transform_bypass_flag: u32,
    pub frame_cropping_flag: u32,
    pub seq_scaling_matrix_present_flag: u32,
    pub vui_parameters_present_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH264ScalingLists {
    pub scaling_list_present_mask: u16,
    pub use_default_scaling_matrix_mask: u16,
    pub scaling_list4x4: [[u8; STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS as usize];
        STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS as usize],
    pub scaling_list8x8: [[u8; STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS as usize];
        STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS as usize],
}
impl Default for StdVideoH264ScalingLists {
    fn default() -> Self {
        Self {
            scaling_list_present_mask: Default::default(),
            use_default_scaling_matrix_mask: Default::default(),
            scaling_list4x4: [[Default::default(); _]; _],
            scaling_list8x8: [[Default::default(); _]; _],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH264SequenceParameterSet<'a> {
    pub flags: StdVideoH264SpsFlags,
    pub profile_idc: StdVideoH264ProfileIdc,
    pub level_idc: StdVideoH264LevelIdc,
    pub chroma_format_idc: StdVideoH264ChromaFormatIdc,
    pub seq_parameter_set_id: u8,
    pub bit_depth_luma_minus8: u8,
    pub bit_depth_chroma_minus8: u8,
    pub log2_max_frame_num_minus4: u8,
    pub pic_order_cnt_type: StdVideoH264PocType,
    pub offset_for_non_ref_pic: i32,
    pub offset_for_top_to_bottom_field: i32,
    pub log2_max_pic_order_cnt_lsb_minus4: u8,
    pub num_ref_frames_in_pic_order_cnt_cycle: u8,
    pub max_num_ref_frames: u8,
    pub reserved1: u8,
    pub pic_width_in_mbs_minus1: u32,
    pub pic_height_in_map_units_minus1: u32,
    pub frame_crop_left_offset: u32,
    pub frame_crop_right_offset: u32,
    pub frame_crop_top_offset: u32,
    pub frame_crop_bottom_offset: u32,
    pub reserved2: u32,
    pub p_offset_for_ref_frame: *const i32,
    pub p_scaling_lists: *const StdVideoH264ScalingLists,
    pub p_sequence_parameter_set_vui: *const StdVideoH264SequenceParameterSetVui<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for StdVideoH264SequenceParameterSet<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            profile_idc: Default::default(),
            level_idc: Default::default(),
            chroma_format_idc: Default::default(),
            seq_parameter_set_id: Default::default(),
            bit_depth_luma_minus8: Default::default(),
            bit_depth_chroma_minus8: Default::default(),
            log2_max_frame_num_minus4: Default::default(),
            pic_order_cnt_type: Default::default(),
            offset_for_non_ref_pic: Default::default(),
            offset_for_top_to_bottom_field: Default::default(),
            log2_max_pic_order_cnt_lsb_minus4: Default::default(),
            num_ref_frames_in_pic_order_cnt_cycle: Default::default(),
            max_num_ref_frames: Default::default(),
            reserved1: Default::default(),
            pic_width_in_mbs_minus1: Default::default(),
            pic_height_in_map_units_minus1: Default::default(),
            frame_crop_left_offset: Default::default(),
            frame_crop_right_offset: Default::default(),
            frame_crop_top_offset: Default::default(),
            frame_crop_bottom_offset: Default::default(),
            reserved2: Default::default(),
            p_offset_for_ref_frame: core::ptr::null(),
            p_scaling_lists: core::ptr::null(),
            p_sequence_parameter_set_vui: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoH264PpsFlags {
    pub transform_8x8_mode_flag: u32,
    pub redundant_pic_cnt_present_flag: u32,
    pub constrained_intra_pred_flag: u32,
    pub deblocking_filter_control_present_flag: u32,
    pub weighted_pred_flag: u32,
    pub bottom_field_pic_order_in_frame_present_flag: u32,
    pub entropy_coding_mode_flag: u32,
    pub pic_scaling_matrix_present_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoH264PictureParameterSet<'a> {
    pub flags: StdVideoH264PpsFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub num_ref_idx_l0_default_active_minus1: u8,
    pub num_ref_idx_l1_default_active_minus1: u8,
    pub weighted_bipred_idc: StdVideoH264WeightedBipredIdc,
    pub pic_init_qp_minus26: i8,
    pub pic_init_qs_minus26: i8,
    pub chroma_qp_index_offset: i8,
    pub second_chroma_qp_index_offset: i8,
    pub p_scaling_lists: *const StdVideoH264ScalingLists,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for StdVideoH264PictureParameterSet<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_parameter_set_id: Default::default(),
            pic_parameter_set_id: Default::default(),
            num_ref_idx_l0_default_active_minus1: Default::default(),
            num_ref_idx_l1_default_active_minus1: Default::default(),
            weighted_bipred_idc: Default::default(),
            pic_init_qp_minus26: Default::default(),
            pic_init_qs_minus26: Default::default(),
            chroma_qp_index_offset: Default::default(),
            second_chroma_qp_index_offset: Default::default(),
            p_scaling_lists: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264ChromaFormatIdc(i32);
impl StdVideoH264ChromaFormatIdc {
    pub const MONOCHROME: Self = Self(0);
    pub const _420: Self = Self(1);
    pub const _422: Self = Self(2);
    pub const _444: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264ProfileIdc(i32);
impl StdVideoH264ProfileIdc {
    pub const BASELINE: Self = Self(66);
    pub const MAIN: Self = Self(77);
    pub const HIGH: Self = Self(100);
    pub const HIGH_444_PREDICTIVE: Self = Self(244);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264LevelIdc(i32);
impl StdVideoH264LevelIdc {
    pub const _1_0: Self = Self(0);
    pub const _1_1: Self = Self(1);
    pub const _1_2: Self = Self(2);
    pub const _1_3: Self = Self(3);
    pub const _2_0: Self = Self(4);
    pub const _2_1: Self = Self(5);
    pub const _2_2: Self = Self(6);
    pub const _3_0: Self = Self(7);
    pub const _3_1: Self = Self(8);
    pub const _3_2: Self = Self(9);
    pub const _4_0: Self = Self(10);
    pub const _4_1: Self = Self(11);
    pub const _4_2: Self = Self(12);
    pub const _5_0: Self = Self(13);
    pub const _5_1: Self = Self(14);
    pub const _5_2: Self = Self(15);
    pub const _6_0: Self = Self(16);
    pub const _6_1: Self = Self(17);
    pub const _6_2: Self = Self(18);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264PocType(i32);
impl StdVideoH264PocType {
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264AspectRatioIdc(i32);
impl StdVideoH264AspectRatioIdc {
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
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264WeightedBipredIdc(i32);
impl StdVideoH264WeightedBipredIdc {
    pub const DEFAULT: Self = Self(0);
    pub const EXPLICIT: Self = Self(1);
    pub const IMPLICIT: Self = Self(2);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264ModificationOfPicNumsIdc(i32);
impl StdVideoH264ModificationOfPicNumsIdc {
    pub const SHORT_TERM_SUBTRACT: Self = Self(0);
    pub const SHORT_TERM_ADD: Self = Self(1);
    pub const LONG_TERM: Self = Self(2);
    pub const END: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264MemMgmtControlOp(i32);
impl StdVideoH264MemMgmtControlOp {
    pub const END: Self = Self(0);
    pub const UNMARK_SHORT_TERM: Self = Self(1);
    pub const UNMARK_LONG_TERM: Self = Self(2);
    pub const MARK_LONG_TERM: Self = Self(3);
    pub const SET_MAX_LONG_TERM_INDEX: Self = Self(4);
    pub const UNMARK_ALL: Self = Self(5);
    pub const MARK_CURRENT_AS_LONG_TERM: Self = Self(6);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264CabacInitIdc(i32);
impl StdVideoH264CabacInitIdc {
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264DisableDeblockingFilterIdc(i32);
impl StdVideoH264DisableDeblockingFilterIdc {
    pub const DISABLED: Self = Self(0);
    pub const ENABLED: Self = Self(1);
    pub const PARTIAL: Self = Self(2);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264SliceType(i32);
impl StdVideoH264SliceType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264PictureType(i32);
impl StdVideoH264PictureType {
    pub const P: Self = Self(0);
    pub const B: Self = Self(1);
    pub const I: Self = Self(2);
    pub const IDR: Self = Self(5);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoH264NonVclNaluType(i32);
impl StdVideoH264NonVclNaluType {
    pub const SPS: Self = Self(0);
    pub const PPS: Self = Self(1);
    pub const AUD: Self = Self(2);
    pub const PREFIX: Self = Self(3);
    pub const END_OF_SEQUENCE: Self = Self(4);
    pub const END_OF_STREAM: Self = Self(5);
    pub const PRECODED: Self = Self(6);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
