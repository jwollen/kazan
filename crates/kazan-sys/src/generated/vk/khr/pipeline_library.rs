#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineLibraryCreateInfoKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub library_count: u32,
    pub p_libraries: *const Pipeline,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineLibraryCreateInfoKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_LIBRARY_CREATE_INFO_KHR,
            p_next: core::ptr::null(),
            library_count: Default::default(),
            p_libraries: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PipelineLibraryCreateInfoKHR<'a> {
    pub fn libraries(mut self, libraries: &'a [Pipeline]) -> Self {
        self.library_count = libraries.len().try_into().unwrap();
        self.p_libraries = libraries.as_ptr();
        self
    }
}
