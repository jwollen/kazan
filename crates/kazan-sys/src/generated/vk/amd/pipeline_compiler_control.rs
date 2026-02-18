#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCompilerControlCreateInfoAMD {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Default)]
    pub struct PipelineCompilerControlFlagsAMD: Flags {
    }
}
