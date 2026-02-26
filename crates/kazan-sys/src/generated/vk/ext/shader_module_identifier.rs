#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
#[repr(C)]
pub struct PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PhysicalDeviceShaderModuleIdentifierPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier_algorithm_uuid: [u8; UUID_SIZE as usize],
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct PipelineShaderStageModuleIdentifierCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
    pub _marker: PhantomData<&'a ()>,
}
#[repr(C)]
pub struct ShaderModuleIdentifierEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub identifier_size: u32,
    pub identifier: [u8; MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT as usize],
    pub _marker: PhantomData<&'a ()>,
}
pub type PFN_vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
    device: Device,
    shader_module: ShaderModule,
    p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
);
pub type PFN_vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
    device: Device,
    p_create_info: *const ShaderModuleCreateInfo<'_>,
    p_identifier: *mut ShaderModuleIdentifierEXT<'_>,
);
