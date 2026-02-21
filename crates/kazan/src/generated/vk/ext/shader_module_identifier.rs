#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_shader_module_identifier_ext: PFN_vkGetShaderModuleIdentifierEXT,
    get_shader_module_create_info_identifier_ext: PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
}
impl DeviceFn {
    pub unsafe fn get_shader_module_identifier_ext(
        &self,
        device: Device,
        shader_module: ShaderModule,
        identifier: &mut ShaderModuleIdentifierEXT,
    ) {
        unsafe { (self.get_shader_module_identifier_ext)(device, shader_module, identifier) }
    }
    pub unsafe fn get_shader_module_create_info_identifier_ext(
        &self,
        device: Device,
        create_info: &ShaderModuleCreateInfo,
        identifier: &mut ShaderModuleIdentifierEXT,
    ) {
        unsafe {
            (self.get_shader_module_create_info_identifier_ext)(device, create_info, identifier)
        }
    }
}
