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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_maximal_reconvergence: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_SHADER_MAXIMAL_RECONVERGENCE_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                shader_maximal_reconvergence: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR<'a> {
        pub fn shader_maximal_reconvergence(mut self, shader_maximal_reconvergence: bool) -> Self {
            self.shader_maximal_reconvergence = shader_maximal_reconvergence.into();
            self
        }
    }
}
