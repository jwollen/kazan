#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264WeightTableFlags {
    pub luma_weight_l0_flag: u32,
    pub chroma_weight_l0_flag: u32,
    pub luma_weight_l1_flag: u32,
    pub chroma_weight_l1_flag: u32,
}
impl StdVideoEncodeH264WeightTableFlags {
    pub fn luma_weight_l0_flag(mut self, luma_weight_l0_flag: u32) -> Self {
        self.luma_weight_l0_flag = luma_weight_l0_flag;
        self
    }
    pub fn chroma_weight_l0_flag(mut self, chroma_weight_l0_flag: u32) -> Self {
        self.chroma_weight_l0_flag = chroma_weight_l0_flag;
        self
    }
    pub fn luma_weight_l1_flag(mut self, luma_weight_l1_flag: u32) -> Self {
        self.luma_weight_l1_flag = luma_weight_l1_flag;
        self
    }
    pub fn chroma_weight_l1_flag(mut self, chroma_weight_l1_flag: u32) -> Self {
        self.chroma_weight_l1_flag = chroma_weight_l1_flag;
        self
    }
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
impl StdVideoEncodeH264WeightTable {
    pub fn flags(mut self, flags: StdVideoEncodeH264WeightTableFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn luma_log2_weight_denom(mut self, luma_log2_weight_denom: u8) -> Self {
        self.luma_log2_weight_denom = luma_log2_weight_denom;
        self
    }
    pub fn chroma_log2_weight_denom(mut self, chroma_log2_weight_denom: u8) -> Self {
        self.chroma_log2_weight_denom = chroma_log2_weight_denom;
        self
    }
    pub fn luma_weight_l0(
        mut self,
        luma_weight_l0: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.luma_weight_l0 = luma_weight_l0;
        self
    }
    pub fn luma_offset_l0(
        mut self,
        luma_offset_l0: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.luma_offset_l0 = luma_offset_l0;
        self
    }
    pub fn chroma_weight_l0(
        mut self,
        chroma_weight_l0: [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize];
            STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.chroma_weight_l0 = chroma_weight_l0;
        self
    }
    pub fn chroma_offset_l0(
        mut self,
        chroma_offset_l0: [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize];
            STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.chroma_offset_l0 = chroma_offset_l0;
        self
    }
    pub fn luma_weight_l1(
        mut self,
        luma_weight_l1: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.luma_weight_l1 = luma_weight_l1;
        self
    }
    pub fn luma_offset_l1(
        mut self,
        luma_offset_l1: [i8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.luma_offset_l1 = luma_offset_l1;
        self
    }
    pub fn chroma_weight_l1(
        mut self,
        chroma_weight_l1: [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize];
            STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.chroma_weight_l1 = chroma_weight_l1;
        self
    }
    pub fn chroma_offset_l1(
        mut self,
        chroma_offset_l1: [[i8; STD_VIDEO_H264_MAX_CHROMA_PLANES as usize];
            STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.chroma_offset_l1 = chroma_offset_l1;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264SliceHeaderFlags {
    pub direct_spatial_mv_pred_flag: u32,
    pub num_ref_idx_active_override_flag: u32,
    pub reserved: u32,
}
impl StdVideoEncodeH264SliceHeaderFlags {
    pub fn direct_spatial_mv_pred_flag(mut self, direct_spatial_mv_pred_flag: u32) -> Self {
        self.direct_spatial_mv_pred_flag = direct_spatial_mv_pred_flag;
        self
    }
    pub fn num_ref_idx_active_override_flag(
        mut self,
        num_ref_idx_active_override_flag: u32,
    ) -> Self {
        self.num_ref_idx_active_override_flag = num_ref_idx_active_override_flag;
        self
    }
    pub fn reserved(mut self, reserved: u32) -> Self {
        self.reserved = reserved;
        self
    }
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
impl StdVideoEncodeH264PictureInfoFlags {
    pub fn idr_pic_flag(mut self, idr_pic_flag: u32) -> Self {
        self.idr_pic_flag = idr_pic_flag;
        self
    }
    pub fn is_reference(mut self, is_reference: u32) -> Self {
        self.is_reference = is_reference;
        self
    }
    pub fn no_output_of_prior_pics_flag(mut self, no_output_of_prior_pics_flag: u32) -> Self {
        self.no_output_of_prior_pics_flag = no_output_of_prior_pics_flag;
        self
    }
    pub fn long_term_reference_flag(mut self, long_term_reference_flag: u32) -> Self {
        self.long_term_reference_flag = long_term_reference_flag;
        self
    }
    pub fn adaptive_ref_pic_marking_mode_flag(
        mut self,
        adaptive_ref_pic_marking_mode_flag: u32,
    ) -> Self {
        self.adaptive_ref_pic_marking_mode_flag = adaptive_ref_pic_marking_mode_flag;
        self
    }
    pub fn reserved(mut self, reserved: u32) -> Self {
        self.reserved = reserved;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub reserved: u32,
}
impl StdVideoEncodeH264ReferenceInfoFlags {
    pub fn used_for_long_term_reference(mut self, used_for_long_term_reference: u32) -> Self {
        self.used_for_long_term_reference = used_for_long_term_reference;
        self
    }
    pub fn reserved(mut self, reserved: u32) -> Self {
        self.reserved = reserved;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264ReferenceListsInfoFlags {
    pub ref_pic_list_modification_flag_l0: u32,
    pub ref_pic_list_modification_flag_l1: u32,
    pub reserved: u32,
}
impl StdVideoEncodeH264ReferenceListsInfoFlags {
    pub fn ref_pic_list_modification_flag_l0(
        mut self,
        ref_pic_list_modification_flag_l0: u32,
    ) -> Self {
        self.ref_pic_list_modification_flag_l0 = ref_pic_list_modification_flag_l0;
        self
    }
    pub fn ref_pic_list_modification_flag_l1(
        mut self,
        ref_pic_list_modification_flag_l1: u32,
    ) -> Self {
        self.ref_pic_list_modification_flag_l1 = ref_pic_list_modification_flag_l1;
        self
    }
    pub fn reserved(mut self, reserved: u32) -> Self {
        self.reserved = reserved;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoEncodeH264RefListModEntry {
    pub modification_of_pic_nums_idc: StdVideoH264ModificationOfPicNumsIdc,
    pub abs_diff_pic_num_minus1: u16,
    pub long_term_pic_num: u16,
}
impl StdVideoEncodeH264RefListModEntry {
    pub fn modification_of_pic_nums_idc(
        mut self,
        modification_of_pic_nums_idc: StdVideoH264ModificationOfPicNumsIdc,
    ) -> Self {
        self.modification_of_pic_nums_idc = modification_of_pic_nums_idc;
        self
    }
    pub fn abs_diff_pic_num_minus1(mut self, abs_diff_pic_num_minus1: u16) -> Self {
        self.abs_diff_pic_num_minus1 = abs_diff_pic_num_minus1;
        self
    }
    pub fn long_term_pic_num(mut self, long_term_pic_num: u16) -> Self {
        self.long_term_pic_num = long_term_pic_num;
        self
    }
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
impl StdVideoEncodeH264RefPicMarkingEntry {
    pub fn memory_management_control_operation(
        mut self,
        memory_management_control_operation: StdVideoH264MemMgmtControlOp,
    ) -> Self {
        self.memory_management_control_operation = memory_management_control_operation;
        self
    }
    pub fn difference_of_pic_nums_minus1(mut self, difference_of_pic_nums_minus1: u16) -> Self {
        self.difference_of_pic_nums_minus1 = difference_of_pic_nums_minus1;
        self
    }
    pub fn long_term_pic_num(mut self, long_term_pic_num: u16) -> Self {
        self.long_term_pic_num = long_term_pic_num;
        self
    }
    pub fn long_term_frame_idx(mut self, long_term_frame_idx: u16) -> Self {
        self.long_term_frame_idx = long_term_frame_idx;
        self
    }
    pub fn max_long_term_frame_idx_plus1(mut self, max_long_term_frame_idx_plus1: u16) -> Self {
        self.max_long_term_frame_idx_plus1 = max_long_term_frame_idx_plus1;
        self
    }
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
impl<'a> StdVideoEncodeH264ReferenceListsInfo<'a> {
    pub fn flags(mut self, flags: StdVideoEncodeH264ReferenceListsInfoFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn num_ref_idx_l0_active_minus1(mut self, num_ref_idx_l0_active_minus1: u8) -> Self {
        self.num_ref_idx_l0_active_minus1 = num_ref_idx_l0_active_minus1;
        self
    }
    pub fn num_ref_idx_l1_active_minus1(mut self, num_ref_idx_l1_active_minus1: u8) -> Self {
        self.num_ref_idx_l1_active_minus1 = num_ref_idx_l1_active_minus1;
        self
    }
    pub fn ref_pic_list0(
        mut self,
        ref_pic_list0: [u8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.ref_pic_list0 = ref_pic_list0;
        self
    }
    pub fn ref_pic_list1(
        mut self,
        ref_pic_list1: [u8; STD_VIDEO_H264_MAX_NUM_LIST_REF as usize],
    ) -> Self {
        self.ref_pic_list1 = ref_pic_list1;
        self
    }
    pub fn ref_list0_mod_operations(
        mut self,
        ref_list0_mod_operations: &'a [StdVideoEncodeH264RefListModEntry],
    ) -> Self {
        self.ref_list0_mod_op_count = ref_list0_mod_operations.len().try_into().unwrap();
        self.p_ref_list0_mod_operations = ref_list0_mod_operations.as_ptr();
        self
    }
    pub fn ref_list1_mod_operations(
        mut self,
        ref_list1_mod_operations: &'a [StdVideoEncodeH264RefListModEntry],
    ) -> Self {
        self.ref_list1_mod_op_count = ref_list1_mod_operations.len().try_into().unwrap();
        self.p_ref_list1_mod_operations = ref_list1_mod_operations.as_ptr();
        self
    }
    pub fn ref_pic_marking_operations(
        mut self,
        ref_pic_marking_operations: &'a [StdVideoEncodeH264RefPicMarkingEntry],
    ) -> Self {
        self.ref_pic_marking_op_count = ref_pic_marking_operations.len().try_into().unwrap();
        self.p_ref_pic_marking_operations = ref_pic_marking_operations.as_ptr();
        self
    }
    pub fn reserved1(mut self, reserved1: [u8; 7]) -> Self {
        self.reserved1 = reserved1;
        self
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
impl<'a> StdVideoEncodeH264PictureInfo<'a> {
    pub fn flags(mut self, flags: StdVideoEncodeH264PictureInfoFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn seq_parameter_set_id(mut self, seq_parameter_set_id: u8) -> Self {
        self.seq_parameter_set_id = seq_parameter_set_id;
        self
    }
    pub fn pic_parameter_set_id(mut self, pic_parameter_set_id: u8) -> Self {
        self.pic_parameter_set_id = pic_parameter_set_id;
        self
    }
    pub fn idr_pic_id(mut self, idr_pic_id: u16) -> Self {
        self.idr_pic_id = idr_pic_id;
        self
    }
    pub fn primary_pic_type(mut self, primary_pic_type: StdVideoH264PictureType) -> Self {
        self.primary_pic_type = primary_pic_type;
        self
    }
    pub fn frame_num(mut self, frame_num: u32) -> Self {
        self.frame_num = frame_num;
        self
    }
    pub fn pic_order_cnt(mut self, pic_order_cnt: i32) -> Self {
        self.pic_order_cnt = pic_order_cnt;
        self
    }
    pub fn temporal_id(mut self, temporal_id: u8) -> Self {
        self.temporal_id = temporal_id;
        self
    }
    pub fn reserved1(mut self, reserved1: [u8; 3]) -> Self {
        self.reserved1 = reserved1;
        self
    }
    pub fn ref_lists(mut self, ref_lists: &'a StdVideoEncodeH264ReferenceListsInfo<'a>) -> Self {
        self.p_ref_lists = ref_lists;
        self
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
impl StdVideoEncodeH264ReferenceInfo {
    pub fn flags(mut self, flags: StdVideoEncodeH264ReferenceInfoFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn primary_pic_type(mut self, primary_pic_type: StdVideoH264PictureType) -> Self {
        self.primary_pic_type = primary_pic_type;
        self
    }
    pub fn frame_num(mut self, frame_num: u32) -> Self {
        self.frame_num = frame_num;
        self
    }
    pub fn pic_order_cnt(mut self, pic_order_cnt: i32) -> Self {
        self.pic_order_cnt = pic_order_cnt;
        self
    }
    pub fn long_term_pic_num(mut self, long_term_pic_num: u16) -> Self {
        self.long_term_pic_num = long_term_pic_num;
        self
    }
    pub fn long_term_frame_idx(mut self, long_term_frame_idx: u16) -> Self {
        self.long_term_frame_idx = long_term_frame_idx;
        self
    }
    pub fn temporal_id(mut self, temporal_id: u8) -> Self {
        self.temporal_id = temporal_id;
        self
    }
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
impl<'a> StdVideoEncodeH264SliceHeader<'a> {
    pub fn flags(mut self, flags: StdVideoEncodeH264SliceHeaderFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn first_mb_in_slice(mut self, first_mb_in_slice: u32) -> Self {
        self.first_mb_in_slice = first_mb_in_slice;
        self
    }
    pub fn slice_type(mut self, slice_type: StdVideoH264SliceType) -> Self {
        self.slice_type = slice_type;
        self
    }
    pub fn slice_alpha_c0_offset_div2(mut self, slice_alpha_c0_offset_div2: i8) -> Self {
        self.slice_alpha_c0_offset_div2 = slice_alpha_c0_offset_div2;
        self
    }
    pub fn slice_beta_offset_div2(mut self, slice_beta_offset_div2: i8) -> Self {
        self.slice_beta_offset_div2 = slice_beta_offset_div2;
        self
    }
    pub fn slice_qp_delta(mut self, slice_qp_delta: i8) -> Self {
        self.slice_qp_delta = slice_qp_delta;
        self
    }
    pub fn reserved1(mut self, reserved1: u8) -> Self {
        self.reserved1 = reserved1;
        self
    }
    pub fn cabac_init_idc(mut self, cabac_init_idc: StdVideoH264CabacInitIdc) -> Self {
        self.cabac_init_idc = cabac_init_idc;
        self
    }
    pub fn disable_deblocking_filter_idc(
        mut self,
        disable_deblocking_filter_idc: StdVideoH264DisableDeblockingFilterIdc,
    ) -> Self {
        self.disable_deblocking_filter_idc = disable_deblocking_filter_idc;
        self
    }
    pub fn weight_table(mut self, weight_table: &'a StdVideoEncodeH264WeightTable) -> Self {
        self.p_weight_table = weight_table;
        self
    }
}
