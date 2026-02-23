#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
}
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
}
