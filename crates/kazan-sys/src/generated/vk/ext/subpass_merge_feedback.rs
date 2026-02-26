#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreationControlEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub disallow_merging: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassCreationControlEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_CREATION_CONTROL_EXT,
            p_next: core::ptr::null(),
            disallow_merging: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct RenderPassCreationFeedbackInfoEXT {
    pub post_merge_subpass_count: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreationFeedbackCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_render_pass_feedback: *mut RenderPassCreationFeedbackInfoEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassCreationFeedbackCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_CREATION_FEEDBACK_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            p_render_pass_feedback: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassSubpassFeedbackInfoEXT {
    pub subpass_merge_status: SubpassMergeStatusEXT,
    pub description: [c_char; MAX_DESCRIPTION_SIZE as usize],
    pub post_merge_index: u32,
}
impl Default for RenderPassSubpassFeedbackInfoEXT {
    fn default() -> Self {
        Self {
            subpass_merge_status: Default::default(),
            description: [Default::default(); _],
            post_merge_index: Default::default(),
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassSubpassFeedbackCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub p_subpass_feedback: *mut RenderPassSubpassFeedbackInfoEXT,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for RenderPassSubpassFeedbackCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::RENDER_PASS_SUBPASS_FEEDBACK_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            p_subpass_feedback: core::ptr::null_mut(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub subpass_merge_feedback: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceSubpassMergeFeedbackFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SUBPASS_MERGE_FEEDBACK_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            subpass_merge_feedback: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
