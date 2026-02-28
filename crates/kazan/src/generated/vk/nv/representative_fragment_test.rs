#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub representative_fragment_test: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_REPRESENTATIVE_FRAGMENT_TEST_FEATURES_NV,
                p_next: core::ptr::null_mut(),
                representative_fragment_test: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRepresentativeFragmentTestFeaturesNV<'a> {
        pub fn representative_fragment_test(
            mut self,
            representative_fragment_test: Bool32,
        ) -> Self {
            self.representative_fragment_test = representative_fragment_test;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub representative_fragment_test_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PipelineRepresentativeFragmentTestStateCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PIPELINE_REPRESENTATIVE_FRAGMENT_TEST_STATE_CREATE_INFO_NV,
                p_next: core::ptr::null(),
                representative_fragment_test_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineRepresentativeFragmentTestStateCreateInfoNV<'a> {
        pub fn representative_fragment_test_enable(
            mut self,
            representative_fragment_test_enable: Bool32,
        ) -> Self {
            self.representative_fragment_test_enable = representative_fragment_test_enable;
            self
        }
    }
}
