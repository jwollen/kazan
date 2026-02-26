#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264WeightTableFlags {
    pub luma_weight_l0_flag: u32,
    pub chroma_weight_l0_flag: u32,
    pub luma_weight_l1_flag: u32,
    pub chroma_weight_l1_flag: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH264WeightTable {
    pub flags: StdVideoEncodeH264WeightTableFlags,
    pub luma_log2_weight_denom: u8,
    pub chroma_log2_weight_denom: u8,
    pub luma_weight_l0: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l0: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_weight_l0:
        [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_offset_l0:
        [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub luma_weight_l1: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub luma_offset_l1: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_weight_l1:
        [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub chroma_offset_l1:
        [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize]; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
}
impl Default for StdVideoEncodeH264WeightTable {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            luma_log2_weight_denom: Default::default(),
            chroma_log2_weight_denom: Default::default(),
            luma_weight_l0: [Default::default(); _],
            luma_offset_l0: [Default::default(); _],
            chroma_weight_l0: [[Default::default(); _]; _],
            chroma_offset_l0: [[Default::default(); _]; _],
            luma_weight_l1: [Default::default(); _],
            luma_offset_l1: [Default::default(); _],
            chroma_weight_l1: [[Default::default(); _]; _],
            chroma_offset_l1: [[Default::default(); _]; _],
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264SliceHeaderFlags {
    pub direct_spatial_mv_pred_flag: u32,
    pub num_ref_idx_active_override_flag: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264PictureInfoFlags {
    pub idr_pic_flag: u32,
    pub is_reference: u32,
    pub no_output_of_prior_pics_flag: u32,
    pub long_term_reference_flag: u32,
    pub adaptive_ref_pic_marking_mode_flag: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264ReferenceListsInfoFlags {
    pub ref_pic_list_modification_flag_l0: u32,
    pub ref_pic_list_modification_flag_l1: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264RefListModEntry {
    pub modification_of_pic_nums_idc: StdVideoH264ModificationOfPicNumsIdc,
    pub abs_diff_pic_num_minus1: u16,
    pub long_term_pic_num: u16,
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264RefPicMarkingEntry {
    pub memory_management_control_operation: StdVideoH264MemMgmtControlOp,
    pub difference_of_pic_nums_minus1: u16,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub max_long_term_frame_idx_plus1: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH264ReferenceListsInfo<'a> {
    pub flags: StdVideoEncodeH264ReferenceListsInfoFlags,
    pub num_ref_idx_l0_active_minus1: u8,
    pub num_ref_idx_l1_active_minus1: u8,
    pub ref_pic_list0: [u8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub ref_pic_list1: [u8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    pub ref_list0_mod_op_count: u8,
    pub ref_list1_mod_op_count: u8,
    pub ref_pic_marking_op_count: u8,
    pub reserved1: [u8; 7],
    pub p_ref_list0_mod_operations: *const StdVideoEncodeH264RefListModEntry,
    pub p_ref_list1_mod_operations: *const StdVideoEncodeH264RefListModEntry,
    pub p_ref_pic_marking_operations: *const StdVideoEncodeH264RefPicMarkingEntry,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for StdVideoEncodeH264ReferenceListsInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            num_ref_idx_l0_active_minus1: Default::default(),
            num_ref_idx_l1_active_minus1: Default::default(),
            ref_pic_list0: [Default::default(); _],
            ref_pic_list1: [Default::default(); _],
            ref_list0_mod_op_count: Default::default(),
            ref_list1_mod_op_count: Default::default(),
            ref_pic_marking_op_count: Default::default(),
            reserved1: [Default::default(); _],
            p_ref_list0_mod_operations: core::ptr::null(),
            p_ref_list1_mod_operations: core::ptr::null(),
            p_ref_pic_marking_operations: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH264PictureInfo<'a> {
    pub flags: StdVideoEncodeH264PictureInfoFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub idr_pic_id: u16,
    pub primary_pic_type: StdVideoH264PictureType,
    pub frame_num: u32,
    pub pic_order_cnt: i32,
    pub temporal_id: u8,
    pub reserved1: [u8; 3],
    pub p_ref_lists: *const StdVideoEncodeH264ReferenceListsInfo<'a>,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for StdVideoEncodeH264PictureInfo<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            seq_parameter_set_id: Default::default(),
            pic_parameter_set_id: Default::default(),
            idr_pic_id: Default::default(),
            primary_pic_type: Default::default(),
            frame_num: Default::default(),
            pic_order_cnt: Default::default(),
            temporal_id: Default::default(),
            reserved1: [Default::default(); _],
            p_ref_lists: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264ReferenceInfo {
    pub flags: StdVideoEncodeH264ReferenceInfoFlags,
    pub primary_pic_type: StdVideoH264PictureType,
    pub frame_num: u32,
    pub pic_order_cnt: i32,
    pub long_term_pic_num: u16,
    pub long_term_frame_idx: u16,
    pub temporal_id: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeH264SliceHeader<'a> {
    pub flags: StdVideoEncodeH264SliceHeaderFlags,
    pub first_mb_in_slice: u32,
    pub slice_type: StdVideoH264SliceType,
    pub slice_alpha_c0_offset_div2: i8,
    pub slice_beta_offset_div2: i8,
    pub slice_qp_delta: i8,
    pub reserved1: u8,
    pub cabac_init_idc: StdVideoH264CabacInitIdc,
    pub disable_deblocking_filter_idc: StdVideoH264DisableDeblockingFilterIdc,
    pub p_weight_table: *const StdVideoEncodeH264WeightTable,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for StdVideoEncodeH264SliceHeader<'_> {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            first_mb_in_slice: Default::default(),
            slice_type: Default::default(),
            slice_alpha_c0_offset_div2: Default::default(),
            slice_beta_offset_div2: Default::default(),
            slice_qp_delta: Default::default(),
            reserved1: Default::default(),
            cabac_init_idc: Default::default(),
            disable_deblocking_filter_idc: Default::default(),
            p_weight_table: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
