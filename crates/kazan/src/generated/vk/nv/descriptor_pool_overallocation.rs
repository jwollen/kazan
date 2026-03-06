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
    pub struct PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub descriptor_pool_overallocation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DESCRIPTOR_POOL_OVERALLOCATION_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a>
    {
    }
    impl Default for PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                descriptor_pool_overallocation: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDescriptorPoolOverallocationFeaturesNV<'a> {
        pub fn descriptor_pool_overallocation(
            mut self,
            descriptor_pool_overallocation: bool,
        ) -> Self {
            self.descriptor_pool_overallocation = descriptor_pool_overallocation.into();
            self
        }
    }
}
