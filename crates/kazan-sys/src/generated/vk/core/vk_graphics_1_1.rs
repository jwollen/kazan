#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
pub type PhysicalDeviceShaderDrawParameterFeatures = PhysicalDeviceShaderDrawParametersFeatures;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassMultiviewCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub subpass_count: u32,
    pub p_view_masks: *const u32,
    pub dependency_count: u32,
    pub p_view_offsets: *const i32,
    pub correlation_mask_count: u32,
    pub p_correlation_masks: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeviceGroupRenderPassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub device_mask: u32,
    pub device_render_area_count: u32,
    pub p_device_render_areas: *const Rect2D,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InputAttachmentAspectReference {
    pub subpass: u32,
    pub input_attachment_index: u32,
    pub aspect_mask: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassInputAttachmentAspectCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub aspect_reference_count: u32,
    pub p_aspect_references: *const InputAttachmentAspectReference,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePointClippingProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub point_clipping_behavior: PointClippingBehavior,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineTessellationDomainOriginStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub domain_origin: TessellationDomainOrigin,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_draw_parameters: Bool32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointClippingBehavior(i32);
impl PointClippingBehavior {
    pub const ALL_CLIP_PLANES: Self = Self(0);
    pub const USER_CLIP_PLANES_ONLY: Self = Self(1);
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TessellationDomainOrigin(i32);
impl TessellationDomainOrigin {
    pub const UPPER_LEFT: Self = Self(0);
    pub const LOWER_LEFT: Self = Self(1);
}
