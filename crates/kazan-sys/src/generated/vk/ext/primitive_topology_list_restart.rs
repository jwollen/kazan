#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub primitive_topology_list_restart: Bool32,
    pub primitive_topology_patch_list_restart: Bool32,
}
