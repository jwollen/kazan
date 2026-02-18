#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: u32 = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoDecodeH264PictureInfoFlags {
    pub field_pic_flag: u32,
    pub is_intra: u32,
    pub idr_pic_flag: u32,
    pub bottom_field_flag: u32,
    pub is_reference: u32,
    pub complementary_field_pair: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoDecodeH264PictureInfo {
    pub flags: StdVideoDecodeH264PictureInfoFlags,
    pub seq_parameter_set_id: u8,
    pub pic_parameter_set_id: u8,
    pub reserved1: u8,
    pub reserved2: u8,
    pub frame_num: u16,
    pub idr_pic_id: u16,
    pub pic_order_cnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoDecodeH264ReferenceInfoFlags {
    pub top_field_flag: u32,
    pub bottom_field_flag: u32,
    pub used_for_long_term_reference: u32,
    pub is_non_existing: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoDecodeH264ReferenceInfo {
    pub flags: StdVideoDecodeH264ReferenceInfoFlags,
    pub frame_num: u16,
    pub reserved: u16,
    pub pic_order_cnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoDecodeH264FieldOrderCount(i32);
impl StdVideoDecodeH264FieldOrderCount {
    pub const TOP: Self = Self(0);
    pub const BOTTOM: Self = Self(1);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
