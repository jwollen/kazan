#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"vulkan_video_codec_h265std";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const STD_VIDEO_H265_CPB_CNT_LIST_SIZE: u32 = 32;
    pub const STD_VIDEO_H265_SUBLAYERS_LIST_SIZE: u32 = 7;
    pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
    pub const STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
    pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
    pub const STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
    pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS: u32 = 6;
    pub const STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS: u32 = 64;
    pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS: u32 = 2;
    pub const STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS: u32 = 64;
    pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE: u32 = 6;
    pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE: u32 = 19;
    pub const STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE: u32 = 21;
    pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE: u32 = 3;
    pub const STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE: u32 = 128;
    pub const STD_VIDEO_H265_MAX_NUM_LIST_REF: u32 = 15;
    pub const STD_VIDEO_H265_MAX_CHROMA_PLANES: u32 = 2;
    pub const STD_VIDEO_H265_MAX_SHORT_TERM_REF_PIC_SETS: u32 = 64;
    pub const STD_VIDEO_H265_MAX_DPB_SIZE: u32 = 16;
    pub const STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS: u32 = 32;
    pub const STD_VIDEO_H265_MAX_LONG_TERM_PICS: u32 = 16;
    pub const STD_VIDEO_H265_MAX_DELTA_POC: u32 = 48;
    pub const STD_VIDEO_H265_NO_REFERENCE_PICTURE: u8 = 0xF;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265ProfileTierLevelFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoH265ProfileTierLevelFlags {
        pub general_tier_flag: u32,
        pub general_progressive_source_flag: u32,
        pub general_interlaced_source_flag: u32,
        pub general_non_packed_constraint_flag: u32,
        pub general_frame_only_constraint_flag: u32,
    }

    impl StdVideoH265ProfileTierLevelFlags {
        #[inline]
        pub fn general_tier_flag(mut self, general_tier_flag: u32) -> Self {
            self.general_tier_flag = general_tier_flag;
            self
        }

        #[inline]
        pub fn general_progressive_source_flag(
            mut self,
            general_progressive_source_flag: u32,
        ) -> Self {
            self.general_progressive_source_flag = general_progressive_source_flag;
            self
        }

        #[inline]
        pub fn general_interlaced_source_flag(
            mut self,
            general_interlaced_source_flag: u32,
        ) -> Self {
            self.general_interlaced_source_flag = general_interlaced_source_flag;
            self
        }

        #[inline]
        pub fn general_non_packed_constraint_flag(
            mut self,
            general_non_packed_constraint_flag: u32,
        ) -> Self {
            self.general_non_packed_constraint_flag = general_non_packed_constraint_flag;
            self
        }

        #[inline]
        pub fn general_frame_only_constraint_flag(
            mut self,
            general_frame_only_constraint_flag: u32,
        ) -> Self {
            self.general_frame_only_constraint_flag = general_frame_only_constraint_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265ProfileTierLevel.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoH265ProfileTierLevel {
        pub flags: StdVideoH265ProfileTierLevelFlags,
        pub general_profile_idc: StdVideoH265ProfileIdc,
        pub general_level_idc: StdVideoH265LevelIdc,
    }

    impl StdVideoH265ProfileTierLevel {
        #[inline]
        pub fn flags(mut self, flags: StdVideoH265ProfileTierLevelFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn general_profile_idc(mut self, general_profile_idc: StdVideoH265ProfileIdc) -> Self {
            self.general_profile_idc = general_profile_idc;
            self
        }

        #[inline]
        pub fn general_level_idc(mut self, general_level_idc: StdVideoH265LevelIdc) -> Self {
            self.general_level_idc = general_level_idc;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265DecPicBufMgr.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265DecPicBufMgr {
        pub max_latency_increase_plus1: [u32; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        pub max_dec_pic_buffering_minus1: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        pub max_num_reorder_pics: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
    }

    impl Default for StdVideoH265DecPicBufMgr {
        fn default() -> Self {
            Self {
                max_latency_increase_plus1: [Default::default(); _],
                max_dec_pic_buffering_minus1: [Default::default(); _],
                max_num_reorder_pics: [Default::default(); _],
            }
        }
    }

    impl StdVideoH265DecPicBufMgr {
        #[inline]
        pub fn max_latency_increase_plus1(
            mut self,
            max_latency_increase_plus1: [u32; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        ) -> Self {
            self.max_latency_increase_plus1 = max_latency_increase_plus1;
            self
        }

        #[inline]
        pub fn max_dec_pic_buffering_minus1(
            mut self,
            max_dec_pic_buffering_minus1: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        ) -> Self {
            self.max_dec_pic_buffering_minus1 = max_dec_pic_buffering_minus1;
            self
        }

        #[inline]
        pub fn max_num_reorder_pics(
            mut self,
            max_num_reorder_pics: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        ) -> Self {
            self.max_num_reorder_pics = max_num_reorder_pics;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265SubLayerHrdParameters.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265SubLayerHrdParameters {
        pub bit_rate_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
        pub cpb_size_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
        pub cpb_size_du_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
        pub bit_rate_du_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
        pub cbr_flag: u32,
    }

    impl Default for StdVideoH265SubLayerHrdParameters {
        fn default() -> Self {
            Self {
                bit_rate_value_minus1: [Default::default(); _],
                cpb_size_value_minus1: [Default::default(); _],
                cpb_size_du_value_minus1: [Default::default(); _],
                bit_rate_du_value_minus1: [Default::default(); _],
                cbr_flag: Default::default(),
            }
        }
    }

    impl StdVideoH265SubLayerHrdParameters {
        #[inline]
        pub fn bit_rate_value_minus1(
            mut self,
            bit_rate_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
        ) -> Self {
            self.bit_rate_value_minus1 = bit_rate_value_minus1;
            self
        }

        #[inline]
        pub fn cpb_size_value_minus1(
            mut self,
            cpb_size_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
        ) -> Self {
            self.cpb_size_value_minus1 = cpb_size_value_minus1;
            self
        }

        #[inline]
        pub fn cpb_size_du_value_minus1(
            mut self,
            cpb_size_du_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
        ) -> Self {
            self.cpb_size_du_value_minus1 = cpb_size_du_value_minus1;
            self
        }

        #[inline]
        pub fn bit_rate_du_value_minus1(
            mut self,
            bit_rate_du_value_minus1: [u32; STD_VIDEO_H265_CPB_CNT_LIST_SIZE as usize],
        ) -> Self {
            self.bit_rate_du_value_minus1 = bit_rate_du_value_minus1;
            self
        }

        #[inline]
        pub fn cbr_flag(mut self, cbr_flag: u32) -> Self {
            self.cbr_flag = cbr_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265HrdFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoH265HrdFlags {
        pub nal_hrd_parameters_present_flag: u32,
        pub vcl_hrd_parameters_present_flag: u32,
        pub sub_pic_hrd_params_present_flag: u32,
        pub sub_pic_cpb_params_in_pic_timing_sei_flag: u32,
        pub fixed_pic_rate_general_flag: u32,
        pub fixed_pic_rate_within_cvs_flag: u32,
        pub low_delay_hrd_flag: u32,
    }

    impl StdVideoH265HrdFlags {
        #[inline]
        pub fn nal_hrd_parameters_present_flag(
            mut self,
            nal_hrd_parameters_present_flag: u32,
        ) -> Self {
            self.nal_hrd_parameters_present_flag = nal_hrd_parameters_present_flag;
            self
        }

        #[inline]
        pub fn vcl_hrd_parameters_present_flag(
            mut self,
            vcl_hrd_parameters_present_flag: u32,
        ) -> Self {
            self.vcl_hrd_parameters_present_flag = vcl_hrd_parameters_present_flag;
            self
        }

        #[inline]
        pub fn sub_pic_hrd_params_present_flag(
            mut self,
            sub_pic_hrd_params_present_flag: u32,
        ) -> Self {
            self.sub_pic_hrd_params_present_flag = sub_pic_hrd_params_present_flag;
            self
        }

        #[inline]
        pub fn sub_pic_cpb_params_in_pic_timing_sei_flag(
            mut self,
            sub_pic_cpb_params_in_pic_timing_sei_flag: u32,
        ) -> Self {
            self.sub_pic_cpb_params_in_pic_timing_sei_flag =
                sub_pic_cpb_params_in_pic_timing_sei_flag;
            self
        }

        #[inline]
        pub fn fixed_pic_rate_general_flag(mut self, fixed_pic_rate_general_flag: u32) -> Self {
            self.fixed_pic_rate_general_flag = fixed_pic_rate_general_flag;
            self
        }

        #[inline]
        pub fn fixed_pic_rate_within_cvs_flag(
            mut self,
            fixed_pic_rate_within_cvs_flag: u32,
        ) -> Self {
            self.fixed_pic_rate_within_cvs_flag = fixed_pic_rate_within_cvs_flag;
            self
        }

        #[inline]
        pub fn low_delay_hrd_flag(mut self, low_delay_hrd_flag: u32) -> Self {
            self.low_delay_hrd_flag = low_delay_hrd_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265HrdParameters.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265HrdParameters<'a> {
        pub flags: StdVideoH265HrdFlags,
        pub tick_divisor_minus2: u8,
        pub du_cpb_removal_delay_increment_length_minus1: u8,
        pub dpb_output_delay_du_length_minus1: u8,
        pub bit_rate_scale: u8,
        pub cpb_size_scale: u8,
        pub cpb_size_du_scale: u8,
        pub initial_cpb_removal_delay_length_minus1: u8,
        pub au_cpb_removal_delay_length_minus1: u8,
        pub dpb_output_delay_length_minus1: u8,
        pub cpb_cnt_minus1: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        pub elemental_duration_in_tc_minus1: [u16; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        pub reserved: [u16; 3],
        pub p_sub_layer_hrd_parameters_nal: *const StdVideoH265SubLayerHrdParameters,
        pub p_sub_layer_hrd_parameters_vcl: *const StdVideoH265SubLayerHrdParameters,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoH265HrdParameters<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoH265HrdParameters")
                .field("flags", &self.flags)
                .field("tick_divisor_minus2", &self.tick_divisor_minus2)
                .field(
                    "du_cpb_removal_delay_increment_length_minus1",
                    &self.du_cpb_removal_delay_increment_length_minus1,
                )
                .field(
                    "dpb_output_delay_du_length_minus1",
                    &self.dpb_output_delay_du_length_minus1,
                )
                .field("bit_rate_scale", &self.bit_rate_scale)
                .field("cpb_size_scale", &self.cpb_size_scale)
                .field("cpb_size_du_scale", &self.cpb_size_du_scale)
                .field(
                    "initial_cpb_removal_delay_length_minus1",
                    &self.initial_cpb_removal_delay_length_minus1,
                )
                .field(
                    "au_cpb_removal_delay_length_minus1",
                    &self.au_cpb_removal_delay_length_minus1,
                )
                .field(
                    "dpb_output_delay_length_minus1",
                    &self.dpb_output_delay_length_minus1,
                )
                .field("cpb_cnt_minus1", &self.cpb_cnt_minus1)
                .field(
                    "elemental_duration_in_tc_minus1",
                    &self.elemental_duration_in_tc_minus1,
                )
                .field("reserved", &self.reserved)
                .field(
                    "p_sub_layer_hrd_parameters_nal",
                    &self.p_sub_layer_hrd_parameters_nal,
                )
                .field(
                    "p_sub_layer_hrd_parameters_vcl",
                    &self.p_sub_layer_hrd_parameters_vcl,
                )
                .finish()
        }
    }

    impl Default for StdVideoH265HrdParameters<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                tick_divisor_minus2: Default::default(),
                du_cpb_removal_delay_increment_length_minus1: Default::default(),
                dpb_output_delay_du_length_minus1: Default::default(),
                bit_rate_scale: Default::default(),
                cpb_size_scale: Default::default(),
                cpb_size_du_scale: Default::default(),
                initial_cpb_removal_delay_length_minus1: Default::default(),
                au_cpb_removal_delay_length_minus1: Default::default(),
                dpb_output_delay_length_minus1: Default::default(),
                cpb_cnt_minus1: [Default::default(); _],
                elemental_duration_in_tc_minus1: [Default::default(); _],
                reserved: [Default::default(); _],
                p_sub_layer_hrd_parameters_nal: core::ptr::null(),
                p_sub_layer_hrd_parameters_vcl: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoH265HrdParameters<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoH265HrdFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn tick_divisor_minus2(mut self, tick_divisor_minus2: u8) -> Self {
            self.tick_divisor_minus2 = tick_divisor_minus2;
            self
        }

        #[inline]
        pub fn du_cpb_removal_delay_increment_length_minus1(
            mut self,
            du_cpb_removal_delay_increment_length_minus1: u8,
        ) -> Self {
            self.du_cpb_removal_delay_increment_length_minus1 =
                du_cpb_removal_delay_increment_length_minus1;
            self
        }

        #[inline]
        pub fn dpb_output_delay_du_length_minus1(
            mut self,
            dpb_output_delay_du_length_minus1: u8,
        ) -> Self {
            self.dpb_output_delay_du_length_minus1 = dpb_output_delay_du_length_minus1;
            self
        }

        #[inline]
        pub fn bit_rate_scale(mut self, bit_rate_scale: u8) -> Self {
            self.bit_rate_scale = bit_rate_scale;
            self
        }

        #[inline]
        pub fn cpb_size_scale(mut self, cpb_size_scale: u8) -> Self {
            self.cpb_size_scale = cpb_size_scale;
            self
        }

        #[inline]
        pub fn cpb_size_du_scale(mut self, cpb_size_du_scale: u8) -> Self {
            self.cpb_size_du_scale = cpb_size_du_scale;
            self
        }

        #[inline]
        pub fn initial_cpb_removal_delay_length_minus1(
            mut self,
            initial_cpb_removal_delay_length_minus1: u8,
        ) -> Self {
            self.initial_cpb_removal_delay_length_minus1 = initial_cpb_removal_delay_length_minus1;
            self
        }

        #[inline]
        pub fn au_cpb_removal_delay_length_minus1(
            mut self,
            au_cpb_removal_delay_length_minus1: u8,
        ) -> Self {
            self.au_cpb_removal_delay_length_minus1 = au_cpb_removal_delay_length_minus1;
            self
        }

        #[inline]
        pub fn dpb_output_delay_length_minus1(
            mut self,
            dpb_output_delay_length_minus1: u8,
        ) -> Self {
            self.dpb_output_delay_length_minus1 = dpb_output_delay_length_minus1;
            self
        }

        #[inline]
        pub fn cpb_cnt_minus1(
            mut self,
            cpb_cnt_minus1: [u8; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        ) -> Self {
            self.cpb_cnt_minus1 = cpb_cnt_minus1;
            self
        }

        #[inline]
        pub fn elemental_duration_in_tc_minus1(
            mut self,
            elemental_duration_in_tc_minus1: [u16; STD_VIDEO_H265_SUBLAYERS_LIST_SIZE as usize],
        ) -> Self {
            self.elemental_duration_in_tc_minus1 = elemental_duration_in_tc_minus1;
            self
        }

        #[inline]
        pub fn reserved(mut self, reserved: [u16; 3]) -> Self {
            self.reserved = reserved;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265VpsFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoH265VpsFlags {
        pub vps_temporal_id_nesting_flag: u32,
        pub vps_sub_layer_ordering_info_present_flag: u32,
        pub vps_timing_info_present_flag: u32,
        pub vps_poc_proportional_to_timing_flag: u32,
    }

    impl StdVideoH265VpsFlags {
        #[inline]
        pub fn vps_temporal_id_nesting_flag(mut self, vps_temporal_id_nesting_flag: u32) -> Self {
            self.vps_temporal_id_nesting_flag = vps_temporal_id_nesting_flag;
            self
        }

        #[inline]
        pub fn vps_sub_layer_ordering_info_present_flag(
            mut self,
            vps_sub_layer_ordering_info_present_flag: u32,
        ) -> Self {
            self.vps_sub_layer_ordering_info_present_flag =
                vps_sub_layer_ordering_info_present_flag;
            self
        }

        #[inline]
        pub fn vps_timing_info_present_flag(mut self, vps_timing_info_present_flag: u32) -> Self {
            self.vps_timing_info_present_flag = vps_timing_info_present_flag;
            self
        }

        #[inline]
        pub fn vps_poc_proportional_to_timing_flag(
            mut self,
            vps_poc_proportional_to_timing_flag: u32,
        ) -> Self {
            self.vps_poc_proportional_to_timing_flag = vps_poc_proportional_to_timing_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265VideoParameterSet.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265VideoParameterSet<'a> {
        pub flags: StdVideoH265VpsFlags,
        pub vps_video_parameter_set_id: u8,
        pub vps_max_sub_layers_minus1: u8,
        pub reserved1: u8,
        pub reserved2: u8,
        pub vps_num_units_in_tick: u32,
        pub vps_time_scale: u32,
        pub vps_num_ticks_poc_diff_one_minus1: u32,
        pub reserved3: u32,
        pub p_dec_pic_buf_mgr: *const StdVideoH265DecPicBufMgr,
        pub p_hrd_parameters: *const StdVideoH265HrdParameters<'a>,
        pub p_profile_tier_level: *const StdVideoH265ProfileTierLevel,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoH265VideoParameterSet<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoH265VideoParameterSet")
                .field("flags", &self.flags)
                .field(
                    "vps_video_parameter_set_id",
                    &self.vps_video_parameter_set_id,
                )
                .field("vps_max_sub_layers_minus1", &self.vps_max_sub_layers_minus1)
                .field("reserved1", &self.reserved1)
                .field("reserved2", &self.reserved2)
                .field("vps_num_units_in_tick", &self.vps_num_units_in_tick)
                .field("vps_time_scale", &self.vps_time_scale)
                .field(
                    "vps_num_ticks_poc_diff_one_minus1",
                    &self.vps_num_ticks_poc_diff_one_minus1,
                )
                .field("reserved3", &self.reserved3)
                .field("p_dec_pic_buf_mgr", &self.p_dec_pic_buf_mgr)
                .field("p_hrd_parameters", &self.p_hrd_parameters)
                .field("p_profile_tier_level", &self.p_profile_tier_level)
                .finish()
        }
    }

    impl Default for StdVideoH265VideoParameterSet<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                vps_video_parameter_set_id: Default::default(),
                vps_max_sub_layers_minus1: Default::default(),
                reserved1: Default::default(),
                reserved2: Default::default(),
                vps_num_units_in_tick: Default::default(),
                vps_time_scale: Default::default(),
                vps_num_ticks_poc_diff_one_minus1: Default::default(),
                reserved3: Default::default(),
                p_dec_pic_buf_mgr: core::ptr::null(),
                p_hrd_parameters: core::ptr::null(),
                p_profile_tier_level: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoH265VideoParameterSet<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoH265VpsFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn vps_video_parameter_set_id(mut self, vps_video_parameter_set_id: u8) -> Self {
            self.vps_video_parameter_set_id = vps_video_parameter_set_id;
            self
        }

        #[inline]
        pub fn vps_max_sub_layers_minus1(mut self, vps_max_sub_layers_minus1: u8) -> Self {
            self.vps_max_sub_layers_minus1 = vps_max_sub_layers_minus1;
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
        pub fn vps_num_units_in_tick(mut self, vps_num_units_in_tick: u32) -> Self {
            self.vps_num_units_in_tick = vps_num_units_in_tick;
            self
        }

        #[inline]
        pub fn vps_time_scale(mut self, vps_time_scale: u32) -> Self {
            self.vps_time_scale = vps_time_scale;
            self
        }

        #[inline]
        pub fn vps_num_ticks_poc_diff_one_minus1(
            mut self,
            vps_num_ticks_poc_diff_one_minus1: u32,
        ) -> Self {
            self.vps_num_ticks_poc_diff_one_minus1 = vps_num_ticks_poc_diff_one_minus1;
            self
        }

        #[inline]
        pub fn reserved3(mut self, reserved3: u32) -> Self {
            self.reserved3 = reserved3;
            self
        }

        #[inline]
        pub fn dec_pic_buf_mgr(mut self, dec_pic_buf_mgr: &'a StdVideoH265DecPicBufMgr) -> Self {
            self.p_dec_pic_buf_mgr = dec_pic_buf_mgr;
            self
        }

        #[inline]
        pub fn hrd_parameters(mut self, hrd_parameters: &'a StdVideoH265HrdParameters<'a>) -> Self {
            self.p_hrd_parameters = hrd_parameters;
            self
        }

        #[inline]
        pub fn profile_tier_level(
            mut self,
            profile_tier_level: &'a StdVideoH265ProfileTierLevel,
        ) -> Self {
            self.p_profile_tier_level = profile_tier_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265ScalingLists.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265ScalingLists {
        pub scaling_list4x4: [[u8; STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS as usize];
            STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS as usize],
        pub scaling_list8x8: [[u8; STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS as usize];
            STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS as usize],
        pub scaling_list16x16: [[u8; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS as usize];
            STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS as usize],
        pub scaling_list32x32: [[u8; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS as usize];
            STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS as usize],
        pub scaling_list_dc_coef16x16: [u8; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS as usize],
        pub scaling_list_dc_coef32x32: [u8; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS as usize],
    }

    impl Default for StdVideoH265ScalingLists {
        fn default() -> Self {
            Self {
                scaling_list4x4: [[Default::default(); _]; _],
                scaling_list8x8: [[Default::default(); _]; _],
                scaling_list16x16: [[Default::default(); _]; _],
                scaling_list32x32: [[Default::default(); _]; _],
                scaling_list_dc_coef16x16: [Default::default(); _],
                scaling_list_dc_coef32x32: [Default::default(); _],
            }
        }
    }

    impl StdVideoH265ScalingLists {
        #[inline]
        pub fn scaling_list4x4(
            mut self,
            scaling_list4x4: [[u8; STD_VIDEO_H265_SCALING_LIST_4X4_NUM_ELEMENTS as usize];
                STD_VIDEO_H265_SCALING_LIST_4X4_NUM_LISTS as usize],
        ) -> Self {
            self.scaling_list4x4 = scaling_list4x4;
            self
        }

        #[inline]
        pub fn scaling_list8x8(
            mut self,
            scaling_list8x8: [[u8; STD_VIDEO_H265_SCALING_LIST_8X8_NUM_ELEMENTS as usize];
                STD_VIDEO_H265_SCALING_LIST_8X8_NUM_LISTS as usize],
        ) -> Self {
            self.scaling_list8x8 = scaling_list8x8;
            self
        }

        #[inline]
        pub fn scaling_list16x16(
            mut self,
            scaling_list16x16: [[u8; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_ELEMENTS as usize];
                STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS as usize],
        ) -> Self {
            self.scaling_list16x16 = scaling_list16x16;
            self
        }

        #[inline]
        pub fn scaling_list32x32(
            mut self,
            scaling_list32x32: [[u8; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_ELEMENTS as usize];
                STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS as usize],
        ) -> Self {
            self.scaling_list32x32 = scaling_list32x32;
            self
        }

        #[inline]
        pub fn scaling_list_dc_coef16x16(
            mut self,
            scaling_list_dc_coef16x16: [u8; STD_VIDEO_H265_SCALING_LIST_16X16_NUM_LISTS as usize],
        ) -> Self {
            self.scaling_list_dc_coef16x16 = scaling_list_dc_coef16x16;
            self
        }

        #[inline]
        pub fn scaling_list_dc_coef32x32(
            mut self,
            scaling_list_dc_coef32x32: [u8; STD_VIDEO_H265_SCALING_LIST_32X32_NUM_LISTS as usize],
        ) -> Self {
            self.scaling_list_dc_coef32x32 = scaling_list_dc_coef32x32;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265ShortTermRefPicSetFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoH265ShortTermRefPicSetFlags {
        pub inter_ref_pic_set_prediction_flag: u32,
        pub delta_rps_sign: u32,
    }

    impl StdVideoH265ShortTermRefPicSetFlags {
        #[inline]
        pub fn inter_ref_pic_set_prediction_flag(
            mut self,
            inter_ref_pic_set_prediction_flag: u32,
        ) -> Self {
            self.inter_ref_pic_set_prediction_flag = inter_ref_pic_set_prediction_flag;
            self
        }

        #[inline]
        pub fn delta_rps_sign(mut self, delta_rps_sign: u32) -> Self {
            self.delta_rps_sign = delta_rps_sign;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265ShortTermRefPicSet.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265ShortTermRefPicSet {
        pub flags: StdVideoH265ShortTermRefPicSetFlags,
        pub delta_idx_minus1: u32,
        pub use_delta_flag: u16,
        pub abs_delta_rps_minus1: u16,
        pub used_by_curr_pic_flag: u16,
        pub used_by_curr_pic_s0_flag: u16,
        pub used_by_curr_pic_s1_flag: u16,
        pub reserved1: u16,
        pub reserved2: u8,
        pub reserved3: u8,
        pub num_negative_pics: u8,
        pub num_positive_pics: u8,
        pub delta_poc_s0_minus1: [u16; STD_VIDEO_H265_MAX_DPB_SIZE as usize],
        pub delta_poc_s1_minus1: [u16; STD_VIDEO_H265_MAX_DPB_SIZE as usize],
    }

    impl Default for StdVideoH265ShortTermRefPicSet {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                delta_idx_minus1: Default::default(),
                use_delta_flag: Default::default(),
                abs_delta_rps_minus1: Default::default(),
                used_by_curr_pic_flag: Default::default(),
                used_by_curr_pic_s0_flag: Default::default(),
                used_by_curr_pic_s1_flag: Default::default(),
                reserved1: Default::default(),
                reserved2: Default::default(),
                reserved3: Default::default(),
                num_negative_pics: Default::default(),
                num_positive_pics: Default::default(),
                delta_poc_s0_minus1: [Default::default(); _],
                delta_poc_s1_minus1: [Default::default(); _],
            }
        }
    }

    impl StdVideoH265ShortTermRefPicSet {
        #[inline]
        pub fn flags(mut self, flags: StdVideoH265ShortTermRefPicSetFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn delta_idx_minus1(mut self, delta_idx_minus1: u32) -> Self {
            self.delta_idx_minus1 = delta_idx_minus1;
            self
        }

        #[inline]
        pub fn use_delta_flag(mut self, use_delta_flag: u16) -> Self {
            self.use_delta_flag = use_delta_flag;
            self
        }

        #[inline]
        pub fn abs_delta_rps_minus1(mut self, abs_delta_rps_minus1: u16) -> Self {
            self.abs_delta_rps_minus1 = abs_delta_rps_minus1;
            self
        }

        #[inline]
        pub fn used_by_curr_pic_flag(mut self, used_by_curr_pic_flag: u16) -> Self {
            self.used_by_curr_pic_flag = used_by_curr_pic_flag;
            self
        }

        #[inline]
        pub fn used_by_curr_pic_s0_flag(mut self, used_by_curr_pic_s0_flag: u16) -> Self {
            self.used_by_curr_pic_s0_flag = used_by_curr_pic_s0_flag;
            self
        }

        #[inline]
        pub fn used_by_curr_pic_s1_flag(mut self, used_by_curr_pic_s1_flag: u16) -> Self {
            self.used_by_curr_pic_s1_flag = used_by_curr_pic_s1_flag;
            self
        }

        #[inline]
        pub fn reserved1(mut self, reserved1: u16) -> Self {
            self.reserved1 = reserved1;
            self
        }

        #[inline]
        pub fn reserved2(mut self, reserved2: u8) -> Self {
            self.reserved2 = reserved2;
            self
        }

        #[inline]
        pub fn reserved3(mut self, reserved3: u8) -> Self {
            self.reserved3 = reserved3;
            self
        }

        #[inline]
        pub fn num_negative_pics(mut self, num_negative_pics: u8) -> Self {
            self.num_negative_pics = num_negative_pics;
            self
        }

        #[inline]
        pub fn num_positive_pics(mut self, num_positive_pics: u8) -> Self {
            self.num_positive_pics = num_positive_pics;
            self
        }

        #[inline]
        pub fn delta_poc_s0_minus1(
            mut self,
            delta_poc_s0_minus1: [u16; STD_VIDEO_H265_MAX_DPB_SIZE as usize],
        ) -> Self {
            self.delta_poc_s0_minus1 = delta_poc_s0_minus1;
            self
        }

        #[inline]
        pub fn delta_poc_s1_minus1(
            mut self,
            delta_poc_s1_minus1: [u16; STD_VIDEO_H265_MAX_DPB_SIZE as usize],
        ) -> Self {
            self.delta_poc_s1_minus1 = delta_poc_s1_minus1;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265LongTermRefPicsSps.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265LongTermRefPicsSps {
        pub used_by_curr_pic_lt_sps_flag: u32,
        pub lt_ref_pic_poc_lsb_sps: [u32; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
    }

    impl Default for StdVideoH265LongTermRefPicsSps {
        fn default() -> Self {
            Self {
                used_by_curr_pic_lt_sps_flag: Default::default(),
                lt_ref_pic_poc_lsb_sps: [Default::default(); _],
            }
        }
    }

    impl StdVideoH265LongTermRefPicsSps {
        #[inline]
        pub fn used_by_curr_pic_lt_sps_flag(mut self, used_by_curr_pic_lt_sps_flag: u32) -> Self {
            self.used_by_curr_pic_lt_sps_flag = used_by_curr_pic_lt_sps_flag;
            self
        }

        #[inline]
        pub fn lt_ref_pic_poc_lsb_sps(
            mut self,
            lt_ref_pic_poc_lsb_sps: [u32; STD_VIDEO_H265_MAX_LONG_TERM_REF_PICS_SPS as usize],
        ) -> Self {
            self.lt_ref_pic_poc_lsb_sps = lt_ref_pic_poc_lsb_sps;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265SpsVuiFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoH265SpsVuiFlags {
        pub aspect_ratio_info_present_flag: u32,
        pub overscan_info_present_flag: u32,
        pub overscan_appropriate_flag: u32,
        pub video_signal_type_present_flag: u32,
        pub video_full_range_flag: u32,
        pub colour_description_present_flag: u32,
        pub chroma_loc_info_present_flag: u32,
        pub neutral_chroma_indication_flag: u32,
        pub field_seq_flag: u32,
        pub frame_field_info_present_flag: u32,
        pub default_display_window_flag: u32,
        pub vui_timing_info_present_flag: u32,
        pub vui_poc_proportional_to_timing_flag: u32,
        pub vui_hrd_parameters_present_flag: u32,
        pub bitstream_restriction_flag: u32,
        pub tiles_fixed_structure_flag: u32,
        pub motion_vectors_over_pic_boundaries_flag: u32,
        pub restricted_ref_pic_lists_flag: u32,
    }

    impl StdVideoH265SpsVuiFlags {
        #[inline]
        pub fn aspect_ratio_info_present_flag(
            mut self,
            aspect_ratio_info_present_flag: u32,
        ) -> Self {
            self.aspect_ratio_info_present_flag = aspect_ratio_info_present_flag;
            self
        }

        #[inline]
        pub fn overscan_info_present_flag(mut self, overscan_info_present_flag: u32) -> Self {
            self.overscan_info_present_flag = overscan_info_present_flag;
            self
        }

        #[inline]
        pub fn overscan_appropriate_flag(mut self, overscan_appropriate_flag: u32) -> Self {
            self.overscan_appropriate_flag = overscan_appropriate_flag;
            self
        }

        #[inline]
        pub fn video_signal_type_present_flag(
            mut self,
            video_signal_type_present_flag: u32,
        ) -> Self {
            self.video_signal_type_present_flag = video_signal_type_present_flag;
            self
        }

        #[inline]
        pub fn video_full_range_flag(mut self, video_full_range_flag: u32) -> Self {
            self.video_full_range_flag = video_full_range_flag;
            self
        }

        #[inline]
        pub fn colour_description_present_flag(
            mut self,
            colour_description_present_flag: u32,
        ) -> Self {
            self.colour_description_present_flag = colour_description_present_flag;
            self
        }

        #[inline]
        pub fn chroma_loc_info_present_flag(mut self, chroma_loc_info_present_flag: u32) -> Self {
            self.chroma_loc_info_present_flag = chroma_loc_info_present_flag;
            self
        }

        #[inline]
        pub fn neutral_chroma_indication_flag(
            mut self,
            neutral_chroma_indication_flag: u32,
        ) -> Self {
            self.neutral_chroma_indication_flag = neutral_chroma_indication_flag;
            self
        }

        #[inline]
        pub fn field_seq_flag(mut self, field_seq_flag: u32) -> Self {
            self.field_seq_flag = field_seq_flag;
            self
        }

        #[inline]
        pub fn frame_field_info_present_flag(mut self, frame_field_info_present_flag: u32) -> Self {
            self.frame_field_info_present_flag = frame_field_info_present_flag;
            self
        }

        #[inline]
        pub fn default_display_window_flag(mut self, default_display_window_flag: u32) -> Self {
            self.default_display_window_flag = default_display_window_flag;
            self
        }

        #[inline]
        pub fn vui_timing_info_present_flag(mut self, vui_timing_info_present_flag: u32) -> Self {
            self.vui_timing_info_present_flag = vui_timing_info_present_flag;
            self
        }

        #[inline]
        pub fn vui_poc_proportional_to_timing_flag(
            mut self,
            vui_poc_proportional_to_timing_flag: u32,
        ) -> Self {
            self.vui_poc_proportional_to_timing_flag = vui_poc_proportional_to_timing_flag;
            self
        }

        #[inline]
        pub fn vui_hrd_parameters_present_flag(
            mut self,
            vui_hrd_parameters_present_flag: u32,
        ) -> Self {
            self.vui_hrd_parameters_present_flag = vui_hrd_parameters_present_flag;
            self
        }

        #[inline]
        pub fn bitstream_restriction_flag(mut self, bitstream_restriction_flag: u32) -> Self {
            self.bitstream_restriction_flag = bitstream_restriction_flag;
            self
        }

        #[inline]
        pub fn tiles_fixed_structure_flag(mut self, tiles_fixed_structure_flag: u32) -> Self {
            self.tiles_fixed_structure_flag = tiles_fixed_structure_flag;
            self
        }

        #[inline]
        pub fn motion_vectors_over_pic_boundaries_flag(
            mut self,
            motion_vectors_over_pic_boundaries_flag: u32,
        ) -> Self {
            self.motion_vectors_over_pic_boundaries_flag = motion_vectors_over_pic_boundaries_flag;
            self
        }

        #[inline]
        pub fn restricted_ref_pic_lists_flag(mut self, restricted_ref_pic_lists_flag: u32) -> Self {
            self.restricted_ref_pic_lists_flag = restricted_ref_pic_lists_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265SequenceParameterSetVui.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265SequenceParameterSetVui<'a> {
        pub flags: StdVideoH265SpsVuiFlags,
        pub aspect_ratio_idc: StdVideoH265AspectRatioIdc,
        pub sar_width: u16,
        pub sar_height: u16,
        pub video_format: u8,
        pub colour_primaries: u8,
        pub transfer_characteristics: u8,
        pub matrix_coeffs: u8,
        pub chroma_sample_loc_type_top_field: u8,
        pub chroma_sample_loc_type_bottom_field: u8,
        pub reserved1: u8,
        pub reserved2: u8,
        pub def_disp_win_left_offset: u16,
        pub def_disp_win_right_offset: u16,
        pub def_disp_win_top_offset: u16,
        pub def_disp_win_bottom_offset: u16,
        pub vui_num_units_in_tick: u32,
        pub vui_time_scale: u32,
        pub vui_num_ticks_poc_diff_one_minus1: u32,
        pub min_spatial_segmentation_idc: u16,
        pub reserved3: u16,
        pub max_bytes_per_pic_denom: u8,
        pub max_bits_per_min_cu_denom: u8,
        pub log2_max_mv_length_horizontal: u8,
        pub log2_max_mv_length_vertical: u8,
        pub p_hrd_parameters: *const StdVideoH265HrdParameters<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoH265SequenceParameterSetVui<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoH265SequenceParameterSetVui")
                .field("flags", &self.flags)
                .field("aspect_ratio_idc", &self.aspect_ratio_idc)
                .field("sar_width", &self.sar_width)
                .field("sar_height", &self.sar_height)
                .field("video_format", &self.video_format)
                .field("colour_primaries", &self.colour_primaries)
                .field("transfer_characteristics", &self.transfer_characteristics)
                .field("matrix_coeffs", &self.matrix_coeffs)
                .field(
                    "chroma_sample_loc_type_top_field",
                    &self.chroma_sample_loc_type_top_field,
                )
                .field(
                    "chroma_sample_loc_type_bottom_field",
                    &self.chroma_sample_loc_type_bottom_field,
                )
                .field("reserved1", &self.reserved1)
                .field("reserved2", &self.reserved2)
                .field("def_disp_win_left_offset", &self.def_disp_win_left_offset)
                .field("def_disp_win_right_offset", &self.def_disp_win_right_offset)
                .field("def_disp_win_top_offset", &self.def_disp_win_top_offset)
                .field(
                    "def_disp_win_bottom_offset",
                    &self.def_disp_win_bottom_offset,
                )
                .field("vui_num_units_in_tick", &self.vui_num_units_in_tick)
                .field("vui_time_scale", &self.vui_time_scale)
                .field(
                    "vui_num_ticks_poc_diff_one_minus1",
                    &self.vui_num_ticks_poc_diff_one_minus1,
                )
                .field(
                    "min_spatial_segmentation_idc",
                    &self.min_spatial_segmentation_idc,
                )
                .field("reserved3", &self.reserved3)
                .field("max_bytes_per_pic_denom", &self.max_bytes_per_pic_denom)
                .field("max_bits_per_min_cu_denom", &self.max_bits_per_min_cu_denom)
                .field(
                    "log2_max_mv_length_horizontal",
                    &self.log2_max_mv_length_horizontal,
                )
                .field(
                    "log2_max_mv_length_vertical",
                    &self.log2_max_mv_length_vertical,
                )
                .field("p_hrd_parameters", &self.p_hrd_parameters)
                .finish()
        }
    }

    impl Default for StdVideoH265SequenceParameterSetVui<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                aspect_ratio_idc: Default::default(),
                sar_width: Default::default(),
                sar_height: Default::default(),
                video_format: Default::default(),
                colour_primaries: Default::default(),
                transfer_characteristics: Default::default(),
                matrix_coeffs: Default::default(),
                chroma_sample_loc_type_top_field: Default::default(),
                chroma_sample_loc_type_bottom_field: Default::default(),
                reserved1: Default::default(),
                reserved2: Default::default(),
                def_disp_win_left_offset: Default::default(),
                def_disp_win_right_offset: Default::default(),
                def_disp_win_top_offset: Default::default(),
                def_disp_win_bottom_offset: Default::default(),
                vui_num_units_in_tick: Default::default(),
                vui_time_scale: Default::default(),
                vui_num_ticks_poc_diff_one_minus1: Default::default(),
                min_spatial_segmentation_idc: Default::default(),
                reserved3: Default::default(),
                max_bytes_per_pic_denom: Default::default(),
                max_bits_per_min_cu_denom: Default::default(),
                log2_max_mv_length_horizontal: Default::default(),
                log2_max_mv_length_vertical: Default::default(),
                p_hrd_parameters: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoH265SequenceParameterSetVui<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoH265SpsVuiFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn aspect_ratio_idc(mut self, aspect_ratio_idc: StdVideoH265AspectRatioIdc) -> Self {
            self.aspect_ratio_idc = aspect_ratio_idc;
            self
        }

        #[inline]
        pub fn sar_width(mut self, sar_width: u16) -> Self {
            self.sar_width = sar_width;
            self
        }

        #[inline]
        pub fn sar_height(mut self, sar_height: u16) -> Self {
            self.sar_height = sar_height;
            self
        }

        #[inline]
        pub fn video_format(mut self, video_format: u8) -> Self {
            self.video_format = video_format;
            self
        }

        #[inline]
        pub fn colour_primaries(mut self, colour_primaries: u8) -> Self {
            self.colour_primaries = colour_primaries;
            self
        }

        #[inline]
        pub fn transfer_characteristics(mut self, transfer_characteristics: u8) -> Self {
            self.transfer_characteristics = transfer_characteristics;
            self
        }

        #[inline]
        pub fn matrix_coeffs(mut self, matrix_coeffs: u8) -> Self {
            self.matrix_coeffs = matrix_coeffs;
            self
        }

        #[inline]
        pub fn chroma_sample_loc_type_top_field(
            mut self,
            chroma_sample_loc_type_top_field: u8,
        ) -> Self {
            self.chroma_sample_loc_type_top_field = chroma_sample_loc_type_top_field;
            self
        }

        #[inline]
        pub fn chroma_sample_loc_type_bottom_field(
            mut self,
            chroma_sample_loc_type_bottom_field: u8,
        ) -> Self {
            self.chroma_sample_loc_type_bottom_field = chroma_sample_loc_type_bottom_field;
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
        pub fn def_disp_win_left_offset(mut self, def_disp_win_left_offset: u16) -> Self {
            self.def_disp_win_left_offset = def_disp_win_left_offset;
            self
        }

        #[inline]
        pub fn def_disp_win_right_offset(mut self, def_disp_win_right_offset: u16) -> Self {
            self.def_disp_win_right_offset = def_disp_win_right_offset;
            self
        }

        #[inline]
        pub fn def_disp_win_top_offset(mut self, def_disp_win_top_offset: u16) -> Self {
            self.def_disp_win_top_offset = def_disp_win_top_offset;
            self
        }

        #[inline]
        pub fn def_disp_win_bottom_offset(mut self, def_disp_win_bottom_offset: u16) -> Self {
            self.def_disp_win_bottom_offset = def_disp_win_bottom_offset;
            self
        }

        #[inline]
        pub fn vui_num_units_in_tick(mut self, vui_num_units_in_tick: u32) -> Self {
            self.vui_num_units_in_tick = vui_num_units_in_tick;
            self
        }

        #[inline]
        pub fn vui_time_scale(mut self, vui_time_scale: u32) -> Self {
            self.vui_time_scale = vui_time_scale;
            self
        }

        #[inline]
        pub fn vui_num_ticks_poc_diff_one_minus1(
            mut self,
            vui_num_ticks_poc_diff_one_minus1: u32,
        ) -> Self {
            self.vui_num_ticks_poc_diff_one_minus1 = vui_num_ticks_poc_diff_one_minus1;
            self
        }

        #[inline]
        pub fn min_spatial_segmentation_idc(mut self, min_spatial_segmentation_idc: u16) -> Self {
            self.min_spatial_segmentation_idc = min_spatial_segmentation_idc;
            self
        }

        #[inline]
        pub fn reserved3(mut self, reserved3: u16) -> Self {
            self.reserved3 = reserved3;
            self
        }

        #[inline]
        pub fn max_bytes_per_pic_denom(mut self, max_bytes_per_pic_denom: u8) -> Self {
            self.max_bytes_per_pic_denom = max_bytes_per_pic_denom;
            self
        }

        #[inline]
        pub fn max_bits_per_min_cu_denom(mut self, max_bits_per_min_cu_denom: u8) -> Self {
            self.max_bits_per_min_cu_denom = max_bits_per_min_cu_denom;
            self
        }

        #[inline]
        pub fn log2_max_mv_length_horizontal(mut self, log2_max_mv_length_horizontal: u8) -> Self {
            self.log2_max_mv_length_horizontal = log2_max_mv_length_horizontal;
            self
        }

        #[inline]
        pub fn log2_max_mv_length_vertical(mut self, log2_max_mv_length_vertical: u8) -> Self {
            self.log2_max_mv_length_vertical = log2_max_mv_length_vertical;
            self
        }

        #[inline]
        pub fn hrd_parameters(mut self, hrd_parameters: &'a StdVideoH265HrdParameters<'a>) -> Self {
            self.p_hrd_parameters = hrd_parameters;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265PredictorPaletteEntries.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265PredictorPaletteEntries {
        pub predictor_palette_entries: [[u16;
            STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE as usize];
            STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE as usize],
    }

    impl Default for StdVideoH265PredictorPaletteEntries {
        fn default() -> Self {
            Self {
                predictor_palette_entries: [[Default::default(); _]; _],
            }
        }
    }

    impl StdVideoH265PredictorPaletteEntries {
        #[inline]
        pub fn predictor_palette_entries(
            mut self,
            predictor_palette_entries: [[u16; STD_VIDEO_H265_PREDICTOR_PALETTE_COMP_ENTRIES_LIST_SIZE as usize];
                STD_VIDEO_H265_PREDICTOR_PALETTE_COMPONENTS_LIST_SIZE
                    as usize],
        ) -> Self {
            self.predictor_palette_entries = predictor_palette_entries;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265SpsFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoH265SpsFlags {
        pub sps_temporal_id_nesting_flag: u32,
        pub separate_colour_plane_flag: u32,
        pub conformance_window_flag: u32,
        pub sps_sub_layer_ordering_info_present_flag: u32,
        pub scaling_list_enabled_flag: u32,
        pub sps_scaling_list_data_present_flag: u32,
        pub amp_enabled_flag: u32,
        pub sample_adaptive_offset_enabled_flag: u32,
        pub pcm_enabled_flag: u32,
        pub pcm_loop_filter_disabled_flag: u32,
        pub long_term_ref_pics_present_flag: u32,
        pub sps_temporal_mvp_enabled_flag: u32,
        pub strong_intra_smoothing_enabled_flag: u32,
        pub vui_parameters_present_flag: u32,
        pub sps_extension_present_flag: u32,
        pub sps_range_extension_flag: u32,
        pub transform_skip_rotation_enabled_flag: u32,
        pub transform_skip_context_enabled_flag: u32,
        pub implicit_rdpcm_enabled_flag: u32,
        pub explicit_rdpcm_enabled_flag: u32,
        pub extended_precision_processing_flag: u32,
        pub intra_smoothing_disabled_flag: u32,
        pub high_precision_offsets_enabled_flag: u32,
        pub persistent_rice_adaptation_enabled_flag: u32,
        pub cabac_bypass_alignment_enabled_flag: u32,
        pub sps_scc_extension_flag: u32,
        pub sps_curr_pic_ref_enabled_flag: u32,
        pub palette_mode_enabled_flag: u32,
        pub sps_palette_predictor_initializers_present_flag: u32,
        pub intra_boundary_filtering_disabled_flag: u32,
    }

    impl StdVideoH265SpsFlags {
        #[inline]
        pub fn sps_temporal_id_nesting_flag(mut self, sps_temporal_id_nesting_flag: u32) -> Self {
            self.sps_temporal_id_nesting_flag = sps_temporal_id_nesting_flag;
            self
        }

        #[inline]
        pub fn separate_colour_plane_flag(mut self, separate_colour_plane_flag: u32) -> Self {
            self.separate_colour_plane_flag = separate_colour_plane_flag;
            self
        }

        #[inline]
        pub fn conformance_window_flag(mut self, conformance_window_flag: u32) -> Self {
            self.conformance_window_flag = conformance_window_flag;
            self
        }

        #[inline]
        pub fn sps_sub_layer_ordering_info_present_flag(
            mut self,
            sps_sub_layer_ordering_info_present_flag: u32,
        ) -> Self {
            self.sps_sub_layer_ordering_info_present_flag =
                sps_sub_layer_ordering_info_present_flag;
            self
        }

        #[inline]
        pub fn scaling_list_enabled_flag(mut self, scaling_list_enabled_flag: u32) -> Self {
            self.scaling_list_enabled_flag = scaling_list_enabled_flag;
            self
        }

        #[inline]
        pub fn sps_scaling_list_data_present_flag(
            mut self,
            sps_scaling_list_data_present_flag: u32,
        ) -> Self {
            self.sps_scaling_list_data_present_flag = sps_scaling_list_data_present_flag;
            self
        }

        #[inline]
        pub fn amp_enabled_flag(mut self, amp_enabled_flag: u32) -> Self {
            self.amp_enabled_flag = amp_enabled_flag;
            self
        }

        #[inline]
        pub fn sample_adaptive_offset_enabled_flag(
            mut self,
            sample_adaptive_offset_enabled_flag: u32,
        ) -> Self {
            self.sample_adaptive_offset_enabled_flag = sample_adaptive_offset_enabled_flag;
            self
        }

        #[inline]
        pub fn pcm_enabled_flag(mut self, pcm_enabled_flag: u32) -> Self {
            self.pcm_enabled_flag = pcm_enabled_flag;
            self
        }

        #[inline]
        pub fn pcm_loop_filter_disabled_flag(mut self, pcm_loop_filter_disabled_flag: u32) -> Self {
            self.pcm_loop_filter_disabled_flag = pcm_loop_filter_disabled_flag;
            self
        }

        #[inline]
        pub fn long_term_ref_pics_present_flag(
            mut self,
            long_term_ref_pics_present_flag: u32,
        ) -> Self {
            self.long_term_ref_pics_present_flag = long_term_ref_pics_present_flag;
            self
        }

        #[inline]
        pub fn sps_temporal_mvp_enabled_flag(mut self, sps_temporal_mvp_enabled_flag: u32) -> Self {
            self.sps_temporal_mvp_enabled_flag = sps_temporal_mvp_enabled_flag;
            self
        }

        #[inline]
        pub fn strong_intra_smoothing_enabled_flag(
            mut self,
            strong_intra_smoothing_enabled_flag: u32,
        ) -> Self {
            self.strong_intra_smoothing_enabled_flag = strong_intra_smoothing_enabled_flag;
            self
        }

        #[inline]
        pub fn vui_parameters_present_flag(mut self, vui_parameters_present_flag: u32) -> Self {
            self.vui_parameters_present_flag = vui_parameters_present_flag;
            self
        }

        #[inline]
        pub fn sps_extension_present_flag(mut self, sps_extension_present_flag: u32) -> Self {
            self.sps_extension_present_flag = sps_extension_present_flag;
            self
        }

        #[inline]
        pub fn sps_range_extension_flag(mut self, sps_range_extension_flag: u32) -> Self {
            self.sps_range_extension_flag = sps_range_extension_flag;
            self
        }

        #[inline]
        pub fn transform_skip_rotation_enabled_flag(
            mut self,
            transform_skip_rotation_enabled_flag: u32,
        ) -> Self {
            self.transform_skip_rotation_enabled_flag = transform_skip_rotation_enabled_flag;
            self
        }

        #[inline]
        pub fn transform_skip_context_enabled_flag(
            mut self,
            transform_skip_context_enabled_flag: u32,
        ) -> Self {
            self.transform_skip_context_enabled_flag = transform_skip_context_enabled_flag;
            self
        }

        #[inline]
        pub fn implicit_rdpcm_enabled_flag(mut self, implicit_rdpcm_enabled_flag: u32) -> Self {
            self.implicit_rdpcm_enabled_flag = implicit_rdpcm_enabled_flag;
            self
        }

        #[inline]
        pub fn explicit_rdpcm_enabled_flag(mut self, explicit_rdpcm_enabled_flag: u32) -> Self {
            self.explicit_rdpcm_enabled_flag = explicit_rdpcm_enabled_flag;
            self
        }

        #[inline]
        pub fn extended_precision_processing_flag(
            mut self,
            extended_precision_processing_flag: u32,
        ) -> Self {
            self.extended_precision_processing_flag = extended_precision_processing_flag;
            self
        }

        #[inline]
        pub fn intra_smoothing_disabled_flag(mut self, intra_smoothing_disabled_flag: u32) -> Self {
            self.intra_smoothing_disabled_flag = intra_smoothing_disabled_flag;
            self
        }

        #[inline]
        pub fn high_precision_offsets_enabled_flag(
            mut self,
            high_precision_offsets_enabled_flag: u32,
        ) -> Self {
            self.high_precision_offsets_enabled_flag = high_precision_offsets_enabled_flag;
            self
        }

        #[inline]
        pub fn persistent_rice_adaptation_enabled_flag(
            mut self,
            persistent_rice_adaptation_enabled_flag: u32,
        ) -> Self {
            self.persistent_rice_adaptation_enabled_flag = persistent_rice_adaptation_enabled_flag;
            self
        }

        #[inline]
        pub fn cabac_bypass_alignment_enabled_flag(
            mut self,
            cabac_bypass_alignment_enabled_flag: u32,
        ) -> Self {
            self.cabac_bypass_alignment_enabled_flag = cabac_bypass_alignment_enabled_flag;
            self
        }

        #[inline]
        pub fn sps_scc_extension_flag(mut self, sps_scc_extension_flag: u32) -> Self {
            self.sps_scc_extension_flag = sps_scc_extension_flag;
            self
        }

        #[inline]
        pub fn sps_curr_pic_ref_enabled_flag(mut self, sps_curr_pic_ref_enabled_flag: u32) -> Self {
            self.sps_curr_pic_ref_enabled_flag = sps_curr_pic_ref_enabled_flag;
            self
        }

        #[inline]
        pub fn palette_mode_enabled_flag(mut self, palette_mode_enabled_flag: u32) -> Self {
            self.palette_mode_enabled_flag = palette_mode_enabled_flag;
            self
        }

        #[inline]
        pub fn sps_palette_predictor_initializers_present_flag(
            mut self,
            sps_palette_predictor_initializers_present_flag: u32,
        ) -> Self {
            self.sps_palette_predictor_initializers_present_flag =
                sps_palette_predictor_initializers_present_flag;
            self
        }

        #[inline]
        pub fn intra_boundary_filtering_disabled_flag(
            mut self,
            intra_boundary_filtering_disabled_flag: u32,
        ) -> Self {
            self.intra_boundary_filtering_disabled_flag = intra_boundary_filtering_disabled_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265SequenceParameterSet.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265SequenceParameterSet<'a> {
        pub flags: StdVideoH265SpsFlags,
        pub chroma_format_idc: StdVideoH265ChromaFormatIdc,
        pub pic_width_in_luma_samples: u32,
        pub pic_height_in_luma_samples: u32,
        pub sps_video_parameter_set_id: u8,
        pub sps_max_sub_layers_minus1: u8,
        pub sps_seq_parameter_set_id: u8,
        pub bit_depth_luma_minus8: u8,
        pub bit_depth_chroma_minus8: u8,
        pub log2_max_pic_order_cnt_lsb_minus4: u8,
        pub log2_min_luma_coding_block_size_minus3: u8,
        pub log2_diff_max_min_luma_coding_block_size: u8,
        pub log2_min_luma_transform_block_size_minus2: u8,
        pub log2_diff_max_min_luma_transform_block_size: u8,
        pub max_transform_hierarchy_depth_inter: u8,
        pub max_transform_hierarchy_depth_intra: u8,
        pub num_short_term_ref_pic_sets: u8,
        pub num_long_term_ref_pics_sps: u8,
        pub pcm_sample_bit_depth_luma_minus1: u8,
        pub pcm_sample_bit_depth_chroma_minus1: u8,
        pub log2_min_pcm_luma_coding_block_size_minus3: u8,
        pub log2_diff_max_min_pcm_luma_coding_block_size: u8,
        pub reserved1: u8,
        pub reserved2: u8,
        pub palette_max_size: u8,
        pub delta_palette_max_predictor_size: u8,
        pub motion_vector_resolution_control_idc: u8,
        pub sps_num_palette_predictor_initializers_minus1: u8,
        pub conf_win_left_offset: u32,
        pub conf_win_right_offset: u32,
        pub conf_win_top_offset: u32,
        pub conf_win_bottom_offset: u32,
        pub p_profile_tier_level: *const StdVideoH265ProfileTierLevel,
        pub p_dec_pic_buf_mgr: *const StdVideoH265DecPicBufMgr,
        pub p_scaling_lists: *const StdVideoH265ScalingLists,
        pub p_short_term_ref_pic_set: *const StdVideoH265ShortTermRefPicSet,
        pub p_long_term_ref_pics_sps: *const StdVideoH265LongTermRefPicsSps,
        pub p_sequence_parameter_set_vui: *const StdVideoH265SequenceParameterSetVui<'a>,
        pub p_predictor_palette_entries: *const StdVideoH265PredictorPaletteEntries,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoH265SequenceParameterSet<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoH265SequenceParameterSet")
                .field("flags", &self.flags)
                .field("chroma_format_idc", &self.chroma_format_idc)
                .field("pic_width_in_luma_samples", &self.pic_width_in_luma_samples)
                .field(
                    "pic_height_in_luma_samples",
                    &self.pic_height_in_luma_samples,
                )
                .field(
                    "sps_video_parameter_set_id",
                    &self.sps_video_parameter_set_id,
                )
                .field("sps_max_sub_layers_minus1", &self.sps_max_sub_layers_minus1)
                .field("sps_seq_parameter_set_id", &self.sps_seq_parameter_set_id)
                .field("bit_depth_luma_minus8", &self.bit_depth_luma_minus8)
                .field("bit_depth_chroma_minus8", &self.bit_depth_chroma_minus8)
                .field(
                    "log2_max_pic_order_cnt_lsb_minus4",
                    &self.log2_max_pic_order_cnt_lsb_minus4,
                )
                .field(
                    "log2_min_luma_coding_block_size_minus3",
                    &self.log2_min_luma_coding_block_size_minus3,
                )
                .field(
                    "log2_diff_max_min_luma_coding_block_size",
                    &self.log2_diff_max_min_luma_coding_block_size,
                )
                .field(
                    "log2_min_luma_transform_block_size_minus2",
                    &self.log2_min_luma_transform_block_size_minus2,
                )
                .field(
                    "log2_diff_max_min_luma_transform_block_size",
                    &self.log2_diff_max_min_luma_transform_block_size,
                )
                .field(
                    "max_transform_hierarchy_depth_inter",
                    &self.max_transform_hierarchy_depth_inter,
                )
                .field(
                    "max_transform_hierarchy_depth_intra",
                    &self.max_transform_hierarchy_depth_intra,
                )
                .field(
                    "num_short_term_ref_pic_sets",
                    &self.num_short_term_ref_pic_sets,
                )
                .field(
                    "num_long_term_ref_pics_sps",
                    &self.num_long_term_ref_pics_sps,
                )
                .field(
                    "pcm_sample_bit_depth_luma_minus1",
                    &self.pcm_sample_bit_depth_luma_minus1,
                )
                .field(
                    "pcm_sample_bit_depth_chroma_minus1",
                    &self.pcm_sample_bit_depth_chroma_minus1,
                )
                .field(
                    "log2_min_pcm_luma_coding_block_size_minus3",
                    &self.log2_min_pcm_luma_coding_block_size_minus3,
                )
                .field(
                    "log2_diff_max_min_pcm_luma_coding_block_size",
                    &self.log2_diff_max_min_pcm_luma_coding_block_size,
                )
                .field("reserved1", &self.reserved1)
                .field("reserved2", &self.reserved2)
                .field("palette_max_size", &self.palette_max_size)
                .field(
                    "delta_palette_max_predictor_size",
                    &self.delta_palette_max_predictor_size,
                )
                .field(
                    "motion_vector_resolution_control_idc",
                    &self.motion_vector_resolution_control_idc,
                )
                .field(
                    "sps_num_palette_predictor_initializers_minus1",
                    &self.sps_num_palette_predictor_initializers_minus1,
                )
                .field("conf_win_left_offset", &self.conf_win_left_offset)
                .field("conf_win_right_offset", &self.conf_win_right_offset)
                .field("conf_win_top_offset", &self.conf_win_top_offset)
                .field("conf_win_bottom_offset", &self.conf_win_bottom_offset)
                .field("p_profile_tier_level", &self.p_profile_tier_level)
                .field("p_dec_pic_buf_mgr", &self.p_dec_pic_buf_mgr)
                .field("p_scaling_lists", &self.p_scaling_lists)
                .field("p_short_term_ref_pic_set", &self.p_short_term_ref_pic_set)
                .field("p_long_term_ref_pics_sps", &self.p_long_term_ref_pics_sps)
                .field(
                    "p_sequence_parameter_set_vui",
                    &self.p_sequence_parameter_set_vui,
                )
                .field(
                    "p_predictor_palette_entries",
                    &self.p_predictor_palette_entries,
                )
                .finish()
        }
    }

    impl Default for StdVideoH265SequenceParameterSet<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                chroma_format_idc: Default::default(),
                pic_width_in_luma_samples: Default::default(),
                pic_height_in_luma_samples: Default::default(),
                sps_video_parameter_set_id: Default::default(),
                sps_max_sub_layers_minus1: Default::default(),
                sps_seq_parameter_set_id: Default::default(),
                bit_depth_luma_minus8: Default::default(),
                bit_depth_chroma_minus8: Default::default(),
                log2_max_pic_order_cnt_lsb_minus4: Default::default(),
                log2_min_luma_coding_block_size_minus3: Default::default(),
                log2_diff_max_min_luma_coding_block_size: Default::default(),
                log2_min_luma_transform_block_size_minus2: Default::default(),
                log2_diff_max_min_luma_transform_block_size: Default::default(),
                max_transform_hierarchy_depth_inter: Default::default(),
                max_transform_hierarchy_depth_intra: Default::default(),
                num_short_term_ref_pic_sets: Default::default(),
                num_long_term_ref_pics_sps: Default::default(),
                pcm_sample_bit_depth_luma_minus1: Default::default(),
                pcm_sample_bit_depth_chroma_minus1: Default::default(),
                log2_min_pcm_luma_coding_block_size_minus3: Default::default(),
                log2_diff_max_min_pcm_luma_coding_block_size: Default::default(),
                reserved1: Default::default(),
                reserved2: Default::default(),
                palette_max_size: Default::default(),
                delta_palette_max_predictor_size: Default::default(),
                motion_vector_resolution_control_idc: Default::default(),
                sps_num_palette_predictor_initializers_minus1: Default::default(),
                conf_win_left_offset: Default::default(),
                conf_win_right_offset: Default::default(),
                conf_win_top_offset: Default::default(),
                conf_win_bottom_offset: Default::default(),
                p_profile_tier_level: core::ptr::null(),
                p_dec_pic_buf_mgr: core::ptr::null(),
                p_scaling_lists: core::ptr::null(),
                p_short_term_ref_pic_set: core::ptr::null(),
                p_long_term_ref_pics_sps: core::ptr::null(),
                p_sequence_parameter_set_vui: core::ptr::null(),
                p_predictor_palette_entries: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoH265SequenceParameterSet<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoH265SpsFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn chroma_format_idc(mut self, chroma_format_idc: StdVideoH265ChromaFormatIdc) -> Self {
            self.chroma_format_idc = chroma_format_idc;
            self
        }

        #[inline]
        pub fn pic_width_in_luma_samples(mut self, pic_width_in_luma_samples: u32) -> Self {
            self.pic_width_in_luma_samples = pic_width_in_luma_samples;
            self
        }

        #[inline]
        pub fn pic_height_in_luma_samples(mut self, pic_height_in_luma_samples: u32) -> Self {
            self.pic_height_in_luma_samples = pic_height_in_luma_samples;
            self
        }

        #[inline]
        pub fn sps_video_parameter_set_id(mut self, sps_video_parameter_set_id: u8) -> Self {
            self.sps_video_parameter_set_id = sps_video_parameter_set_id;
            self
        }

        #[inline]
        pub fn sps_max_sub_layers_minus1(mut self, sps_max_sub_layers_minus1: u8) -> Self {
            self.sps_max_sub_layers_minus1 = sps_max_sub_layers_minus1;
            self
        }

        #[inline]
        pub fn sps_seq_parameter_set_id(mut self, sps_seq_parameter_set_id: u8) -> Self {
            self.sps_seq_parameter_set_id = sps_seq_parameter_set_id;
            self
        }

        #[inline]
        pub fn bit_depth_luma_minus8(mut self, bit_depth_luma_minus8: u8) -> Self {
            self.bit_depth_luma_minus8 = bit_depth_luma_minus8;
            self
        }

        #[inline]
        pub fn bit_depth_chroma_minus8(mut self, bit_depth_chroma_minus8: u8) -> Self {
            self.bit_depth_chroma_minus8 = bit_depth_chroma_minus8;
            self
        }

        #[inline]
        pub fn log2_max_pic_order_cnt_lsb_minus4(
            mut self,
            log2_max_pic_order_cnt_lsb_minus4: u8,
        ) -> Self {
            self.log2_max_pic_order_cnt_lsb_minus4 = log2_max_pic_order_cnt_lsb_minus4;
            self
        }

        #[inline]
        pub fn log2_min_luma_coding_block_size_minus3(
            mut self,
            log2_min_luma_coding_block_size_minus3: u8,
        ) -> Self {
            self.log2_min_luma_coding_block_size_minus3 = log2_min_luma_coding_block_size_minus3;
            self
        }

        #[inline]
        pub fn log2_diff_max_min_luma_coding_block_size(
            mut self,
            log2_diff_max_min_luma_coding_block_size: u8,
        ) -> Self {
            self.log2_diff_max_min_luma_coding_block_size =
                log2_diff_max_min_luma_coding_block_size;
            self
        }

        #[inline]
        pub fn log2_min_luma_transform_block_size_minus2(
            mut self,
            log2_min_luma_transform_block_size_minus2: u8,
        ) -> Self {
            self.log2_min_luma_transform_block_size_minus2 =
                log2_min_luma_transform_block_size_minus2;
            self
        }

        #[inline]
        pub fn log2_diff_max_min_luma_transform_block_size(
            mut self,
            log2_diff_max_min_luma_transform_block_size: u8,
        ) -> Self {
            self.log2_diff_max_min_luma_transform_block_size =
                log2_diff_max_min_luma_transform_block_size;
            self
        }

        #[inline]
        pub fn max_transform_hierarchy_depth_inter(
            mut self,
            max_transform_hierarchy_depth_inter: u8,
        ) -> Self {
            self.max_transform_hierarchy_depth_inter = max_transform_hierarchy_depth_inter;
            self
        }

        #[inline]
        pub fn max_transform_hierarchy_depth_intra(
            mut self,
            max_transform_hierarchy_depth_intra: u8,
        ) -> Self {
            self.max_transform_hierarchy_depth_intra = max_transform_hierarchy_depth_intra;
            self
        }

        #[inline]
        pub fn short_term_ref_pic_set(
            mut self,
            short_term_ref_pic_set: &'a [StdVideoH265ShortTermRefPicSet],
        ) -> Self {
            self.num_short_term_ref_pic_sets = short_term_ref_pic_set.len().try_into().unwrap();
            self.p_short_term_ref_pic_set = short_term_ref_pic_set.as_ptr();
            self
        }

        #[inline]
        pub fn num_long_term_ref_pics_sps(mut self, num_long_term_ref_pics_sps: u8) -> Self {
            self.num_long_term_ref_pics_sps = num_long_term_ref_pics_sps;
            self
        }

        #[inline]
        pub fn pcm_sample_bit_depth_luma_minus1(
            mut self,
            pcm_sample_bit_depth_luma_minus1: u8,
        ) -> Self {
            self.pcm_sample_bit_depth_luma_minus1 = pcm_sample_bit_depth_luma_minus1;
            self
        }

        #[inline]
        pub fn pcm_sample_bit_depth_chroma_minus1(
            mut self,
            pcm_sample_bit_depth_chroma_minus1: u8,
        ) -> Self {
            self.pcm_sample_bit_depth_chroma_minus1 = pcm_sample_bit_depth_chroma_minus1;
            self
        }

        #[inline]
        pub fn log2_min_pcm_luma_coding_block_size_minus3(
            mut self,
            log2_min_pcm_luma_coding_block_size_minus3: u8,
        ) -> Self {
            self.log2_min_pcm_luma_coding_block_size_minus3 =
                log2_min_pcm_luma_coding_block_size_minus3;
            self
        }

        #[inline]
        pub fn log2_diff_max_min_pcm_luma_coding_block_size(
            mut self,
            log2_diff_max_min_pcm_luma_coding_block_size: u8,
        ) -> Self {
            self.log2_diff_max_min_pcm_luma_coding_block_size =
                log2_diff_max_min_pcm_luma_coding_block_size;
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
        pub fn palette_max_size(mut self, palette_max_size: u8) -> Self {
            self.palette_max_size = palette_max_size;
            self
        }

        #[inline]
        pub fn delta_palette_max_predictor_size(
            mut self,
            delta_palette_max_predictor_size: u8,
        ) -> Self {
            self.delta_palette_max_predictor_size = delta_palette_max_predictor_size;
            self
        }

        #[inline]
        pub fn motion_vector_resolution_control_idc(
            mut self,
            motion_vector_resolution_control_idc: u8,
        ) -> Self {
            self.motion_vector_resolution_control_idc = motion_vector_resolution_control_idc;
            self
        }

        #[inline]
        pub fn sps_num_palette_predictor_initializers_minus1(
            mut self,
            sps_num_palette_predictor_initializers_minus1: u8,
        ) -> Self {
            self.sps_num_palette_predictor_initializers_minus1 =
                sps_num_palette_predictor_initializers_minus1;
            self
        }

        #[inline]
        pub fn conf_win_left_offset(mut self, conf_win_left_offset: u32) -> Self {
            self.conf_win_left_offset = conf_win_left_offset;
            self
        }

        #[inline]
        pub fn conf_win_right_offset(mut self, conf_win_right_offset: u32) -> Self {
            self.conf_win_right_offset = conf_win_right_offset;
            self
        }

        #[inline]
        pub fn conf_win_top_offset(mut self, conf_win_top_offset: u32) -> Self {
            self.conf_win_top_offset = conf_win_top_offset;
            self
        }

        #[inline]
        pub fn conf_win_bottom_offset(mut self, conf_win_bottom_offset: u32) -> Self {
            self.conf_win_bottom_offset = conf_win_bottom_offset;
            self
        }

        #[inline]
        pub fn profile_tier_level(
            mut self,
            profile_tier_level: &'a StdVideoH265ProfileTierLevel,
        ) -> Self {
            self.p_profile_tier_level = profile_tier_level;
            self
        }

        #[inline]
        pub fn dec_pic_buf_mgr(mut self, dec_pic_buf_mgr: &'a StdVideoH265DecPicBufMgr) -> Self {
            self.p_dec_pic_buf_mgr = dec_pic_buf_mgr;
            self
        }

        #[inline]
        pub fn scaling_lists(mut self, scaling_lists: &'a StdVideoH265ScalingLists) -> Self {
            self.p_scaling_lists = scaling_lists;
            self
        }

        #[inline]
        pub fn long_term_ref_pics_sps(
            mut self,
            long_term_ref_pics_sps: &'a StdVideoH265LongTermRefPicsSps,
        ) -> Self {
            self.p_long_term_ref_pics_sps = long_term_ref_pics_sps;
            self
        }

        #[inline]
        pub fn sequence_parameter_set_vui(
            mut self,
            sequence_parameter_set_vui: &'a StdVideoH265SequenceParameterSetVui<'a>,
        ) -> Self {
            self.p_sequence_parameter_set_vui = sequence_parameter_set_vui;
            self
        }

        #[inline]
        pub fn predictor_palette_entries(
            mut self,
            predictor_palette_entries: &'a StdVideoH265PredictorPaletteEntries,
        ) -> Self {
            self.p_predictor_palette_entries = predictor_palette_entries;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265PpsFlags.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct StdVideoH265PpsFlags {
        pub dependent_slice_segments_enabled_flag: u32,
        pub output_flag_present_flag: u32,
        pub sign_data_hiding_enabled_flag: u32,
        pub cabac_init_present_flag: u32,
        pub constrained_intra_pred_flag: u32,
        pub transform_skip_enabled_flag: u32,
        pub cu_qp_delta_enabled_flag: u32,
        pub pps_slice_chroma_qp_offsets_present_flag: u32,
        pub weighted_pred_flag: u32,
        pub weighted_bipred_flag: u32,
        pub transquant_bypass_enabled_flag: u32,
        pub tiles_enabled_flag: u32,
        pub entropy_coding_sync_enabled_flag: u32,
        pub uniform_spacing_flag: u32,
        pub loop_filter_across_tiles_enabled_flag: u32,
        pub pps_loop_filter_across_slices_enabled_flag: u32,
        pub deblocking_filter_control_present_flag: u32,
        pub deblocking_filter_override_enabled_flag: u32,
        pub pps_deblocking_filter_disabled_flag: u32,
        pub pps_scaling_list_data_present_flag: u32,
        pub lists_modification_present_flag: u32,
        pub slice_segment_header_extension_present_flag: u32,
        pub pps_extension_present_flag: u32,
        pub cross_component_prediction_enabled_flag: u32,
        pub chroma_qp_offset_list_enabled_flag: u32,
        pub pps_curr_pic_ref_enabled_flag: u32,
        pub residual_adaptive_colour_transform_enabled_flag: u32,
        pub pps_slice_act_qp_offsets_present_flag: u32,
        pub pps_palette_predictor_initializers_present_flag: u32,
        pub monochrome_palette_flag: u32,
        pub pps_range_extension_flag: u32,
    }

    impl StdVideoH265PpsFlags {
        #[inline]
        pub fn dependent_slice_segments_enabled_flag(
            mut self,
            dependent_slice_segments_enabled_flag: u32,
        ) -> Self {
            self.dependent_slice_segments_enabled_flag = dependent_slice_segments_enabled_flag;
            self
        }

        #[inline]
        pub fn output_flag_present_flag(mut self, output_flag_present_flag: u32) -> Self {
            self.output_flag_present_flag = output_flag_present_flag;
            self
        }

        #[inline]
        pub fn sign_data_hiding_enabled_flag(mut self, sign_data_hiding_enabled_flag: u32) -> Self {
            self.sign_data_hiding_enabled_flag = sign_data_hiding_enabled_flag;
            self
        }

        #[inline]
        pub fn cabac_init_present_flag(mut self, cabac_init_present_flag: u32) -> Self {
            self.cabac_init_present_flag = cabac_init_present_flag;
            self
        }

        #[inline]
        pub fn constrained_intra_pred_flag(mut self, constrained_intra_pred_flag: u32) -> Self {
            self.constrained_intra_pred_flag = constrained_intra_pred_flag;
            self
        }

        #[inline]
        pub fn transform_skip_enabled_flag(mut self, transform_skip_enabled_flag: u32) -> Self {
            self.transform_skip_enabled_flag = transform_skip_enabled_flag;
            self
        }

        #[inline]
        pub fn cu_qp_delta_enabled_flag(mut self, cu_qp_delta_enabled_flag: u32) -> Self {
            self.cu_qp_delta_enabled_flag = cu_qp_delta_enabled_flag;
            self
        }

        #[inline]
        pub fn pps_slice_chroma_qp_offsets_present_flag(
            mut self,
            pps_slice_chroma_qp_offsets_present_flag: u32,
        ) -> Self {
            self.pps_slice_chroma_qp_offsets_present_flag =
                pps_slice_chroma_qp_offsets_present_flag;
            self
        }

        #[inline]
        pub fn weighted_pred_flag(mut self, weighted_pred_flag: u32) -> Self {
            self.weighted_pred_flag = weighted_pred_flag;
            self
        }

        #[inline]
        pub fn weighted_bipred_flag(mut self, weighted_bipred_flag: u32) -> Self {
            self.weighted_bipred_flag = weighted_bipred_flag;
            self
        }

        #[inline]
        pub fn transquant_bypass_enabled_flag(
            mut self,
            transquant_bypass_enabled_flag: u32,
        ) -> Self {
            self.transquant_bypass_enabled_flag = transquant_bypass_enabled_flag;
            self
        }

        #[inline]
        pub fn tiles_enabled_flag(mut self, tiles_enabled_flag: u32) -> Self {
            self.tiles_enabled_flag = tiles_enabled_flag;
            self
        }

        #[inline]
        pub fn entropy_coding_sync_enabled_flag(
            mut self,
            entropy_coding_sync_enabled_flag: u32,
        ) -> Self {
            self.entropy_coding_sync_enabled_flag = entropy_coding_sync_enabled_flag;
            self
        }

        #[inline]
        pub fn uniform_spacing_flag(mut self, uniform_spacing_flag: u32) -> Self {
            self.uniform_spacing_flag = uniform_spacing_flag;
            self
        }

        #[inline]
        pub fn loop_filter_across_tiles_enabled_flag(
            mut self,
            loop_filter_across_tiles_enabled_flag: u32,
        ) -> Self {
            self.loop_filter_across_tiles_enabled_flag = loop_filter_across_tiles_enabled_flag;
            self
        }

        #[inline]
        pub fn pps_loop_filter_across_slices_enabled_flag(
            mut self,
            pps_loop_filter_across_slices_enabled_flag: u32,
        ) -> Self {
            self.pps_loop_filter_across_slices_enabled_flag =
                pps_loop_filter_across_slices_enabled_flag;
            self
        }

        #[inline]
        pub fn deblocking_filter_control_present_flag(
            mut self,
            deblocking_filter_control_present_flag: u32,
        ) -> Self {
            self.deblocking_filter_control_present_flag = deblocking_filter_control_present_flag;
            self
        }

        #[inline]
        pub fn deblocking_filter_override_enabled_flag(
            mut self,
            deblocking_filter_override_enabled_flag: u32,
        ) -> Self {
            self.deblocking_filter_override_enabled_flag = deblocking_filter_override_enabled_flag;
            self
        }

        #[inline]
        pub fn pps_deblocking_filter_disabled_flag(
            mut self,
            pps_deblocking_filter_disabled_flag: u32,
        ) -> Self {
            self.pps_deblocking_filter_disabled_flag = pps_deblocking_filter_disabled_flag;
            self
        }

        #[inline]
        pub fn pps_scaling_list_data_present_flag(
            mut self,
            pps_scaling_list_data_present_flag: u32,
        ) -> Self {
            self.pps_scaling_list_data_present_flag = pps_scaling_list_data_present_flag;
            self
        }

        #[inline]
        pub fn lists_modification_present_flag(
            mut self,
            lists_modification_present_flag: u32,
        ) -> Self {
            self.lists_modification_present_flag = lists_modification_present_flag;
            self
        }

        #[inline]
        pub fn slice_segment_header_extension_present_flag(
            mut self,
            slice_segment_header_extension_present_flag: u32,
        ) -> Self {
            self.slice_segment_header_extension_present_flag =
                slice_segment_header_extension_present_flag;
            self
        }

        #[inline]
        pub fn pps_extension_present_flag(mut self, pps_extension_present_flag: u32) -> Self {
            self.pps_extension_present_flag = pps_extension_present_flag;
            self
        }

        #[inline]
        pub fn cross_component_prediction_enabled_flag(
            mut self,
            cross_component_prediction_enabled_flag: u32,
        ) -> Self {
            self.cross_component_prediction_enabled_flag = cross_component_prediction_enabled_flag;
            self
        }

        #[inline]
        pub fn chroma_qp_offset_list_enabled_flag(
            mut self,
            chroma_qp_offset_list_enabled_flag: u32,
        ) -> Self {
            self.chroma_qp_offset_list_enabled_flag = chroma_qp_offset_list_enabled_flag;
            self
        }

        #[inline]
        pub fn pps_curr_pic_ref_enabled_flag(mut self, pps_curr_pic_ref_enabled_flag: u32) -> Self {
            self.pps_curr_pic_ref_enabled_flag = pps_curr_pic_ref_enabled_flag;
            self
        }

        #[inline]
        pub fn residual_adaptive_colour_transform_enabled_flag(
            mut self,
            residual_adaptive_colour_transform_enabled_flag: u32,
        ) -> Self {
            self.residual_adaptive_colour_transform_enabled_flag =
                residual_adaptive_colour_transform_enabled_flag;
            self
        }

        #[inline]
        pub fn pps_slice_act_qp_offsets_present_flag(
            mut self,
            pps_slice_act_qp_offsets_present_flag: u32,
        ) -> Self {
            self.pps_slice_act_qp_offsets_present_flag = pps_slice_act_qp_offsets_present_flag;
            self
        }

        #[inline]
        pub fn pps_palette_predictor_initializers_present_flag(
            mut self,
            pps_palette_predictor_initializers_present_flag: u32,
        ) -> Self {
            self.pps_palette_predictor_initializers_present_flag =
                pps_palette_predictor_initializers_present_flag;
            self
        }

        #[inline]
        pub fn monochrome_palette_flag(mut self, monochrome_palette_flag: u32) -> Self {
            self.monochrome_palette_flag = monochrome_palette_flag;
            self
        }

        #[inline]
        pub fn pps_range_extension_flag(mut self, pps_range_extension_flag: u32) -> Self {
            self.pps_range_extension_flag = pps_range_extension_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265PictureParameterSet.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct StdVideoH265PictureParameterSet<'a> {
        pub flags: StdVideoH265PpsFlags,
        pub pps_pic_parameter_set_id: u8,
        pub pps_seq_parameter_set_id: u8,
        pub sps_video_parameter_set_id: u8,
        pub num_extra_slice_header_bits: u8,
        pub num_ref_idx_l0_default_active_minus1: u8,
        pub num_ref_idx_l1_default_active_minus1: u8,
        pub init_qp_minus26: i8,
        pub diff_cu_qp_delta_depth: u8,
        pub pps_cb_qp_offset: i8,
        pub pps_cr_qp_offset: i8,
        pub pps_beta_offset_div2: i8,
        pub pps_tc_offset_div2: i8,
        pub log2_parallel_merge_level_minus2: u8,
        pub log2_max_transform_skip_block_size_minus2: u8,
        pub diff_cu_chroma_qp_offset_depth: u8,
        pub chroma_qp_offset_list_len_minus1: u8,
        pub cb_qp_offset_list: [i8; STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
        pub cr_qp_offset_list: [i8; STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
        pub log2_sao_offset_scale_luma: u8,
        pub log2_sao_offset_scale_chroma: u8,
        pub pps_act_y_qp_offset_plus5: i8,
        pub pps_act_cb_qp_offset_plus5: i8,
        pub pps_act_cr_qp_offset_plus3: i8,
        pub pps_num_palette_predictor_initializers: u8,
        pub luma_bit_depth_entry_minus8: u8,
        pub chroma_bit_depth_entry_minus8: u8,
        pub num_tile_columns_minus1: u8,
        pub num_tile_rows_minus1: u8,
        pub reserved1: u8,
        pub reserved2: u8,
        pub column_width_minus1:
            [u16; STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE as usize],
        pub row_height_minus1: [u16; STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE as usize],
        pub reserved3: u32,
        pub p_scaling_lists: *const StdVideoH265ScalingLists,
        pub p_predictor_palette_entries: *const StdVideoH265PredictorPaletteEntries,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for StdVideoH265PictureParameterSet<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoH265PictureParameterSet")
                .field("flags", &self.flags)
                .field("pps_pic_parameter_set_id", &self.pps_pic_parameter_set_id)
                .field("pps_seq_parameter_set_id", &self.pps_seq_parameter_set_id)
                .field(
                    "sps_video_parameter_set_id",
                    &self.sps_video_parameter_set_id,
                )
                .field(
                    "num_extra_slice_header_bits",
                    &self.num_extra_slice_header_bits,
                )
                .field(
                    "num_ref_idx_l0_default_active_minus1",
                    &self.num_ref_idx_l0_default_active_minus1,
                )
                .field(
                    "num_ref_idx_l1_default_active_minus1",
                    &self.num_ref_idx_l1_default_active_minus1,
                )
                .field("init_qp_minus26", &self.init_qp_minus26)
                .field("diff_cu_qp_delta_depth", &self.diff_cu_qp_delta_depth)
                .field("pps_cb_qp_offset", &self.pps_cb_qp_offset)
                .field("pps_cr_qp_offset", &self.pps_cr_qp_offset)
                .field("pps_beta_offset_div2", &self.pps_beta_offset_div2)
                .field("pps_tc_offset_div2", &self.pps_tc_offset_div2)
                .field(
                    "log2_parallel_merge_level_minus2",
                    &self.log2_parallel_merge_level_minus2,
                )
                .field(
                    "log2_max_transform_skip_block_size_minus2",
                    &self.log2_max_transform_skip_block_size_minus2,
                )
                .field(
                    "diff_cu_chroma_qp_offset_depth",
                    &self.diff_cu_chroma_qp_offset_depth,
                )
                .field(
                    "chroma_qp_offset_list_len_minus1",
                    &self.chroma_qp_offset_list_len_minus1,
                )
                .field("cb_qp_offset_list", &self.cb_qp_offset_list)
                .field("cr_qp_offset_list", &self.cr_qp_offset_list)
                .field(
                    "log2_sao_offset_scale_luma",
                    &self.log2_sao_offset_scale_luma,
                )
                .field(
                    "log2_sao_offset_scale_chroma",
                    &self.log2_sao_offset_scale_chroma,
                )
                .field("pps_act_y_qp_offset_plus5", &self.pps_act_y_qp_offset_plus5)
                .field(
                    "pps_act_cb_qp_offset_plus5",
                    &self.pps_act_cb_qp_offset_plus5,
                )
                .field(
                    "pps_act_cr_qp_offset_plus3",
                    &self.pps_act_cr_qp_offset_plus3,
                )
                .field(
                    "pps_num_palette_predictor_initializers",
                    &self.pps_num_palette_predictor_initializers,
                )
                .field(
                    "luma_bit_depth_entry_minus8",
                    &self.luma_bit_depth_entry_minus8,
                )
                .field(
                    "chroma_bit_depth_entry_minus8",
                    &self.chroma_bit_depth_entry_minus8,
                )
                .field("num_tile_columns_minus1", &self.num_tile_columns_minus1)
                .field("num_tile_rows_minus1", &self.num_tile_rows_minus1)
                .field("reserved1", &self.reserved1)
                .field("reserved2", &self.reserved2)
                .field("column_width_minus1", &self.column_width_minus1)
                .field("row_height_minus1", &self.row_height_minus1)
                .field("reserved3", &self.reserved3)
                .field("p_scaling_lists", &self.p_scaling_lists)
                .field(
                    "p_predictor_palette_entries",
                    &self.p_predictor_palette_entries,
                )
                .finish()
        }
    }

    impl Default for StdVideoH265PictureParameterSet<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                pps_pic_parameter_set_id: Default::default(),
                pps_seq_parameter_set_id: Default::default(),
                sps_video_parameter_set_id: Default::default(),
                num_extra_slice_header_bits: Default::default(),
                num_ref_idx_l0_default_active_minus1: Default::default(),
                num_ref_idx_l1_default_active_minus1: Default::default(),
                init_qp_minus26: Default::default(),
                diff_cu_qp_delta_depth: Default::default(),
                pps_cb_qp_offset: Default::default(),
                pps_cr_qp_offset: Default::default(),
                pps_beta_offset_div2: Default::default(),
                pps_tc_offset_div2: Default::default(),
                log2_parallel_merge_level_minus2: Default::default(),
                log2_max_transform_skip_block_size_minus2: Default::default(),
                diff_cu_chroma_qp_offset_depth: Default::default(),
                chroma_qp_offset_list_len_minus1: Default::default(),
                cb_qp_offset_list: [Default::default(); _],
                cr_qp_offset_list: [Default::default(); _],
                log2_sao_offset_scale_luma: Default::default(),
                log2_sao_offset_scale_chroma: Default::default(),
                pps_act_y_qp_offset_plus5: Default::default(),
                pps_act_cb_qp_offset_plus5: Default::default(),
                pps_act_cr_qp_offset_plus3: Default::default(),
                pps_num_palette_predictor_initializers: Default::default(),
                luma_bit_depth_entry_minus8: Default::default(),
                chroma_bit_depth_entry_minus8: Default::default(),
                num_tile_columns_minus1: Default::default(),
                num_tile_rows_minus1: Default::default(),
                reserved1: Default::default(),
                reserved2: Default::default(),
                column_width_minus1: [Default::default(); _],
                row_height_minus1: [Default::default(); _],
                reserved3: Default::default(),
                p_scaling_lists: core::ptr::null(),
                p_predictor_palette_entries: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoH265PictureParameterSet<'a> {
        #[inline]
        pub fn flags(mut self, flags: StdVideoH265PpsFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn pps_pic_parameter_set_id(mut self, pps_pic_parameter_set_id: u8) -> Self {
            self.pps_pic_parameter_set_id = pps_pic_parameter_set_id;
            self
        }

        #[inline]
        pub fn pps_seq_parameter_set_id(mut self, pps_seq_parameter_set_id: u8) -> Self {
            self.pps_seq_parameter_set_id = pps_seq_parameter_set_id;
            self
        }

        #[inline]
        pub fn sps_video_parameter_set_id(mut self, sps_video_parameter_set_id: u8) -> Self {
            self.sps_video_parameter_set_id = sps_video_parameter_set_id;
            self
        }

        #[inline]
        pub fn num_extra_slice_header_bits(mut self, num_extra_slice_header_bits: u8) -> Self {
            self.num_extra_slice_header_bits = num_extra_slice_header_bits;
            self
        }

        #[inline]
        pub fn num_ref_idx_l0_default_active_minus1(
            mut self,
            num_ref_idx_l0_default_active_minus1: u8,
        ) -> Self {
            self.num_ref_idx_l0_default_active_minus1 = num_ref_idx_l0_default_active_minus1;
            self
        }

        #[inline]
        pub fn num_ref_idx_l1_default_active_minus1(
            mut self,
            num_ref_idx_l1_default_active_minus1: u8,
        ) -> Self {
            self.num_ref_idx_l1_default_active_minus1 = num_ref_idx_l1_default_active_minus1;
            self
        }

        #[inline]
        pub fn init_qp_minus26(mut self, init_qp_minus26: i8) -> Self {
            self.init_qp_minus26 = init_qp_minus26;
            self
        }

        #[inline]
        pub fn diff_cu_qp_delta_depth(mut self, diff_cu_qp_delta_depth: u8) -> Self {
            self.diff_cu_qp_delta_depth = diff_cu_qp_delta_depth;
            self
        }

        #[inline]
        pub fn pps_cb_qp_offset(mut self, pps_cb_qp_offset: i8) -> Self {
            self.pps_cb_qp_offset = pps_cb_qp_offset;
            self
        }

        #[inline]
        pub fn pps_cr_qp_offset(mut self, pps_cr_qp_offset: i8) -> Self {
            self.pps_cr_qp_offset = pps_cr_qp_offset;
            self
        }

        #[inline]
        pub fn pps_beta_offset_div2(mut self, pps_beta_offset_div2: i8) -> Self {
            self.pps_beta_offset_div2 = pps_beta_offset_div2;
            self
        }

        #[inline]
        pub fn pps_tc_offset_div2(mut self, pps_tc_offset_div2: i8) -> Self {
            self.pps_tc_offset_div2 = pps_tc_offset_div2;
            self
        }

        #[inline]
        pub fn log2_parallel_merge_level_minus2(
            mut self,
            log2_parallel_merge_level_minus2: u8,
        ) -> Self {
            self.log2_parallel_merge_level_minus2 = log2_parallel_merge_level_minus2;
            self
        }

        #[inline]
        pub fn log2_max_transform_skip_block_size_minus2(
            mut self,
            log2_max_transform_skip_block_size_minus2: u8,
        ) -> Self {
            self.log2_max_transform_skip_block_size_minus2 =
                log2_max_transform_skip_block_size_minus2;
            self
        }

        #[inline]
        pub fn diff_cu_chroma_qp_offset_depth(
            mut self,
            diff_cu_chroma_qp_offset_depth: u8,
        ) -> Self {
            self.diff_cu_chroma_qp_offset_depth = diff_cu_chroma_qp_offset_depth;
            self
        }

        #[inline]
        pub fn chroma_qp_offset_list_len_minus1(
            mut self,
            chroma_qp_offset_list_len_minus1: u8,
        ) -> Self {
            self.chroma_qp_offset_list_len_minus1 = chroma_qp_offset_list_len_minus1;
            self
        }

        #[inline]
        pub fn cb_qp_offset_list(
            mut self,
            cb_qp_offset_list: [i8; STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
        ) -> Self {
            self.cb_qp_offset_list = cb_qp_offset_list;
            self
        }

        #[inline]
        pub fn cr_qp_offset_list(
            mut self,
            cr_qp_offset_list: [i8; STD_VIDEO_H265_CHROMA_QP_OFFSET_LIST_SIZE as usize],
        ) -> Self {
            self.cr_qp_offset_list = cr_qp_offset_list;
            self
        }

        #[inline]
        pub fn log2_sao_offset_scale_luma(mut self, log2_sao_offset_scale_luma: u8) -> Self {
            self.log2_sao_offset_scale_luma = log2_sao_offset_scale_luma;
            self
        }

        #[inline]
        pub fn log2_sao_offset_scale_chroma(mut self, log2_sao_offset_scale_chroma: u8) -> Self {
            self.log2_sao_offset_scale_chroma = log2_sao_offset_scale_chroma;
            self
        }

        #[inline]
        pub fn pps_act_y_qp_offset_plus5(mut self, pps_act_y_qp_offset_plus5: i8) -> Self {
            self.pps_act_y_qp_offset_plus5 = pps_act_y_qp_offset_plus5;
            self
        }

        #[inline]
        pub fn pps_act_cb_qp_offset_plus5(mut self, pps_act_cb_qp_offset_plus5: i8) -> Self {
            self.pps_act_cb_qp_offset_plus5 = pps_act_cb_qp_offset_plus5;
            self
        }

        #[inline]
        pub fn pps_act_cr_qp_offset_plus3(mut self, pps_act_cr_qp_offset_plus3: i8) -> Self {
            self.pps_act_cr_qp_offset_plus3 = pps_act_cr_qp_offset_plus3;
            self
        }

        #[inline]
        pub fn pps_num_palette_predictor_initializers(
            mut self,
            pps_num_palette_predictor_initializers: u8,
        ) -> Self {
            self.pps_num_palette_predictor_initializers = pps_num_palette_predictor_initializers;
            self
        }

        #[inline]
        pub fn luma_bit_depth_entry_minus8(mut self, luma_bit_depth_entry_minus8: u8) -> Self {
            self.luma_bit_depth_entry_minus8 = luma_bit_depth_entry_minus8;
            self
        }

        #[inline]
        pub fn chroma_bit_depth_entry_minus8(mut self, chroma_bit_depth_entry_minus8: u8) -> Self {
            self.chroma_bit_depth_entry_minus8 = chroma_bit_depth_entry_minus8;
            self
        }

        #[inline]
        pub fn num_tile_columns_minus1(mut self, num_tile_columns_minus1: u8) -> Self {
            self.num_tile_columns_minus1 = num_tile_columns_minus1;
            self
        }

        #[inline]
        pub fn num_tile_rows_minus1(mut self, num_tile_rows_minus1: u8) -> Self {
            self.num_tile_rows_minus1 = num_tile_rows_minus1;
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
        pub fn column_width_minus1(
            mut self,
            column_width_minus1: [u16; STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_COLS_LIST_SIZE
                as usize],
        ) -> Self {
            self.column_width_minus1 = column_width_minus1;
            self
        }

        #[inline]
        pub fn row_height_minus1(
            mut self,
            row_height_minus1: [u16; STD_VIDEO_H265_CHROMA_QP_OFFSET_TILE_ROWS_LIST_SIZE as usize],
        ) -> Self {
            self.row_height_minus1 = row_height_minus1;
            self
        }

        #[inline]
        pub fn reserved3(mut self, reserved3: u32) -> Self {
            self.reserved3 = reserved3;
            self
        }

        #[inline]
        pub fn scaling_lists(mut self, scaling_lists: &'a StdVideoH265ScalingLists) -> Self {
            self.p_scaling_lists = scaling_lists;
            self
        }

        #[inline]
        pub fn predictor_palette_entries(
            mut self,
            predictor_palette_entries: &'a StdVideoH265PredictorPaletteEntries,
        ) -> Self {
            self.p_predictor_palette_entries = predictor_palette_entries;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265ChromaFormatIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH265ChromaFormatIdc(i32);

    impl StdVideoH265ChromaFormatIdc {
        pub const MONOCHROME: Self = Self(0);
        pub const _420: Self = Self(1);
        pub const _422: Self = Self(2);
        pub const _444: Self = Self(3);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH265ChromaFormatIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MONOCHROME => Some("MONOCHROME"),
                Self::_420 => Some("_420"),
                Self::_422 => Some("_422"),
                Self::_444 => Some("_444"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265ProfileIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH265ProfileIdc(i32);

    impl StdVideoH265ProfileIdc {
        pub const MAIN: Self = Self(1);
        pub const MAIN_10: Self = Self(2);
        pub const MAIN_STILL_PICTURE: Self = Self(3);
        pub const FORMAT_RANGE_EXTENSIONS: Self = Self(4);
        pub const SCC_EXTENSIONS: Self = Self(9);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH265ProfileIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::MAIN => Some("MAIN"),
                Self::MAIN_10 => Some("MAIN_10"),
                Self::MAIN_STILL_PICTURE => Some("MAIN_STILL_PICTURE"),
                Self::FORMAT_RANGE_EXTENSIONS => Some("FORMAT_RANGE_EXTENSIONS"),
                Self::SCC_EXTENSIONS => Some("SCC_EXTENSIONS"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265LevelIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH265LevelIdc(i32);

    impl StdVideoH265LevelIdc {
        pub const _1_0: Self = Self(0);
        pub const _2_0: Self = Self(1);
        pub const _2_1: Self = Self(2);
        pub const _3_0: Self = Self(3);
        pub const _3_1: Self = Self(4);
        pub const _4_0: Self = Self(5);
        pub const _4_1: Self = Self(6);
        pub const _5_0: Self = Self(7);
        pub const _5_1: Self = Self(8);
        pub const _5_2: Self = Self(9);
        pub const _6_0: Self = Self(10);
        pub const _6_1: Self = Self(11);
        pub const _6_2: Self = Self(12);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH265LevelIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1_0 => Some("_1_0"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265SliceType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH265SliceType(i32);

    impl StdVideoH265SliceType {
        pub const B: Self = Self(0);
        pub const P: Self = Self(1);
        pub const I: Self = Self(2);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH265SliceType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::B => Some("B"),
                Self::P => Some("P"),
                Self::I => Some("I"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265PictureType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH265PictureType(i32);

    impl StdVideoH265PictureType {
        pub const P: Self = Self(0);
        pub const B: Self = Self(1);
        pub const I: Self = Self(2);
        pub const IDR: Self = Self(3);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH265PictureType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::P => Some("P"),
                Self::B => Some("B"),
                Self::I => Some("I"),
                Self::IDR => Some("IDR"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH265AspectRatioIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH265AspectRatioIdc(i32);

    impl StdVideoH265AspectRatioIdc {
        pub const UNSPECIFIED: Self = Self(0);
        pub const SQUARE: Self = Self(1);
        pub const _12_11: Self = Self(2);
        pub const _10_11: Self = Self(3);
        pub const _16_11: Self = Self(4);
        pub const _40_33: Self = Self(5);
        pub const _24_11: Self = Self(6);
        pub const _20_11: Self = Self(7);
        pub const _32_11: Self = Self(8);
        pub const _80_33: Self = Self(9);
        pub const _18_11: Self = Self(10);
        pub const _15_11: Self = Self(11);
        pub const _64_33: Self = Self(12);
        pub const _160_99: Self = Self(13);
        pub const _4_3: Self = Self(14);
        pub const _3_2: Self = Self(15);
        pub const _2_1: Self = Self(16);
        pub const EXTENDED_SAR: Self = Self(255);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH265AspectRatioIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNSPECIFIED => Some("UNSPECIFIED"),
                Self::SQUARE => Some("SQUARE"),
                Self::_12_11 => Some("_12_11"),
                Self::_10_11 => Some("_10_11"),
                Self::_16_11 => Some("_16_11"),
                Self::_40_33 => Some("_40_33"),
                Self::_24_11 => Some("_24_11"),
                Self::_20_11 => Some("_20_11"),
                Self::_32_11 => Some("_32_11"),
                Self::_80_33 => Some("_80_33"),
                Self::_18_11 => Some("_18_11"),
                Self::_15_11 => Some("_15_11"),
                Self::_64_33 => Some("_64_33"),
                Self::_160_99 => Some("_160_99"),
                Self::_4_3 => Some("_4_3"),
                Self::_3_2 => Some("_3_2"),
                Self::_2_1 => Some("_2_1"),
                Self::EXTENDED_SAR => Some("EXTENDED_SAR"),
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
