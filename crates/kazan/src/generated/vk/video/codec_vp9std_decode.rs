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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeVP9PictureInfoFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct StdVideoDecodeVP9PictureInfoFlags {
        pub error_resilient_mode: u32,
        pub intra_only: u32,
        pub allow_high_precision_mv: u32,
        pub refresh_frame_context: u32,
        pub frame_parallel_decoding_mode: u32,
        pub segmentation_enabled: u32,
        pub show_frame: u32,
        pub use_prev_frame_mvs: u32,
        pub reserved: u32,
    }

    impl StdVideoDecodeVP9PictureInfoFlags {
        pub fn error_resilient_mode(mut self, error_resilient_mode: u32) -> Self {
            self.error_resilient_mode = error_resilient_mode;
            self
        }

        pub fn intra_only(mut self, intra_only: u32) -> Self {
            self.intra_only = intra_only;
            self
        }

        pub fn allow_high_precision_mv(mut self, allow_high_precision_mv: u32) -> Self {
            self.allow_high_precision_mv = allow_high_precision_mv;
            self
        }

        pub fn refresh_frame_context(mut self, refresh_frame_context: u32) -> Self {
            self.refresh_frame_context = refresh_frame_context;
            self
        }

        pub fn frame_parallel_decoding_mode(mut self, frame_parallel_decoding_mode: u32) -> Self {
            self.frame_parallel_decoding_mode = frame_parallel_decoding_mode;
            self
        }

        pub fn segmentation_enabled(mut self, segmentation_enabled: u32) -> Self {
            self.segmentation_enabled = segmentation_enabled;
            self
        }

        pub fn show_frame(mut self, show_frame: u32) -> Self {
            self.show_frame = show_frame;
            self
        }

        pub fn use_prev_frame_mvs(mut self, use_prev_frame_mvs: u32) -> Self {
            self.use_prev_frame_mvs = use_prev_frame_mvs;
            self
        }

        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeVP9PictureInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct StdVideoDecodeVP9PictureInfo<'a> {
        pub flags: StdVideoDecodeVP9PictureInfoFlags,
        pub profile: StdVideoVP9Profile,
        pub frame_type: StdVideoVP9FrameType,
        pub frame_context_idx: u8,
        pub reset_frame_context: u8,
        pub refresh_frame_flags: u8,
        pub ref_frame_sign_bias_mask: u8,
        pub interpolation_filter: StdVideoVP9InterpolationFilter,
        pub base_q_idx: u8,
        pub delta_q_y_dc: i8,
        pub delta_q_uv_dc: i8,
        pub delta_q_uv_ac: i8,
        pub tile_cols_log2: u8,
        pub tile_rows_log2: u8,
        pub reserved1: [u16; 3],
        pub p_color_config: *const StdVideoVP9ColorConfig,
        pub p_loop_filter: *const StdVideoVP9LoopFilter,
        pub p_segmentation: *const StdVideoVP9Segmentation,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for StdVideoDecodeVP9PictureInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoDecodeVP9PictureInfo")
                .field("flags", &self.flags)
                .field("profile", &self.profile)
                .field("frame_type", &self.frame_type)
                .field("frame_context_idx", &self.frame_context_idx)
                .field("reset_frame_context", &self.reset_frame_context)
                .field("refresh_frame_flags", &self.refresh_frame_flags)
                .field("ref_frame_sign_bias_mask", &self.ref_frame_sign_bias_mask)
                .field("interpolation_filter", &self.interpolation_filter)
                .field("base_q_idx", &self.base_q_idx)
                .field("delta_q_y_dc", &self.delta_q_y_dc)
                .field("delta_q_uv_dc", &self.delta_q_uv_dc)
                .field("delta_q_uv_ac", &self.delta_q_uv_ac)
                .field("tile_cols_log2", &self.tile_cols_log2)
                .field("tile_rows_log2", &self.tile_rows_log2)
                .field("reserved1", &self.reserved1)
                .field("p_color_config", &self.p_color_config)
                .field("p_loop_filter", &self.p_loop_filter)
                .field("p_segmentation", &self.p_segmentation)
                .finish()
        }
    }

    impl Default for StdVideoDecodeVP9PictureInfo<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                profile: Default::default(),
                frame_type: Default::default(),
                frame_context_idx: Default::default(),
                reset_frame_context: Default::default(),
                refresh_frame_flags: Default::default(),
                ref_frame_sign_bias_mask: Default::default(),
                interpolation_filter: Default::default(),
                base_q_idx: Default::default(),
                delta_q_y_dc: Default::default(),
                delta_q_uv_dc: Default::default(),
                delta_q_uv_ac: Default::default(),
                tile_cols_log2: Default::default(),
                tile_rows_log2: Default::default(),
                reserved1: [Default::default(); _],
                p_color_config: core::ptr::null(),
                p_loop_filter: core::ptr::null(),
                p_segmentation: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoDecodeVP9PictureInfo<'a> {
        pub fn flags(mut self, flags: StdVideoDecodeVP9PictureInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        pub fn profile(mut self, profile: StdVideoVP9Profile) -> Self {
            self.profile = profile;
            self
        }

        pub fn frame_type(mut self, frame_type: StdVideoVP9FrameType) -> Self {
            self.frame_type = frame_type;
            self
        }

        pub fn frame_context_idx(mut self, frame_context_idx: u8) -> Self {
            self.frame_context_idx = frame_context_idx;
            self
        }

        pub fn reset_frame_context(mut self, reset_frame_context: u8) -> Self {
            self.reset_frame_context = reset_frame_context;
            self
        }

        pub fn refresh_frame_flags(mut self, refresh_frame_flags: u8) -> Self {
            self.refresh_frame_flags = refresh_frame_flags;
            self
        }

        pub fn ref_frame_sign_bias_mask(mut self, ref_frame_sign_bias_mask: u8) -> Self {
            self.ref_frame_sign_bias_mask = ref_frame_sign_bias_mask;
            self
        }

        pub fn interpolation_filter(
            mut self,
            interpolation_filter: StdVideoVP9InterpolationFilter,
        ) -> Self {
            self.interpolation_filter = interpolation_filter;
            self
        }

        pub fn base_q_idx(mut self, base_q_idx: u8) -> Self {
            self.base_q_idx = base_q_idx;
            self
        }

        pub fn delta_q_y_dc(mut self, delta_q_y_dc: i8) -> Self {
            self.delta_q_y_dc = delta_q_y_dc;
            self
        }

        pub fn delta_q_uv_dc(mut self, delta_q_uv_dc: i8) -> Self {
            self.delta_q_uv_dc = delta_q_uv_dc;
            self
        }

        pub fn delta_q_uv_ac(mut self, delta_q_uv_ac: i8) -> Self {
            self.delta_q_uv_ac = delta_q_uv_ac;
            self
        }

        pub fn tile_cols_log2(mut self, tile_cols_log2: u8) -> Self {
            self.tile_cols_log2 = tile_cols_log2;
            self
        }

        pub fn tile_rows_log2(mut self, tile_rows_log2: u8) -> Self {
            self.tile_rows_log2 = tile_rows_log2;
            self
        }

        pub fn reserved1(mut self, reserved1: [u16; 3]) -> Self {
            self.reserved1 = reserved1;
            self
        }

        pub fn color_config(mut self, color_config: &'a StdVideoVP9ColorConfig) -> Self {
            self.p_color_config = color_config;
            self
        }

        pub fn loop_filter(mut self, loop_filter: &'a StdVideoVP9LoopFilter) -> Self {
            self.p_loop_filter = loop_filter;
            self
        }

        pub fn segmentation(mut self, segmentation: &'a StdVideoVP9Segmentation) -> Self {
            self.p_segmentation = segmentation;
            self
        }
    }
}
