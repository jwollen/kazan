//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_ycbcr_2plane_444_formats.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_ycbcr_2plane_444_formats";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ycbcr2plane444_formats: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ycbcr2plane444_formats", &self.ycbcr2plane444_formats)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_YCBCR_2_PLANE_444_FORMATS_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                ycbcr2plane444_formats: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'a> {
        #[inline]
        pub fn ycbcr2plane444_formats(mut self, ycbcr2plane444_formats: bool) -> Self {
            self.ycbcr2plane444_formats = ycbcr2plane444_formats.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT =
        PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'static>;
    impl PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
