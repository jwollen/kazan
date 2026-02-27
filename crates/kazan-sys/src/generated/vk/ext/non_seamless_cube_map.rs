#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub non_seamless_cube_map: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_NON_SEAMLESS_CUBE_MAP_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            non_seamless_cube_map: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceNonSeamlessCubeMapFeaturesEXT<'a> {
    pub fn non_seamless_cube_map(mut self, non_seamless_cube_map: Bool32) -> Self {
        self.non_seamless_cube_map = non_seamless_cube_map;
        self
    }
}
