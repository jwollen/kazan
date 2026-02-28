#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_core_mask: u64,
        pub shader_core_count: u32,
        pub shader_warps_per_core: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM,
                p_next: core::ptr::null_mut(),
                shader_core_mask: Default::default(),
                shader_core_count: Default::default(),
                shader_warps_per_core: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {
        pub fn shader_core_mask(mut self, shader_core_mask: u64) -> Self {
            self.shader_core_mask = shader_core_mask;
            self
        }
        pub fn shader_core_count(mut self, shader_core_count: u32) -> Self {
            self.shader_core_count = shader_core_count;
            self
        }
        pub fn shader_warps_per_core(mut self, shader_warps_per_core: u32) -> Self {
            self.shader_warps_per_core = shader_warps_per_core;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_core_builtins: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM,
                p_next: core::ptr::null_mut(),
                shader_core_builtins: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {
        pub fn shader_core_builtins(mut self, shader_core_builtins: Bool32) -> Self {
            self.shader_core_builtins = shader_core_builtins;
            self
        }
    }
}
