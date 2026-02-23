#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
#[repr(C)]
pub struct PhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier: Bool32,
}
#[repr(C)]
pub struct PhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier_algorithm_uuid: [u8; UUID_SIZE as usize],
}
#[repr(C)]
pub struct PipelineShaderStageModuleIdentifierCreateInfoEXT {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
}
#[repr(C)]
pub struct ShaderModuleIdentifierEXT {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub identifier_size: u32,
    pub identifier: [u8; MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT as usize],
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
