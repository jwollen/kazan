#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExternalMemoryAcquireUnmodifiedEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub acquire_unmodified_memory: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ExternalMemoryAcquireUnmodifiedEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::EXTERNAL_MEMORY_ACQUIRE_UNMODIFIED_EXT,
            p_next: core::ptr::null(),
            acquire_unmodified_memory: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> ExternalMemoryAcquireUnmodifiedEXT<'a> {
    pub fn acquire_unmodified_memory(mut self, acquire_unmodified_memory: Bool32) -> Self {
        self.acquire_unmodified_memory = acquire_unmodified_memory;
        self
    }
}
