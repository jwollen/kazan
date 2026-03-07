//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_shader_core_properties2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_shader_core_properties2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderCoreProperties2AMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderCoreProperties2AMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_core_features: ShaderCorePropertiesFlagsAMD,
        pub active_compute_unit_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderCoreProperties2AMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderCoreProperties2AMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_core_features", &self.shader_core_features)
                .field("active_compute_unit_count", &self.active_compute_unit_count)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderCoreProperties2AMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceShaderCoreProperties2AMD<'a>
    {
    }

    impl Default for PhysicalDeviceShaderCoreProperties2AMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_core_features: Default::default(),
                active_compute_unit_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderCoreProperties2AMD<'a> {
        #[inline]
        pub fn shader_core_features(
            mut self,
            shader_core_features: ShaderCorePropertiesFlagsAMD,
        ) -> Self {
            self.shader_core_features = shader_core_features;
            self
        }

        #[inline]
        pub fn active_compute_unit_count(mut self, active_compute_unit_count: u32) -> Self {
            self.active_compute_unit_count = active_compute_unit_count;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderCorePropertiesFlagsAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct ShaderCorePropertiesFlagsAMD(Flags);
    vk_bitflags_wrapped!(ShaderCorePropertiesFlagsAMD, Flags);

    impl ShaderCorePropertiesFlagsAMD {}

    impl fmt::Debug for ShaderCorePropertiesFlagsAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkShaderCorePropertiesFlagBitsAMD.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ShaderCorePropertiesFlagBitsAMD(u32);

    impl ShaderCorePropertiesFlagBitsAMD {}

    impl fmt::Debug for ShaderCorePropertiesFlagBitsAMD {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}
