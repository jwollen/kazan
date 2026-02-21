#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingAreaInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub p_color_attachment_formats: *const Format,
    pub depth_attachment_format: Format,
    pub stencil_attachment_format: Format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct VertexInputBindingDivisorDescription {
    pub binding: u32,
    pub divisor: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineVertexInputDivisorStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub vertex_binding_divisor_count: u32,
    pub p_vertex_binding_divisors: *const VertexInputBindingDivisorDescription,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVertexAttributeDivisorProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub max_vertex_attrib_divisor: u32,
    pub supports_non_zero_first_instance: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceVertexAttributeDivisorFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub vertex_attribute_instance_rate_divisor: Bool32,
    pub vertex_attribute_instance_rate_zero_divisor: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLineRasterizationFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rectangular_lines: Bool32,
    pub bresenham_lines: Bool32,
    pub smooth_lines: Bool32,
    pub stippled_rectangular_lines: Bool32,
    pub stippled_bresenham_lines: Bool32,
    pub stippled_smooth_lines: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceLineRasterizationProperties {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub line_sub_pixel_precision_bits: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineRasterizationLineStateCreateInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub line_rasterization_mode: LineRasterizationMode,
    pub stippled_line_enable: Bool32,
    pub line_stipple_factor: u32,
    pub line_stipple_pattern: u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDynamicRenderingLocalReadFeatures {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_rendering_local_read: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingAttachmentLocationInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_locations: *const u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RenderingInputAttachmentIndexInfo {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub color_attachment_count: u32,
    pub p_color_attachment_input_indices: *const u32,
    pub p_depth_input_attachment_index: *const u32,
    pub p_stencil_input_attachment_index: *const u32,
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineRasterizationMode(i32);
impl LineRasterizationMode {
    pub const DEFAULT: Self = Self(0);
    pub const RECTANGULAR: Self = Self(1);
    pub const BRESENHAM: Self = Self(2);
    pub const RECTANGULAR_SMOOTH: Self = Self(3);
}
pub type PFN_vkGetRenderingAreaGranularity = unsafe extern "system" fn(
    device: Device,
    p_rendering_area_info: *const RenderingAreaInfo,
    p_granularity: *mut Extent2D,
);
pub type PFN_vkCmdSetLineStipple = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
);
pub type PFN_vkCmdBindIndexBuffer2 = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
);
pub type PFN_vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_location_info: *const RenderingAttachmentLocationInfo,
);
pub type PFN_vkCmdSetRenderingInputAttachmentIndices = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
);
