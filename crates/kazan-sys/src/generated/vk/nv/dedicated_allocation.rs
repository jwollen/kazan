#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DedicatedAllocationImageCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DedicatedAllocationImageCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEDICATED_ALLOCATION_IMAGE_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            dedicated_allocation: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DedicatedAllocationBufferCreateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub dedicated_allocation: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DedicatedAllocationBufferCreateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEDICATED_ALLOCATION_BUFFER_CREATE_INFO_NV,
            p_next: core::ptr::null(),
            dedicated_allocation: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DedicatedAllocationMemoryAllocateInfoNV<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub image: Image,
    pub buffer: Buffer,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for DedicatedAllocationMemoryAllocateInfoNV<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::DEDICATED_ALLOCATION_MEMORY_ALLOCATE_INFO_NV,
            p_next: core::ptr::null(),
            image: Default::default(),
            buffer: Default::default(),
            _marker: PhantomData,
        }
    }
}
