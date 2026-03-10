//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_dedicated_allocation_image_aliasing.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_dedicated_allocation_image_aliasing";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

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

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'_>
    {
    }

    impl Default for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV =
        PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'static>;
    impl PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
