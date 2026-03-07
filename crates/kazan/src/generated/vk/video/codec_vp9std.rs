#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const STD_VIDEO_VP9_NUM_REF_FRAMES: u32 = 8;
    pub const STD_VIDEO_VP9_REFS_PER_FRAME: u32 = 3;
    pub const STD_VIDEO_VP9_MAX_REF_FRAMES: u32 = 4;
    pub const STD_VIDEO_VP9_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
    pub const STD_VIDEO_VP9_MAX_SEGMENTS: u32 = 8;
    pub const STD_VIDEO_VP9_SEG_LVL_MAX: u32 = 4;
    pub const STD_VIDEO_VP9_MAX_SEGMENTATION_TREE_PROBS: u32 = 7;
    pub const STD_VIDEO_VP9_MAX_SEGMENTATION_PRED_PROB: u32 = 3;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9ColorConfigFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9ColorConfig.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9LoopFilterFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9LoopFilter.html>
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9SegmentationFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9Segmentation.html>
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9Profile.html>
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

    impl fmt::Debug for StdVideoVP9Profile {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_0 => Some("_0"),
                Self::_1 => Some("_1"),
                Self::_2 => Some("_2"),
                Self::_3 => Some("_3"),
                Self::INVALID => Some("INVALID"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9Level.html>
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

    impl fmt::Debug for StdVideoVP9Level {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1_0 => Some("_1_0"),
                Self::_1_1 => Some("_1_1"),
                Self::_2_0 => Some("_2_0"),
                Self::_2_1 => Some("_2_1"),
                Self::_3_0 => Some("_3_0"),
                Self::_3_1 => Some("_3_1"),
                Self::_4_0 => Some("_4_0"),
                Self::_4_1 => Some("_4_1"),
                Self::_5_0 => Some("_5_0"),
                Self::_5_1 => Some("_5_1"),
                Self::_5_2 => Some("_5_2"),
                Self::_6_0 => Some("_6_0"),
                Self::_6_1 => Some("_6_1"),
                Self::_6_2 => Some("_6_2"),
                Self::INVALID => Some("INVALID"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9FrameType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoVP9FrameType(i32);

    impl StdVideoVP9FrameType {
        pub const KEY: Self = Self(0);
        pub const NON_KEY: Self = Self(1);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoVP9FrameType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::KEY => Some("KEY"),
                Self::NON_KEY => Some("NON_KEY"),
                Self::INVALID => Some("INVALID"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9ReferenceName.html>
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

    impl fmt::Debug for StdVideoVP9ReferenceName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INTRA_FRAME => Some("INTRA_FRAME"),
                Self::LAST_FRAME => Some("LAST_FRAME"),
                Self::GOLDEN_FRAME => Some("GOLDEN_FRAME"),
                Self::ALTREF_FRAME => Some("ALTREF_FRAME"),
                Self::INVALID => Some("INVALID"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9InterpolationFilter.html>
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

    impl fmt::Debug for StdVideoVP9InterpolationFilter {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::EIGHTTAP => Some("EIGHTTAP"),
                Self::EIGHTTAP_SMOOTH => Some("EIGHTTAP_SMOOTH"),
                Self::EIGHTTAP_SHARP => Some("EIGHTTAP_SHARP"),
                Self::BILINEAR => Some("BILINEAR"),
                Self::SWITCHABLE => Some("SWITCHABLE"),
                Self::INVALID => Some("INVALID"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoVP9ColorSpace.html>
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

    impl fmt::Debug for StdVideoVP9ColorSpace {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNKNOWN => Some("UNKNOWN"),
                Self::BT_601 => Some("BT_601"),
                Self::BT_709 => Some("BT_709"),
                Self::SMPTE_170 => Some("SMPTE_170"),
                Self::SMPTE_240 => Some("SMPTE_240"),
                Self::BT_2020 => Some("BT_2020"),
                Self::RESERVED => Some("RESERVED"),
                Self::RGB => Some("RGB"),
                Self::INVALID => Some("INVALID"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}
