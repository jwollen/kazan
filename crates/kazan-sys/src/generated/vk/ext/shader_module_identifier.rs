#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use std::ffi::{c_char, c_int, c_void};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier: Bool32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier_algorithm_uuid: u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageModuleIdentifierCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModuleIdentifierEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub identifier_size: u32,
    pub identifier: u8,
}
pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
    device: Device,
    shader_module: ShaderModule,
    p_identifier: *mut ShaderModuleIdentifierEXT,
);
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo,
    p_identifier: *mut ShaderModuleIdentifierEXT,
);
