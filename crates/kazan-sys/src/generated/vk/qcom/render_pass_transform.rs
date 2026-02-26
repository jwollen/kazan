#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct RenderPassTransformBeginInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagBitsKHR,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagBitsKHR,
    pub render_area: Rect2D,
    pub _marker: PhantomData<&'a ()>,
}
