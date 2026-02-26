#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PresentFrameTokenGGP<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub frame_token: GgpFrameToken,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PresentFrameTokenGGP<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PRESENT_FRAME_TOKEN_GGP,
            p_next: core::ptr::null(),
            frame_token: Default::default(),
            _marker: PhantomData,
        }
    }
}
