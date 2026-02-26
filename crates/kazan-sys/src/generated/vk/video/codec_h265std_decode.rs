#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const STD_VIDEO_DECODE_H265_REF_PIC_SET_LIST_SIZE: u32 = 8;
#[repr(C)]
pub struct StdVideoDecodeH265PictureInfoFlags {
    pub irap_pic_flag: u32,
    pub idr_pic_flag: u32,
    pub is_reference: u32,
    pub short_term_ref_pic_set_sps_flag: u32,
}
#[repr(C)]
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
#[repr(C)]
pub struct StdVideoDecodeH265ReferenceInfoFlags {
    pub used_for_long_term_reference: u32,
    pub unused_for_reference: u32,
}
#[repr(C)]
pub struct StdVideoDecodeH265ReferenceInfo {
    pub flags: StdVideoDecodeH265ReferenceInfoFlags,
    pub pic_order_cnt_val: i32,
}
