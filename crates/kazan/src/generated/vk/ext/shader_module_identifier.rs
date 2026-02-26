#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
pub struct DeviceFn {
    get_shader_module_identifier_ext: PFN_vkGetShaderModuleIdentifierEXT,
    get_shader_module_create_info_identifier_ext: PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_shader_module_identifier_ext: transmute(
                    load(c"vkGetShaderModuleIdentifierEXT").ok_or(LoadingError)?,
                ),
                get_shader_module_create_info_identifier_ext: transmute(
                    load(c"vkGetShaderModuleCreateInfoIdentifierEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_shader_module_identifier_ext(
        &self,
        device: Device,
        shader_module: ShaderModule,
    ) -> ShaderModuleIdentifierEXT<'_> {
        unsafe {
            let mut identifier = core::mem::MaybeUninit::uninit();
            (self.get_shader_module_identifier_ext)(device, shader_module, identifier.as_mut_ptr());
            identifier.assume_init()
        }
    }
    pub unsafe fn get_shader_module_create_info_identifier_ext(
        &self,
        device: Device,
        create_info: &ShaderModuleCreateInfo<'_>,
    ) -> ShaderModuleIdentifierEXT<'_> {
        unsafe {
            let mut identifier = core::mem::MaybeUninit::uninit();
            (self.get_shader_module_create_info_identifier_ext)(
                device,
                create_info,
                identifier.as_mut_ptr(),
            );
            identifier.assume_init()
        }
    }
}
