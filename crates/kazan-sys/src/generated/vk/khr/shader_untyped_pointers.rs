#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderUntypedPointersFeaturesKHR<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_untyped_pointers: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderUntypedPointersFeaturesKHR<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_UNTYPED_POINTERS_FEATURES_KHR,
            p_next: core::ptr::null_mut(),
            shader_untyped_pointers: Default::default(),
            _marker: PhantomData,
        }
    }
}
