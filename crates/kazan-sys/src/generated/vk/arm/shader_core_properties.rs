#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderCorePropertiesARM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub pixel_rate: u32,
    pub texel_rate: u32,
    pub fma_rate: u32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderCorePropertiesARM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_ARM,
            p_next: core::ptr::null_mut(),
            pixel_rate: Default::default(),
            texel_rate: Default::default(),
            fma_rate: Default::default(),
            _marker: PhantomData,
        }
    }
}
