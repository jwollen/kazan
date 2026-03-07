//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_dedicated_allocation_image_aliasing.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_dedicated_allocation_image_aliasing";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dedicated_allocation_image_aliasing: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "dedicated_allocation_image_aliasing",
                    &self.dedicated_allocation_image_aliasing,
                )
                .finish()
        }
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
        #[inline]
        pub fn dedicated_allocation_image_aliasing(
            mut self,
            dedicated_allocation_image_aliasing: bool,
        ) -> Self {
            self.dedicated_allocation_image_aliasing = dedicated_allocation_image_aliasing.into();
            self
        }
    }
}
