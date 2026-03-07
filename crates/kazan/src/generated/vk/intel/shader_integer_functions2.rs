#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_INTEL_shader_integer_functions2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_integer_functions2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_integer_functions2", &self.shader_integer_functions2)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_INTEGER_FUNCTIONS_2_FEATURES_INTEL;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a>
    {
    }

    impl Default for PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_integer_functions2: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {
        #[inline]
        pub fn shader_integer_functions2(mut self, shader_integer_functions2: bool) -> Self {
            self.shader_integer_functions2 = shader_integer_functions2.into();
            self
        }
    }
}
