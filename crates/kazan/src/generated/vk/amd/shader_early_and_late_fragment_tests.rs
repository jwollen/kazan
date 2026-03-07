#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_shader_early_and_late_fragment_tests";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_early_and_late_fragment_tests: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "shader_early_and_late_fragment_tests",
                    &self.shader_early_and_late_fragment_tests,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_EARLY_AND_LATE_FRAGMENT_TESTS_FEATURES_AMD;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a>
    {
    }

    impl Default for PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_early_and_late_fragment_tests: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD<'a> {
        pub fn shader_early_and_late_fragment_tests(
            mut self,
            shader_early_and_late_fragment_tests: bool,
        ) -> Self {
            self.shader_early_and_late_fragment_tests = shader_early_and_late_fragment_tests.into();
            self
        }
    }
}
