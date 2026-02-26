#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitive_topology_list_restart: Bool32,
    pub primitive_topology_patch_list_restart: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            primitive_topology_list_restart: Default::default(),
            primitive_topology_patch_list_restart: Default::default(),
            _marker: PhantomData,
        }
    }
}
