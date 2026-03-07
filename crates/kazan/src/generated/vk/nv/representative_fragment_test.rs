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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub representative_fragment_test: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                representative_fragment_test: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {
        pub fn representative_fragment_test(mut self, representative_fragment_test: bool) -> Self {
            self.representative_fragment_test = representative_fragment_test.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRepresentativeFragmentTestStateCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub representative_fragment_test_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>>
        for PipelineRepresentativeFragmentTestStateCreateInfoNV<'a>
    {
    }

    impl Default for PipelineRepresentativeFragmentTestStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                representative_fragment_test_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {
        pub fn representative_fragment_test_enable(
            mut self,
            representative_fragment_test_enable: bool,
        ) -> Self {
            self.representative_fragment_test_enable = representative_fragment_test_enable.into();
            self
        }
    }
}
