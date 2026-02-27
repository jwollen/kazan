#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
pub const STD_VIDEO_VP9_NUM_REF_FRAMES: u32 = 8;
pub const STD_VIDEO_VP9_REFS_PER_FRAME: u32 = 3;
pub const STD_VIDEO_VP9_MAX_REF_FRAMES: u32 = 4;
pub const STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
pub const STD_VIDEO_VP9_MAX_SEGMENTS: u32 = 8;
pub const STD_VIDEO_VP9_SEG_LVL_MAX: u32 = 4;
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS: u32 = 7;
pub const STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB: u32 = 3;
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoVP9ColorConfigFlags {
    pub color_range: u32,
    pub reserved: u32,
}
impl StdVideoVP9ColorConfigFlags {
    pub fn color_range(mut self, color_range: u32) -> Self {
        self.color_range = color_range;
        self
    }
    pub fn reserved(mut self, reserved: u32) -> Self {
        self.reserved = reserved;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoVP9ColorConfig {
    pub flags: StdVideoVP9ColorConfigFlags,
    pub bit_depth: u8,
    pub subsampling_x: u8,
    pub subsampling_y: u8,
    pub reserved1: u8,
    pub color_space: StdVideoVP9ColorSpace,
}
impl StdVideoVP9ColorConfig {
    pub fn flags(mut self, flags: StdVideoVP9ColorConfigFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn bit_depth(mut self, bit_depth: u8) -> Self {
        self.bit_depth = bit_depth;
        self
    }
    pub fn subsampling_x(mut self, subsampling_x: u8) -> Self {
        self.subsampling_x = subsampling_x;
        self
    }
    pub fn subsampling_y(mut self, subsampling_y: u8) -> Self {
        self.subsampling_y = subsampling_y;
        self
    }
    pub fn reserved1(mut self, reserved1: u8) -> Self {
        self.reserved1 = reserved1;
        self
    }
    pub fn color_space(mut self, color_space: StdVideoVP9ColorSpace) -> Self {
        self.color_space = color_space;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoVP9LoopFilterFlags {
    pub loop_filter_delta_enabled: u32,
    pub loop_filter_delta_update: u32,
    pub reserved: u32,
}
impl StdVideoVP9LoopFilterFlags {
    pub fn loop_filter_delta_enabled(mut self, loop_filter_delta_enabled: u32) -> Self {
        self.loop_filter_delta_enabled = loop_filter_delta_enabled;
        self
    }
    pub fn loop_filter_delta_update(mut self, loop_filter_delta_update: u32) -> Self {
        self.loop_filter_delta_update = loop_filter_delta_update;
        self
    }
    pub fn reserved(mut self, reserved: u32) -> Self {
        self.reserved = reserved;
        self
    }
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
impl Default for StdVideoVP9LoopFilter {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            loop_filter_level: Default::default(),
            loop_filter_sharpness: Default::default(),
            update_ref_delta: Default::default(),
            loop_filter_ref_deltas: [Default::default(); _],
            update_mode_delta: Default::default(),
            loop_filter_mode_deltas: [Default::default(); _],
        }
    }
}
impl StdVideoVP9LoopFilter {
    pub fn flags(mut self, flags: StdVideoVP9LoopFilterFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn loop_filter_level(mut self, loop_filter_level: u8) -> Self {
        self.loop_filter_level = loop_filter_level;
        self
    }
    pub fn loop_filter_sharpness(mut self, loop_filter_sharpness: u8) -> Self {
        self.loop_filter_sharpness = loop_filter_sharpness;
        self
    }
    pub fn update_ref_delta(mut self, update_ref_delta: u8) -> Self {
        self.update_ref_delta = update_ref_delta;
        self
    }
    pub fn loop_filter_ref_deltas(
        mut self,
        loop_filter_ref_deltas: [i8; STD_VIDEO_VP9_MAX_REF_FRAMES as usize],
    ) -> Self {
        self.loop_filter_ref_deltas = loop_filter_ref_deltas;
        self
    }
    pub fn update_mode_delta(mut self, update_mode_delta: u8) -> Self {
        self.update_mode_delta = update_mode_delta;
        self
    }
    pub fn loop_filter_mode_deltas(
        mut self,
        loop_filter_mode_deltas: [i8; STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS as usize],
    ) -> Self {
        self.loop_filter_mode_deltas = loop_filter_mode_deltas;
        self
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct StdVideoVP9SegmentationFlags {
    pub segmentation_update_map: u32,
    pub segmentation_temporal_update: u32,
    pub segmentation_update_data: u32,
    pub segmentation_abs_or_delta_update: u32,
    pub reserved: u32,
}
impl StdVideoVP9SegmentationFlags {
    pub fn segmentation_update_map(mut self, segmentation_update_map: u32) -> Self {
        self.segmentation_update_map = segmentation_update_map;
        self
    }
    pub fn segmentation_temporal_update(mut self, segmentation_temporal_update: u32) -> Self {
        self.segmentation_temporal_update = segmentation_temporal_update;
        self
    }
    pub fn segmentation_update_data(mut self, segmentation_update_data: u32) -> Self {
        self.segmentation_update_data = segmentation_update_data;
        self
    }
    pub fn segmentation_abs_or_delta_update(
        mut self,
        segmentation_abs_or_delta_update: u32,
    ) -> Self {
        self.segmentation_abs_or_delta_update = segmentation_abs_or_delta_update;
        self
    }
    pub fn reserved(mut self, reserved: u32) -> Self {
        self.reserved = reserved;
        self
    }
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
impl Default for StdVideoVP9Segmentation {
    fn default() -> Self {
        Self {
            flags: Default::default(),
            segmentation_tree_probs: [Default::default(); _],
            segmentation_pred_prob: [Default::default(); _],
            feature_enabled: [Default::default(); _],
            feature_data: [[Default::default(); _]; _],
        }
    }
}
impl StdVideoVP9Segmentation {
    pub fn flags(mut self, flags: StdVideoVP9SegmentationFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn segmentation_tree_probs(
        mut self,
        segmentation_tree_probs: [u8; STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS as usize],
    ) -> Self {
        self.segmentation_tree_probs = segmentation_tree_probs;
        self
    }
    pub fn segmentation_pred_prob(
        mut self,
        segmentation_pred_prob: [u8; STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB as usize],
    ) -> Self {
        self.segmentation_pred_prob = segmentation_pred_prob;
        self
    }
    pub fn feature_enabled(
        mut self,
        feature_enabled: [u8; STD_VIDEO_VP9_MAX_SEGMENTS as usize],
    ) -> Self {
        self.feature_enabled = feature_enabled;
        self
    }
    pub fn feature_data(
        mut self,
        feature_data: [[i16; STD_VIDEO_VP9_SEG_LVL_MAX as usize];
            STD_VIDEO_VP9_MAX_SEGMENTS as usize],
    ) -> Self {
        self.feature_data = feature_data;
        self
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9Profile(i32);
impl StdVideoVP9Profile {
    pub const _0: Self = Self(0);
    pub const _1: Self = Self(1);
    pub const _2: Self = Self(2);
    pub const _3: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9FrameType(i32);
impl StdVideoVP9FrameType {
    pub const KEY: Self = Self(0);
    pub const NON_KEY: Self = Self(1);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StdVideoVP9ReferenceName(i32);
impl StdVideoVP9ReferenceName {
    pub const INTRA_FRAME: Self = Self(0);
    pub const LAST_FRAME: Self = Self(1);
    pub const GOLDEN_FRAME: Self = Self(2);
    pub const ALTREF_FRAME: Self = Self(3);
    pub const INVALID: Self = Self(0x7FFFFFFF);
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
