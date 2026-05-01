//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_maintenance11.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_maintenance11";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMaintenance11FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMaintenance11FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub maintenance11: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMaintenance11FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMaintenance11FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("maintenance11", &self.maintenance11)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMaintenance11FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MAINTENANCE_11_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceMaintenance11FeaturesKHR<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceMaintenance11FeaturesKHR<'_> {}

    impl Default for PhysicalDeviceMaintenance11FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                maintenance11: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMaintenance11FeaturesKHR<'a> {
        #[inline]
        pub fn maintenance11(mut self, maintenance11: bool) -> Self {
            self.maintenance11 = maintenance11.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueueFamilyOptimalImageTransferGranularityPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub optimal_image_transfer_granularity: Extent3D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueueFamilyOptimalImageTransferGranularityPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueueFamilyOptimalImageTransferGranularityPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "optimal_image_transfer_granularity",
                    &self.optimal_image_transfer_granularity,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for QueueFamilyOptimalImageTransferGranularityPropertiesKHR<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::QUEUE_FAMILY_OPTIMAL_IMAGE_TRANSFER_GRANULARITY_PROPERTIES_KHR;
    }

    unsafe impl Extends<QueueFamilyProperties2<'_>>
        for QueueFamilyOptimalImageTransferGranularityPropertiesKHR<'_>
    {
    }

    impl Default for QueueFamilyOptimalImageTransferGranularityPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                optimal_image_transfer_granularity: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueueFamilyOptimalImageTransferGranularityPropertiesKHR<'a> {
        #[inline]
        pub fn optimal_image_transfer_granularity(
            mut self,
            optimal_image_transfer_granularity: Extent3D,
        ) -> Self {
            self.optimal_image_transfer_granularity = optimal_image_transfer_granularity;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceMaintenance11FeaturesKHR =
        PhysicalDeviceMaintenance11FeaturesKHR<'static>;
    pub type VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR =
        QueueFamilyOptimalImageTransferGranularityPropertiesKHR<'static>;
    impl PhysicalDeviceMaintenance11FeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceMaintenance11FeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl QueueFamilyOptimalImageTransferGranularityPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
