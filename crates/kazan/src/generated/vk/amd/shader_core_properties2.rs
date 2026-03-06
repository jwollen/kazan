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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderCoreProperties2AMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_core_features: ShaderCorePropertiesFlagsAMD,
        pub active_compute_unit_count: u32,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn shader_core_features(
            mut self,
            shader_core_features: ShaderCorePropertiesFlagsAMD,
        ) -> Self {
            self.shader_core_features = shader_core_features;
            self
        }
        pub fn active_compute_unit_count(mut self, active_compute_unit_count: u32) -> Self {
            self.active_compute_unit_count = active_compute_unit_count;
            self
        }
    }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct ShaderCorePropertiesFlagBitsAMD(u32);
    impl ShaderCorePropertiesFlagBitsAMD {}
}
