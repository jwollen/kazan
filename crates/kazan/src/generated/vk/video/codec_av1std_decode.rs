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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeAV1PictureInfoFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct StdVideoDecodeAV1PictureInfoFlags {
        pub error_resilient_mode: u32,
        pub disable_cdf_update: u32,
        pub use_superres: u32,
        pub render_and_frame_size_different: u32,
        pub allow_screen_content_tools: u32,
        pub is_filter_switchable: u32,
        pub force_integer_mv: u32,
        pub frame_size_override_flag: u32,
        pub buffer_removal_time_present_flag: u32,
        pub allow_intrabc: u32,
        pub frame_refs_short_signaling: u32,
        pub allow_high_precision_mv: u32,
        pub is_motion_mode_switchable: u32,
        pub use_ref_frame_mvs: u32,
        pub disable_frame_end_update_cdf: u32,
        pub allow_warped_motion: u32,
        pub reduced_tx_set: u32,
        pub reference_select: u32,
        pub skip_mode_present: u32,
        pub delta_q_present: u32,
        pub delta_lf_present: u32,
        pub delta_lf_multi: u32,
        pub segmentation_enabled: u32,
        pub segmentation_update_map: u32,
        pub segmentation_temporal_update: u32,
        pub segmentation_update_data: u32,
        pub uses_lr: u32,
        pub uses_chroma_lr: u32,
        pub apply_grain: u32,
        pub reserved: u32,
    }

    impl StdVideoDecodeAV1PictureInfoFlags {
        pub fn error_resilient_mode(mut self, error_resilient_mode: u32) -> Self {
            self.error_resilient_mode = error_resilient_mode;
            self
        }

        pub fn disable_cdf_update(mut self, disable_cdf_update: u32) -> Self {
            self.disable_cdf_update = disable_cdf_update;
            self
        }

        pub fn use_superres(mut self, use_superres: u32) -> Self {
            self.use_superres = use_superres;
            self
        }

        pub fn render_and_frame_size_different(
            mut self,
            render_and_frame_size_different: u32,
        ) -> Self {
            self.render_and_frame_size_different = render_and_frame_size_different;
            self
        }

        pub fn allow_screen_content_tools(mut self, allow_screen_content_tools: u32) -> Self {
            self.allow_screen_content_tools = allow_screen_content_tools;
            self
        }

        pub fn is_filter_switchable(mut self, is_filter_switchable: u32) -> Self {
            self.is_filter_switchable = is_filter_switchable;
            self
        }

        pub fn force_integer_mv(mut self, force_integer_mv: u32) -> Self {
            self.force_integer_mv = force_integer_mv;
            self
        }

        pub fn frame_size_override_flag(mut self, frame_size_override_flag: u32) -> Self {
            self.frame_size_override_flag = frame_size_override_flag;
            self
        }

        pub fn buffer_removal_time_present_flag(
            mut self,
            buffer_removal_time_present_flag: u32,
        ) -> Self {
            self.buffer_removal_time_present_flag = buffer_removal_time_present_flag;
            self
        }

        pub fn allow_intrabc(mut self, allow_intrabc: u32) -> Self {
            self.allow_intrabc = allow_intrabc;
            self
        }

        pub fn frame_refs_short_signaling(mut self, frame_refs_short_signaling: u32) -> Self {
            self.frame_refs_short_signaling = frame_refs_short_signaling;
            self
        }

        pub fn allow_high_precision_mv(mut self, allow_high_precision_mv: u32) -> Self {
            self.allow_high_precision_mv = allow_high_precision_mv;
            self
        }

        pub fn is_motion_mode_switchable(mut self, is_motion_mode_switchable: u32) -> Self {
            self.is_motion_mode_switchable = is_motion_mode_switchable;
            self
        }

        pub fn use_ref_frame_mvs(mut self, use_ref_frame_mvs: u32) -> Self {
            self.use_ref_frame_mvs = use_ref_frame_mvs;
            self
        }

        pub fn disable_frame_end_update_cdf(mut self, disable_frame_end_update_cdf: u32) -> Self {
            self.disable_frame_end_update_cdf = disable_frame_end_update_cdf;
            self
        }

        pub fn allow_warped_motion(mut self, allow_warped_motion: u32) -> Self {
            self.allow_warped_motion = allow_warped_motion;
            self
        }

        pub fn reduced_tx_set(mut self, reduced_tx_set: u32) -> Self {
            self.reduced_tx_set = reduced_tx_set;
            self
        }

        pub fn reference_select(mut self, reference_select: u32) -> Self {
            self.reference_select = reference_select;
            self
        }

        pub fn skip_mode_present(mut self, skip_mode_present: u32) -> Self {
            self.skip_mode_present = skip_mode_present;
            self
        }

        pub fn delta_q_present(mut self, delta_q_present: u32) -> Self {
            self.delta_q_present = delta_q_present;
            self
        }

        pub fn delta_lf_present(mut self, delta_lf_present: u32) -> Self {
            self.delta_lf_present = delta_lf_present;
            self
        }

        pub fn delta_lf_multi(mut self, delta_lf_multi: u32) -> Self {
            self.delta_lf_multi = delta_lf_multi;
            self
        }

        pub fn segmentation_enabled(mut self, segmentation_enabled: u32) -> Self {
            self.segmentation_enabled = segmentation_enabled;
            self
        }

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

        pub fn uses_lr(mut self, uses_lr: u32) -> Self {
            self.uses_lr = uses_lr;
            self
        }

        pub fn uses_chroma_lr(mut self, uses_chroma_lr: u32) -> Self {
            self.uses_chroma_lr = uses_chroma_lr;
            self
        }

        pub fn apply_grain(mut self, apply_grain: u32) -> Self {
            self.apply_grain = apply_grain;
            self
        }

        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeAV1PictureInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct StdVideoDecodeAV1PictureInfo<'a> {
        pub flags: StdVideoDecodeAV1PictureInfoFlags,
        pub frame_type: StdVideoAV1FrameType,
        pub current_frame_id: u32,
        pub order_hint: u8,
        pub primary_ref_frame: u8,
        pub refresh_frame_flags: u8,
        pub reserved1: u8,
        pub interpolation_filter: StdVideoAV1InterpolationFilter,
        pub tx_mode: StdVideoAV1TxMode,
        pub delta_q_res: u8,
        pub delta_lf_res: u8,
        pub skip_mode_frame: [u8; STD_VIDEO_AV1_SKIP_MODE_FRAMES as usize],
        pub coded_denom: u8,
        pub reserved2: [u8; 3],
        pub order_hints: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        pub expected_frame_id: [u32; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        pub p_tile_info: *const StdVideoAV1TileInfo<'a>,
        pub p_quantization: *const StdVideoAV1Quantization,
        pub p_segmentation: *const StdVideoAV1Segmentation,
        pub p_loop_filter: *const StdVideoAV1LoopFilter,
        pub p_cdef: *const StdVideoAV1CDEF,
        pub p_loop_restoration: *const StdVideoAV1LoopRestoration,
        pub p_global_motion: *const StdVideoAV1GlobalMotion,
        pub p_film_grain: *const StdVideoAV1FilmGrain,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for StdVideoDecodeAV1PictureInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoDecodeAV1PictureInfo")
                .field("flags", &self.flags)
                .field("frame_type", &self.frame_type)
                .field("current_frame_id", &self.current_frame_id)
                .field("order_hint", &self.order_hint)
                .field("primary_ref_frame", &self.primary_ref_frame)
                .field("refresh_frame_flags", &self.refresh_frame_flags)
                .field("reserved1", &self.reserved1)
                .field("interpolation_filter", &self.interpolation_filter)
                .field("tx_mode", &self.tx_mode)
                .field("delta_q_res", &self.delta_q_res)
                .field("delta_lf_res", &self.delta_lf_res)
                .field("skip_mode_frame", &self.skip_mode_frame)
                .field("coded_denom", &self.coded_denom)
                .field("reserved2", &self.reserved2)
                .field("order_hints", &self.order_hints)
                .field("expected_frame_id", &self.expected_frame_id)
                .field("p_tile_info", &self.p_tile_info)
                .field("p_quantization", &self.p_quantization)
                .field("p_segmentation", &self.p_segmentation)
                .field("p_loop_filter", &self.p_loop_filter)
                .field("p_cdef", &self.p_cdef)
                .field("p_loop_restoration", &self.p_loop_restoration)
                .field("p_global_motion", &self.p_global_motion)
                .field("p_film_grain", &self.p_film_grain)
                .finish()
        }
    }

    impl Default for StdVideoDecodeAV1PictureInfo<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                frame_type: Default::default(),
                current_frame_id: Default::default(),
                order_hint: Default::default(),
                primary_ref_frame: Default::default(),
                refresh_frame_flags: Default::default(),
                reserved1: Default::default(),
                interpolation_filter: Default::default(),
                tx_mode: Default::default(),
                delta_q_res: Default::default(),
                delta_lf_res: Default::default(),
                skip_mode_frame: [Default::default(); _],
                coded_denom: Default::default(),
                reserved2: [Default::default(); _],
                order_hints: [Default::default(); _],
                expected_frame_id: [Default::default(); _],
                p_tile_info: core::ptr::null(),
                p_quantization: core::ptr::null(),
                p_segmentation: core::ptr::null(),
                p_loop_filter: core::ptr::null(),
                p_cdef: core::ptr::null(),
                p_loop_restoration: core::ptr::null(),
                p_global_motion: core::ptr::null(),
                p_film_grain: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoDecodeAV1PictureInfo<'a> {
        pub fn flags(mut self, flags: StdVideoDecodeAV1PictureInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        pub fn frame_type(mut self, frame_type: StdVideoAV1FrameType) -> Self {
            self.frame_type = frame_type;
            self
        }

        pub fn current_frame_id(mut self, current_frame_id: u32) -> Self {
            self.current_frame_id = current_frame_id;
            self
        }

        pub fn order_hint(mut self, order_hint: u8) -> Self {
            self.order_hint = order_hint;
            self
        }

        pub fn primary_ref_frame(mut self, primary_ref_frame: u8) -> Self {
            self.primary_ref_frame = primary_ref_frame;
            self
        }

        pub fn refresh_frame_flags(mut self, refresh_frame_flags: u8) -> Self {
            self.refresh_frame_flags = refresh_frame_flags;
            self
        }

        pub fn reserved1(mut self, reserved1: u8) -> Self {
            self.reserved1 = reserved1;
            self
        }

        pub fn interpolation_filter(
            mut self,
            interpolation_filter: StdVideoAV1InterpolationFilter,
        ) -> Self {
            self.interpolation_filter = interpolation_filter;
            self
        }

        pub fn tx_mode(mut self, tx_mode: StdVideoAV1TxMode) -> Self {
            self.tx_mode = tx_mode;
            self
        }

        pub fn delta_q_res(mut self, delta_q_res: u8) -> Self {
            self.delta_q_res = delta_q_res;
            self
        }

        pub fn delta_lf_res(mut self, delta_lf_res: u8) -> Self {
            self.delta_lf_res = delta_lf_res;
            self
        }

        pub fn skip_mode_frame(
            mut self,
            skip_mode_frame: [u8; STD_VIDEO_AV1_SKIP_MODE_FRAMES as usize],
        ) -> Self {
            self.skip_mode_frame = skip_mode_frame;
            self
        }

        pub fn coded_denom(mut self, coded_denom: u8) -> Self {
            self.coded_denom = coded_denom;
            self
        }

        pub fn reserved2(mut self, reserved2: [u8; 3]) -> Self {
            self.reserved2 = reserved2;
            self
        }

        pub fn order_hints(
            mut self,
            order_hints: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        ) -> Self {
            self.order_hints = order_hints;
            self
        }

        pub fn expected_frame_id(
            mut self,
            expected_frame_id: [u32; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        ) -> Self {
            self.expected_frame_id = expected_frame_id;
            self
        }

        pub fn tile_info(mut self, tile_info: &'a StdVideoAV1TileInfo<'a>) -> Self {
            self.p_tile_info = tile_info;
            self
        }

        pub fn quantization(mut self, quantization: &'a StdVideoAV1Quantization) -> Self {
            self.p_quantization = quantization;
            self
        }

        pub fn segmentation(mut self, segmentation: &'a StdVideoAV1Segmentation) -> Self {
            self.p_segmentation = segmentation;
            self
        }

        pub fn loop_filter(mut self, loop_filter: &'a StdVideoAV1LoopFilter) -> Self {
            self.p_loop_filter = loop_filter;
            self
        }

        pub fn cdef(mut self, cdef: &'a StdVideoAV1CDEF) -> Self {
            self.p_cdef = cdef;
            self
        }

        pub fn loop_restoration(
            mut self,
            loop_restoration: &'a StdVideoAV1LoopRestoration,
        ) -> Self {
            self.p_loop_restoration = loop_restoration;
            self
        }

        pub fn global_motion(mut self, global_motion: &'a StdVideoAV1GlobalMotion) -> Self {
            self.p_global_motion = global_motion;
            self
        }

        pub fn film_grain(mut self, film_grain: &'a StdVideoAV1FilmGrain) -> Self {
            self.p_film_grain = film_grain;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeAV1ReferenceInfoFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct StdVideoDecodeAV1ReferenceInfoFlags {
        pub disable_frame_end_update_cdf: u32,
        pub segmentation_enabled: u32,
        pub reserved: u32,
    }

    impl StdVideoDecodeAV1ReferenceInfoFlags {
        pub fn disable_frame_end_update_cdf(mut self, disable_frame_end_update_cdf: u32) -> Self {
            self.disable_frame_end_update_cdf = disable_frame_end_update_cdf;
            self
        }

        pub fn segmentation_enabled(mut self, segmentation_enabled: u32) -> Self {
            self.segmentation_enabled = segmentation_enabled;
            self
        }

        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoDecodeAV1ReferenceInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct StdVideoDecodeAV1ReferenceInfo {
        pub flags: StdVideoDecodeAV1ReferenceInfoFlags,
        pub frame_type: u8,
        pub ref_frame_sign_bias: u8,
        pub order_hint: u8,
        pub saved_order_hints: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
    }

    impl Default for StdVideoDecodeAV1ReferenceInfo {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                frame_type: Default::default(),
                ref_frame_sign_bias: Default::default(),
                order_hint: Default::default(),
                saved_order_hints: [Default::default(); _],
            }
        }
    }

    impl StdVideoDecodeAV1ReferenceInfo {
        pub fn flags(mut self, flags: StdVideoDecodeAV1ReferenceInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        pub fn frame_type(mut self, frame_type: u8) -> Self {
            self.frame_type = frame_type;
            self
        }

        pub fn ref_frame_sign_bias(mut self, ref_frame_sign_bias: u8) -> Self {
            self.ref_frame_sign_bias = ref_frame_sign_bias;
            self
        }

        pub fn order_hint(mut self, order_hint: u8) -> Self {
            self.order_hint = order_hint;
            self
        }

        pub fn saved_order_hints(
            mut self,
            saved_order_hints: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        ) -> Self {
            self.saved_order_hints = saved_order_hints;
            self
        }
    }
}
