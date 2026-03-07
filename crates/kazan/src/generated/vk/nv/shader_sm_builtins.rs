#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_shader_sm_builtins";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderSMBuiltinsPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderSMBuiltinsPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_sm_count: u32,
        pub shader_warps_per_sm: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderSMBuiltinsPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderSMBuiltinsPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_sm_count", &self.shader_sm_count)
                .field("shader_warps_per_sm", &self.shader_warps_per_sm)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderSMBuiltinsPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderSMBuiltinsPropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceShaderSMBuiltinsPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_sm_count: Default::default(),
                shader_warps_per_sm: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderSMBuiltinsPropertiesNV<'a> {
        #[inline]
        pub fn shader_sm_count(mut self, shader_sm_count: u32) -> Self {
            self.shader_sm_count = shader_sm_count;
            self
        }

        #[inline]
        pub fn shader_warps_per_sm(mut self, shader_warps_per_sm: u32) -> Self {
            self.shader_warps_per_sm = shader_warps_per_sm;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderSMBuiltinsFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_sm_builtins: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderSMBuiltinsFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderSMBuiltinsFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_sm_builtins", &self.shader_sm_builtins)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_SM_BUILTINS_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderSMBuiltinsFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {}

    impl Default for PhysicalDeviceShaderSMBuiltinsFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_sm_builtins: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderSMBuiltinsFeaturesNV<'a> {
        #[inline]
        pub fn shader_sm_builtins(mut self, shader_sm_builtins: bool) -> Self {
            self.shader_sm_builtins = shader_sm_builtins.into();
            self
        }
    }
}
