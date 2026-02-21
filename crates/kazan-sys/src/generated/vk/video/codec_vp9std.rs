#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const STD_VIDEO_VP9_NUM_REF_FRAMES: u32 = 8;
pub const STD_VIDEO_VP9_REFS_PER_FRAME: u32 = 3;
pub const STD_VIDEO_VP9_MAX_REF_FRAMES: u32 = 4;
pub const STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
pub const STD_VIDEO_VP9_MAX_SEGMENTS: u32 = 8;
pub const STD_VIDEO_VP9_SEG_LVL_MAX: u32 = 4;
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS: u32 = 7;
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB: u32 = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoVP9ColorConfigFlags {
    pub color_range: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoVP9ColorConfig {
    pub flags: StdVideoVP9ColorConfigFlags,
    pub bit_depth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub reserved1: u8,
    pub color_space: StdVideoVP9ColorSpace,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoVP9LoopFilterFlags {
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoVP9LoopFilter {
    pub flags: StdVideoVP9LoopFilterFlags,
    pub loop_filter_level: u8,
    pub loop_filter_sharpness: u8,
    pub update_ref_delta: u8,
    pub loop_filter_ref_deltas: [i8; STD_VIDEO_VP9_MAX_REF_FRAMES as usize],
    pub update_mode_delta: u8,
    pub loop_filter_mode_deltas: [i8; STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS as usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoVP9SegmentationFlags {
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub segmentation_abs_or_delta_update: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoVP9Segmentation {
    pub flags: StdVideoVP9SegmentationFlags,
    pub segmentation_tree_probs: [u8; STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS as usize],
    pub segmentation_pred_prob: [u8; STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB as usize],
    pub feature_enabled: [u8; STD_VIDEO_VP9_MAX_SEGMENTS as usize],
    pub feature_data:
        [[i16; STD_VIDEO_VP9_SEG_LVL_MAX as usize]; STD_VIDEO_VP9_MAX_SEGMENTS as usize],
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9Profile(i32);
impl StdVideoVP9Profile {
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const _3: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9Level(i32);
impl StdVideoVP9Level {
    pub const _1_0: Self = Self(0);
    pub const _1_1: Self = Self(1);
    pub const _2_0: Self = Self(2);
    pub const _2_1: Self = Self(3);
    pub const _3_0: Self = Self(4);
    pub const _3_1: Self = Self(5);
    pub const _4_0: Self = Self(6);
    pub const _4_1: Self = Self(7);
    pub const _5_0: Self = Self(8);
    pub const _5_1: Self = Self(9);
    pub const _5_2: Self = Self(10);
    pub const _6_0: Self = Self(11);
    pub const _6_1: Self = Self(12);
    pub const _6_2: Self = Self(13);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9FrameType(i32);
impl StdVideoVP9FrameType {
    pub const KEY: Self = Self(0);
    pub const NON_KEY: Self = Self(1);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9ReferenceName(i32);
impl StdVideoVP9ReferenceName {
    pub const INTRA_FRAME: Self = Self(0);
    pub const LAST_FRAME: Self = Self(1);
    pub const GOLDEN_FRAME: Self = Self(2);
    pub const ALTREF_FRAME: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9InterpolationFilter(i32);
impl StdVideoVP9InterpolationFilter {
    pub const EIGHTTAP: Self = Self(0);
    pub const EIGHTTAP_SMOOTH: Self = Self(1);
    pub const EIGHTTAP_SHARP: Self = Self(2);
    pub const BILINEAR: Self = Self(3);
    pub const SWITCHABLE: Self = Self(4);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9ColorSpace(i32);
impl StdVideoVP9ColorSpace {
    pub const UNKNOWN: Self = Self(0);
    pub const BT_601: Self = Self(1);
    pub const BT_709: Self = Self(2);
    pub const SMPTE_170: Self = Self(3);
    pub const SMPTE_240: Self = Self(4);
    pub const BT_2020: Self = Self(5);
    pub const RESERVED: Self = Self(6);
    pub const RGB: Self = Self(7);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
