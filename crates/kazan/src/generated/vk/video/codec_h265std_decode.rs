#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"vulkan_video_codec_h265std_decode";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE: u32 = 8;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH265PictureInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoDecodeH265PictureInfoFlags {
        pub irap_pic_flag: u32,
        pub idr_pic_flag: u32,
        pub is_reference: u32,
        pub short_term_ref_pic_set_sps_flag: u32,
    }

    impl StdVideoDecodeH265PictureInfoFlags {
        #[inline]
        pub fn irap_pic_flag(mut self, irap_pic_flag: u32) -> Self {
            self.irap_pic_flag = irap_pic_flag;
            self
        }

        #[inline]
        pub fn idr_pic_flag(mut self, idr_pic_flag: u32) -> Self {
            self.idr_pic_flag = idr_pic_flag;
            self
        }

        #[inline]
        pub fn is_reference(mut self, is_reference: u32) -> Self {
            self.is_reference = is_reference;
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
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH265PictureInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoDecodeH265PictureInfo {
        pub flags: StdVideoDecodeH265PictureInfoFlags,
        pub sps_video_parameter_set_id: u8,
        pub pps_seq_parameter_set_id: u8,
        pub pps_pic_parameter_set_id: u8,
        pub num_delta_pocs_of_ref_rps_idx: u8,
        pub pic_order_cnt_val: i32,
        pub num_bits_for_st_ref_pic_set_in_slice: u16,
        pub reserved: u16,
        pub ref_pic_set_st_curr_before: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
        pub ref_pic_set_st_curr_after: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
        pub ref_pic_set_lt_curr: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
    }

    impl Default for StdVideoDecodeH265PictureInfo {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                sps_video_parameter_set_id: Default::default(),
                pps_seq_parameter_set_id: Default::default(),
                pps_pic_parameter_set_id: Default::default(),
                num_delta_pocs_of_ref_rps_idx: Default::default(),
                pic_order_cnt_val: Default::default(),
                num_bits_for_st_ref_pic_set_in_slice: Default::default(),
                reserved: Default::default(),
                ref_pic_set_st_curr_before: [Default::default(); _],
                ref_pic_set_st_curr_after: [Default::default(); _],
                ref_pic_set_lt_curr: [Default::default(); _],
            }
        }
    }

    impl StdVideoDecodeH265PictureInfo {
        #[inline]
        pub fn flags(mut self, flags: StdVideoDecodeH265PictureInfoFlags) -> Self {
            self.flags = flags;
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
        pub fn num_delta_pocs_of_ref_rps_idx(mut self, num_delta_pocs_of_ref_rps_idx: u8) -> Self {
            self.num_delta_pocs_of_ref_rps_idx = num_delta_pocs_of_ref_rps_idx;
            self
        }

        #[inline]
        pub fn pic_order_cnt_val(mut self, pic_order_cnt_val: i32) -> Self {
            self.pic_order_cnt_val = pic_order_cnt_val;
            self
        }

        #[inline]
        pub fn num_bits_for_st_ref_pic_set_in_slice(
            mut self,
            num_bits_for_st_ref_pic_set_in_slice: u16,
        ) -> Self {
            self.num_bits_for_st_ref_pic_set_in_slice = num_bits_for_st_ref_pic_set_in_slice;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u16) -> Self {
            self.reserved = reserved;
            self
        }

        #[inline]
        pub fn ref_pic_set_st_curr_before(
            mut self,
            ref_pic_set_st_curr_before: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
        ) -> Self {
            self.ref_pic_set_st_curr_before = ref_pic_set_st_curr_before;
            self
        }

        #[inline]
        pub fn ref_pic_set_st_curr_after(
            mut self,
            ref_pic_set_st_curr_after: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
        ) -> Self {
            self.ref_pic_set_st_curr_after = ref_pic_set_st_curr_after;
            self
        }

        #[inline]
        pub fn ref_pic_set_lt_curr(
            mut self,
            ref_pic_set_lt_curr: [u8; STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE as usize],
        ) -> Self {
            self.ref_pic_set_lt_curr = ref_pic_set_lt_curr;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH265ReferenceInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoDecodeH265ReferenceInfoFlags {
        pub used_for_long_term_reference: u32,
        pub unused_for_reference: u32,
    }

    impl StdVideoDecodeH265ReferenceInfoFlags {
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
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH265ReferenceInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoDecodeH265ReferenceInfo {
        pub flags: StdVideoDecodeH265ReferenceInfoFlags,
        pub pic_order_cnt_val: i32,
    }

    impl StdVideoDecodeH265ReferenceInfo {
        #[inline]
        pub fn flags(mut self, flags: StdVideoDecodeH265ReferenceInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn pic_order_cnt_val(mut self, pic_order_cnt_val: i32) -> Self {
            self.pic_order_cnt_val = pic_order_cnt_val;
            self
        }
    }
}
