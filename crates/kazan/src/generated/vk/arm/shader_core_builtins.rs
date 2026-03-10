//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_shader_core_builtins.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_shader_core_builtins";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

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

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'_>
    {
    }

    impl Default for PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {}

    impl Default for PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM =
        PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'static>;
    pub type VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM =
        PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'static>;
    impl PhysicalDeviceShaderCoreBuiltinsPropertiesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceShaderCoreBuiltinsFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
