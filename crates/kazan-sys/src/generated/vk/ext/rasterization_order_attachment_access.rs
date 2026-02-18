#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub rasterization_order_color_attachment_access: Bool32,
    pub rasterization_order_depth_attachment_access: Bool32,
    pub rasterization_order_stencil_attachment_access: Bool32,
}
