#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderSubgroupPartitionedFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_subgroup_partitioned: Bool32,
}
