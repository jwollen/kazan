#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265CapabilitiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub flags: VideoEncodeH265CapabilityFlagsKHR,
    pub max_level_idc: StdVideoH265LevelIdc,
    pub max_slice_segment_count: u32,
    pub max_tiles: Extent2D,
    pub ctb_sizes: VideoEncodeH265CtbSizeFlagsKHR,
    pub transform_block_sizes: VideoEncodeH265TransformBlockSizeFlagsKHR,
    pub max_p_picture_l0_reference_count: u32,
    pub max_b_picture_l0_reference_count: u32,
    pub max_l1_reference_count: u32,
    pub max_sub_layer_count: u32,
    pub expect_dyadic_temporal_sub_layer_pattern: Bool32,
    pub min_qp: i32,
    pub max_qp: i32,
    pub prefers_gop_remaining_frames: Bool32,
    pub requires_gop_remaining_frames: Bool32,
    pub std_syntax_flags: VideoEncodeH265StdFlagsKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265QualityLevelPropertiesKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub preferred_rate_control_flags: VideoEncodeH265RateControlFlagsKHR,
    pub preferred_gop_frame_count: u32,
    pub preferred_idr_period: u32,
    pub preferred_consecutive_b_frame_count: u32,
    pub preferred_sub_layer_count: u32,
    pub preferred_constant_qp: VideoEncodeH265QpKHR,
    pub preferred_max_l0_reference_count: u32,
    pub preferred_max_l1_reference_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265SessionCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_max_level_idc: Bool32,
    pub max_level_idc: StdVideoH265LevelIdc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265SessionParametersAddInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_vps_count: u32,
    pub p_std_vp_ss: *const StdVideoH265VideoParameterSet,
    pub std_sps_count: u32,
    pub p_std_sp_ss: *const StdVideoH265SequenceParameterSet,
    pub std_pps_count: u32,
    pub p_std_pp_ss: *const StdVideoH265PictureParameterSet,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265SessionParametersCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub max_std_vps_count: u32,
    pub max_std_sps_count: u32,
    pub max_std_pps_count: u32,
    pub p_parameters_add_info: *const VideoEncodeH265SessionParametersAddInfoKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265SessionParametersGetInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub write_std_vps: Bool32,
    pub write_std_sps: Bool32,
    pub write_std_pps: Bool32,
    pub std_vps_id: u32,
    pub std_sps_id: u32,
    pub std_pps_id: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265SessionParametersFeedbackInfoKHR {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_std_vps_overrides: Bool32,
    pub has_std_sps_overrides: Bool32,
    pub has_std_pps_overrides: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265PictureInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub nalu_slice_segment_entry_count: u32,
    pub p_nalu_slice_segment_entries: *const VideoEncodeH265NaluSliceSegmentInfoKHR,
    pub p_std_picture_info: *const StdVideoEncodeH265PictureInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265NaluSliceSegmentInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub constant_qp: i32,
    pub p_std_slice_segment_header: *const StdVideoEncodeH265SliceSegmentHeader,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265RateControlInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: VideoEncodeH265RateControlFlagsKHR,
    pub gop_frame_count: u32,
    pub idr_period: u32,
    pub consecutive_b_frame_count: u32,
    pub sub_layer_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265QpKHR {
    pub qp_i: i32,
    pub qp_p: i32,
    pub qp_b: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265FrameSizeKHR {
    pub frame_i_size: u32,
    pub frame_p_size: u32,
    pub frame_b_size: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265GopRemainingFrameInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_gop_remaining_frames: Bool32,
    pub gop_remaining_i: u32,
    pub gop_remaining_p: u32,
    pub gop_remaining_b: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265RateControlLayerInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub use_min_qp: Bool32,
    pub min_qp: VideoEncodeH265QpKHR,
    pub use_max_qp: Bool32,
    pub max_qp: VideoEncodeH265QpKHR,
    pub use_max_frame_size: Bool32,
    pub max_frame_size: VideoEncodeH265FrameSizeKHR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265ProfileInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub std_profile_idc: StdVideoH265ProfileIdc,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VideoEncodeH265DpbSlotInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_std_reference_info: *const StdVideoEncodeH265ReferenceInfo,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265CapabilityFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265StdFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265RateControlFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265CtbSizeFlagsKHR: Flags {
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct VideoEncodeH265TransformBlockSizeFlagsKHR: Flags {
    }
}
