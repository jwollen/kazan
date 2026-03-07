//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_corner_sampled_image.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_corner_sampled_image";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCornerSampledImageFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCornerSampledImageFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub corner_sampled_image: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCornerSampledImageFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCornerSampledImageFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("corner_sampled_image", &self.corner_sampled_image)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCornerSampledImageFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCornerSampledImageFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCornerSampledImageFeaturesNV<'a> {}

    impl Default for PhysicalDeviceCornerSampledImageFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                corner_sampled_image: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCornerSampledImageFeaturesNV<'a> {
        #[inline]
        pub fn corner_sampled_image(mut self, corner_sampled_image: bool) -> Self {
            self.corner_sampled_image = corner_sampled_image.into();
            self
        }
    }
}
