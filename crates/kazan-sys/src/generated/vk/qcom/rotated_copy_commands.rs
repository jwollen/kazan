#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CopyCommandTransformInfoQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub transform: SurfaceTransformFlagBitsKHR,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for CopyCommandTransformInfoQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::COPY_COMMAND_TRANSFORM_INFO_QCOM,
            p_next: core::ptr::null(),
            transform: Default::default(),
            _marker: PhantomData,
        }
    }
}
