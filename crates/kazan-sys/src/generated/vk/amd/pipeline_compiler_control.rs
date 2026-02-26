#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineCompilerControlCreateInfoAMD<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub compiler_control_flags: PipelineCompilerControlFlagsAMD,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineCompilerControlCreateInfoAMD<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_COMPILER_CONTROL_CREATE_INFO_AMD,
            p_next: core::ptr::null(),
            compiler_control_flags: Default::default(),
            _marker: PhantomData,
        }
    }
}
bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PipelineCompilerControlFlagsAMD: Flags {
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PipelineCompilerControlFlagBitsAMD(u32);
impl PipelineCompilerControlFlagBitsAMD {}
