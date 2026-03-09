//! <https://registry.khronos.org/vulkan/specs/latest/man/html/vulkan_video_codec_h265std_encode.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"vulkan_video_codec_h265std_encode";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265WeightTableFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeH265WeightTableFlags {
        pub luma_weight_l0_flag: u16,
        pub chroma_weight_l0_flag: u16,
        pub luma_weight_l1_flag: u16,
        pub chroma_weight_l1_flag: u16,
    }

    impl StdVideoEncodeH265WeightTableFlags {
        #[inline]
        pub fn luma_weight_l0_flag(mut self, luma_weight_l0_flag: u16) -> Self {
            self.luma_weight_l0_flag = luma_weight_l0_flag;
            self
        }

        #[inline]
        pub fn chroma_weight_l0_flag(mut self, chroma_weight_l0_flag: u16) -> Self {
            self.chroma_weight_l0_flag = chroma_weight_l0_flag;
            self
        }

        #[inline]
        pub fn luma_weight_l1_flag(mut self, luma_weight_l1_flag: u16) -> Self {
            self.luma_weight_l1_flag = luma_weight_l1_flag;
            self
        }

        #[inline]
        pub fn chroma_weight_l1_flag(mut self, chroma_weight_l1_flag: u16) -> Self {
            self.chroma_weight_l1_flag = chroma_weight_l1_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265WeightTable.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoEncodeH265WeightTable {
        pub flags: StdVideoEncodeH265WeightTableFlags,
        pub luma_log2_weight_denom: u8,
        pub delta_chroma_log2_weight_denom: i8,
        pub delta_luma_weight_l0: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub luma_offset_l0: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub delta_chroma_weight_l0: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize];
            STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub delta_chroma_offset_l0: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize];
            STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub delta_luma_weight_l1: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub luma_offset_l1: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub delta_chroma_weight_l1: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize];
            STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub delta_chroma_offset_l1: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize];
            STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    }

    impl Default for StdVideoEncodeH265WeightTable {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                luma_log2_weight_denom: Default::default(),
                delta_chroma_log2_weight_denom: Default::default(),
                delta_luma_weight_l0: [Default::default(); _],
                luma_offset_l0: [Default::default(); _],
                delta_chroma_weight_l0: [[Default::default(); _]; _],
                delta_chroma_offset_l0: [[Default::default(); _]; _],
                delta_luma_weight_l1: [Default::default(); _],
                luma_offset_l1: [Default::default(); _],
                delta_chroma_weight_l1: [[Default::default(); _]; _],
                delta_chroma_offset_l1: [[Default::default(); _]; _],
            }
        }
    }

    impl StdVideoEncodeH265WeightTable {
        #[inline]
        pub fn flags(mut self, flags: StdVideoEncodeH265WeightTableFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn luma_log2_weight_denom(mut self, luma_log2_weight_denom: u8) -> Self {
            self.luma_log2_weight_denom = luma_log2_weight_denom;
            self
        }

        #[inline]
        pub fn delta_chroma_log2_weight_denom(
            mut self,
            delta_chroma_log2_weight_denom: i8,
        ) -> Self {
            self.delta_chroma_log2_weight_denom = delta_chroma_log2_weight_denom;
            self
        }

        #[inline]
        pub fn delta_luma_weight_l0(
            mut self,
            delta_luma_weight_l0: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.delta_luma_weight_l0 = delta_luma_weight_l0;
            self
        }

        #[inline]
        pub fn luma_offset_l0(
            mut self,
            luma_offset_l0: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.luma_offset_l0 = luma_offset_l0;
            self
        }

        #[inline]
        pub fn delta_chroma_weight_l0(
            mut self,
            delta_chroma_weight_l0: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize];
                STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.delta_chroma_weight_l0 = delta_chroma_weight_l0;
            self
        }

        #[inline]
        pub fn delta_chroma_offset_l0(
            mut self,
            delta_chroma_offset_l0: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize];
                STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.delta_chroma_offset_l0 = delta_chroma_offset_l0;
            self
        }

        #[inline]
        pub fn delta_luma_weight_l1(
            mut self,
            delta_luma_weight_l1: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.delta_luma_weight_l1 = delta_luma_weight_l1;
            self
        }

        #[inline]
        pub fn luma_offset_l1(
            mut self,
            luma_offset_l1: [i8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.luma_offset_l1 = luma_offset_l1;
            self
        }

        #[inline]
        pub fn delta_chroma_weight_l1(
            mut self,
            delta_chroma_weight_l1: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize];
                STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.delta_chroma_weight_l1 = delta_chroma_weight_l1;
            self
        }

        #[inline]
        pub fn delta_chroma_offset_l1(
            mut self,
            delta_chroma_offset_l1: [[i8; STD_VIDEO_H265_MAX_CHROMA_PLANES as usize];
                STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.delta_chroma_offset_l1 = delta_chroma_offset_l1;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265LongTermRefPics.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoEncodeH265LongTermRefPics {
        pub num_long_term_sps: u8,
        pub num_long_term_pics: u8,
        pub lt_idx_sps: [u8; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
        pub poc_lsb_lt: [u8; STD_VIDEO_H265_MAX_LONG_TERM_PICS as usize],
        pub used_by_curr_pic_lt_flag: u16,
        pub delta_poc_msb_present_flag: [u8; STD_VIDEO_H265_MAX_DELTA_POC as usize],
        pub delta_poc_msb_cycle_lt: [u8; STD_VIDEO_H265_MAX_DELTA_POC as usize],
    }

    impl Default for StdVideoEncodeH265LongTermRefPics {
        fn default() -> Self {
            Self {
                num_long_term_sps: Default::default(),
                num_long_term_pics: Default::default(),
                lt_idx_sps: [Default::default(); _],
                poc_lsb_lt: [Default::default(); _],
                used_by_curr_pic_lt_flag: Default::default(),
                delta_poc_msb_present_flag: [Default::default(); _],
                delta_poc_msb_cycle_lt: [Default::default(); _],
            }
        }
    }

    impl StdVideoEncodeH265LongTermRefPics {
        #[inline]
        pub fn num_long_term_sps(mut self, num_long_term_sps: u8) -> Self {
            self.num_long_term_sps = num_long_term_sps;
            self
        }

        #[inline]
        pub fn num_long_term_pics(mut self, num_long_term_pics: u8) -> Self {
            self.num_long_term_pics = num_long_term_pics;
            self
        }

        #[inline]
        pub fn lt_idx_sps(
            mut self,
            lt_idx_sps: [u8; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
        ) -> Self {
            self.lt_idx_sps = lt_idx_sps;
            self
        }

        #[inline]
        pub fn poc_lsb_lt(
            mut self,
            poc_lsb_lt: [u8; STD_VIDEO_H265_MAX_LONG_TERM_PICS as usize],
        ) -> Self {
            self.poc_lsb_lt = poc_lsb_lt;
            self
        }

        #[inline]
        pub fn used_by_curr_pic_lt_flag(mut self, used_by_curr_pic_lt_flag: u16) -> Self {
            self.used_by_curr_pic_lt_flag = used_by_curr_pic_lt_flag;
            self
        }

        #[inline]
        pub fn delta_poc_msb_present_flag(
            mut self,
            delta_poc_msb_present_flag: [u8; STD_VIDEO_H265_MAX_DELTA_POC as usize],
        ) -> Self {
            self.delta_poc_msb_present_flag = delta_poc_msb_present_flag;
            self
        }

        #[inline]
        pub fn delta_poc_msb_cycle_lt(
            mut self,
            delta_poc_msb_cycle_lt: [u8; STD_VIDEO_H265_MAX_DELTA_POC as usize],
        ) -> Self {
            self.delta_poc_msb_cycle_lt = delta_poc_msb_cycle_lt;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265SliceSegmentHeaderFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
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

    impl StdVideoEncodeH265SliceSegmentHeaderFlags {
        #[inline]
        pub fn first_slice_segment_in_pic_flag(
            mut self,
            first_slice_segment_in_pic_flag: u32,
        ) -> Self {
            self.first_slice_segment_in_pic_flag = first_slice_segment_in_pic_flag;
            self
        }

        #[inline]
        pub fn dependent_slice_segment_flag(mut self, dependent_slice_segment_flag: u32) -> Self {
            self.dependent_slice_segment_flag = dependent_slice_segment_flag;
            self
        }

        #[inline]
        pub fn slice_sao_luma_flag(mut self, slice_sao_luma_flag: u32) -> Self {
            self.slice_sao_luma_flag = slice_sao_luma_flag;
            self
        }

        #[inline]
        pub fn slice_sao_chroma_flag(mut self, slice_sao_chroma_flag: u32) -> Self {
            self.slice_sao_chroma_flag = slice_sao_chroma_flag;
            self
        }

        #[inline]
        pub fn num_ref_idx_active_override_flag(
            mut self,
            num_ref_idx_active_override_flag: u32,
        ) -> Self {
            self.num_ref_idx_active_override_flag = num_ref_idx_active_override_flag;
            self
        }

        #[inline]
        pub fn mvd_l1_zero_flag(mut self, mvd_l1_zero_flag: u32) -> Self {
            self.mvd_l1_zero_flag = mvd_l1_zero_flag;
            self
        }

        #[inline]
        pub fn cabac_init_flag(mut self, cabac_init_flag: u32) -> Self {
            self.cabac_init_flag = cabac_init_flag;
            self
        }

        #[inline]
        pub fn cu_chroma_qp_offset_enabled_flag(
            mut self,
            cu_chroma_qp_offset_enabled_flag: u32,
        ) -> Self {
            self.cu_chroma_qp_offset_enabled_flag = cu_chroma_qp_offset_enabled_flag;
            self
        }

        #[inline]
        pub fn deblocking_filter_override_flag(
            mut self,
            deblocking_filter_override_flag: u32,
        ) -> Self {
            self.deblocking_filter_override_flag = deblocking_filter_override_flag;
            self
        }

        #[inline]
        pub fn slice_deblocking_filter_disabled_flag(
            mut self,
            slice_deblocking_filter_disabled_flag: u32,
        ) -> Self {
            self.slice_deblocking_filter_disabled_flag = slice_deblocking_filter_disabled_flag;
            self
        }

        #[inline]
        pub fn collocated_from_l0_flag(mut self, collocated_from_l0_flag: u32) -> Self {
            self.collocated_from_l0_flag = collocated_from_l0_flag;
            self
        }

        #[inline]
        pub fn slice_loop_filter_across_slices_enabled_flag(
            mut self,
            slice_loop_filter_across_slices_enabled_flag: u32,
        ) -> Self {
            self.slice_loop_filter_across_slices_enabled_flag =
                slice_loop_filter_across_slices_enabled_flag;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265SliceSegmentHeader.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoEncodeH265SliceSegmentHeader<'a> {
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
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoEncodeH265SliceSegmentHeader<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoEncodeH265SliceSegmentHeader")
                .field("flags", &self.flags)
                .field("slice_type", &self.slice_type)
                .field("slice_segment_address", &self.slice_segment_address)
                .field("collocated_ref_idx", &self.collocated_ref_idx)
                .field("max_num_merge_cand", &self.max_num_merge_cand)
                .field("slice_cb_qp_offset", &self.slice_cb_qp_offset)
                .field("slice_cr_qp_offset", &self.slice_cr_qp_offset)
                .field("slice_beta_offset_div2", &self.slice_beta_offset_div2)
                .field("slice_tc_offset_div2", &self.slice_tc_offset_div2)
                .field("slice_act_y_qp_offset", &self.slice_act_y_qp_offset)
                .field("slice_act_cb_qp_offset", &self.slice_act_cb_qp_offset)
                .field("slice_act_cr_qp_offset", &self.slice_act_cr_qp_offset)
                .field("slice_qp_delta", &self.slice_qp_delta)
                .field("reserved1", &self.reserved1)
                .field("p_weight_table", &self.p_weight_table)
                .finish()
        }
    }

    impl Default for StdVideoEncodeH265SliceSegmentHeader<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                slice_type: Default::default(),
                slice_segment_address: Default::default(),
                collocated_ref_idx: Default::default(),
                max_num_merge_cand: Default::default(),
                slice_cb_qp_offset: Default::default(),
                slice_cr_qp_offset: Default::default(),
                slice_beta_offset_div2: Default::default(),
                slice_tc_offset_div2: Default::default(),
                slice_act_y_qp_offset: Default::default(),
                slice_act_cb_qp_offset: Default::default(),
                slice_act_cr_qp_offset: Default::default(),
                slice_qp_delta: Default::default(),
                reserved1: Default::default(),
                p_weight_table: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoEncodeH265SliceSegmentHeader<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoEncodeH265SliceSegmentHeaderFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn slice_type(mut self, slice_type: StdVideoH265SliceType) -> Self {
            self.slice_type = slice_type;
            self
        }

        #[inline]
        pub fn slice_segment_address(mut self, slice_segment_address: u32) -> Self {
            self.slice_segment_address = slice_segment_address;
            self
        }

        #[inline]
        pub fn collocated_ref_idx(mut self, collocated_ref_idx: u8) -> Self {
            self.collocated_ref_idx = collocated_ref_idx;
            self
        }

        #[inline]
        pub fn max_num_merge_cand(mut self, max_num_merge_cand: u8) -> Self {
            self.max_num_merge_cand = max_num_merge_cand;
            self
        }

        #[inline]
        pub fn slice_cb_qp_offset(mut self, slice_cb_qp_offset: i8) -> Self {
            self.slice_cb_qp_offset = slice_cb_qp_offset;
            self
        }

        #[inline]
        pub fn slice_cr_qp_offset(mut self, slice_cr_qp_offset: i8) -> Self {
            self.slice_cr_qp_offset = slice_cr_qp_offset;
            self
        }

        #[inline]
        pub fn slice_beta_offset_div2(mut self, slice_beta_offset_div2: i8) -> Self {
            self.slice_beta_offset_div2 = slice_beta_offset_div2;
            self
        }

        #[inline]
        pub fn slice_tc_offset_div2(mut self, slice_tc_offset_div2: i8) -> Self {
            self.slice_tc_offset_div2 = slice_tc_offset_div2;
            self
        }

        #[inline]
        pub fn slice_act_y_qp_offset(mut self, slice_act_y_qp_offset: i8) -> Self {
            self.slice_act_y_qp_offset = slice_act_y_qp_offset;
            self
        }

        #[inline]
        pub fn slice_act_cb_qp_offset(mut self, slice_act_cb_qp_offset: i8) -> Self {
            self.slice_act_cb_qp_offset = slice_act_cb_qp_offset;
            self
        }

        #[inline]
        pub fn slice_act_cr_qp_offset(mut self, slice_act_cr_qp_offset: i8) -> Self {
            self.slice_act_cr_qp_offset = slice_act_cr_qp_offset;
            self
        }

        #[inline]
        pub fn slice_qp_delta(mut self, slice_qp_delta: i8) -> Self {
            self.slice_qp_delta = slice_qp_delta;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: u16) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn weight_table(mut self, weight_table: &'a StdVideoEncodeH265WeightTable) -> Self {
            self.p_weight_table = weight_table;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265ReferenceListsInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeH265ReferenceListsInfoFlags {
        pub ref_pic_list_modification_flag_l0: u32,
        pub ref_pic_list_modification_flag_l1: u32,
        pub reserved: u32,
    }

    impl StdVideoEncodeH265ReferenceListsInfoFlags {
        #[inline]
        pub fn ref_pic_list_modification_flag_l0(
            mut self,
            ref_pic_list_modification_flag_l0: u32,
        ) -> Self {
            self.ref_pic_list_modification_flag_l0 = ref_pic_list_modification_flag_l0;
            self
        }

        #[inline]
        pub fn ref_pic_list_modification_flag_l1(
            mut self,
            ref_pic_list_modification_flag_l1: u32,
        ) -> Self {
            self.ref_pic_list_modification_flag_l1 = ref_pic_list_modification_flag_l1;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265ReferenceListsInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoEncodeH265ReferenceListsInfo {
        pub flags: StdVideoEncodeH265ReferenceListsInfoFlags,
        pub num_ref_idx_l0_active_minus1: u8,
        pub num_ref_idx_l1_active_minus1: u8,
        pub ref_pic_list0: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub ref_pic_list1: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub list_entry_l0: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        pub list_entry_l1: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
    }

    impl Default for StdVideoEncodeH265ReferenceListsInfo {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                num_ref_idx_l0_active_minus1: Default::default(),
                num_ref_idx_l1_active_minus1: Default::default(),
                ref_pic_list0: [Default::default(); _],
                ref_pic_list1: [Default::default(); _],
                list_entry_l0: [Default::default(); _],
                list_entry_l1: [Default::default(); _],
            }
        }
    }

    impl StdVideoEncodeH265ReferenceListsInfo {
        #[inline]
        pub fn flags(mut self, flags: StdVideoEncodeH265ReferenceListsInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn num_ref_idx_l0_active_minus1(mut self, num_ref_idx_l0_active_minus1: u8) -> Self {
            self.num_ref_idx_l0_active_minus1 = num_ref_idx_l0_active_minus1;
            self
        }

        #[inline]
        pub fn num_ref_idx_l1_active_minus1(mut self, num_ref_idx_l1_active_minus1: u8) -> Self {
            self.num_ref_idx_l1_active_minus1 = num_ref_idx_l1_active_minus1;
            self
        }

        #[inline]
        pub fn ref_pic_list0(
            mut self,
            ref_pic_list0: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.ref_pic_list0 = ref_pic_list0;
            self
        }

        #[inline]
        pub fn ref_pic_list1(
            mut self,
            ref_pic_list1: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.ref_pic_list1 = ref_pic_list1;
            self
        }

        #[inline]
        pub fn list_entry_l0(
            mut self,
            list_entry_l0: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.list_entry_l0 = list_entry_l0;
            self
        }

        #[inline]
        pub fn list_entry_l1(
            mut self,
            list_entry_l1: [u8; STD_VIDEO_H265_MAX_NUM_LIST_REF as usize],
        ) -> Self {
            self.list_entry_l1 = list_entry_l1;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265PictureInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
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

    impl StdVideoEncodeH265PictureInfoFlags {
        #[inline]
        pub fn is_reference(mut self, is_reference: u32) -> Self {
            self.is_reference = is_reference;
            self
        }

        #[inline]
        pub fn irap_pic_flag(mut self, irap_pic_flag: u32) -> Self {
            self.irap_pic_flag = irap_pic_flag;
            self
        }

        #[inline]
        pub fn used_for_long_term_reference(mut self, used_for_long_term_reference: u32) -> Self {
            self.used_for_long_term_reference = used_for_long_term_reference;
            self
        }

        #[inline]
        pub fn discardable_flag(mut self, discardable_flag: u32) -> Self {
            self.discardable_flag = discardable_flag;
            self
        }

        #[inline]
        pub fn cross_layer_bla_flag(mut self, cross_layer_bla_flag: u32) -> Self {
            self.cross_layer_bla_flag = cross_layer_bla_flag;
            self
        }

        #[inline]
        pub fn pic_output_flag(mut self, pic_output_flag: u32) -> Self {
            self.pic_output_flag = pic_output_flag;
            self
        }

        #[inline]
        pub fn no_output_of_prior_pics_flag(mut self, no_output_of_prior_pics_flag: u32) -> Self {
            self.no_output_of_prior_pics_flag = no_output_of_prior_pics_flag;
            self
        }

        #[inline]
        pub fn short_term_ref_pic_set_sps_flag(
            mut self,
            short_term_ref_pic_set_sps_flag: u32,
        ) -> Self {
            self.short_term_ref_pic_set_sps_flag = short_term_ref_pic_set_sps_flag;
            self
        }

        #[inline]
        pub fn slice_temporal_mvp_enabled_flag(
            mut self,
            slice_temporal_mvp_enabled_flag: u32,
        ) -> Self {
            self.slice_temporal_mvp_enabled_flag = slice_temporal_mvp_enabled_flag;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265PictureInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoEncodeH265PictureInfo<'a> {
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
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoEncodeH265PictureInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoEncodeH265PictureInfo")
                .field("flags", &self.flags)
                .field("pic_type", &self.pic_type)
                .field(
                    "sps_video_parameter_set_id",
                    &self.sps_video_parameter_set_id,
                )
                .field("pps_seq_parameter_set_id", &self.pps_seq_parameter_set_id)
                .field("pps_pic_parameter_set_id", &self.pps_pic_parameter_set_id)
                .field(
                    "short_term_ref_pic_set_idx",
                    &self.short_term_ref_pic_set_idx,
                )
                .field("pic_order_cnt_val", &self.pic_order_cnt_val)
                .field("temporal_id", &self.temporal_id)
                .field("reserved1", &self.reserved1)
                .field("p_ref_lists", &self.p_ref_lists)
                .field("p_short_term_ref_pic_set", &self.p_short_term_ref_pic_set)
                .field("p_long_term_ref_pics", &self.p_long_term_ref_pics)
                .finish()
        }
    }

    impl Default for StdVideoEncodeH265PictureInfo<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                pic_type: Default::default(),
                sps_video_parameter_set_id: Default::default(),
                pps_seq_parameter_set_id: Default::default(),
                pps_pic_parameter_set_id: Default::default(),
                short_term_ref_pic_set_idx: Default::default(),
                pic_order_cnt_val: Default::default(),
                temporal_id: Default::default(),
                reserved1: [Default::default(); _],
                p_ref_lists: ptr::null(),
                p_short_term_ref_pic_set: ptr::null(),
                p_long_term_ref_pics: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoEncodeH265PictureInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoEncodeH265PictureInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn pic_type(mut self, pic_type: StdVideoH265PictureType) -> Self {
            self.pic_type = pic_type;
            self
        }

        #[inline]
        pub fn sps_video_parameter_set_id(mut self, sps_video_parameter_set_id: u8) -> Self {
            self.sps_video_parameter_set_id = sps_video_parameter_set_id;
            self
        }

        #[inline]
        pub fn pps_seq_parameter_set_id(mut self, pps_seq_parameter_set_id: u8) -> Self {
            self.pps_seq_parameter_set_id = pps_seq_parameter_set_id;
            self
        }

        #[inline]
        pub fn pps_pic_parameter_set_id(mut self, pps_pic_parameter_set_id: u8) -> Self {
            self.pps_pic_parameter_set_id = pps_pic_parameter_set_id;
            self
        }

        #[inline]
        pub fn short_term_ref_pic_set_idx(mut self, short_term_ref_pic_set_idx: u8) -> Self {
            self.short_term_ref_pic_set_idx = short_term_ref_pic_set_idx;
            self
        }

        #[inline]
        pub fn pic_order_cnt_val(mut self, pic_order_cnt_val: i32) -> Self {
            self.pic_order_cnt_val = pic_order_cnt_val;
            self
        }

        #[inline]
        pub fn temporal_id(mut self, temporal_id: u8) -> Self {
            self.temporal_id = temporal_id;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: [u8; 7]) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn ref_lists(mut self, ref_lists: &'a StdVideoEncodeH265ReferenceListsInfo) -> Self {
            self.p_ref_lists = ref_lists;
            self
        }

        #[inline]
        pub fn short_term_ref_pic_set(
            mut self,
            short_term_ref_pic_set: &'a StdVideoH265ShortTermRefPicSet,
        ) -> Self {
            self.p_short_term_ref_pic_set = short_term_ref_pic_set;
            self
        }

        #[inline]
        pub fn long_term_ref_pics(
            mut self,
            long_term_ref_pics: &'a StdVideoEncodeH265LongTermRefPics,
        ) -> Self {
            self.p_long_term_ref_pics = long_term_ref_pics;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265ReferenceInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeH265ReferenceInfoFlags {
        pub used_for_long_term_reference: u32,
        pub unused_for_reference: u32,
        pub reserved: u32,
    }

    impl StdVideoEncodeH265ReferenceInfoFlags {
        #[inline]
        pub fn used_for_long_term_reference(mut self, used_for_long_term_reference: u32) -> Self {
            self.used_for_long_term_reference = used_for_long_term_reference;
            self
        }

        #[inline]
        pub fn unused_for_reference(mut self, unused_for_reference: u32) -> Self {
            self.unused_for_reference = unused_for_reference;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeH265ReferenceInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeH265ReferenceInfo {
        pub flags: StdVideoEncodeH265ReferenceInfoFlags,
        pub pic_type: StdVideoH265PictureType,
        pub pic_order_cnt_val: i32,
        pub temporal_id: u8,
    }

    impl StdVideoEncodeH265ReferenceInfo {
        #[inline]
        pub fn flags(mut self, flags: StdVideoEncodeH265ReferenceInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn pic_type(mut self, pic_type: StdVideoH265PictureType) -> Self {
            self.pic_type = pic_type;
            self
        }

        #[inline]
        pub fn pic_order_cnt_val(mut self, pic_order_cnt_val: i32) -> Self {
            self.pic_order_cnt_val = pic_order_cnt_val;
            self
        }

        #[inline]
        pub fn temporal_id(mut self, temporal_id: u8) -> Self {
            self.temporal_id = temporal_id;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type StdVideoEncodeH265WeightTableFlags = super::defs::StdVideoEncodeH265WeightTableFlags;
    pub type StdVideoEncodeH265WeightTable = super::defs::StdVideoEncodeH265WeightTable;
    pub type StdVideoEncodeH265LongTermRefPics = super::defs::StdVideoEncodeH265LongTermRefPics;
    pub type StdVideoEncodeH265SliceSegmentHeaderFlags =
        super::defs::StdVideoEncodeH265SliceSegmentHeaderFlags;
    pub type StdVideoEncodeH265SliceSegmentHeader =
        super::defs::StdVideoEncodeH265SliceSegmentHeader<'static>;
    pub type StdVideoEncodeH265ReferenceListsInfoFlags =
        super::defs::StdVideoEncodeH265ReferenceListsInfoFlags;
    pub type StdVideoEncodeH265ReferenceListsInfo =
        super::defs::StdVideoEncodeH265ReferenceListsInfo;
    pub type StdVideoEncodeH265PictureInfoFlags = super::defs::StdVideoEncodeH265PictureInfoFlags;
    pub type StdVideoEncodeH265PictureInfo = super::defs::StdVideoEncodeH265PictureInfo<'static>;
    pub type StdVideoEncodeH265ReferenceInfoFlags =
        super::defs::StdVideoEncodeH265ReferenceInfoFlags;
    pub type StdVideoEncodeH265ReferenceInfo = super::defs::StdVideoEncodeH265ReferenceInfo;
    impl super::defs::StdVideoEncodeH265SliceSegmentHeader<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &StdVideoEncodeH265SliceSegmentHeader {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl super::defs::StdVideoEncodeH265PictureInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &StdVideoEncodeH265PictureInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
}
