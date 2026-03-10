//! <https://registry.khronos.org/vulkan/specs/latest/man/html/vulkan_video_codec_av1std_encode.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"vulkan_video_codec_av1std_encode";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeAV1ExtensionHeader.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeAV1ExtensionHeader {
        pub temporal_id: u8,
        pub spatial_id: u8,
    }

    impl StdVideoEncodeAV1ExtensionHeader {
        #[inline]
        pub fn temporal_id(mut self, temporal_id: u8) -> Self {
            self.temporal_id = temporal_id;
            self
        }

        #[inline]
        pub fn spatial_id(mut self, spatial_id: u8) -> Self {
            self.spatial_id = spatial_id;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeAV1DecoderModelInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeAV1DecoderModelInfo {
        pub buffer_delay_length_minus_1: u8,
        pub buffer_removal_time_length_minus_1: u8,
        pub frame_presentation_time_length_minus_1: u8,
        pub reserved1: u8,
        pub num_units_in_decoding_tick: u32,
    }

    impl StdVideoEncodeAV1DecoderModelInfo {
        #[inline]
        pub fn buffer_delay_length_minus_1(mut self, buffer_delay_length_minus_1: u8) -> Self {
            self.buffer_delay_length_minus_1 = buffer_delay_length_minus_1;
            self
        }

        #[inline]
        pub fn buffer_removal_time_length_minus_1(
            mut self,
            buffer_removal_time_length_minus_1: u8,
        ) -> Self {
            self.buffer_removal_time_length_minus_1 = buffer_removal_time_length_minus_1;
            self
        }

        #[inline]
        pub fn frame_presentation_time_length_minus_1(
            mut self,
            frame_presentation_time_length_minus_1: u8,
        ) -> Self {
            self.frame_presentation_time_length_minus_1 = frame_presentation_time_length_minus_1;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: u8) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn num_units_in_decoding_tick(mut self, num_units_in_decoding_tick: u32) -> Self {
            self.num_units_in_decoding_tick = num_units_in_decoding_tick;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeAV1OperatingPointInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeAV1OperatingPointInfoFlags {
        pub _bitfield_0: u32,
    }

    impl StdVideoEncodeAV1OperatingPointInfoFlags {
        #[inline]
        pub fn decoder_model_present_for_this_op(
            mut self,
            decoder_model_present_for_this_op: bool,
        ) -> Self {
            set_bitfield_bool::<0>(&mut self._bitfield_0, decoder_model_present_for_this_op);
            self
        }

        #[inline]
        pub fn low_delay_mode_flag(mut self, low_delay_mode_flag: bool) -> Self {
            set_bitfield_bool::<1>(&mut self._bitfield_0, low_delay_mode_flag);
            self
        }

        #[inline]
        pub fn initial_display_delay_present_for_this_op(
            mut self,
            initial_display_delay_present_for_this_op: bool,
        ) -> Self {
            set_bitfield_bool::<2>(
                &mut self._bitfield_0,
                initial_display_delay_present_for_this_op,
            );
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeAV1OperatingPointInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeAV1OperatingPointInfo {
        pub flags: StdVideoEncodeAV1OperatingPointInfoFlags,
        pub operating_point_idc: u16,
        pub seq_level_idx: u8,
        pub seq_tier: u8,
        pub decoder_buffer_delay: u32,
        pub encoder_buffer_delay: u32,
        pub initial_display_delay_minus_1: u8,
    }

    impl StdVideoEncodeAV1OperatingPointInfo {
        #[inline]
        pub fn flags(mut self, flags: StdVideoEncodeAV1OperatingPointInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn operating_point_idc(mut self, operating_point_idc: u16) -> Self {
            self.operating_point_idc = operating_point_idc;
            self
        }

        #[inline]
        pub fn seq_level_idx(mut self, seq_level_idx: u8) -> Self {
            self.seq_level_idx = seq_level_idx;
            self
        }

        #[inline]
        pub fn seq_tier(mut self, seq_tier: u8) -> Self {
            self.seq_tier = seq_tier;
            self
        }

        #[inline]
        pub fn decoder_buffer_delay(mut self, decoder_buffer_delay: u32) -> Self {
            self.decoder_buffer_delay = decoder_buffer_delay;
            self
        }

        #[inline]
        pub fn encoder_buffer_delay(mut self, encoder_buffer_delay: u32) -> Self {
            self.encoder_buffer_delay = encoder_buffer_delay;
            self
        }

        #[inline]
        pub fn initial_display_delay_minus_1(mut self, initial_display_delay_minus_1: u8) -> Self {
            self.initial_display_delay_minus_1 = initial_display_delay_minus_1;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeAV1PictureInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeAV1PictureInfoFlags {
        pub _bitfield_0: u32,
    }

    impl StdVideoEncodeAV1PictureInfoFlags {
        #[inline]
        pub fn error_resilient_mode(mut self, error_resilient_mode: bool) -> Self {
            set_bitfield_bool::<0>(&mut self._bitfield_0, error_resilient_mode);
            self
        }

        #[inline]
        pub fn disable_cdf_update(mut self, disable_cdf_update: bool) -> Self {
            set_bitfield_bool::<1>(&mut self._bitfield_0, disable_cdf_update);
            self
        }

        #[inline]
        pub fn use_superres(mut self, use_superres: bool) -> Self {
            set_bitfield_bool::<2>(&mut self._bitfield_0, use_superres);
            self
        }

        #[inline]
        pub fn render_and_frame_size_different(
            mut self,
            render_and_frame_size_different: bool,
        ) -> Self {
            set_bitfield_bool::<3>(&mut self._bitfield_0, render_and_frame_size_different);
            self
        }

        #[inline]
        pub fn allow_screen_content_tools(mut self, allow_screen_content_tools: bool) -> Self {
            set_bitfield_bool::<4>(&mut self._bitfield_0, allow_screen_content_tools);
            self
        }

        #[inline]
        pub fn is_filter_switchable(mut self, is_filter_switchable: bool) -> Self {
            set_bitfield_bool::<5>(&mut self._bitfield_0, is_filter_switchable);
            self
        }

        #[inline]
        pub fn force_integer_mv(mut self, force_integer_mv: bool) -> Self {
            set_bitfield_bool::<6>(&mut self._bitfield_0, force_integer_mv);
            self
        }

        #[inline]
        pub fn frame_size_override_flag(mut self, frame_size_override_flag: bool) -> Self {
            set_bitfield_bool::<7>(&mut self._bitfield_0, frame_size_override_flag);
            self
        }

        #[inline]
        pub fn buffer_removal_time_present_flag(
            mut self,
            buffer_removal_time_present_flag: bool,
        ) -> Self {
            set_bitfield_bool::<8>(&mut self._bitfield_0, buffer_removal_time_present_flag);
            self
        }

        #[inline]
        pub fn allow_intrabc(mut self, allow_intrabc: bool) -> Self {
            set_bitfield_bool::<9>(&mut self._bitfield_0, allow_intrabc);
            self
        }

        #[inline]
        pub fn frame_refs_short_signaling(mut self, frame_refs_short_signaling: bool) -> Self {
            set_bitfield_bool::<10>(&mut self._bitfield_0, frame_refs_short_signaling);
            self
        }

        #[inline]
        pub fn allow_high_precision_mv(mut self, allow_high_precision_mv: bool) -> Self {
            set_bitfield_bool::<11>(&mut self._bitfield_0, allow_high_precision_mv);
            self
        }

        #[inline]
        pub fn is_motion_mode_switchable(mut self, is_motion_mode_switchable: bool) -> Self {
            set_bitfield_bool::<12>(&mut self._bitfield_0, is_motion_mode_switchable);
            self
        }

        #[inline]
        pub fn use_ref_frame_mvs(mut self, use_ref_frame_mvs: bool) -> Self {
            set_bitfield_bool::<13>(&mut self._bitfield_0, use_ref_frame_mvs);
            self
        }

        #[inline]
        pub fn disable_frame_end_update_cdf(mut self, disable_frame_end_update_cdf: bool) -> Self {
            set_bitfield_bool::<14>(&mut self._bitfield_0, disable_frame_end_update_cdf);
            self
        }

        #[inline]
        pub fn allow_warped_motion(mut self, allow_warped_motion: bool) -> Self {
            set_bitfield_bool::<15>(&mut self._bitfield_0, allow_warped_motion);
            self
        }

        #[inline]
        pub fn reduced_tx_set(mut self, reduced_tx_set: bool) -> Self {
            set_bitfield_bool::<16>(&mut self._bitfield_0, reduced_tx_set);
            self
        }

        #[inline]
        pub fn skip_mode_present(mut self, skip_mode_present: bool) -> Self {
            set_bitfield_bool::<17>(&mut self._bitfield_0, skip_mode_present);
            self
        }

        #[inline]
        pub fn delta_q_present(mut self, delta_q_present: bool) -> Self {
            set_bitfield_bool::<18>(&mut self._bitfield_0, delta_q_present);
            self
        }

        #[inline]
        pub fn delta_lf_present(mut self, delta_lf_present: bool) -> Self {
            set_bitfield_bool::<19>(&mut self._bitfield_0, delta_lf_present);
            self
        }

        #[inline]
        pub fn delta_lf_multi(mut self, delta_lf_multi: bool) -> Self {
            set_bitfield_bool::<20>(&mut self._bitfield_0, delta_lf_multi);
            self
        }

        #[inline]
        pub fn segmentation_enabled(mut self, segmentation_enabled: bool) -> Self {
            set_bitfield_bool::<21>(&mut self._bitfield_0, segmentation_enabled);
            self
        }

        #[inline]
        pub fn segmentation_update_map(mut self, segmentation_update_map: bool) -> Self {
            set_bitfield_bool::<22>(&mut self._bitfield_0, segmentation_update_map);
            self
        }

        #[inline]
        pub fn segmentation_temporal_update(mut self, segmentation_temporal_update: bool) -> Self {
            set_bitfield_bool::<23>(&mut self._bitfield_0, segmentation_temporal_update);
            self
        }

        #[inline]
        pub fn segmentation_update_data(mut self, segmentation_update_data: bool) -> Self {
            set_bitfield_bool::<24>(&mut self._bitfield_0, segmentation_update_data);
            self
        }

        #[inline]
        pub fn uses_lr(mut self, uses_lr: bool) -> Self {
            set_bitfield_bool::<25>(&mut self._bitfield_0, uses_lr);
            self
        }

        #[inline]
        pub fn uses_chroma_lr(mut self, uses_chroma_lr: bool) -> Self {
            set_bitfield_bool::<26>(&mut self._bitfield_0, uses_chroma_lr);
            self
        }

        #[inline]
        pub fn show_frame(mut self, show_frame: bool) -> Self {
            set_bitfield_bool::<27>(&mut self._bitfield_0, show_frame);
            self
        }

        #[inline]
        pub fn showable_frame(mut self, showable_frame: bool) -> Self {
            set_bitfield_bool::<28>(&mut self._bitfield_0, showable_frame);
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeAV1PictureInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoEncodeAV1PictureInfo<'a> {
        pub flags: StdVideoEncodeAV1PictureInfoFlags,
        pub frame_type: StdVideoAV1FrameType,
        pub frame_presentation_time: u32,
        pub current_frame_id: u32,
        pub order_hint: u8,
        pub primary_ref_frame: u8,
        pub refresh_frame_flags: u8,
        pub coded_denom: u8,
        pub render_width_minus_1: u16,
        pub render_height_minus_1: u16,
        pub interpolation_filter: StdVideoAV1InterpolationFilter,
        pub tx_mode: StdVideoAV1TxMode,
        pub delta_q_res: u8,
        pub delta_lf_res: u8,
        pub ref_order_hint: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        pub ref_frame_idx: [i8; STD_VIDEO_AV1_REFS_PER_FRAME as usize],
        pub reserved1: [u8; 3],
        pub delta_frame_id_minus_1: [u32; STD_VIDEO_AV1_REFS_PER_FRAME as usize],
        pub p_tile_info: *const StdVideoAV1TileInfo<'a>,
        pub p_quantization: *const StdVideoAV1Quantization,
        pub p_segmentation: *const StdVideoAV1Segmentation,
        pub p_loop_filter: *const StdVideoAV1LoopFilter,
        pub p_cdef: *const StdVideoAV1CDEF,
        pub p_loop_restoration: *const StdVideoAV1LoopRestoration,
        pub p_global_motion: *const StdVideoAV1GlobalMotion,
        pub p_extension_header: *const StdVideoEncodeAV1ExtensionHeader,
        pub p_buffer_removal_times: *const u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoEncodeAV1PictureInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoEncodeAV1PictureInfo")
                .field("flags", &self.flags)
                .field("frame_type", &self.frame_type)
                .field("frame_presentation_time", &self.frame_presentation_time)
                .field("current_frame_id", &self.current_frame_id)
                .field("order_hint", &self.order_hint)
                .field("primary_ref_frame", &self.primary_ref_frame)
                .field("refresh_frame_flags", &self.refresh_frame_flags)
                .field("coded_denom", &self.coded_denom)
                .field("render_width_minus_1", &self.render_width_minus_1)
                .field("render_height_minus_1", &self.render_height_minus_1)
                .field("interpolation_filter", &self.interpolation_filter)
                .field("tx_mode", &self.tx_mode)
                .field("delta_q_res", &self.delta_q_res)
                .field("delta_lf_res", &self.delta_lf_res)
                .field("ref_order_hint", &self.ref_order_hint)
                .field("ref_frame_idx", &self.ref_frame_idx)
                .field("reserved1", &self.reserved1)
                .field("delta_frame_id_minus_1", &self.delta_frame_id_minus_1)
                .field("p_tile_info", &self.p_tile_info)
                .field("p_quantization", &self.p_quantization)
                .field("p_segmentation", &self.p_segmentation)
                .field("p_loop_filter", &self.p_loop_filter)
                .field("p_cdef", &self.p_cdef)
                .field("p_loop_restoration", &self.p_loop_restoration)
                .field("p_global_motion", &self.p_global_motion)
                .field("p_extension_header", &self.p_extension_header)
                .field("p_buffer_removal_times", &self.p_buffer_removal_times)
                .finish()
        }
    }

    impl Default for StdVideoEncodeAV1PictureInfo<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                frame_type: Default::default(),
                frame_presentation_time: Default::default(),
                current_frame_id: Default::default(),
                order_hint: Default::default(),
                primary_ref_frame: Default::default(),
                refresh_frame_flags: Default::default(),
                coded_denom: Default::default(),
                render_width_minus_1: Default::default(),
                render_height_minus_1: Default::default(),
                interpolation_filter: Default::default(),
                tx_mode: Default::default(),
                delta_q_res: Default::default(),
                delta_lf_res: Default::default(),
                ref_order_hint: [Default::default(); _],
                ref_frame_idx: [Default::default(); _],
                reserved1: [Default::default(); _],
                delta_frame_id_minus_1: [Default::default(); _],
                p_tile_info: ptr::null(),
                p_quantization: ptr::null(),
                p_segmentation: ptr::null(),
                p_loop_filter: ptr::null(),
                p_cdef: ptr::null(),
                p_loop_restoration: ptr::null(),
                p_global_motion: ptr::null(),
                p_extension_header: ptr::null(),
                p_buffer_removal_times: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoEncodeAV1PictureInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoEncodeAV1PictureInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn frame_type(mut self, frame_type: StdVideoAV1FrameType) -> Self {
            self.frame_type = frame_type;
            self
        }

        #[inline]
        pub fn frame_presentation_time(mut self, frame_presentation_time: u32) -> Self {
            self.frame_presentation_time = frame_presentation_time;
            self
        }

        #[inline]
        pub fn current_frame_id(mut self, current_frame_id: u32) -> Self {
            self.current_frame_id = current_frame_id;
            self
        }

        #[inline]
        pub fn order_hint(mut self, order_hint: u8) -> Self {
            self.order_hint = order_hint;
            self
        }

        #[inline]
        pub fn primary_ref_frame(mut self, primary_ref_frame: u8) -> Self {
            self.primary_ref_frame = primary_ref_frame;
            self
        }

        #[inline]
        pub fn refresh_frame_flags(mut self, refresh_frame_flags: u8) -> Self {
            self.refresh_frame_flags = refresh_frame_flags;
            self
        }

        #[inline]
        pub fn coded_denom(mut self, coded_denom: u8) -> Self {
            self.coded_denom = coded_denom;
            self
        }

        #[inline]
        pub fn render_width_minus_1(mut self, render_width_minus_1: u16) -> Self {
            self.render_width_minus_1 = render_width_minus_1;
            self
        }

        #[inline]
        pub fn render_height_minus_1(mut self, render_height_minus_1: u16) -> Self {
            self.render_height_minus_1 = render_height_minus_1;
            self
        }

        #[inline]
        pub fn interpolation_filter(
            mut self,
            interpolation_filter: StdVideoAV1InterpolationFilter,
        ) -> Self {
            self.interpolation_filter = interpolation_filter;
            self
        }

        #[inline]
        pub fn tx_mode(mut self, tx_mode: StdVideoAV1TxMode) -> Self {
            self.tx_mode = tx_mode;
            self
        }

        #[inline]
        pub fn delta_q_res(mut self, delta_q_res: u8) -> Self {
            self.delta_q_res = delta_q_res;
            self
        }

        #[inline]
        pub fn delta_lf_res(mut self, delta_lf_res: u8) -> Self {
            self.delta_lf_res = delta_lf_res;
            self
        }

        #[inline]
        pub fn ref_order_hint(
            mut self,
            ref_order_hint: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        ) -> Self {
            self.ref_order_hint = ref_order_hint;
            self
        }

        #[inline]
        pub fn ref_frame_idx(
            mut self,
            ref_frame_idx: [i8; STD_VIDEO_AV1_REFS_PER_FRAME as usize],
        ) -> Self {
            self.ref_frame_idx = ref_frame_idx;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: [u8; 3]) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn delta_frame_id_minus_1(
            mut self,
            delta_frame_id_minus_1: [u32; STD_VIDEO_AV1_REFS_PER_FRAME as usize],
        ) -> Self {
            self.delta_frame_id_minus_1 = delta_frame_id_minus_1;
            self
        }

        #[inline]
        pub fn tile_info(mut self, tile_info: &'a StdVideoAV1TileInfo<'a>) -> Self {
            self.p_tile_info = tile_info;
            self
        }

        #[inline]
        pub fn quantization(mut self, quantization: &'a StdVideoAV1Quantization) -> Self {
            self.p_quantization = quantization;
            self
        }

        #[inline]
        pub fn segmentation(mut self, segmentation: &'a StdVideoAV1Segmentation) -> Self {
            self.p_segmentation = segmentation;
            self
        }

        #[inline]
        pub fn loop_filter(mut self, loop_filter: &'a StdVideoAV1LoopFilter) -> Self {
            self.p_loop_filter = loop_filter;
            self
        }

        #[inline]
        pub fn cdef(mut self, cdef: &'a StdVideoAV1CDEF) -> Self {
            self.p_cdef = cdef;
            self
        }

        #[inline]
        pub fn loop_restoration(
            mut self,
            loop_restoration: &'a StdVideoAV1LoopRestoration,
        ) -> Self {
            self.p_loop_restoration = loop_restoration;
            self
        }

        #[inline]
        pub fn global_motion(mut self, global_motion: &'a StdVideoAV1GlobalMotion) -> Self {
            self.p_global_motion = global_motion;
            self
        }

        #[inline]
        pub fn extension_header(
            mut self,
            extension_header: &'a StdVideoEncodeAV1ExtensionHeader,
        ) -> Self {
            self.p_extension_header = extension_header;
            self
        }

        #[inline]
        pub fn buffer_removal_times(mut self, buffer_removal_times: &'a u32) -> Self {
            self.p_buffer_removal_times = buffer_removal_times;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeAV1ReferenceInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoEncodeAV1ReferenceInfoFlags {
        pub _bitfield_0: u32,
    }

    impl StdVideoEncodeAV1ReferenceInfoFlags {
        #[inline]
        pub fn disable_frame_end_update_cdf(mut self, disable_frame_end_update_cdf: bool) -> Self {
            set_bitfield_bool::<0>(&mut self._bitfield_0, disable_frame_end_update_cdf);
            self
        }

        #[inline]
        pub fn segmentation_enabled(mut self, segmentation_enabled: bool) -> Self {
            set_bitfield_bool::<1>(&mut self._bitfield_0, segmentation_enabled);
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoEncodeAV1ReferenceInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoEncodeAV1ReferenceInfo<'a> {
        pub flags: StdVideoEncodeAV1ReferenceInfoFlags,
        pub ref_frame_id: u32,
        pub frame_type: StdVideoAV1FrameType,
        pub order_hint: u8,
        pub reserved1: [u8; 3],
        pub p_extension_header: *const StdVideoEncodeAV1ExtensionHeader,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoEncodeAV1ReferenceInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoEncodeAV1ReferenceInfo")
                .field("flags", &self.flags)
                .field("ref_frame_id", &self.ref_frame_id)
                .field("frame_type", &self.frame_type)
                .field("order_hint", &self.order_hint)
                .field("reserved1", &self.reserved1)
                .field("p_extension_header", &self.p_extension_header)
                .finish()
        }
    }

    impl Default for StdVideoEncodeAV1ReferenceInfo<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                ref_frame_id: Default::default(),
                frame_type: Default::default(),
                order_hint: Default::default(),
                reserved1: [Default::default(); _],
                p_extension_header: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoEncodeAV1ReferenceInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoEncodeAV1ReferenceInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn ref_frame_id(mut self, ref_frame_id: u32) -> Self {
            self.ref_frame_id = ref_frame_id;
            self
        }

        #[inline]
        pub fn frame_type(mut self, frame_type: StdVideoAV1FrameType) -> Self {
            self.frame_type = frame_type;
            self
        }

        #[inline]
        pub fn order_hint(mut self, order_hint: u8) -> Self {
            self.order_hint = order_hint;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: [u8; 3]) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn extension_header(
            mut self,
            extension_header: &'a StdVideoEncodeAV1ExtensionHeader,
        ) -> Self {
            self.p_extension_header = extension_header;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type StdVideoEncodeAV1ExtensionHeader = super::defs::StdVideoEncodeAV1ExtensionHeader;
    pub type StdVideoEncodeAV1DecoderModelInfo = super::defs::StdVideoEncodeAV1DecoderModelInfo;
    pub type StdVideoEncodeAV1OperatingPointInfoFlags =
        super::defs::StdVideoEncodeAV1OperatingPointInfoFlags;
    pub type StdVideoEncodeAV1OperatingPointInfo = super::defs::StdVideoEncodeAV1OperatingPointInfo;
    pub type StdVideoEncodeAV1PictureInfoFlags = super::defs::StdVideoEncodeAV1PictureInfoFlags;
    pub type StdVideoEncodeAV1PictureInfo = super::defs::StdVideoEncodeAV1PictureInfo<'static>;
    pub type StdVideoEncodeAV1ReferenceInfoFlags = super::defs::StdVideoEncodeAV1ReferenceInfoFlags;
    pub type StdVideoEncodeAV1ReferenceInfo = super::defs::StdVideoEncodeAV1ReferenceInfo<'static>;
    impl super::defs::StdVideoEncodeAV1PictureInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &StdVideoEncodeAV1PictureInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl super::defs::StdVideoEncodeAV1ReferenceInfo<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &StdVideoEncodeAV1ReferenceInfo {
            unsafe { core::mem::transmute(self) }
        }
    }
}
