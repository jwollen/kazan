#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeAV1ExtensionHeader {
    pub temporal_id: u8,
    pub spatial_id: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeAV1DecoderModelInfo {
    pub buffer_delay_length_minus_1: u8,
    pub buffer_removal_time_length_minus_1: u8,
    pub frame_presentation_time_length_minus_1: u8,
    pub reserved1: u8,
    pub num_units_in_decoding_tick: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeAV1OperatingPointInfoFlags {
    pub decoder_model_present_for_this_op: u32,
    pub low_delay_mode_flag: u32,
    pub initial_display_delay_present_for_this_op: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeAV1OperatingPointInfo {
    pub flags: StdVideoEncodeAV1OperatingPointInfoFlags,
    pub operating_point_idc: u16,
    pub seq_level_idx: u8,
    pub seq_tier: u8,
    pub decoder_buffer_delay: u32,
    pub encoder_buffer_delay: u32,
    pub initial_display_delay_minus_1: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeAV1PictureInfo {
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
    pub ref_order_hint: u8,
    pub ref_frame_idx: i8,
    pub reserved1: u8,
    pub delta_frame_id_minus_1: u32,
    pub p_tile_info: *const StdVideoAV1TileInfo,
    pub p_quantization: *const StdVideoAV1Quantization,
    pub p_segmentation: *const StdVideoAV1Segmentation,
    pub p_loop_filter: *const StdVideoAV1LoopFilter,
    pub p_cdef: *const StdVideoAV1CDEF,
    pub p_loop_restoration: *const StdVideoAV1LoopRestoration,
    pub p_global_motion: *const StdVideoAV1GlobalMotion,
    pub p_extension_header: *const StdVideoEncodeAV1ExtensionHeader,
    pub p_buffer_removal_times: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeAV1ReferenceInfoFlags {
    pub disable_frame_end_update_cdf: u32,
    pub segmentation_enabled: u32,
    pub reserved: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StdVideoEncodeAV1ReferenceInfo {
    pub flags: StdVideoEncodeAV1ReferenceInfoFlags,
    pub ref_frame_id: u32,
    pub frame_type: StdVideoAV1FrameType,
    pub order_hint: u8,
    pub reserved1: u8,
    pub p_extension_header: *const StdVideoEncodeAV1ExtensionHeader,
}
