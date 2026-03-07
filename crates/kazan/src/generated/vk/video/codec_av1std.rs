#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"vulkan_video_codec_av1std";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const STD_VIDEO_AV1_NUM_REF_FRAMES: u32 = 8;
    pub const STD_VIDEO_AV1_REFS_PER_FRAME: u32 = 7;
    pub const STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME: u32 = 8;
    pub const STD_VIDEO_AV1_MAX_TILE_COLS: u32 = 64;
    pub const STD_VIDEO_AV1_MAX_TILE_ROWS: u32 = 64;
    pub const STD_VIDEO_AV1_MAX_SEGMENTS: u32 = 8;
    pub const STD_VIDEO_AV1_SEG_LVL_MAX: u32 = 8;
    pub const STD_VIDEO_AV1_PRIMARY_REF_NONE: u8 = 7;
    pub const STD_VIDEO_AV1_SELECT_INTEGER_MV: u8 = 2;
    pub const STD_VIDEO_AV1_SELECT_SCREEN_CONTENT_TOOLS: u32 = 2;
    pub const STD_VIDEO_AV1_SKIP_MODE_FRAMES: u32 = 2;
    pub const STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS: u32 = 4;
    pub const STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS: u32 = 2;
    pub const STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS: u32 = 8;
    pub const STD_VIDEO_AV1_MAX_NUM_PLANES: u32 = 3;
    pub const STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS: u32 = 6;
    pub const STD_VIDEO_AV1_MAX_NUM_Y_POINTS: u32 = 14;
    pub const STD_VIDEO_AV1_MAX_NUM_CB_POINTS: u32 = 10;
    pub const STD_VIDEO_AV1_MAX_NUM_CR_POINTS: u32 = 10;
    pub const STD_VIDEO_AV1_MAX_NUM_POS_LUMA: u32 = 24;
    pub const STD_VIDEO_AV1_MAX_NUM_POS_CHROMA: u32 = 25;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1ColorConfigFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1ColorConfigFlags {
        pub mono_chrome: u32,
        pub color_range: u32,
        pub separate_uv_delta_q: u32,
        pub color_description_present_flag: u32,
        pub reserved: u32,
    }

    impl StdVideoAV1ColorConfigFlags {
        #[inline]
        pub fn mono_chrome(mut self, mono_chrome: u32) -> Self {
            self.mono_chrome = mono_chrome;
            self
        }

        #[inline]
        pub fn color_range(mut self, color_range: u32) -> Self {
            self.color_range = color_range;
            self
        }

        #[inline]
        pub fn separate_uv_delta_q(mut self, separate_uv_delta_q: u32) -> Self {
            self.separate_uv_delta_q = separate_uv_delta_q;
            self
        }

        #[inline]
        pub fn color_description_present_flag(
            mut self,
            color_description_present_flag: u32,
        ) -> Self {
            self.color_description_present_flag = color_description_present_flag;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1ColorConfig.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1ColorConfig {
        pub flags: StdVideoAV1ColorConfigFlags,
        pub bit_depth: u8,
        pub subsampling_x: u8,
        pub subsampling_y: u8,
        pub reserved1: u8,
        pub color_primaries: StdVideoAV1ColorPrimaries,
        pub transfer_characteristics: StdVideoAV1TransferCharacteristics,
        pub matrix_coefficients: StdVideoAV1MatrixCoefficients,
        pub chroma_sample_position: StdVideoAV1ChromaSamplePosition,
    }

    impl StdVideoAV1ColorConfig {
        #[inline]
        pub fn flags(mut self, flags: StdVideoAV1ColorConfigFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn bit_depth(mut self, bit_depth: u8) -> Self {
            self.bit_depth = bit_depth;
            self
        }

        #[inline]
        pub fn subsampling_x(mut self, subsampling_x: u8) -> Self {
            self.subsampling_x = subsampling_x;
            self
        }

        #[inline]
        pub fn subsampling_y(mut self, subsampling_y: u8) -> Self {
            self.subsampling_y = subsampling_y;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: u8) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn color_primaries(mut self, color_primaries: StdVideoAV1ColorPrimaries) -> Self {
            self.color_primaries = color_primaries;
            self
        }

        #[inline]
        pub fn transfer_characteristics(
            mut self,
            transfer_characteristics: StdVideoAV1TransferCharacteristics,
        ) -> Self {
            self.transfer_characteristics = transfer_characteristics;
            self
        }

        #[inline]
        pub fn matrix_coefficients(
            mut self,
            matrix_coefficients: StdVideoAV1MatrixCoefficients,
        ) -> Self {
            self.matrix_coefficients = matrix_coefficients;
            self
        }

        #[inline]
        pub fn chroma_sample_position(
            mut self,
            chroma_sample_position: StdVideoAV1ChromaSamplePosition,
        ) -> Self {
            self.chroma_sample_position = chroma_sample_position;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1TimingInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1TimingInfoFlags {
        pub equal_picture_interval: u32,
        pub reserved: u32,
    }

    impl StdVideoAV1TimingInfoFlags {
        #[inline]
        pub fn equal_picture_interval(mut self, equal_picture_interval: u32) -> Self {
            self.equal_picture_interval = equal_picture_interval;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1TimingInfo.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1TimingInfo {
        pub flags: StdVideoAV1TimingInfoFlags,
        pub num_units_in_display_tick: u32,
        pub time_scale: u32,
        pub num_ticks_per_picture_minus_1: u32,
    }

    impl StdVideoAV1TimingInfo {
        #[inline]
        pub fn flags(mut self, flags: StdVideoAV1TimingInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn num_units_in_display_tick(mut self, num_units_in_display_tick: u32) -> Self {
            self.num_units_in_display_tick = num_units_in_display_tick;
            self
        }

        #[inline]
        pub fn time_scale(mut self, time_scale: u32) -> Self {
            self.time_scale = time_scale;
            self
        }

        #[inline]
        pub fn num_ticks_per_picture_minus_1(mut self, num_ticks_per_picture_minus_1: u32) -> Self {
            self.num_ticks_per_picture_minus_1 = num_ticks_per_picture_minus_1;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1SequenceHeaderFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1SequenceHeaderFlags {
        pub still_picture: u32,
        pub reduced_still_picture_header: u32,
        pub use_128x128_superblock: u32,
        pub enable_filter_intra: u32,
        pub enable_intra_edge_filter: u32,
        pub enable_interintra_compound: u32,
        pub enable_masked_compound: u32,
        pub enable_warped_motion: u32,
        pub enable_dual_filter: u32,
        pub enable_order_hint: u32,
        pub enable_jnt_comp: u32,
        pub enable_ref_frame_mvs: u32,
        pub frame_id_numbers_present_flag: u32,
        pub enable_superres: u32,
        pub enable_cdef: u32,
        pub enable_restoration: u32,
        pub film_grain_params_present: u32,
        pub timing_info_present_flag: u32,
        pub initial_display_delay_present_flag: u32,
        pub reserved: u32,
    }

    impl StdVideoAV1SequenceHeaderFlags {
        #[inline]
        pub fn still_picture(mut self, still_picture: u32) -> Self {
            self.still_picture = still_picture;
            self
        }

        #[inline]
        pub fn reduced_still_picture_header(mut self, reduced_still_picture_header: u32) -> Self {
            self.reduced_still_picture_header = reduced_still_picture_header;
            self
        }

        #[inline]
        pub fn use_128x128_superblock(mut self, use_128x128_superblock: u32) -> Self {
            self.use_128x128_superblock = use_128x128_superblock;
            self
        }

        #[inline]
        pub fn enable_filter_intra(mut self, enable_filter_intra: u32) -> Self {
            self.enable_filter_intra = enable_filter_intra;
            self
        }

        #[inline]
        pub fn enable_intra_edge_filter(mut self, enable_intra_edge_filter: u32) -> Self {
            self.enable_intra_edge_filter = enable_intra_edge_filter;
            self
        }

        #[inline]
        pub fn enable_interintra_compound(mut self, enable_interintra_compound: u32) -> Self {
            self.enable_interintra_compound = enable_interintra_compound;
            self
        }

        #[inline]
        pub fn enable_masked_compound(mut self, enable_masked_compound: u32) -> Self {
            self.enable_masked_compound = enable_masked_compound;
            self
        }

        #[inline]
        pub fn enable_warped_motion(mut self, enable_warped_motion: u32) -> Self {
            self.enable_warped_motion = enable_warped_motion;
            self
        }

        #[inline]
        pub fn enable_dual_filter(mut self, enable_dual_filter: u32) -> Self {
            self.enable_dual_filter = enable_dual_filter;
            self
        }

        #[inline]
        pub fn enable_order_hint(mut self, enable_order_hint: u32) -> Self {
            self.enable_order_hint = enable_order_hint;
            self
        }

        #[inline]
        pub fn enable_jnt_comp(mut self, enable_jnt_comp: u32) -> Self {
            self.enable_jnt_comp = enable_jnt_comp;
            self
        }

        #[inline]
        pub fn enable_ref_frame_mvs(mut self, enable_ref_frame_mvs: u32) -> Self {
            self.enable_ref_frame_mvs = enable_ref_frame_mvs;
            self
        }

        #[inline]
        pub fn frame_id_numbers_present_flag(mut self, frame_id_numbers_present_flag: u32) -> Self {
            self.frame_id_numbers_present_flag = frame_id_numbers_present_flag;
            self
        }

        #[inline]
        pub fn enable_superres(mut self, enable_superres: u32) -> Self {
            self.enable_superres = enable_superres;
            self
        }

        #[inline]
        pub fn enable_cdef(mut self, enable_cdef: u32) -> Self {
            self.enable_cdef = enable_cdef;
            self
        }

        #[inline]
        pub fn enable_restoration(mut self, enable_restoration: u32) -> Self {
            self.enable_restoration = enable_restoration;
            self
        }

        #[inline]
        pub fn film_grain_params_present(mut self, film_grain_params_present: u32) -> Self {
            self.film_grain_params_present = film_grain_params_present;
            self
        }

        #[inline]
        pub fn timing_info_present_flag(mut self, timing_info_present_flag: u32) -> Self {
            self.timing_info_present_flag = timing_info_present_flag;
            self
        }

        #[inline]
        pub fn initial_display_delay_present_flag(
            mut self,
            initial_display_delay_present_flag: u32,
        ) -> Self {
            self.initial_display_delay_present_flag = initial_display_delay_present_flag;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1SequenceHeader.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoAV1SequenceHeader<'a> {
        pub flags: StdVideoAV1SequenceHeaderFlags,
        pub seq_profile: StdVideoAV1Profile,
        pub frame_width_bits_minus_1: u8,
        pub frame_height_bits_minus_1: u8,
        pub max_frame_width_minus_1: u16,
        pub max_frame_height_minus_1: u16,
        pub delta_frame_id_length_minus_2: u8,
        pub additional_frame_id_length_minus_1: u8,
        pub order_hint_bits_minus_1: u8,
        pub seq_force_integer_mv: u8,
        pub seq_force_screen_content_tools: u8,
        pub reserved1: [u8; 5],
        pub p_color_config: *const StdVideoAV1ColorConfig,
        pub p_timing_info: *const StdVideoAV1TimingInfo,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoAV1SequenceHeader<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoAV1SequenceHeader")
                .field("flags", &self.flags)
                .field("seq_profile", &self.seq_profile)
                .field("frame_width_bits_minus_1", &self.frame_width_bits_minus_1)
                .field("frame_height_bits_minus_1", &self.frame_height_bits_minus_1)
                .field("max_frame_width_minus_1", &self.max_frame_width_minus_1)
                .field("max_frame_height_minus_1", &self.max_frame_height_minus_1)
                .field(
                    "delta_frame_id_length_minus_2",
                    &self.delta_frame_id_length_minus_2,
                )
                .field(
                    "additional_frame_id_length_minus_1",
                    &self.additional_frame_id_length_minus_1,
                )
                .field("order_hint_bits_minus_1", &self.order_hint_bits_minus_1)
                .field("seq_force_integer_mv", &self.seq_force_integer_mv)
                .field(
                    "seq_force_screen_content_tools",
                    &self.seq_force_screen_content_tools,
                )
                .field("reserved1", &self.reserved1)
                .field("p_color_config", &self.p_color_config)
                .field("p_timing_info", &self.p_timing_info)
                .finish()
        }
    }

    impl Default for StdVideoAV1SequenceHeader<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                seq_profile: Default::default(),
                frame_width_bits_minus_1: Default::default(),
                frame_height_bits_minus_1: Default::default(),
                max_frame_width_minus_1: Default::default(),
                max_frame_height_minus_1: Default::default(),
                delta_frame_id_length_minus_2: Default::default(),
                additional_frame_id_length_minus_1: Default::default(),
                order_hint_bits_minus_1: Default::default(),
                seq_force_integer_mv: Default::default(),
                seq_force_screen_content_tools: Default::default(),
                reserved1: [Default::default(); _],
                p_color_config: core::ptr::null(),
                p_timing_info: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoAV1SequenceHeader<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoAV1SequenceHeaderFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn seq_profile(mut self, seq_profile: StdVideoAV1Profile) -> Self {
            self.seq_profile = seq_profile;
            self
        }

        #[inline]
        pub fn frame_width_bits_minus_1(mut self, frame_width_bits_minus_1: u8) -> Self {
            self.frame_width_bits_minus_1 = frame_width_bits_minus_1;
            self
        }

        #[inline]
        pub fn frame_height_bits_minus_1(mut self, frame_height_bits_minus_1: u8) -> Self {
            self.frame_height_bits_minus_1 = frame_height_bits_minus_1;
            self
        }

        #[inline]
        pub fn max_frame_width_minus_1(mut self, max_frame_width_minus_1: u16) -> Self {
            self.max_frame_width_minus_1 = max_frame_width_minus_1;
            self
        }

        #[inline]
        pub fn max_frame_height_minus_1(mut self, max_frame_height_minus_1: u16) -> Self {
            self.max_frame_height_minus_1 = max_frame_height_minus_1;
            self
        }

        #[inline]
        pub fn delta_frame_id_length_minus_2(mut self, delta_frame_id_length_minus_2: u8) -> Self {
            self.delta_frame_id_length_minus_2 = delta_frame_id_length_minus_2;
            self
        }

        #[inline]
        pub fn additional_frame_id_length_minus_1(
            mut self,
            additional_frame_id_length_minus_1: u8,
        ) -> Self {
            self.additional_frame_id_length_minus_1 = additional_frame_id_length_minus_1;
            self
        }

        #[inline]
        pub fn order_hint_bits_minus_1(mut self, order_hint_bits_minus_1: u8) -> Self {
            self.order_hint_bits_minus_1 = order_hint_bits_minus_1;
            self
        }

        #[inline]
        pub fn seq_force_integer_mv(mut self, seq_force_integer_mv: u8) -> Self {
            self.seq_force_integer_mv = seq_force_integer_mv;
            self
        }

        #[inline]
        pub fn seq_force_screen_content_tools(
            mut self,
            seq_force_screen_content_tools: u8,
        ) -> Self {
            self.seq_force_screen_content_tools = seq_force_screen_content_tools;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: [u8; 5]) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn color_config(mut self, color_config: &'a StdVideoAV1ColorConfig) -> Self {
            self.p_color_config = color_config;
            self
        }

        #[inline]
        pub fn timing_info(mut self, timing_info: &'a StdVideoAV1TimingInfo) -> Self {
            self.p_timing_info = timing_info;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1LoopFilterFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1LoopFilterFlags {
        pub loop_filter_delta_enabled: u32,
        pub loop_filter_delta_update: u32,
        pub reserved: u32,
    }

    impl StdVideoAV1LoopFilterFlags {
        #[inline]
        pub fn loop_filter_delta_enabled(mut self, loop_filter_delta_enabled: u32) -> Self {
            self.loop_filter_delta_enabled = loop_filter_delta_enabled;
            self
        }

        #[inline]
        pub fn loop_filter_delta_update(mut self, loop_filter_delta_update: u32) -> Self {
            self.loop_filter_delta_update = loop_filter_delta_update;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1LoopFilter.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoAV1LoopFilter {
        pub flags: StdVideoAV1LoopFilterFlags,
        pub loop_filter_level: [u8; STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS as usize],
        pub loop_filter_sharpness: u8,
        pub update_ref_delta: u8,
        pub loop_filter_ref_deltas: [i8; STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME as usize],
        pub update_mode_delta: u8,
        pub loop_filter_mode_deltas: [i8; STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS as usize],
    }

    impl Default for StdVideoAV1LoopFilter {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                loop_filter_level: [Default::default(); _],
                loop_filter_sharpness: Default::default(),
                update_ref_delta: Default::default(),
                loop_filter_ref_deltas: [Default::default(); _],
                update_mode_delta: Default::default(),
                loop_filter_mode_deltas: [Default::default(); _],
            }
        }
    }

    impl StdVideoAV1LoopFilter {
        #[inline]
        pub fn flags(mut self, flags: StdVideoAV1LoopFilterFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn loop_filter_level(
            mut self,
            loop_filter_level: [u8; STD_VIDEO_AV1_MAX_LOOP_FILTER_STRENGTHS as usize],
        ) -> Self {
            self.loop_filter_level = loop_filter_level;
            self
        }

        #[inline]
        pub fn loop_filter_sharpness(mut self, loop_filter_sharpness: u8) -> Self {
            self.loop_filter_sharpness = loop_filter_sharpness;
            self
        }

        #[inline]
        pub fn update_ref_delta(mut self, update_ref_delta: u8) -> Self {
            self.update_ref_delta = update_ref_delta;
            self
        }

        #[inline]
        pub fn loop_filter_ref_deltas(
            mut self,
            loop_filter_ref_deltas: [i8; STD_VIDEO_AV1_TOTAL_REFS_PER_FRAME as usize],
        ) -> Self {
            self.loop_filter_ref_deltas = loop_filter_ref_deltas;
            self
        }

        #[inline]
        pub fn update_mode_delta(mut self, update_mode_delta: u8) -> Self {
            self.update_mode_delta = update_mode_delta;
            self
        }

        #[inline]
        pub fn loop_filter_mode_deltas(
            mut self,
            loop_filter_mode_deltas: [i8; STD_VIDEO_AV1_LOOP_FILTER_ADJUSTMENTS as usize],
        ) -> Self {
            self.loop_filter_mode_deltas = loop_filter_mode_deltas;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1QuantizationFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1QuantizationFlags {
        pub using_qmatrix: u32,
        pub diff_uv_delta: u32,
        pub reserved: u32,
    }

    impl StdVideoAV1QuantizationFlags {
        #[inline]
        pub fn using_qmatrix(mut self, using_qmatrix: u32) -> Self {
            self.using_qmatrix = using_qmatrix;
            self
        }

        #[inline]
        pub fn diff_uv_delta(mut self, diff_uv_delta: u32) -> Self {
            self.diff_uv_delta = diff_uv_delta;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1Quantization.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1Quantization {
        pub flags: StdVideoAV1QuantizationFlags,
        pub base_q_idx: u8,
        pub delta_qy_dc: i8,
        pub delta_qu_dc: i8,
        pub delta_qu_ac: i8,
        pub delta_qv_dc: i8,
        pub delta_qv_ac: i8,
        pub qm_y: u8,
        pub qm_u: u8,
        pub qm_v: u8,
    }

    impl StdVideoAV1Quantization {
        #[inline]
        pub fn flags(mut self, flags: StdVideoAV1QuantizationFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn base_q_idx(mut self, base_q_idx: u8) -> Self {
            self.base_q_idx = base_q_idx;
            self
        }

        #[inline]
        pub fn delta_qy_dc(mut self, delta_qy_dc: i8) -> Self {
            self.delta_qy_dc = delta_qy_dc;
            self
        }

        #[inline]
        pub fn delta_qu_dc(mut self, delta_qu_dc: i8) -> Self {
            self.delta_qu_dc = delta_qu_dc;
            self
        }

        #[inline]
        pub fn delta_qu_ac(mut self, delta_qu_ac: i8) -> Self {
            self.delta_qu_ac = delta_qu_ac;
            self
        }

        #[inline]
        pub fn delta_qv_dc(mut self, delta_qv_dc: i8) -> Self {
            self.delta_qv_dc = delta_qv_dc;
            self
        }

        #[inline]
        pub fn delta_qv_ac(mut self, delta_qv_ac: i8) -> Self {
            self.delta_qv_ac = delta_qv_ac;
            self
        }

        #[inline]
        pub fn qm_y(mut self, qm_y: u8) -> Self {
            self.qm_y = qm_y;
            self
        }

        #[inline]
        pub fn qm_u(mut self, qm_u: u8) -> Self {
            self.qm_u = qm_u;
            self
        }

        #[inline]
        pub fn qm_v(mut self, qm_v: u8) -> Self {
            self.qm_v = qm_v;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1Segmentation.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoAV1Segmentation {
        pub feature_enabled: [u8; STD_VIDEO_AV1_MAX_SEGMENTS as usize],
        pub feature_data:
            [[i16; STD_VIDEO_AV1_SEG_LVL_MAX as usize]; STD_VIDEO_AV1_MAX_SEGMENTS as usize],
    }

    impl Default for StdVideoAV1Segmentation {
        fn default() -> Self {
            Self {
                feature_enabled: [Default::default(); _],
                feature_data: [[Default::default(); _]; _],
            }
        }
    }

    impl StdVideoAV1Segmentation {
        #[inline]
        pub fn feature_enabled(
            mut self,
            feature_enabled: [u8; STD_VIDEO_AV1_MAX_SEGMENTS as usize],
        ) -> Self {
            self.feature_enabled = feature_enabled;
            self
        }

        #[inline]
        pub fn feature_data(
            mut self,
            feature_data: [[i16; STD_VIDEO_AV1_SEG_LVL_MAX as usize];
                STD_VIDEO_AV1_MAX_SEGMENTS as usize],
        ) -> Self {
            self.feature_data = feature_data;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1TileInfoFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1TileInfoFlags {
        pub uniform_tile_spacing_flag: u32,
        pub reserved: u32,
    }

    impl StdVideoAV1TileInfoFlags {
        #[inline]
        pub fn uniform_tile_spacing_flag(mut self, uniform_tile_spacing_flag: u32) -> Self {
            self.uniform_tile_spacing_flag = uniform_tile_spacing_flag;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1TileInfo.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoAV1TileInfo<'a> {
        pub flags: StdVideoAV1TileInfoFlags,
        pub tile_cols: u8,
        pub tile_rows: u8,
        pub context_update_tile_id: u16,
        pub tile_size_bytes_minus_1: u8,
        pub reserved1: [u8; 7],
        pub p_mi_col_starts: *const u16,
        pub p_mi_row_starts: *const u16,
        pub p_width_in_sbs_minus1: *const u16,
        pub p_height_in_sbs_minus1: *const u16,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoAV1TileInfo<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoAV1TileInfo")
                .field("flags", &self.flags)
                .field("tile_cols", &self.tile_cols)
                .field("tile_rows", &self.tile_rows)
                .field("context_update_tile_id", &self.context_update_tile_id)
                .field("tile_size_bytes_minus_1", &self.tile_size_bytes_minus_1)
                .field("reserved1", &self.reserved1)
                .field("p_mi_col_starts", &self.p_mi_col_starts)
                .field("p_mi_row_starts", &self.p_mi_row_starts)
                .field("p_width_in_sbs_minus1", &self.p_width_in_sbs_minus1)
                .field("p_height_in_sbs_minus1", &self.p_height_in_sbs_minus1)
                .finish()
        }
    }

    impl Default for StdVideoAV1TileInfo<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                tile_cols: Default::default(),
                tile_rows: Default::default(),
                context_update_tile_id: Default::default(),
                tile_size_bytes_minus_1: Default::default(),
                reserved1: [Default::default(); _],
                p_mi_col_starts: core::ptr::null(),
                p_mi_row_starts: core::ptr::null(),
                p_width_in_sbs_minus1: core::ptr::null(),
                p_height_in_sbs_minus1: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoAV1TileInfo<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoAV1TileInfoFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn mi_col_starts(mut self, mi_col_starts: &'a [u16]) -> Self {
            self.tile_cols = mi_col_starts.len().try_into().unwrap();
            self.p_mi_col_starts = mi_col_starts.as_ptr();
            self
        }

        #[inline]
        pub fn width_in_sbs_minus1(mut self, width_in_sbs_minus1: &'a [u16]) -> Self {
            self.tile_cols = width_in_sbs_minus1.len().try_into().unwrap();
            self.p_width_in_sbs_minus1 = width_in_sbs_minus1.as_ptr();
            self
        }

        #[inline]
        pub fn mi_row_starts(mut self, mi_row_starts: &'a [u16]) -> Self {
            self.tile_rows = mi_row_starts.len().try_into().unwrap();
            self.p_mi_row_starts = mi_row_starts.as_ptr();
            self
        }

        #[inline]
        pub fn height_in_sbs_minus1(mut self, height_in_sbs_minus1: &'a [u16]) -> Self {
            self.tile_rows = height_in_sbs_minus1.len().try_into().unwrap();
            self.p_height_in_sbs_minus1 = height_in_sbs_minus1.as_ptr();
            self
        }

        #[inline]
        pub fn context_update_tile_id(mut self, context_update_tile_id: u16) -> Self {
            self.context_update_tile_id = context_update_tile_id;
            self
        }

        #[inline]
        pub fn tile_size_bytes_minus_1(mut self, tile_size_bytes_minus_1: u8) -> Self {
            self.tile_size_bytes_minus_1 = tile_size_bytes_minus_1;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: [u8; 7]) -> Self {
            self.reserved1 = reserved1;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1CDEF.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoAV1CDEF {
        pub cdef_damping_minus_3: u8,
        pub cdef_bits: u8,
        pub cdef_y_pri_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
        pub cdef_y_sec_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
        pub cdef_uv_pri_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
        pub cdef_uv_sec_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
    }

    impl Default for StdVideoAV1CDEF {
        fn default() -> Self {
            Self {
                cdef_damping_minus_3: Default::default(),
                cdef_bits: Default::default(),
                cdef_y_pri_strength: [Default::default(); _],
                cdef_y_sec_strength: [Default::default(); _],
                cdef_uv_pri_strength: [Default::default(); _],
                cdef_uv_sec_strength: [Default::default(); _],
            }
        }
    }

    impl StdVideoAV1CDEF {
        #[inline]
        pub fn cdef_damping_minus_3(mut self, cdef_damping_minus_3: u8) -> Self {
            self.cdef_damping_minus_3 = cdef_damping_minus_3;
            self
        }

        #[inline]
        pub fn cdef_bits(mut self, cdef_bits: u8) -> Self {
            self.cdef_bits = cdef_bits;
            self
        }

        #[inline]
        pub fn cdef_y_pri_strength(
            mut self,
            cdef_y_pri_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
        ) -> Self {
            self.cdef_y_pri_strength = cdef_y_pri_strength;
            self
        }

        #[inline]
        pub fn cdef_y_sec_strength(
            mut self,
            cdef_y_sec_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
        ) -> Self {
            self.cdef_y_sec_strength = cdef_y_sec_strength;
            self
        }

        #[inline]
        pub fn cdef_uv_pri_strength(
            mut self,
            cdef_uv_pri_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
        ) -> Self {
            self.cdef_uv_pri_strength = cdef_uv_pri_strength;
            self
        }

        #[inline]
        pub fn cdef_uv_sec_strength(
            mut self,
            cdef_uv_sec_strength: [u8; STD_VIDEO_AV1_MAX_CDEF_FILTER_STRENGTHS as usize],
        ) -> Self {
            self.cdef_uv_sec_strength = cdef_uv_sec_strength;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1LoopRestoration.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoAV1LoopRestoration {
        pub frame_restoration_type:
            [StdVideoAV1FrameRestorationType; STD_VIDEO_AV1_MAX_NUM_PLANES as usize],
        pub loop_restoration_size: [u16; STD_VIDEO_AV1_MAX_NUM_PLANES as usize],
    }

    impl Default for StdVideoAV1LoopRestoration {
        fn default() -> Self {
            Self {
                frame_restoration_type: [Default::default(); _],
                loop_restoration_size: [Default::default(); _],
            }
        }
    }

    impl StdVideoAV1LoopRestoration {
        #[inline]
        pub fn frame_restoration_type(
            mut self,
            frame_restoration_type: [StdVideoAV1FrameRestorationType;
                STD_VIDEO_AV1_MAX_NUM_PLANES as usize],
        ) -> Self {
            self.frame_restoration_type = frame_restoration_type;
            self
        }

        #[inline]
        pub fn loop_restoration_size(
            mut self,
            loop_restoration_size: [u16; STD_VIDEO_AV1_MAX_NUM_PLANES as usize],
        ) -> Self {
            self.loop_restoration_size = loop_restoration_size;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1GlobalMotion.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoAV1GlobalMotion {
        pub gm_type: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        pub gm_params: [[i32; STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS as usize];
            STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
    }

    impl Default for StdVideoAV1GlobalMotion {
        fn default() -> Self {
            Self {
                gm_type: [Default::default(); _],
                gm_params: [[Default::default(); _]; _],
            }
        }
    }

    impl StdVideoAV1GlobalMotion {
        #[inline]
        pub fn gm_type(mut self, gm_type: [u8; STD_VIDEO_AV1_NUM_REF_FRAMES as usize]) -> Self {
            self.gm_type = gm_type;
            self
        }

        #[inline]
        pub fn gm_params(
            mut self,
            gm_params: [[i32; STD_VIDEO_AV1_GLOBAL_MOTION_PARAMS as usize];
                STD_VIDEO_AV1_NUM_REF_FRAMES as usize],
        ) -> Self {
            self.gm_params = gm_params;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1FilmGrainFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoAV1FilmGrainFlags {
        pub chroma_scaling_from_luma: u32,
        pub overlap_flag: u32,
        pub clip_to_restricted_range: u32,
        pub update_grain: u32,
        pub reserved: u32,
    }

    impl StdVideoAV1FilmGrainFlags {
        #[inline]
        pub fn chroma_scaling_from_luma(mut self, chroma_scaling_from_luma: u32) -> Self {
            self.chroma_scaling_from_luma = chroma_scaling_from_luma;
            self
        }

        #[inline]
        pub fn overlap_flag(mut self, overlap_flag: u32) -> Self {
            self.overlap_flag = overlap_flag;
            self
        }

        #[inline]
        pub fn clip_to_restricted_range(mut self, clip_to_restricted_range: u32) -> Self {
            self.clip_to_restricted_range = clip_to_restricted_range;
            self
        }

        #[inline]
        pub fn update_grain(mut self, update_grain: u32) -> Self {
            self.update_grain = update_grain;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: u32) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1FilmGrain.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoAV1FilmGrain {
        pub flags: StdVideoAV1FilmGrainFlags,
        pub grain_scaling_minus_8: u8,
        pub ar_coeff_lag: u8,
        pub ar_coeff_shift_minus_6: u8,
        pub grain_scale_shift: u8,
        pub grain_seed: u16,
        pub film_grain_params_ref_idx: u8,
        pub num_y_points: u8,
        pub point_y_value: [u8; STD_VIDEO_AV1_MAX_NUM_Y_POINTS as usize],
        pub point_y_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_Y_POINTS as usize],
        pub num_cb_points: u8,
        pub point_cb_value: [u8; STD_VIDEO_AV1_MAX_NUM_CB_POINTS as usize],
        pub point_cb_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_CB_POINTS as usize],
        pub num_cr_points: u8,
        pub point_cr_value: [u8; STD_VIDEO_AV1_MAX_NUM_CR_POINTS as usize],
        pub point_cr_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_CR_POINTS as usize],
        pub ar_coeffs_y_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_LUMA as usize],
        pub ar_coeffs_cb_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_CHROMA as usize],
        pub ar_coeffs_cr_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_CHROMA as usize],
        pub cb_mult: u8,
        pub cb_luma_mult: u8,
        pub cb_offset: u16,
        pub cr_mult: u8,
        pub cr_luma_mult: u8,
        pub cr_offset: u16,
    }

    impl Default for StdVideoAV1FilmGrain {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                grain_scaling_minus_8: Default::default(),
                ar_coeff_lag: Default::default(),
                ar_coeff_shift_minus_6: Default::default(),
                grain_scale_shift: Default::default(),
                grain_seed: Default::default(),
                film_grain_params_ref_idx: Default::default(),
                num_y_points: Default::default(),
                point_y_value: [Default::default(); _],
                point_y_scaling: [Default::default(); _],
                num_cb_points: Default::default(),
                point_cb_value: [Default::default(); _],
                point_cb_scaling: [Default::default(); _],
                num_cr_points: Default::default(),
                point_cr_value: [Default::default(); _],
                point_cr_scaling: [Default::default(); _],
                ar_coeffs_y_plus_128: [Default::default(); _],
                ar_coeffs_cb_plus_128: [Default::default(); _],
                ar_coeffs_cr_plus_128: [Default::default(); _],
                cb_mult: Default::default(),
                cb_luma_mult: Default::default(),
                cb_offset: Default::default(),
                cr_mult: Default::default(),
                cr_luma_mult: Default::default(),
                cr_offset: Default::default(),
            }
        }
    }

    impl StdVideoAV1FilmGrain {
        #[inline]
        pub fn flags(mut self, flags: StdVideoAV1FilmGrainFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn grain_scaling_minus_8(mut self, grain_scaling_minus_8: u8) -> Self {
            self.grain_scaling_minus_8 = grain_scaling_minus_8;
            self
        }

        #[inline]
        pub fn ar_coeff_lag(mut self, ar_coeff_lag: u8) -> Self {
            self.ar_coeff_lag = ar_coeff_lag;
            self
        }

        #[inline]
        pub fn ar_coeff_shift_minus_6(mut self, ar_coeff_shift_minus_6: u8) -> Self {
            self.ar_coeff_shift_minus_6 = ar_coeff_shift_minus_6;
            self
        }

        #[inline]
        pub fn grain_scale_shift(mut self, grain_scale_shift: u8) -> Self {
            self.grain_scale_shift = grain_scale_shift;
            self
        }

        #[inline]
        pub fn grain_seed(mut self, grain_seed: u16) -> Self {
            self.grain_seed = grain_seed;
            self
        }

        #[inline]
        pub fn film_grain_params_ref_idx(mut self, film_grain_params_ref_idx: u8) -> Self {
            self.film_grain_params_ref_idx = film_grain_params_ref_idx;
            self
        }

        #[inline]
        pub fn num_y_points(mut self, num_y_points: u8) -> Self {
            self.num_y_points = num_y_points;
            self
        }

        #[inline]
        pub fn point_y_value(
            mut self,
            point_y_value: [u8; STD_VIDEO_AV1_MAX_NUM_Y_POINTS as usize],
        ) -> Self {
            self.point_y_value = point_y_value;
            self
        }

        #[inline]
        pub fn point_y_scaling(
            mut self,
            point_y_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_Y_POINTS as usize],
        ) -> Self {
            self.point_y_scaling = point_y_scaling;
            self
        }

        #[inline]
        pub fn num_cb_points(mut self, num_cb_points: u8) -> Self {
            self.num_cb_points = num_cb_points;
            self
        }

        #[inline]
        pub fn point_cb_value(
            mut self,
            point_cb_value: [u8; STD_VIDEO_AV1_MAX_NUM_CB_POINTS as usize],
        ) -> Self {
            self.point_cb_value = point_cb_value;
            self
        }

        #[inline]
        pub fn point_cb_scaling(
            mut self,
            point_cb_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_CB_POINTS as usize],
        ) -> Self {
            self.point_cb_scaling = point_cb_scaling;
            self
        }

        #[inline]
        pub fn num_cr_points(mut self, num_cr_points: u8) -> Self {
            self.num_cr_points = num_cr_points;
            self
        }

        #[inline]
        pub fn point_cr_value(
            mut self,
            point_cr_value: [u8; STD_VIDEO_AV1_MAX_NUM_CR_POINTS as usize],
        ) -> Self {
            self.point_cr_value = point_cr_value;
            self
        }

        #[inline]
        pub fn point_cr_scaling(
            mut self,
            point_cr_scaling: [u8; STD_VIDEO_AV1_MAX_NUM_CR_POINTS as usize],
        ) -> Self {
            self.point_cr_scaling = point_cr_scaling;
            self
        }

        #[inline]
        pub fn ar_coeffs_y_plus_128(
            mut self,
            ar_coeffs_y_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_LUMA as usize],
        ) -> Self {
            self.ar_coeffs_y_plus_128 = ar_coeffs_y_plus_128;
            self
        }

        #[inline]
        pub fn ar_coeffs_cb_plus_128(
            mut self,
            ar_coeffs_cb_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_CHROMA as usize],
        ) -> Self {
            self.ar_coeffs_cb_plus_128 = ar_coeffs_cb_plus_128;
            self
        }

        #[inline]
        pub fn ar_coeffs_cr_plus_128(
            mut self,
            ar_coeffs_cr_plus_128: [i8; STD_VIDEO_AV1_MAX_NUM_POS_CHROMA as usize],
        ) -> Self {
            self.ar_coeffs_cr_plus_128 = ar_coeffs_cr_plus_128;
            self
        }

        #[inline]
        pub fn cb_mult(mut self, cb_mult: u8) -> Self {
            self.cb_mult = cb_mult;
            self
        }

        #[inline]
        pub fn cb_luma_mult(mut self, cb_luma_mult: u8) -> Self {
            self.cb_luma_mult = cb_luma_mult;
            self
        }

        #[inline]
        pub fn cb_offset(mut self, cb_offset: u16) -> Self {
            self.cb_offset = cb_offset;
            self
        }

        #[inline]
        pub fn cr_mult(mut self, cr_mult: u8) -> Self {
            self.cr_mult = cr_mult;
            self
        }

        #[inline]
        pub fn cr_luma_mult(mut self, cr_luma_mult: u8) -> Self {
            self.cr_luma_mult = cr_luma_mult;
            self
        }

        #[inline]
        pub fn cr_offset(mut self, cr_offset: u16) -> Self {
            self.cr_offset = cr_offset;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1Profile.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1Profile(i32);

    impl StdVideoAV1Profile {
        pub const MAIN: Self = Self(0);
        pub const HIGH: Self = Self(1);
        pub const PROFESSIONAL: Self = Self(2);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1Profile {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MAIN => Some("MAIN"),
                Self::HIGH => Some("HIGH"),
                Self::PROFESSIONAL => Some("PROFESSIONAL"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1Level.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1Level(i32);

    impl StdVideoAV1Level {
        pub const _2_0: Self = Self(0);
        pub const _2_1: Self = Self(1);
        pub const _2_2: Self = Self(2);
        pub const _2_3: Self = Self(3);
        pub const _3_0: Self = Self(4);
        pub const _3_1: Self = Self(5);
        pub const _3_2: Self = Self(6);
        pub const _3_3: Self = Self(7);
        pub const _4_0: Self = Self(8);
        pub const _4_1: Self = Self(9);
        pub const _4_2: Self = Self(10);
        pub const _4_3: Self = Self(11);
        pub const _5_0: Self = Self(12);
        pub const _5_1: Self = Self(13);
        pub const _5_2: Self = Self(14);
        pub const _5_3: Self = Self(15);
        pub const _6_0: Self = Self(16);
        pub const _6_1: Self = Self(17);
        pub const _6_2: Self = Self(18);
        pub const _6_3: Self = Self(19);
        pub const _7_0: Self = Self(20);
        pub const _7_1: Self = Self(21);
        pub const _7_2: Self = Self(22);
        pub const _7_3: Self = Self(23);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1Level {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_2_0 => Some("_2_0"),
                Self::_2_1 => Some("_2_1"),
                Self::_2_2 => Some("_2_2"),
                Self::_2_3 => Some("_2_3"),
                Self::_3_0 => Some("_3_0"),
                Self::_3_1 => Some("_3_1"),
                Self::_3_2 => Some("_3_2"),
                Self::_3_3 => Some("_3_3"),
                Self::_4_0 => Some("_4_0"),
                Self::_4_1 => Some("_4_1"),
                Self::_4_2 => Some("_4_2"),
                Self::_4_3 => Some("_4_3"),
                Self::_5_0 => Some("_5_0"),
                Self::_5_1 => Some("_5_1"),
                Self::_5_2 => Some("_5_2"),
                Self::_5_3 => Some("_5_3"),
                Self::_6_0 => Some("_6_0"),
                Self::_6_1 => Some("_6_1"),
                Self::_6_2 => Some("_6_2"),
                Self::_6_3 => Some("_6_3"),
                Self::_7_0 => Some("_7_0"),
                Self::_7_1 => Some("_7_1"),
                Self::_7_2 => Some("_7_2"),
                Self::_7_3 => Some("_7_3"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1FrameType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1FrameType(i32);

    impl StdVideoAV1FrameType {
        pub const KEY: Self = Self(0);
        pub const INTER: Self = Self(1);
        pub const INTRA_ONLY: Self = Self(2);
        pub const SWITCH: Self = Self(3);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1FrameType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::KEY => Some("KEY"),
                Self::INTER => Some("INTER"),
                Self::INTRA_ONLY => Some("INTRA_ONLY"),
                Self::SWITCH => Some("SWITCH"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1ReferenceName.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1ReferenceName(i32);

    impl StdVideoAV1ReferenceName {
        pub const INTRA_FRAME: Self = Self(0);
        pub const LAST_FRAME: Self = Self(1);
        pub const LAST2_FRAME: Self = Self(2);
        pub const LAST3_FRAME: Self = Self(3);
        pub const GOLDEN_FRAME: Self = Self(4);
        pub const BWDREF_FRAME: Self = Self(5);
        pub const ALTREF2_FRAME: Self = Self(6);
        pub const ALTREF_FRAME: Self = Self(7);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1ReferenceName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::INTRA_FRAME => Some("INTRA_FRAME"),
                Self::LAST_FRAME => Some("LAST_FRAME"),
                Self::LAST2_FRAME => Some("LAST2_FRAME"),
                Self::LAST3_FRAME => Some("LAST3_FRAME"),
                Self::GOLDEN_FRAME => Some("GOLDEN_FRAME"),
                Self::BWDREF_FRAME => Some("BWDREF_FRAME"),
                Self::ALTREF2_FRAME => Some("ALTREF2_FRAME"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1InterpolationFilter.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1InterpolationFilter(i32);

    impl StdVideoAV1InterpolationFilter {
        pub const EIGHTTAP: Self = Self(0);
        pub const EIGHTTAP_SMOOTH: Self = Self(1);
        pub const EIGHTTAP_SHARP: Self = Self(2);
        pub const BILINEAR: Self = Self(3);
        pub const SWITCHABLE: Self = Self(4);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1InterpolationFilter {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1TxMode.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1TxMode(i32);

    impl StdVideoAV1TxMode {
        pub const ONLY_4X4: Self = Self(0);
        pub const LARGEST: Self = Self(1);
        pub const SELECT: Self = Self(2);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1TxMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::ONLY_4X4 => Some("ONLY_4X4"),
                Self::LARGEST => Some("LARGEST"),
                Self::SELECT => Some("SELECT"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1FrameRestorationType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1FrameRestorationType(i32);

    impl StdVideoAV1FrameRestorationType {
        pub const NONE: Self = Self(0);
        pub const WIENER: Self = Self(1);
        pub const SGRPROJ: Self = Self(2);
        pub const SWITCHABLE: Self = Self(3);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1FrameRestorationType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE => Some("NONE"),
                Self::WIENER => Some("WIENER"),
                Self::SGRPROJ => Some("SGRPROJ"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1ColorPrimaries.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1ColorPrimaries(i32);

    impl StdVideoAV1ColorPrimaries {
        pub const BT_709: Self = Self(1);
        pub const UNSPECIFIED: Self = Self(2);
        pub const BT_470_M: Self = Self(4);
        pub const BT_470_B_G: Self = Self(5);
        pub const BT_601: Self = Self(6);
        pub const SMPTE_240: Self = Self(7);
        pub const GENERIC_FILM: Self = Self(8);
        pub const BT_2020: Self = Self(9);
        pub const XYZ: Self = Self(10);
        pub const SMPTE_431: Self = Self(11);
        pub const SMPTE_432: Self = Self(12);
        pub const EBU_3213: Self = Self(22);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1ColorPrimaries {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BT_709 => Some("BT_709"),
                Self::UNSPECIFIED => Some("UNSPECIFIED"),
                Self::BT_470_M => Some("BT_470_M"),
                Self::BT_470_B_G => Some("BT_470_B_G"),
                Self::BT_601 => Some("BT_601"),
                Self::SMPTE_240 => Some("SMPTE_240"),
                Self::GENERIC_FILM => Some("GENERIC_FILM"),
                Self::BT_2020 => Some("BT_2020"),
                Self::XYZ => Some("XYZ"),
                Self::SMPTE_431 => Some("SMPTE_431"),
                Self::SMPTE_432 => Some("SMPTE_432"),
                Self::EBU_3213 => Some("EBU_3213"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1TransferCharacteristics.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1TransferCharacteristics(i32);

    impl StdVideoAV1TransferCharacteristics {
        pub const RESERVED_0: Self = Self(0);
        pub const BT_709: Self = Self(1);
        pub const UNSPECIFIED: Self = Self(2);
        pub const RESERVED_3: Self = Self(3);
        pub const BT_470_M: Self = Self(4);
        pub const BT_470_B_G: Self = Self(5);
        pub const BT_601: Self = Self(6);
        pub const SMPTE_240: Self = Self(7);
        pub const LINEAR: Self = Self(8);
        pub const LOG_100: Self = Self(9);
        pub const LOG_100_SQRT10: Self = Self(10);
        pub const IEC_61966: Self = Self(11);
        pub const BT_1361: Self = Self(12);
        pub const SRGB: Self = Self(13);
        pub const BT_2020_10_BIT: Self = Self(14);
        pub const BT_2020_12_BIT: Self = Self(15);
        pub const SMPTE_2084: Self = Self(16);
        pub const SMPTE_428: Self = Self(17);
        pub const HLG: Self = Self(18);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1TransferCharacteristics {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::RESERVED_0 => Some("RESERVED_0"),
                Self::BT_709 => Some("BT_709"),
                Self::UNSPECIFIED => Some("UNSPECIFIED"),
                Self::RESERVED_3 => Some("RESERVED_3"),
                Self::BT_470_M => Some("BT_470_M"),
                Self::BT_470_B_G => Some("BT_470_B_G"),
                Self::BT_601 => Some("BT_601"),
                Self::SMPTE_240 => Some("SMPTE_240"),
                Self::LINEAR => Some("LINEAR"),
                Self::LOG_100 => Some("LOG_100"),
                Self::LOG_100_SQRT10 => Some("LOG_100_SQRT10"),
                Self::IEC_61966 => Some("IEC_61966"),
                Self::BT_1361 => Some("BT_1361"),
                Self::SRGB => Some("SRGB"),
                Self::BT_2020_10_BIT => Some("BT_2020_10_BIT"),
                Self::BT_2020_12_BIT => Some("BT_2020_12_BIT"),
                Self::SMPTE_2084 => Some("SMPTE_2084"),
                Self::SMPTE_428 => Some("SMPTE_428"),
                Self::HLG => Some("HLG"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1MatrixCoefficients.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1MatrixCoefficients(i32);

    impl StdVideoAV1MatrixCoefficients {
        pub const IDENTITY: Self = Self(0);
        pub const BT_709: Self = Self(1);
        pub const UNSPECIFIED: Self = Self(2);
        pub const RESERVED_3: Self = Self(3);
        pub const FCC: Self = Self(4);
        pub const BT_470_B_G: Self = Self(5);
        pub const BT_601: Self = Self(6);
        pub const SMPTE_240: Self = Self(7);
        pub const SMPTE_YCGCO: Self = Self(8);
        pub const BT_2020_NCL: Self = Self(9);
        pub const BT_2020_CL: Self = Self(10);
        pub const SMPTE_2085: Self = Self(11);
        pub const CHROMAT_NCL: Self = Self(12);
        pub const CHROMAT_CL: Self = Self(13);
        pub const ICTCP: Self = Self(14);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1MatrixCoefficients {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::IDENTITY => Some("IDENTITY"),
                Self::BT_709 => Some("BT_709"),
                Self::UNSPECIFIED => Some("UNSPECIFIED"),
                Self::RESERVED_3 => Some("RESERVED_3"),
                Self::FCC => Some("FCC"),
                Self::BT_470_B_G => Some("BT_470_B_G"),
                Self::BT_601 => Some("BT_601"),
                Self::SMPTE_240 => Some("SMPTE_240"),
                Self::SMPTE_YCGCO => Some("SMPTE_YCGCO"),
                Self::BT_2020_NCL => Some("BT_2020_NCL"),
                Self::BT_2020_CL => Some("BT_2020_CL"),
                Self::SMPTE_2085 => Some("SMPTE_2085"),
                Self::CHROMAT_NCL => Some("CHROMAT_NCL"),
                Self::CHROMAT_CL => Some("CHROMAT_CL"),
                Self::ICTCP => Some("ICTCP"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoAV1ChromaSamplePosition.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoAV1ChromaSamplePosition(i32);

    impl StdVideoAV1ChromaSamplePosition {
        pub const UNKNOWN: Self = Self(0);
        pub const VERTICAL: Self = Self(1);
        pub const COLOCATED: Self = Self(2);
        pub const RESERVED: Self = Self(3);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoAV1ChromaSamplePosition {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNKNOWN => Some("UNKNOWN"),
                Self::VERTICAL => Some("VERTICAL"),
                Self::COLOCATED => Some("COLOCATED"),
                Self::RESERVED => Some("RESERVED"),
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
