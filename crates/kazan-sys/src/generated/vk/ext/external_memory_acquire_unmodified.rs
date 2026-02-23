#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct ExternalMemoryAcquireUnmodifiedEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_unmodified_memory: Bool32,
}
