#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct StdVideoEncodeAV1ExtensionHeader {
        pub temporal_id: u8,
        pub spatial_id: u8,
    }
    impl StdVideoEncodeAV1ExtensionHeader {
        pub fn temporal_id(mut self, temporal_id: u8) -> Self {
            self.temporal_id = temporal_id;
            self
        }
        pub fn spatial_id(mut self, spatial_id: u8) -> Self {
            self.spatial_id = spatial_id;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct StdVideoEncodeAV1DecoderModelInfo {
        pub buffer_delay_length_minus_1: u8,
        pub buffer_removal_time_length_minus_1: u8,
        pub frame_presentation_time_length_minus_1: u8,
        pub reserved1: u8,
        pub num_units_in_decoding_tick: u32,
    }
    impl StdVideoEncodeAV1DecoderModelInfo {
        pub fn buffer_delay_length_minus_1(mut self, buffer_delay_length_minus_1: u8) -> Self {
            self.buffer_delay_length_minus_1 = buffer_delay_length_minus_1;
            self
        }
        pub fn buffer_removal_time_length_minus_1(
            mut self,
            buffer_removal_time_length_minus_1: u8,
        ) -> Self {
            self.buffer_removal_time_length_minus_1 = buffer_removal_time_length_minus_1;
            self
        }
        pub fn frame_presentation_time_length_minus_1(
            mut self,
            frame_presentation_time_length_minus_1: u8,
        ) -> Self {
            self.frame_presentation_time_length_minus_1 = frame_presentation_time_length_minus_1;
            self
        }
        pub fn reserved1(mut self, reserved1: u8) -> Self {
            self.reserved1 = reserved1;
            self
        }
        pub fn num_units_in_decoding_tick(mut self, num_units_in_decoding_tick: u32) -> Self {
            self.num_units_in_decoding_tick = num_units_in_decoding_tick;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct StdVideoEncodeAV1OperatingPointInfoFlags {
        pub decoder_model_present_for_this_op: u32,
        pub low_delay_mode_flag: u32,
        pub initial_display_delay_present_for_this_op: u32,
        pub reserved: u32,
    }
    impl StdVideoEncodeAV1OperatingPointInfoFlags {
        pub fn decoder_model_present_for_this_op(
            mut self,
            decoder_model_present_for_this_op: u32,
        ) -> Self {
            self.decoder_model_present_for_this_op = decoder_model_present_for_this_op;
            self
        }
        pub fn low_delay_mode_flag(mut self, low_delay_mode_flag: u32) -> Self {
            self.low_delay_mode_flag = low_delay_mode_flag;
            self
        }
        pub fn initial_display_delay_present_for_this_op(
            mut self,
            initial_display_delay_present_for_this_op: u32,
        ) -> Self {
            self.initial_display_delay_present_for_this_op =
                initial_display_delay_present_for_this_op;
            self
        }
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
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
        pub fn flags(mut self, flags: StdVideoEncodeAV1OperatingPointInfoFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn operating_point_idc(mut self, operating_point_idc: u16) -> Self {
            self.operating_point_idc = operating_point_idc;
            self
        }
        pub fn seq_level_idx(mut self, seq_level_idx: u8) -> Self {
            self.seq_level_idx = seq_level_idx;
            self
        }
        pub fn seq_tier(mut self, seq_tier: u8) -> Self {
            self.seq_tier = seq_tier;
            self
        }
        pub fn decoder_buffer_delay(mut self, decoder_buffer_delay: u32) -> Self {
            self.decoder_buffer_delay = decoder_buffer_delay;
            self
        }
        pub fn encoder_buffer_delay(mut self, encoder_buffer_delay: u32) -> Self {
            self.encoder_buffer_delay = encoder_buffer_delay;
            self
        }
        pub fn initial_display_delay_minus_1(mut self, initial_display_delay_minus_1: u8) -> Self {
            self.initial_display_delay_minus_1 = initial_display_delay_minus_1;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct StdVideoEncodeAV1PictureInfoFlags {
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
        pub show_frame: u32,
        pub showable_frame: u32,
        pub reserved: u32,
    }
    impl StdVideoEncodeAV1PictureInfoFlags {
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
        pub fn show_frame(mut self, show_frame: u32) -> Self {
            self.show_frame = show_frame;
            self
        }
        pub fn showable_frame(mut self, showable_frame: u32) -> Self {
            self.showable_frame = showable_frame;
            self
        }
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
                p_tile_info: core::ptr::null(),
                p_quantization: core::ptr::null(),
                p_segmentation: core::ptr::null(),
                p_loop_filter: core::ptr::null(),
                p_cdef: core::ptr::null(),
                p_loop_restoration: core::ptr::null(),
                p_global_motion: core::ptr::null(),
                p_extension_header: core::ptr::null(),
                p_buffer_removal_times: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> StdVideoEncodeAV1PictureInfo<'a> {
        pub fn flags(mut self, flags: StdVideoEncodeAV1PictureInfoFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn frame_type(mut self, frame_type: StdVideoAV1FrameType) -> Self {
            self.frame_type = frame_type;
            self
        }
        pub fn frame_presentation_time(mut self, frame_presentation_time: u32) -> Self {
            self.frame_presentation_time = frame_presentation_time;
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
        pub fn coded_denom(mut self, coded_denom: u8) -> Self {
            self.coded_denom = coded_denom;
            self
        }
        pub fn render_width_minus_1(mut self, render_width_minus_1: u16) -> Self {
            self.render_width_minus_1 = render_width_minus_1;
            self
        }
        pub fn render_height_minus_1(mut self, render_height_minus_1: u16) -> Self {
            self.render_height_minus_1 = render_height_minus_1;
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
        pub fn ref_order_hint(
            mut self,
            ref_order_hint: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        ) -> Self {
            self.ref_order_hint = ref_order_hint;
            self
        }
        pub fn ref_frame_idx(
            mut self,
            ref_frame_idx: [i8; STD_VIDEO_AV1_REFS_PER_FRAME as usize],
        ) -> Self {
            self.ref_frame_idx = ref_frame_idx;
            self
        }
        pub fn reserved1(mut self, reserved1: [u8; 3]) -> Self {
            self.reserved1 = reserved1;
            self
        }
        pub fn delta_frame_id_minus_1(
            mut self,
            delta_frame_id_minus_1: [u32; STD_VIDEO_AV1_REFS_PER_FRAME as usize],
        ) -> Self {
            self.delta_frame_id_minus_1 = delta_frame_id_minus_1;
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
        pub fn extension_header(
            mut self,
            extension_header: &'a StdVideoEncodeAV1ExtensionHeader,
        ) -> Self {
            self.p_extension_header = extension_header;
            self
        }
        pub fn buffer_removal_times(mut self, buffer_removal_times: &'a u32) -> Self {
            self.p_buffer_removal_times = buffer_removal_times;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct StdVideoEncodeAV1ReferenceInfoFlags {
        pub disable_frame_end_update_cdf: u32,
        pub segmentation_enabled: u32,
        pub reserved: u32,
    }
    impl StdVideoEncodeAV1ReferenceInfoFlags {
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct StdVideoEncodeAV1ReferenceInfo<'a> {
        pub flags: StdVideoEncodeAV1ReferenceInfoFlags,
        pub ref_frame_id: u32,
        pub frame_type: StdVideoAV1FrameType,
        pub order_hint: u8,
        pub reserved1: [u8; 3],
        pub p_extension_header: *const StdVideoEncodeAV1ExtensionHeader,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for StdVideoEncodeAV1ReferenceInfo<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                ref_frame_id: Default::default(),
                frame_type: Default::default(),
                order_hint: Default::default(),
                reserved1: [Default::default(); _],
                p_extension_header: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> StdVideoEncodeAV1ReferenceInfo<'a> {
        pub fn flags(mut self, flags: StdVideoEncodeAV1ReferenceInfoFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn ref_frame_id(mut self, ref_frame_id: u32) -> Self {
            self.ref_frame_id = ref_frame_id;
            self
        }
        pub fn frame_type(mut self, frame_type: StdVideoAV1FrameType) -> Self {
            self.frame_type = frame_type;
            self
        }
        pub fn order_hint(mut self, order_hint: u8) -> Self {
            self.order_hint = order_hint;
            self
        }
        pub fn reserved1(mut self, reserved1: [u8; 3]) -> Self {
            self.reserved1 = reserved1;
            self
        }
        pub fn extension_header(
            mut self,
            extension_header: &'a StdVideoEncodeAV1ExtensionHeader,
        ) -> Self {
            self.p_extension_header = extension_header;
            self
        }
    }
}
