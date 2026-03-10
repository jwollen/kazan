//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_legacy_dithering.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_legacy_dithering";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLegacyDitheringFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceLegacyDitheringFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub legacy_dithering: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceLegacyDitheringFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceLegacyDitheringFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("legacy_dithering", &self.legacy_dithering)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceLegacyDitheringFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_LEGACY_DITHERING_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceLegacyDitheringFeaturesEXT<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceLegacyDitheringFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceLegacyDitheringFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                legacy_dithering: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceLegacyDitheringFeaturesEXT<'a> {
        #[inline]
        pub fn legacy_dithering(mut self, legacy_dithering: bool) -> Self {
            self.legacy_dithering = legacy_dithering.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceLegacyDitheringFeaturesEXT =
        PhysicalDeviceLegacyDitheringFeaturesEXT<'static>;
    impl PhysicalDeviceLegacyDitheringFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceLegacyDitheringFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
