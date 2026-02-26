#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
pub struct DedicatedAllocationImageCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DedicatedAllocationBufferCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct DedicatedAllocationMemoryAllocateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
    pub _marker: PhantomData<&'a ()>,
}
