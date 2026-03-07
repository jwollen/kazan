#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"vulkan_video_codec_h264std_decode";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE: u32 = 2;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH264PictureInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoDecodeH264PictureInfoFlags {
        pub field_pic_flag: u32,
        pub is_intra: u32,
        pub idr_pic_flag: u32,
        pub bottom_field_flag: u32,
        pub is_reference: u32,
        pub complementary_field_pair: u32,
    }

    impl StdVideoDecodeH264PictureInfoFlags {
        #[inline]
        pub fn field_pic_flag(mut self, field_pic_flag: u32) -> Self {
            self.field_pic_flag = field_pic_flag;
            self
        }

        #[inline]
        pub fn is_intra(mut self, is_intra: u32) -> Self {
            self.is_intra = is_intra;
            self
        }

        #[inline]
        pub fn idr_pic_flag(mut self, idr_pic_flag: u32) -> Self {
            self.idr_pic_flag = idr_pic_flag;
            self
        }

        #[inline]
        pub fn bottom_field_flag(mut self, bottom_field_flag: u32) -> Self {
            self.bottom_field_flag = bottom_field_flag;
            self
        }

        #[inline]
        pub fn is_reference(mut self, is_reference: u32) -> Self {
            self.is_reference = is_reference;
            self
        }

        #[inline]
        pub fn complementary_field_pair(mut self, complementary_field_pair: u32) -> Self {
            self.complementary_field_pair = complementary_field_pair;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH264PictureInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
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

    impl Default for StdVideoDecodeH264PictureInfo {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                seq_parameter_set_id: Default::default(),
                pic_parameter_set_id: Default::default(),
                reserved1: Default::default(),
                reserved2: Default::default(),
                frame_num: Default::default(),
                idr_pic_id: Default::default(),
                pic_order_cnt: [Default::default(); _],
            }
        }
    }

    impl StdVideoDecodeH264PictureInfo {
        #[inline]
        pub fn flags(mut self, flags: StdVideoDecodeH264PictureInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn seq_parameter_set_id(mut self, seq_parameter_set_id: u8) -> Self {
            self.seq_parameter_set_id = seq_parameter_set_id;
            self
        }

        #[inline]
        pub fn pic_parameter_set_id(mut self, pic_parameter_set_id: u8) -> Self {
            self.pic_parameter_set_id = pic_parameter_set_id;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: u8) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn reserved2(mut self, reserved2: u8) -> Self {
            self.reserved2 = reserved2;
            self
        }

        #[inline]
        pub fn frame_num(mut self, frame_num: u16) -> Self {
            self.frame_num = frame_num;
            self
        }

        #[inline]
        pub fn idr_pic_id(mut self, idr_pic_id: u16) -> Self {
            self.idr_pic_id = idr_pic_id;
            self
        }

        #[inline]
        pub fn pic_order_cnt(
            mut self,
            pic_order_cnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
        ) -> Self {
            self.pic_order_cnt = pic_order_cnt;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH264ReferenceInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoDecodeH264ReferenceInfoFlags {
        pub top_field_flag: u32,
        pub bottom_field_flag: u32,
        pub used_for_long_term_reference: u32,
        pub is_non_existing: u32,
    }

    impl StdVideoDecodeH264ReferenceInfoFlags {
        #[inline]
        pub fn top_field_flag(mut self, top_field_flag: u32) -> Self {
            self.top_field_flag = top_field_flag;
            self
        }

        #[inline]
        pub fn bottom_field_flag(mut self, bottom_field_flag: u32) -> Self {
            self.bottom_field_flag = bottom_field_flag;
            self
        }

        #[inline]
        pub fn used_for_long_term_reference(mut self, used_for_long_term_reference: u32) -> Self {
            self.used_for_long_term_reference = used_for_long_term_reference;
            self
        }

        #[inline]
        pub fn is_non_existing(mut self, is_non_existing: u32) -> Self {
            self.is_non_existing = is_non_existing;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH264ReferenceInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoDecodeH264ReferenceInfo {
        pub flags: StdVideoDecodeH264ReferenceInfoFlags,
        pub frame_num: u16,
        pub reserved: u16,
        pub pic_order_cnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
    }

    impl Default for StdVideoDecodeH264ReferenceInfo {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                frame_num: Default::default(),
                reserved: Default::default(),
                pic_order_cnt: [Default::default(); _],
            }
        }
    }

    impl StdVideoDecodeH264ReferenceInfo {
        #[inline]
        pub fn flags(mut self, flags: StdVideoDecodeH264ReferenceInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn frame_num(mut self, frame_num: u16) -> Self {
            self.frame_num = frame_num;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u16) -> Self {
            self.reserved = reserved;
            self
        }

        #[inline]
        pub fn pic_order_cnt(
            mut self,
            pic_order_cnt: [i32; STD_VIDEO_DECODE_H264_FIELD_ORDER_COUNT_LIST_SIZE as usize],
        ) -> Self {
            self.pic_order_cnt = pic_order_cnt;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeH264FieldOrderCount.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoDecodeH264FieldOrderCount(i32);

    impl StdVideoDecodeH264FieldOrderCount {
        pub const TOP: Self = Self(0);
        pub const BOTTOM: Self = Self(1);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoDecodeH264FieldOrderCount {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::TOP => Some("TOP"),
                Self::BOTTOM => Some("BOTTOM"),
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
