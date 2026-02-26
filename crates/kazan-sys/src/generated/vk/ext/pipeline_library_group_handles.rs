#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pipeline_library_group_handles: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_LIBRARY_GROUP_HANDLES_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            pipeline_library_group_handles: Default::default(),
            _marker: PhantomData,
        }
    }
}
