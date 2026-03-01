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
    pub struct PhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_coherent_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCoherentMemoryFeaturesAMD<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCoherentMemoryFeaturesAMD<'a> {}
    impl Default for PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                device_coherent_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
        pub fn device_coherent_memory(mut self, device_coherent_memory: Bool32) -> Self {
            self.device_coherent_memory = device_coherent_memory;
            self
        }
    }
}
