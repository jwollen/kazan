#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineLibraryCreateInfoKHR {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub library_count: u32,
    pub p_libraries: *const Pipeline,
}
