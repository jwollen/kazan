#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{c_char, c_int, c_void};
use core::marker::PhantomData;
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            shader_module_identifier: Default::default(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceShaderModuleIdentifierPropertiesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub shader_module_identifier_algorithm_uuid: [u8; UUID_SIZE as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceShaderModuleIdentifierPropertiesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT,
            p_next: core::ptr::null_mut(),
            shader_module_identifier_algorithm_uuid: [Default::default(); _],
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PipelineShaderStageModuleIdentifierCreateInfoEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *const c_void,
    pub identifier_size: u32,
    pub p_identifier: *const u8,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PipelineShaderStageModuleIdentifierCreateInfoEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT,
            p_next: core::ptr::null(),
            identifier_size: Default::default(),
            p_identifier: core::ptr::null(),
            _marker: PhantomData,
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ShaderModuleIdentifierEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub identifier_size: u32,
    pub identifier: [u8; MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT as usize],
    pub _marker: PhantomData<&'a ()>,
}
impl Default for ShaderModuleIdentifierEXT<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::SHADER_MODULE_IDENTIFIER_EXT,
            p_next: core::ptr::null_mut(),
            identifier_size: Default::default(),
            identifier: [Default::default(); _],
            _marker: PhantomData,
        }
    }
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
