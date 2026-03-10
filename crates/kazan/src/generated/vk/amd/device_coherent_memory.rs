//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMD_device_coherent_memory.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_device_coherent_memory";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCoherentMemoryFeaturesAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub device_coherent_memory: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCoherentMemoryFeaturesAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("device_coherent_memory", &self.device_coherent_memory)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COHERENT_MEMORY_FEATURES_AMD;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {}

    impl Default for PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                device_coherent_memory: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCoherentMemoryFeaturesAMD<'a> {
        #[inline]
        pub fn device_coherent_memory(mut self, device_coherent_memory: bool) -> Self {
            self.device_coherent_memory = device_coherent_memory.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCoherentMemoryFeaturesAMD =
        PhysicalDeviceCoherentMemoryFeaturesAMD<'static>;
    impl PhysicalDeviceCoherentMemoryFeaturesAMD<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceCoherentMemoryFeaturesAMD {
            unsafe { core::mem::transmute(self) }
        }
    }
}
