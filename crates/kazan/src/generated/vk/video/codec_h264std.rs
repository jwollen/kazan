#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"vulkan_video_codec_h264std";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    pub const STD_VIDEO_H264_CPB_CNT_LIST_SIZE: u32 = 32;
    pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS: u32 = 6;
    pub const STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS: u32 = 16;
    pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS: u32 = 6;
    pub const STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS: u32 = 64;
    pub const STD_VIDEO_H264_MAX_NUM_LIST_REF: u32 = 32;
    pub const STD_VIDEO_H264_MAX_CHROMA_PLANES: u32 = 2;
    pub const STD_VIDEO_H264_NO_REFERENCE_PICTURE: u8 = 0xF;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264SpsVuiFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct StdVideoH264SpsVuiFlags {
        pub aspect_ratio_info_present_flag: u32,
        pub overscan_info_present_flag: u32,
        pub overscan_appropriate_flag: u32,
        pub video_signal_type_present_flag: u32,
        pub video_full_range_flag: u32,
        pub color_description_present_flag: u32,
        pub chroma_loc_info_present_flag: u32,
        pub timing_info_present_flag: u32,
        pub fixed_frame_rate_flag: u32,
        pub bitstream_restriction_flag: u32,
        pub nal_hrd_parameters_present_flag: u32,
        pub vcl_hrd_parameters_present_flag: u32,
    }

    impl StdVideoH264SpsVuiFlags {
        pub fn aspect_ratio_info_present_flag(
            mut self,
            aspect_ratio_info_present_flag: u32,
        ) -> Self {
            self.aspect_ratio_info_present_flag = aspect_ratio_info_present_flag;
            self
        }

        pub fn overscan_info_present_flag(mut self, overscan_info_present_flag: u32) -> Self {
            self.overscan_info_present_flag = overscan_info_present_flag;
            self
        }

        pub fn overscan_appropriate_flag(mut self, overscan_appropriate_flag: u32) -> Self {
            self.overscan_appropriate_flag = overscan_appropriate_flag;
            self
        }

        pub fn video_signal_type_present_flag(
            mut self,
            video_signal_type_present_flag: u32,
        ) -> Self {
            self.video_signal_type_present_flag = video_signal_type_present_flag;
            self
        }

        pub fn video_full_range_flag(mut self, video_full_range_flag: u32) -> Self {
            self.video_full_range_flag = video_full_range_flag;
            self
        }

        pub fn color_description_present_flag(
            mut self,
            color_description_present_flag: u32,
        ) -> Self {
            self.color_description_present_flag = color_description_present_flag;
            self
        }

        pub fn chroma_loc_info_present_flag(mut self, chroma_loc_info_present_flag: u32) -> Self {
            self.chroma_loc_info_present_flag = chroma_loc_info_present_flag;
            self
        }

        pub fn timing_info_present_flag(mut self, timing_info_present_flag: u32) -> Self {
            self.timing_info_present_flag = timing_info_present_flag;
            self
        }

        pub fn fixed_frame_rate_flag(mut self, fixed_frame_rate_flag: u32) -> Self {
            self.fixed_frame_rate_flag = fixed_frame_rate_flag;
            self
        }

        pub fn bitstream_restriction_flag(mut self, bitstream_restriction_flag: u32) -> Self {
            self.bitstream_restriction_flag = bitstream_restriction_flag;
            self
        }

        pub fn nal_hrd_parameters_present_flag(
            mut self,
            nal_hrd_parameters_present_flag: u32,
        ) -> Self {
            self.nal_hrd_parameters_present_flag = nal_hrd_parameters_present_flag;
            self
        }

        pub fn vcl_hrd_parameters_present_flag(
            mut self,
            vcl_hrd_parameters_present_flag: u32,
        ) -> Self {
            self.vcl_hrd_parameters_present_flag = vcl_hrd_parameters_present_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264HrdParameters.html>
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct StdVideoH264HrdParameters {
        pub cpb_cnt_minus1: u8,
        pub bit_rate_scale: u8,
        pub cpb_size_scale: u8,
        pub reserved1: u8,
        pub bit_rate_value_minus1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
        pub cpb_size_value_minus1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
        pub cbr_flag: [u8; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
        pub initial_cpb_removal_delay_length_minus1: u32,
        pub cpb_removal_delay_length_minus1: u32,
        pub dpb_output_delay_length_minus1: u32,
        pub time_offset_length: u32,
    }

    impl Default for StdVideoH264HrdParameters {
        fn default() -> Self {
            Self {
                cpb_cnt_minus1: Default::default(),
                bit_rate_scale: Default::default(),
                cpb_size_scale: Default::default(),
                reserved1: Default::default(),
                bit_rate_value_minus1: [Default::default(); _],
                cpb_size_value_minus1: [Default::default(); _],
                cbr_flag: [Default::default(); _],
                initial_cpb_removal_delay_length_minus1: Default::default(),
                cpb_removal_delay_length_minus1: Default::default(),
                dpb_output_delay_length_minus1: Default::default(),
                time_offset_length: Default::default(),
            }
        }
    }

    impl StdVideoH264HrdParameters {
        pub fn cpb_cnt_minus1(mut self, cpb_cnt_minus1: u8) -> Self {
            self.cpb_cnt_minus1 = cpb_cnt_minus1;
            self
        }

        pub fn bit_rate_scale(mut self, bit_rate_scale: u8) -> Self {
            self.bit_rate_scale = bit_rate_scale;
            self
        }

        pub fn cpb_size_scale(mut self, cpb_size_scale: u8) -> Self {
            self.cpb_size_scale = cpb_size_scale;
            self
        }

        pub fn reserved1(mut self, reserved1: u8) -> Self {
            self.reserved1 = reserved1;
            self
        }

        pub fn bit_rate_value_minus1(
            mut self,
            bit_rate_value_minus1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
        ) -> Self {
            self.bit_rate_value_minus1 = bit_rate_value_minus1;
            self
        }

        pub fn cpb_size_value_minus1(
            mut self,
            cpb_size_value_minus1: [u32; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
        ) -> Self {
            self.cpb_size_value_minus1 = cpb_size_value_minus1;
            self
        }

        pub fn cbr_flag(
            mut self,
            cbr_flag: [u8; STD_VIDEO_H264_CPB_CNT_LIST_SIZE as usize],
        ) -> Self {
            self.cbr_flag = cbr_flag;
            self
        }

        pub fn initial_cpb_removal_delay_length_minus1(
            mut self,
            initial_cpb_removal_delay_length_minus1: u32,
        ) -> Self {
            self.initial_cpb_removal_delay_length_minus1 = initial_cpb_removal_delay_length_minus1;
            self
        }

        pub fn cpb_removal_delay_length_minus1(
            mut self,
            cpb_removal_delay_length_minus1: u32,
        ) -> Self {
            self.cpb_removal_delay_length_minus1 = cpb_removal_delay_length_minus1;
            self
        }

        pub fn dpb_output_delay_length_minus1(
            mut self,
            dpb_output_delay_length_minus1: u32,
        ) -> Self {
            self.dpb_output_delay_length_minus1 = dpb_output_delay_length_minus1;
            self
        }

        pub fn time_offset_length(mut self, time_offset_length: u32) -> Self {
            self.time_offset_length = time_offset_length;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264SequenceParameterSetVui.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct StdVideoH264SequenceParameterSetVui<'a> {
        pub flags: StdVideoH264SpsVuiFlags,
        pub aspect_ratio_idc: StdVideoH264AspectRatioIdc,
        pub sar_width: u16,
        pub sar_height: u16,
        pub video_format: u8,
        pub colour_primaries: u8,
        pub transfer_characteristics: u8,
        pub matrix_coefficients: u8,
        pub num_units_in_tick: u32,
        pub time_scale: u32,
        pub max_num_reorder_frames: u8,
        pub max_dec_frame_buffering: u8,
        pub chroma_sample_loc_type_top_field: u8,
        pub chroma_sample_loc_type_bottom_field: u8,
        pub reserved1: u32,
        pub p_hrd_parameters: *const StdVideoH264HrdParameters,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for StdVideoH264SequenceParameterSetVui<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoH264SequenceParameterSetVui")
                .field("flags", &self.flags)
                .field("aspect_ratio_idc", &self.aspect_ratio_idc)
                .field("sar_width", &self.sar_width)
                .field("sar_height", &self.sar_height)
                .field("video_format", &self.video_format)
                .field("colour_primaries", &self.colour_primaries)
                .field("transfer_characteristics", &self.transfer_characteristics)
                .field("matrix_coefficients", &self.matrix_coefficients)
                .field("num_units_in_tick", &self.num_units_in_tick)
                .field("time_scale", &self.time_scale)
                .field("max_num_reorder_frames", &self.max_num_reorder_frames)
                .field("max_dec_frame_buffering", &self.max_dec_frame_buffering)
                .field(
                    "chroma_sample_loc_type_top_field",
                    &self.chroma_sample_loc_type_top_field,
                )
                .field(
                    "chroma_sample_loc_type_bottom_field",
                    &self.chroma_sample_loc_type_bottom_field,
                )
                .field("reserved1", &self.reserved1)
                .field("p_hrd_parameters", &self.p_hrd_parameters)
                .finish()
        }
    }

    impl Default for StdVideoH264SequenceParameterSetVui<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                aspect_ratio_idc: Default::default(),
                sar_width: Default::default(),
                sar_height: Default::default(),
                video_format: Default::default(),
                colour_primaries: Default::default(),
                transfer_characteristics: Default::default(),
                matrix_coefficients: Default::default(),
                num_units_in_tick: Default::default(),
                time_scale: Default::default(),
                max_num_reorder_frames: Default::default(),
                max_dec_frame_buffering: Default::default(),
                chroma_sample_loc_type_top_field: Default::default(),
                chroma_sample_loc_type_bottom_field: Default::default(),
                reserved1: Default::default(),
                p_hrd_parameters: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoH264SequenceParameterSetVui<'a> {
        pub fn flags(mut self, flags: StdVideoH264SpsVuiFlags) -> Self {
            self.flags = flags;
            self
        }

        pub fn aspect_ratio_idc(mut self, aspect_ratio_idc: StdVideoH264AspectRatioIdc) -> Self {
            self.aspect_ratio_idc = aspect_ratio_idc;
            self
        }

        pub fn sar_width(mut self, sar_width: u16) -> Self {
            self.sar_width = sar_width;
            self
        }

        pub fn sar_height(mut self, sar_height: u16) -> Self {
            self.sar_height = sar_height;
            self
        }

        pub fn video_format(mut self, video_format: u8) -> Self {
            self.video_format = video_format;
            self
        }

        pub fn colour_primaries(mut self, colour_primaries: u8) -> Self {
            self.colour_primaries = colour_primaries;
            self
        }

        pub fn transfer_characteristics(mut self, transfer_characteristics: u8) -> Self {
            self.transfer_characteristics = transfer_characteristics;
            self
        }

        pub fn matrix_coefficients(mut self, matrix_coefficients: u8) -> Self {
            self.matrix_coefficients = matrix_coefficients;
            self
        }

        pub fn num_units_in_tick(mut self, num_units_in_tick: u32) -> Self {
            self.num_units_in_tick = num_units_in_tick;
            self
        }

        pub fn time_scale(mut self, time_scale: u32) -> Self {
            self.time_scale = time_scale;
            self
        }

        pub fn max_num_reorder_frames(mut self, max_num_reorder_frames: u8) -> Self {
            self.max_num_reorder_frames = max_num_reorder_frames;
            self
        }

        pub fn max_dec_frame_buffering(mut self, max_dec_frame_buffering: u8) -> Self {
            self.max_dec_frame_buffering = max_dec_frame_buffering;
            self
        }

        pub fn chroma_sample_loc_type_top_field(
            mut self,
            chroma_sample_loc_type_top_field: u8,
        ) -> Self {
            self.chroma_sample_loc_type_top_field = chroma_sample_loc_type_top_field;
            self
        }

        pub fn chroma_sample_loc_type_bottom_field(
            mut self,
            chroma_sample_loc_type_bottom_field: u8,
        ) -> Self {
            self.chroma_sample_loc_type_bottom_field = chroma_sample_loc_type_bottom_field;
            self
        }

        pub fn reserved1(mut self, reserved1: u32) -> Self {
            self.reserved1 = reserved1;
            self
        }

        pub fn hrd_parameters(mut self, hrd_parameters: &'a StdVideoH264HrdParameters) -> Self {
            self.p_hrd_parameters = hrd_parameters;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264SpsFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct StdVideoH264SpsFlags {
        pub constraint_set0_flag: u32,
        pub constraint_set1_flag: u32,
        pub constraint_set2_flag: u32,
        pub constraint_set3_flag: u32,
        pub constraint_set4_flag: u32,
        pub constraint_set5_flag: u32,
        pub direct_8x8_inference_flag: u32,
        pub mb_adaptive_frame_field_flag: u32,
        pub frame_mbs_only_flag: u32,
        pub delta_pic_order_always_zero_flag: u32,
        pub separate_colour_plane_flag: u32,
        pub gaps_in_frame_num_value_allowed_flag: u32,
        pub qpprime_y_zero_transform_bypass_flag: u32,
        pub frame_cropping_flag: u32,
        pub seq_scaling_matrix_present_flag: u32,
        pub vui_parameters_present_flag: u32,
    }

    impl StdVideoH264SpsFlags {
        pub fn constraint_set0_flag(mut self, constraint_set0_flag: u32) -> Self {
            self.constraint_set0_flag = constraint_set0_flag;
            self
        }

        pub fn constraint_set1_flag(mut self, constraint_set1_flag: u32) -> Self {
            self.constraint_set1_flag = constraint_set1_flag;
            self
        }

        pub fn constraint_set2_flag(mut self, constraint_set2_flag: u32) -> Self {
            self.constraint_set2_flag = constraint_set2_flag;
            self
        }

        pub fn constraint_set3_flag(mut self, constraint_set3_flag: u32) -> Self {
            self.constraint_set3_flag = constraint_set3_flag;
            self
        }

        pub fn constraint_set4_flag(mut self, constraint_set4_flag: u32) -> Self {
            self.constraint_set4_flag = constraint_set4_flag;
            self
        }

        pub fn constraint_set5_flag(mut self, constraint_set5_flag: u32) -> Self {
            self.constraint_set5_flag = constraint_set5_flag;
            self
        }

        pub fn direct_8x8_inference_flag(mut self, direct_8x8_inference_flag: u32) -> Self {
            self.direct_8x8_inference_flag = direct_8x8_inference_flag;
            self
        }

        pub fn mb_adaptive_frame_field_flag(mut self, mb_adaptive_frame_field_flag: u32) -> Self {
            self.mb_adaptive_frame_field_flag = mb_adaptive_frame_field_flag;
            self
        }

        pub fn frame_mbs_only_flag(mut self, frame_mbs_only_flag: u32) -> Self {
            self.frame_mbs_only_flag = frame_mbs_only_flag;
            self
        }

        pub fn delta_pic_order_always_zero_flag(
            mut self,
            delta_pic_order_always_zero_flag: u32,
        ) -> Self {
            self.delta_pic_order_always_zero_flag = delta_pic_order_always_zero_flag;
            self
        }

        pub fn separate_colour_plane_flag(mut self, separate_colour_plane_flag: u32) -> Self {
            self.separate_colour_plane_flag = separate_colour_plane_flag;
            self
        }

        pub fn gaps_in_frame_num_value_allowed_flag(
            mut self,
            gaps_in_frame_num_value_allowed_flag: u32,
        ) -> Self {
            self.gaps_in_frame_num_value_allowed_flag = gaps_in_frame_num_value_allowed_flag;
            self
        }

        pub fn qpprime_y_zero_transform_bypass_flag(
            mut self,
            qpprime_y_zero_transform_bypass_flag: u32,
        ) -> Self {
            self.qpprime_y_zero_transform_bypass_flag = qpprime_y_zero_transform_bypass_flag;
            self
        }

        pub fn frame_cropping_flag(mut self, frame_cropping_flag: u32) -> Self {
            self.frame_cropping_flag = frame_cropping_flag;
            self
        }

        pub fn seq_scaling_matrix_present_flag(
            mut self,
            seq_scaling_matrix_present_flag: u32,
        ) -> Self {
            self.seq_scaling_matrix_present_flag = seq_scaling_matrix_present_flag;
            self
        }

        pub fn vui_parameters_present_flag(mut self, vui_parameters_present_flag: u32) -> Self {
            self.vui_parameters_present_flag = vui_parameters_present_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264ScalingLists.html>
    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct StdVideoH264ScalingLists {
        pub scaling_list_present_mask: u16,
        pub use_default_scaling_matrix_mask: u16,
        pub scaling_list4x4: [[u8; STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS as usize];
            STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS as usize],
        pub scaling_list8x8: [[u8; STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS as usize];
            STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS as usize],
    }

    impl Default for StdVideoH264ScalingLists {
        fn default() -> Self {
            Self {
                scaling_list_present_mask: Default::default(),
                use_default_scaling_matrix_mask: Default::default(),
                scaling_list4x4: [[Default::default(); _]; _],
                scaling_list8x8: [[Default::default(); _]; _],
            }
        }
    }

    impl StdVideoH264ScalingLists {
        pub fn scaling_list_present_mask(mut self, scaling_list_present_mask: u16) -> Self {
            self.scaling_list_present_mask = scaling_list_present_mask;
            self
        }

        pub fn use_default_scaling_matrix_mask(
            mut self,
            use_default_scaling_matrix_mask: u16,
        ) -> Self {
            self.use_default_scaling_matrix_mask = use_default_scaling_matrix_mask;
            self
        }

        pub fn scaling_list4x4(
            mut self,
            scaling_list4x4: [[u8; STD_VIDEO_H264_SCALING_LIST_4X4_NUM_ELEMENTS as usize];
                STD_VIDEO_H264_SCALING_LIST_4X4_NUM_LISTS as usize],
        ) -> Self {
            self.scaling_list4x4 = scaling_list4x4;
            self
        }

        pub fn scaling_list8x8(
            mut self,
            scaling_list8x8: [[u8; STD_VIDEO_H264_SCALING_LIST_8X8_NUM_ELEMENTS as usize];
                STD_VIDEO_H264_SCALING_LIST_8X8_NUM_LISTS as usize],
        ) -> Self {
            self.scaling_list8x8 = scaling_list8x8;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264SequenceParameterSet.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct StdVideoH264SequenceParameterSet<'a> {
        pub flags: StdVideoH264SpsFlags,
        pub profile_idc: StdVideoH264ProfileIdc,
        pub level_idc: StdVideoH264LevelIdc,
        pub chroma_format_idc: StdVideoH264ChromaFormatIdc,
        pub seq_parameter_set_id: u8,
        pub bit_depth_luma_minus8: u8,
        pub bit_depth_chroma_minus8: u8,
        pub log2_max_frame_num_minus4: u8,
        pub pic_order_cnt_type: StdVideoH264PocType,
        pub offset_for_non_ref_pic: i32,
        pub offset_for_top_to_bottom_field: i32,
        pub log2_max_pic_order_cnt_lsb_minus4: u8,
        pub num_ref_frames_in_pic_order_cnt_cycle: u8,
        pub max_num_ref_frames: u8,
        pub reserved1: u8,
        pub pic_width_in_mbs_minus1: u32,
        pub pic_height_in_map_units_minus1: u32,
        pub frame_crop_left_offset: u32,
        pub frame_crop_right_offset: u32,
        pub frame_crop_top_offset: u32,
        pub frame_crop_bottom_offset: u32,
        pub reserved2: u32,
        pub p_offset_for_ref_frame: *const i32,
        pub p_scaling_lists: *const StdVideoH264ScalingLists,
        pub p_sequence_parameter_set_vui: *const StdVideoH264SequenceParameterSetVui<'a>,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for StdVideoH264SequenceParameterSet<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoH264SequenceParameterSet")
                .field("flags", &self.flags)
                .field("profile_idc", &self.profile_idc)
                .field("level_idc", &self.level_idc)
                .field("chroma_format_idc", &self.chroma_format_idc)
                .field("seq_parameter_set_id", &self.seq_parameter_set_id)
                .field("bit_depth_luma_minus8", &self.bit_depth_luma_minus8)
                .field("bit_depth_chroma_minus8", &self.bit_depth_chroma_minus8)
                .field("log2_max_frame_num_minus4", &self.log2_max_frame_num_minus4)
                .field("pic_order_cnt_type", &self.pic_order_cnt_type)
                .field("offset_for_non_ref_pic", &self.offset_for_non_ref_pic)
                .field(
                    "offset_for_top_to_bottom_field",
                    &self.offset_for_top_to_bottom_field,
                )
                .field(
                    "log2_max_pic_order_cnt_lsb_minus4",
                    &self.log2_max_pic_order_cnt_lsb_minus4,
                )
                .field(
                    "num_ref_frames_in_pic_order_cnt_cycle",
                    &self.num_ref_frames_in_pic_order_cnt_cycle,
                )
                .field("max_num_ref_frames", &self.max_num_ref_frames)
                .field("reserved1", &self.reserved1)
                .field("pic_width_in_mbs_minus1", &self.pic_width_in_mbs_minus1)
                .field(
                    "pic_height_in_map_units_minus1",
                    &self.pic_height_in_map_units_minus1,
                )
                .field("frame_crop_left_offset", &self.frame_crop_left_offset)
                .field("frame_crop_right_offset", &self.frame_crop_right_offset)
                .field("frame_crop_top_offset", &self.frame_crop_top_offset)
                .field("frame_crop_bottom_offset", &self.frame_crop_bottom_offset)
                .field("reserved2", &self.reserved2)
                .field("p_offset_for_ref_frame", &self.p_offset_for_ref_frame)
                .field("p_scaling_lists", &self.p_scaling_lists)
                .field(
                    "p_sequence_parameter_set_vui",
                    &self.p_sequence_parameter_set_vui,
                )
                .finish()
        }
    }

    impl Default for StdVideoH264SequenceParameterSet<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                profile_idc: Default::default(),
                level_idc: Default::default(),
                chroma_format_idc: Default::default(),
                seq_parameter_set_id: Default::default(),
                bit_depth_luma_minus8: Default::default(),
                bit_depth_chroma_minus8: Default::default(),
                log2_max_frame_num_minus4: Default::default(),
                pic_order_cnt_type: Default::default(),
                offset_for_non_ref_pic: Default::default(),
                offset_for_top_to_bottom_field: Default::default(),
                log2_max_pic_order_cnt_lsb_minus4: Default::default(),
                num_ref_frames_in_pic_order_cnt_cycle: Default::default(),
                max_num_ref_frames: Default::default(),
                reserved1: Default::default(),
                pic_width_in_mbs_minus1: Default::default(),
                pic_height_in_map_units_minus1: Default::default(),
                frame_crop_left_offset: Default::default(),
                frame_crop_right_offset: Default::default(),
                frame_crop_top_offset: Default::default(),
                frame_crop_bottom_offset: Default::default(),
                reserved2: Default::default(),
                p_offset_for_ref_frame: core::ptr::null(),
                p_scaling_lists: core::ptr::null(),
                p_sequence_parameter_set_vui: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoH264SequenceParameterSet<'a> {
        pub fn flags(mut self, flags: StdVideoH264SpsFlags) -> Self {
            self.flags = flags;
            self
        }

        pub fn profile_idc(mut self, profile_idc: StdVideoH264ProfileIdc) -> Self {
            self.profile_idc = profile_idc;
            self
        }

        pub fn level_idc(mut self, level_idc: StdVideoH264LevelIdc) -> Self {
            self.level_idc = level_idc;
            self
        }

        pub fn chroma_format_idc(mut self, chroma_format_idc: StdVideoH264ChromaFormatIdc) -> Self {
            self.chroma_format_idc = chroma_format_idc;
            self
        }

        pub fn seq_parameter_set_id(mut self, seq_parameter_set_id: u8) -> Self {
            self.seq_parameter_set_id = seq_parameter_set_id;
            self
        }

        pub fn bit_depth_luma_minus8(mut self, bit_depth_luma_minus8: u8) -> Self {
            self.bit_depth_luma_minus8 = bit_depth_luma_minus8;
            self
        }

        pub fn bit_depth_chroma_minus8(mut self, bit_depth_chroma_minus8: u8) -> Self {
            self.bit_depth_chroma_minus8 = bit_depth_chroma_minus8;
            self
        }

        pub fn log2_max_frame_num_minus4(mut self, log2_max_frame_num_minus4: u8) -> Self {
            self.log2_max_frame_num_minus4 = log2_max_frame_num_minus4;
            self
        }

        pub fn pic_order_cnt_type(mut self, pic_order_cnt_type: StdVideoH264PocType) -> Self {
            self.pic_order_cnt_type = pic_order_cnt_type;
            self
        }

        pub fn offset_for_non_ref_pic(mut self, offset_for_non_ref_pic: i32) -> Self {
            self.offset_for_non_ref_pic = offset_for_non_ref_pic;
            self
        }

        pub fn offset_for_top_to_bottom_field(
            mut self,
            offset_for_top_to_bottom_field: i32,
        ) -> Self {
            self.offset_for_top_to_bottom_field = offset_for_top_to_bottom_field;
            self
        }

        pub fn log2_max_pic_order_cnt_lsb_minus4(
            mut self,
            log2_max_pic_order_cnt_lsb_minus4: u8,
        ) -> Self {
            self.log2_max_pic_order_cnt_lsb_minus4 = log2_max_pic_order_cnt_lsb_minus4;
            self
        }

        pub fn offset_for_ref_frame(mut self, offset_for_ref_frame: &'a [i32]) -> Self {
            self.num_ref_frames_in_pic_order_cnt_cycle =
                offset_for_ref_frame.len().try_into().unwrap();
            self.p_offset_for_ref_frame = offset_for_ref_frame.as_ptr();
            self
        }

        pub fn max_num_ref_frames(mut self, max_num_ref_frames: u8) -> Self {
            self.max_num_ref_frames = max_num_ref_frames;
            self
        }

        pub fn reserved1(mut self, reserved1: u8) -> Self {
            self.reserved1 = reserved1;
            self
        }

        pub fn pic_width_in_mbs_minus1(mut self, pic_width_in_mbs_minus1: u32) -> Self {
            self.pic_width_in_mbs_minus1 = pic_width_in_mbs_minus1;
            self
        }

        pub fn pic_height_in_map_units_minus1(
            mut self,
            pic_height_in_map_units_minus1: u32,
        ) -> Self {
            self.pic_height_in_map_units_minus1 = pic_height_in_map_units_minus1;
            self
        }

        pub fn frame_crop_left_offset(mut self, frame_crop_left_offset: u32) -> Self {
            self.frame_crop_left_offset = frame_crop_left_offset;
            self
        }

        pub fn frame_crop_right_offset(mut self, frame_crop_right_offset: u32) -> Self {
            self.frame_crop_right_offset = frame_crop_right_offset;
            self
        }

        pub fn frame_crop_top_offset(mut self, frame_crop_top_offset: u32) -> Self {
            self.frame_crop_top_offset = frame_crop_top_offset;
            self
        }

        pub fn frame_crop_bottom_offset(mut self, frame_crop_bottom_offset: u32) -> Self {
            self.frame_crop_bottom_offset = frame_crop_bottom_offset;
            self
        }

        pub fn reserved2(mut self, reserved2: u32) -> Self {
            self.reserved2 = reserved2;
            self
        }

        pub fn scaling_lists(mut self, scaling_lists: &'a StdVideoH264ScalingLists) -> Self {
            self.p_scaling_lists = scaling_lists;
            self
        }

        pub fn sequence_parameter_set_vui(
            mut self,
            sequence_parameter_set_vui: &'a StdVideoH264SequenceParameterSetVui<'a>,
        ) -> Self {
            self.p_sequence_parameter_set_vui = sequence_parameter_set_vui;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264PpsFlags.html>
    #[repr(C)]
    #[derive(Copy, Clone, Default, Debug)]
    pub struct StdVideoH264PpsFlags {
        pub transform_8x8_mode_flag: u32,
        pub redundant_pic_cnt_present_flag: u32,
        pub constrained_intra_pred_flag: u32,
        pub deblocking_filter_control_present_flag: u32,
        pub weighted_pred_flag: u32,
        pub bottom_field_pic_order_in_frame_present_flag: u32,
        pub entropy_coding_mode_flag: u32,
        pub pic_scaling_matrix_present_flag: u32,
    }

    impl StdVideoH264PpsFlags {
        pub fn transform_8x8_mode_flag(mut self, transform_8x8_mode_flag: u32) -> Self {
            self.transform_8x8_mode_flag = transform_8x8_mode_flag;
            self
        }

        pub fn redundant_pic_cnt_present_flag(
            mut self,
            redundant_pic_cnt_present_flag: u32,
        ) -> Self {
            self.redundant_pic_cnt_present_flag = redundant_pic_cnt_present_flag;
            self
        }

        pub fn constrained_intra_pred_flag(mut self, constrained_intra_pred_flag: u32) -> Self {
            self.constrained_intra_pred_flag = constrained_intra_pred_flag;
            self
        }

        pub fn deblocking_filter_control_present_flag(
            mut self,
            deblocking_filter_control_present_flag: u32,
        ) -> Self {
            self.deblocking_filter_control_present_flag = deblocking_filter_control_present_flag;
            self
        }

        pub fn weighted_pred_flag(mut self, weighted_pred_flag: u32) -> Self {
            self.weighted_pred_flag = weighted_pred_flag;
            self
        }

        pub fn bottom_field_pic_order_in_frame_present_flag(
            mut self,
            bottom_field_pic_order_in_frame_present_flag: u32,
        ) -> Self {
            self.bottom_field_pic_order_in_frame_present_flag =
                bottom_field_pic_order_in_frame_present_flag;
            self
        }

        pub fn entropy_coding_mode_flag(mut self, entropy_coding_mode_flag: u32) -> Self {
            self.entropy_coding_mode_flag = entropy_coding_mode_flag;
            self
        }

        pub fn pic_scaling_matrix_present_flag(
            mut self,
            pic_scaling_matrix_present_flag: u32,
        ) -> Self {
            self.pic_scaling_matrix_present_flag = pic_scaling_matrix_present_flag;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264PictureParameterSet.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct StdVideoH264PictureParameterSet<'a> {
        pub flags: StdVideoH264PpsFlags,
        pub seq_parameter_set_id: u8,
        pub pic_parameter_set_id: u8,
        pub num_ref_idx_l0_default_active_minus1: u8,
        pub num_ref_idx_l1_default_active_minus1: u8,
        pub weighted_bipred_idc: StdVideoH264WeightedBipredIdc,
        pub pic_init_qp_minus26: i8,
        pub pic_init_qs_minus26: i8,
        pub chroma_qp_index_offset: i8,
        pub second_chroma_qp_index_offset: i8,
        pub p_scaling_lists: *const StdVideoH264ScalingLists,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for StdVideoH264PictureParameterSet<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("StdVideoH264PictureParameterSet")
                .field("flags", &self.flags)
                .field("seq_parameter_set_id", &self.seq_parameter_set_id)
                .field("pic_parameter_set_id", &self.pic_parameter_set_id)
                .field(
                    "num_ref_idx_l0_default_active_minus1",
                    &self.num_ref_idx_l0_default_active_minus1,
                )
                .field(
                    "num_ref_idx_l1_default_active_minus1",
                    &self.num_ref_idx_l1_default_active_minus1,
                )
                .field("weighted_bipred_idc", &self.weighted_bipred_idc)
                .field("pic_init_qp_minus26", &self.pic_init_qp_minus26)
                .field("pic_init_qs_minus26", &self.pic_init_qs_minus26)
                .field("chroma_qp_index_offset", &self.chroma_qp_index_offset)
                .field(
                    "second_chroma_qp_index_offset",
                    &self.second_chroma_qp_index_offset,
                )
                .field("p_scaling_lists", &self.p_scaling_lists)
                .finish()
        }
    }

    impl Default for StdVideoH264PictureParameterSet<'_> {
        fn default() -> Self {
            Self {
                flags: Default::default(),
                seq_parameter_set_id: Default::default(),
                pic_parameter_set_id: Default::default(),
                num_ref_idx_l0_default_active_minus1: Default::default(),
                num_ref_idx_l1_default_active_minus1: Default::default(),
                weighted_bipred_idc: Default::default(),
                pic_init_qp_minus26: Default::default(),
                pic_init_qs_minus26: Default::default(),
                chroma_qp_index_offset: Default::default(),
                second_chroma_qp_index_offset: Default::default(),
                p_scaling_lists: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> StdVideoH264PictureParameterSet<'a> {
        pub fn flags(mut self, flags: StdVideoH264PpsFlags) -> Self {
            self.flags = flags;
            self
        }

        pub fn seq_parameter_set_id(mut self, seq_parameter_set_id: u8) -> Self {
            self.seq_parameter_set_id = seq_parameter_set_id;
            self
        }

        pub fn pic_parameter_set_id(mut self, pic_parameter_set_id: u8) -> Self {
            self.pic_parameter_set_id = pic_parameter_set_id;
            self
        }

        pub fn num_ref_idx_l0_default_active_minus1(
            mut self,
            num_ref_idx_l0_default_active_minus1: u8,
        ) -> Self {
            self.num_ref_idx_l0_default_active_minus1 = num_ref_idx_l0_default_active_minus1;
            self
        }

        pub fn num_ref_idx_l1_default_active_minus1(
            mut self,
            num_ref_idx_l1_default_active_minus1: u8,
        ) -> Self {
            self.num_ref_idx_l1_default_active_minus1 = num_ref_idx_l1_default_active_minus1;
            self
        }

        pub fn weighted_bipred_idc(
            mut self,
            weighted_bipred_idc: StdVideoH264WeightedBipredIdc,
        ) -> Self {
            self.weighted_bipred_idc = weighted_bipred_idc;
            self
        }

        pub fn pic_init_qp_minus26(mut self, pic_init_qp_minus26: i8) -> Self {
            self.pic_init_qp_minus26 = pic_init_qp_minus26;
            self
        }

        pub fn pic_init_qs_minus26(mut self, pic_init_qs_minus26: i8) -> Self {
            self.pic_init_qs_minus26 = pic_init_qs_minus26;
            self
        }

        pub fn chroma_qp_index_offset(mut self, chroma_qp_index_offset: i8) -> Self {
            self.chroma_qp_index_offset = chroma_qp_index_offset;
            self
        }

        pub fn second_chroma_qp_index_offset(mut self, second_chroma_qp_index_offset: i8) -> Self {
            self.second_chroma_qp_index_offset = second_chroma_qp_index_offset;
            self
        }

        pub fn scaling_lists(mut self, scaling_lists: &'a StdVideoH264ScalingLists) -> Self {
            self.p_scaling_lists = scaling_lists;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264ChromaFormatIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264ChromaFormatIdc(i32);

    impl StdVideoH264ChromaFormatIdc {
        pub const MONOCHROME: Self = Self(0);
        pub const _420: Self = Self(1);
        pub const _422: Self = Self(2);
        pub const _444: Self = Self(3);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264ChromaFormatIdc {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264ProfileIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264ProfileIdc(i32);

    impl StdVideoH264ProfileIdc {
        /// Only constrained baseline is supported
        pub const BASELINE: Self = Self(66);
        pub const MAIN: Self = Self(77);
        pub const HIGH: Self = Self(100);
        pub const HIGH_444_PREDICTIVE: Self = Self(244);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264ProfileIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::BASELINE => Some("BASELINE"),
                Self::MAIN => Some("MAIN"),
                Self::HIGH => Some("HIGH"),
                Self::HIGH_444_PREDICTIVE => Some("HIGH_444_PREDICTIVE"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264LevelIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264LevelIdc(i32);

    impl StdVideoH264LevelIdc {
        pub const _1_0: Self = Self(0);
        pub const _1_1: Self = Self(1);
        pub const _1_2: Self = Self(2);
        pub const _1_3: Self = Self(3);
        pub const _2_0: Self = Self(4);
        pub const _2_1: Self = Self(5);
        pub const _2_2: Self = Self(6);
        pub const _3_0: Self = Self(7);
        pub const _3_1: Self = Self(8);
        pub const _3_2: Self = Self(9);
        pub const _4_0: Self = Self(10);
        pub const _4_1: Self = Self(11);
        pub const _4_2: Self = Self(12);
        pub const _5_0: Self = Self(13);
        pub const _5_1: Self = Self(14);
        pub const _5_2: Self = Self(15);
        pub const _6_0: Self = Self(16);
        pub const _6_1: Self = Self(17);
        pub const _6_2: Self = Self(18);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264LevelIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_1_0 => Some("_1_0"),
                Self::_1_1 => Some("_1_1"),
                Self::_1_2 => Some("_1_2"),
                Self::_1_3 => Some("_1_3"),
                Self::_2_0 => Some("_2_0"),
                Self::_2_1 => Some("_2_1"),
                Self::_2_2 => Some("_2_2"),
                Self::_3_0 => Some("_3_0"),
                Self::_3_1 => Some("_3_1"),
                Self::_3_2 => Some("_3_2"),
                Self::_4_0 => Some("_4_0"),
                Self::_4_1 => Some("_4_1"),
                Self::_4_2 => Some("_4_2"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264PocType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264PocType(i32);

    impl StdVideoH264PocType {
        pub const _0: Self = Self(0);
        pub const _1: Self = Self(1);
        pub const _2: Self = Self(2);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264PocType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_0 => Some("_0"),
                Self::_1 => Some("_1"),
                Self::_2 => Some("_2"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264AspectRatioIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264AspectRatioIdc(i32);

    impl StdVideoH264AspectRatioIdc {
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

    impl fmt::Debug for StdVideoH264AspectRatioIdc {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264WeightedBipredIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264WeightedBipredIdc(i32);

    impl StdVideoH264WeightedBipredIdc {
        pub const DEFAULT: Self = Self(0);
        pub const EXPLICIT: Self = Self(1);
        pub const IMPLICIT: Self = Self(2);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264WeightedBipredIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT => Some("DEFAULT"),
                Self::EXPLICIT => Some("EXPLICIT"),
                Self::IMPLICIT => Some("IMPLICIT"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264ModificationOfPicNumsIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264ModificationOfPicNumsIdc(i32);

    impl StdVideoH264ModificationOfPicNumsIdc {
        pub const SHORT_TERM_SUBTRACT: Self = Self(0);
        pub const SHORT_TERM_ADD: Self = Self(1);
        pub const LONG_TERM: Self = Self(2);
        pub const END: Self = Self(3);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264ModificationOfPicNumsIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SHORT_TERM_SUBTRACT => Some("SHORT_TERM_SUBTRACT"),
                Self::SHORT_TERM_ADD => Some("SHORT_TERM_ADD"),
                Self::LONG_TERM => Some("LONG_TERM"),
                Self::END => Some("END"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264MemMgmtControlOp.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264MemMgmtControlOp(i32);

    impl StdVideoH264MemMgmtControlOp {
        pub const END: Self = Self(0);
        pub const UNMARK_SHORT_TERM: Self = Self(1);
        pub const UNMARK_LONG_TERM: Self = Self(2);
        pub const MARK_LONG_TERM: Self = Self(3);
        pub const SET_MAX_LONG_TERM_INDEX: Self = Self(4);
        pub const UNMARK_ALL: Self = Self(5);
        pub const MARK_CURRENT_AS_LONG_TERM: Self = Self(6);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264MemMgmtControlOp {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::END => Some("END"),
                Self::UNMARK_SHORT_TERM => Some("UNMARK_SHORT_TERM"),
                Self::UNMARK_LONG_TERM => Some("UNMARK_LONG_TERM"),
                Self::MARK_LONG_TERM => Some("MARK_LONG_TERM"),
                Self::SET_MAX_LONG_TERM_INDEX => Some("SET_MAX_LONG_TERM_INDEX"),
                Self::UNMARK_ALL => Some("UNMARK_ALL"),
                Self::MARK_CURRENT_AS_LONG_TERM => Some("MARK_CURRENT_AS_LONG_TERM"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264CabacInitIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264CabacInitIdc(i32);

    impl StdVideoH264CabacInitIdc {
        pub const _0: Self = Self(0);
        pub const _1: Self = Self(1);
        pub const _2: Self = Self(2);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264CabacInitIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::_0 => Some("_0"),
                Self::_1 => Some("_1"),
                Self::_2 => Some("_2"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264DisableDeblockingFilterIdc.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264DisableDeblockingFilterIdc(i32);

    impl StdVideoH264DisableDeblockingFilterIdc {
        pub const DISABLED: Self = Self(0);
        pub const ENABLED: Self = Self(1);
        pub const PARTIAL: Self = Self(2);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264DisableDeblockingFilterIdc {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DISABLED => Some("DISABLED"),
                Self::ENABLED => Some("ENABLED"),
                Self::PARTIAL => Some("PARTIAL"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264SliceType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264SliceType(i32);

    impl StdVideoH264SliceType {
        pub const P: Self = Self(0);
        pub const B: Self = Self(1);
        pub const I: Self = Self(2);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264SliceType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::P => Some("P"),
                Self::B => Some("B"),
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264PictureType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264PictureType(i32);

    impl StdVideoH264PictureType {
        pub const P: Self = Self(0);
        pub const B: Self = Self(1);
        pub const I: Self = Self(2);
        pub const IDR: Self = Self(5);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264PictureType {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/StdVideoH264NonVclNaluType.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct StdVideoH264NonVclNaluType(i32);

    impl StdVideoH264NonVclNaluType {
        pub const SPS: Self = Self(0);
        pub const PPS: Self = Self(1);
        pub const AUD: Self = Self(2);
        pub const PREFIX: Self = Self(3);
        pub const END_OF_SEQUENCE: Self = Self(4);
        pub const END_OF_STREAM: Self = Self(5);
        pub const PRECODED: Self = Self(6);
        pub const INVALID: Self = Self(0x7FFFFFFF);
    }

    impl fmt::Debug for StdVideoH264NonVclNaluType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::SPS => Some("SPS"),
                Self::PPS => Some("PPS"),
                Self::AUD => Some("AUD"),
                Self::PREFIX => Some("PREFIX"),
                Self::END_OF_SEQUENCE => Some("END_OF_SEQUENCE"),
                Self::END_OF_STREAM => Some("END_OF_STREAM"),
                Self::PRECODED => Some("PRECODED"),
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
