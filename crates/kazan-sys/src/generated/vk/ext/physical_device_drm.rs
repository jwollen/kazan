#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDrmPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub has_primary: Bool32,
    pub has_render: Bool32,
    pub primary_major: i64,
    pub primary_minor: i64,
    pub render_major: i64,
    pub render_minor: i64,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDrmPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_DRM_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            has_primary: Default::default(),
            has_render: Default::default(),
            primary_major: Default::default(),
            primary_minor: Default::default(),
            render_major: Default::default(),
            render_minor: Default::default(),
            _marker: PhantomData,
        }
    }
}
