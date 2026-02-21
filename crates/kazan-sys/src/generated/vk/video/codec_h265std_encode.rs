#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265WeightTableFlags {
    pub luma_weight_l0_flag: u16,
    pub chroma_weight_l0_flag: u16,
    pub luma_weight_l1_flag: u16,
    pub chroma_weight_l1_flag: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265WeightTable {
    pub flags: StdVideoEncodeH265WeightTableFlags,
    pub luma_log2_weight_denom: u8,
    pub delta_chroma_log2_weight_denom: i8,
    pub delta_luma_weight_l0: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l0: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_weight_l0:
        [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_offset_l0:
        [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_luma_weight_l1: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l1: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_weight_l1:
        [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub delta_chroma_offset_l1:
        [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265LongTermRefPics {
    pub num_long_term_sps: u8,
    pub num_long_term_pics: u8,
    pub lt_idx_sps: [u8; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
    pub poc_lsb_lt: [u8; STD_VIDEO_H265_MAX_LONG_TERM_PICS as usize],
    pub used_by_curr_pic_lt_flag: u16,
    pub delta_poc_msb_present_flag: [u8; STD_VIDEO_H265_MAX_DELTA_POC as usize],
    pub delta_poc_msb_cycle_lt: [u8; STD_VIDEO_H265_MAX_DELTA_POC as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265SliceSegmentHeaderFlags {
    pub first_slice_segment_in_pic_flag: u32,
    pub dependent_slice_segment_flag: u32,
    pub slice_sao_luma_flag: u32,
    pub slice_sao_chroma_flag: u32,
    pub num_ref_idx_active_override_flag: u32,
    pub mvd_l1_zero_flag: u32,
    pub cabac_init_flag: u32,
    pub cu_chroma_qp_offset_enabled_flag: u32,
    pub deblocking_filter_override_flag: u32,
    pub slice_deblocking_filter_disabled_flag: u32,
    pub collocated_from_l0_flag: u32,
    pub slice_loop_filter_across_slices_enabled_flag: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265SliceSegmentHeader {
    pub flags: StdVideoEncodeH265SliceSegmentHeaderFlags,
    pub slice_type: StdVideoH265SliceType,
    pub slice_segment_address: u32,
    pub collocated_ref_idx: u8,
    pub max_num_merge_cand: u8,
    pub slice_cb_qp_offset: i8,
    pub slice_cr_qp_offset: i8,
    pub slice_beta_offset_div2: i8,
    pub slice_tc_offset_div2: i8,
    pub slice_act_y_qp_offset: i8,
    pub slice_act_cb_qp_offset: i8,
    pub slice_act_cr_qp_offset: i8,
    pub slice_qp_delta: i8,
    pub reserved1: u16,
    pub p_weight_table: *const StdVideoEncodeH265WeightTable,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265ReferenceListsInfoFlags {
    pub ref_pic_list_modification_flag_l0: u32,
    pub ref_pic_list_modification_flag_l1: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265ReferenceListsInfo {
    pub flags: StdVideoEncodeH265ReferenceListsInfoFlags,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub ref_pic_list0: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub ref_pic_list1: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub list_entry_l0: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    pub list_entry_l1: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265PictureInfoFlags {
    pub is_reference: u32,
    pub irap_pic_flag: u32,
    pub used_for_long_term_reference: u32,
    pub discardable_flag: u32,
    pub cross_layer_bla_flag: u32,
    pub pic_output_flag: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub short_term_ref_pic_set_sps_flag: u32,
    pub slice_temporal_mvp_enabled_flag: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265PictureInfo {
    pub flags: StdVideoEncodeH265PictureInfoFlags,
    pub pic_type: StdVideoH265PictureType,
    pub sps_video_parameter_set_id: u8,
    pub pps_seq_parameter_set_id: u8,
    pub pps_pic_parameter_set_id: u8,
    pub short_term_ref_pic_set_idx: u8,
    pub pic_order_cnt_val: i32,
    pub temporal_id: u8,
    pub reserved1: [u8; 7],
    pub p_ref_lists: *const StdVideoEncodeH265ReferenceListsInfo,
    pub p_short_term_ref_pic_set: *const StdVideoH265ShortTermRefPicSet,
    pub p_long_term_ref_pics: *const StdVideoEncodeH265LongTermRefPics,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub unused_for_reference: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH265ReferenceInfo {
    pub flags: StdVideoEncodeH265ReferenceInfoFlags,
    pub pic_type: StdVideoH265PictureType,
    pub pic_order_cnt_val: i32,
    pub temporal_id: u8,
}
