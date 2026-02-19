#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentDescription2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlags,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentReference2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment: u32,
    pub layout: ImageLayout,
    pub aspect_mask: ImageAspectFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDescription2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub view_mask: u32,
    pub input_attachment_count: u32,
    pub p_input_attachments: *const AttachmentReference2,
    pub color_attachment_count: u32,
    pub p_color_attachments: *const AttachmentReference2,
    pub p_resolve_attachments: *const AttachmentReference2,
    pub p_depth_stencil_attachment: *const AttachmentReference2,
    pub preserve_attachment_count: u32,
    pub p_preserve_attachments: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDependency2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
    pub view_offset: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassCreateInfo2 {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub p_attachments: *const AttachmentDescription2,
    pub subpass_count: u32,
    pub p_subpasses: *const SubpassDescription2,
    pub dependency_count: u32,
    pub p_dependencies: *const SubpassDependency2,
    pub correlated_view_mask_count: u32,
    pub p_correlated_view_masks: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub contents: SubpassContents,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassEndInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: Bool32,
    pub independent_resolve: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SubpassDescriptionDepthStencilResolve {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub depth_resolve_mode: ResolveModeFlags,
    pub stencil_resolve_mode: ResolveModeFlags,
    pub p_depth_stencil_resolve_attachment: *const AttachmentReference2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageStencilUsageCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub stencil_usage: ImageUsageFlags,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub imageless_framebuffer: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferAttachmentsCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_image_info_count: u32,
    pub p_attachment_image_infos: *const FramebufferAttachmentImageInfo,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FramebufferAttachmentImageInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub flags: ImageCreateFlags,
    pub usage: ImageUsageFlags,
    pub width: u32,
    pub height: u32,
    pub layer_count: u32,
    pub view_format_count: u32,
    pub p_view_formats: *const Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderPassAttachmentBeginInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub attachment_count: u32,
    pub p_attachments: *const ImageView,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub separate_depth_stencil_layouts: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentReferenceStencilLayout {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stencil_layout: ImageLayout,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AttachmentDescriptionStencilLayout {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub stencil_initial_layout: ImageLayout,
    pub stencil_final_layout: ImageLayout,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct ResolveModeFlags: Flags {
        const SAMPLE_ZERO = 1 << 0;
        const AVERAGE = 1 << 1;
        const MIN = 1 << 2;
        const MAX = 1 << 3;
        const NONE = 0;
    }
}
pub type PFN_vkCreateRenderPass2 = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const RenderPassCreateInfo2,
    p_allocator: *const AllocationCallbacks,
    p_render_pass: *mut RenderPass,
) -> Result;
pub type PFN_vkCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_render_pass_begin: *const RenderPassBeginInfo,
    p_subpass_begin_info: *const SubpassBeginInfo,
);
pub type PFN_vkCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_begin_info: *const SubpassBeginInfo,
    p_subpass_end_info: *const SubpassEndInfo,
);
pub type PFN_vkCmdEndRenderPass2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_subpass_end_info: *const SubpassEndInfo,
);
pub type PFN_vkCmdDrawIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
pub type PFN_vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);
