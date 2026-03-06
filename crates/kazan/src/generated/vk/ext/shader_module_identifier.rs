#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_module_identifier: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderModuleIdentifierFeaturesEXT<'a> {
        pub fn shader_module_identifier(mut self, shader_module_identifier: Bool32) -> Self {
            self.shader_module_identifier = shader_module_identifier;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderModuleIdentifierPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_MODULE_IDENTIFIER_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderModuleIdentifierPropertiesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceShaderModuleIdentifierPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_module_identifier_algorithm_uuid: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderModuleIdentifierPropertiesEXT<'a> {
        pub fn shader_module_identifier_algorithm_uuid(
            mut self,
            shader_module_identifier_algorithm_uuid: [u8; UUID_SIZE as usize],
        ) -> Self {
            self.shader_module_identifier_algorithm_uuid = shader_module_identifier_algorithm_uuid;
            self
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
    unsafe impl<'a> TaggedStructure<'a> for PipelineShaderStageModuleIdentifierCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_SHADER_STAGE_MODULE_IDENTIFIER_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<PipelineShaderStageCreateInfo<'a>>
        for PipelineShaderStageModuleIdentifierCreateInfoEXT<'a>
    {
    }
    impl Default for PipelineShaderStageModuleIdentifierCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                identifier_size: Default::default(),
                p_identifier: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineShaderStageModuleIdentifierCreateInfoEXT<'a> {
        pub fn identifier(mut self, identifier: &'a [u8]) -> Self {
            self.identifier_size = identifier.len().try_into().unwrap();
            self.p_identifier = identifier.as_ptr();
            self
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
    unsafe impl<'a> TaggedStructure<'a> for ShaderModuleIdentifierEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SHADER_MODULE_IDENTIFIER_EXT;
    }
    impl Default for ShaderModuleIdentifierEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                identifier_size: Default::default(),
                identifier: [Default::default(); _],
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ShaderModuleIdentifierEXT<'a> {
        pub fn identifier(mut self, identifier: &[u8]) -> Self {
            self.identifier_size = identifier.len().try_into().unwrap();
            self.identifier[..identifier.len()].copy_from_slice(identifier);
            self
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
}
pub struct DeviceFn {
    get_shader_module_identifier_ext: PFN_vkGetShaderModuleIdentifierEXT,
    get_shader_module_create_info_identifier_ext: PFN_vkGetShaderModuleCreateInfoIdentifierEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_shader_module_identifier_ext: transmute(
                    load(c"vkGetShaderModuleIdentifierEXT").ok_or(MissingEntryPointError)?,
                ),
                get_shader_module_create_info_identifier_ext: transmute(
                    load(c"vkGetShaderModuleCreateInfoIdentifierEXT")
                        .ok_or(MissingEntryPointError)?,
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
        identifier: &mut ShaderModuleIdentifierEXT<'_>,
    ) {
        unsafe { (self.get_shader_module_identifier_ext)(device, shader_module, identifier) }
    }
    pub unsafe fn get_shader_module_create_info_identifier_ext(
        &self,
        device: Device,
        create_info: &ShaderModuleCreateInfo<'_>,
        identifier: &mut ShaderModuleIdentifierEXT<'_>,
    ) {
        unsafe {
            (self.get_shader_module_create_info_identifier_ext)(device, create_info, identifier)
        }
    }
}
