#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_integer_functions2: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn shader_integer_functions2(mut self, shader_integer_functions2: Bool32) -> Self {
            self.shader_integer_functions2 = shader_integer_functions2;
            self
        }
    }
}
