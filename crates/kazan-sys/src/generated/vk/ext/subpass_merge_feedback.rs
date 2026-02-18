#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreationControlEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disallow_merging: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreationFeedbackInfoEXT {
    pub post_merge_subpass_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreationFeedbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_render_pass_feedback: *mut RenderPassCreationFeedbackInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassSubpassFeedbackInfoEXT {
    pub subpass_merge_status: SubpassMergeStatusEXT,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub post_merge_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassSubpassFeedbackCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_subpass_feedback: *mut RenderPassSubpassFeedbackInfoEXT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subpass_merge_feedback: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubpassMergeStatusEXT(i32);
impl SubpassMergeStatusEXT {
    pub const MERGED_EXT: Self = Self(0);
    pub const DISALLOWED_EXT: Self = Self(1);
    pub const NOT_MERGED_SIDE_EFFECTS_EXT: Self = Self(2);
    pub const NOT_MERGED_SAMPLES_MISMATCH_EXT: Self = Self(3);
    pub const NOT_MERGED_VIEWS_MISMATCH_EXT: Self = Self(4);
    pub const NOT_MERGED_ALIASING_EXT: Self = Self(5);
    pub const NOT_MERGED_DEPENDENCIES_EXT: Self = Self(6);
    pub const NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT: Self = Self(7);
    pub const NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT: Self = Self(8);
    pub const NOT_MERGED_INSUFFICIENT_STORAGE_EXT: Self = Self(9);
    pub const NOT_MERGED_DEPTH_STENCIL_COUNT_EXT: Self = Self(10);
    pub const NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT: Self = Self(11);
    pub const NOT_MERGED_SINGLE_SUBPASS_EXT: Self = Self(12);
    pub const NOT_MERGED_UNSPECIFIED_EXT: Self = Self(13);
}
