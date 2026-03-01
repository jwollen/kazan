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
    pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dedicated_allocation_image_aliasing: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEDICATED_ALLOCATION_IMAGE_ALIASING_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a>
    {
    }
    impl Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                dedicated_allocation_image_aliasing: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a> {
        pub fn dedicated_allocation_image_aliasing(
            mut self,
            dedicated_allocation_image_aliasing: Bool32,
        ) -> Self {
            self.dedicated_allocation_image_aliasing = dedicated_allocation_image_aliasing;
            self
        }
    }
}
