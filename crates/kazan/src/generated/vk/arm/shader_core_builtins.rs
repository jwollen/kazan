//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_shader_core_builtins.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_shader_core_builtins";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_core_mask: u64,
        pub shader_core_count: u32,
        pub shader_warps_per_core: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderCoreBuiltinsPropertiesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_core_mask", &self.shader_core_mask)
                .field("shader_core_count", &self.shader_core_count)
                .field("shader_warps_per_core", &self.shader_warps_per_core)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_PROPERTIES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a>
    {
    }

    impl Default for PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_core_mask: Default::default(),
                shader_core_count: Default::default(),
                shader_warps_per_core: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'a> {
        #[inline]
        pub fn shader_core_mask(mut self, shader_core_mask: u64) -> Self {
            self.shader_core_mask = shader_core_mask;
            self
        }

        #[inline]
        pub fn shader_core_count(mut self, shader_core_count: u32) -> Self {
            self.shader_core_count = shader_core_count;
            self
        }

        #[inline]
        pub fn shader_warps_per_core(mut self, shader_warps_per_core: u32) -> Self {
            self.shader_warps_per_core = shader_warps_per_core;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_core_builtins: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderCoreBuiltinsFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_core_builtins", &self.shader_core_builtins)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_CORE_BUILTINS_FEATURES_ARM;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {}

    impl Default for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_core_builtins: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'a> {
        #[inline]
        pub fn shader_core_builtins(mut self, shader_core_builtins: bool) -> Self {
            self.shader_core_builtins = shader_core_builtins.into();
            self
        }
    }
}
