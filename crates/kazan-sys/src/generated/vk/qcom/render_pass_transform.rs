#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct RenderPassTransformBeginInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagBitsKHR,
}
#[repr(C)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagBitsKHR,
    pub render_area: Rect2D,
}
