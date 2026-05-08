//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_image_processing3.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_image_processing3";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageProcessing3FeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageProcessing3FeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_gather_linear: Bool32,
        pub image_gather_extended_modes: Bool32,
        pub block_match_extended_clamp_to_edge: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageProcessing3FeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageProcessing3FeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_gather_linear", &self.image_gather_linear)
                .field(
                    "image_gather_extended_modes",
                    &self.image_gather_extended_modes,
                )
                .field(
                    "block_match_extended_clamp_to_edge",
                    &self.block_match_extended_clamp_to_edge,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageProcessing3FeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_PROCESSING_3_FEATURES_QCOM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceImageProcessing3FeaturesQCOM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceImageProcessing3FeaturesQCOM<'_> {}

    impl Default for PhysicalDeviceImageProcessing3FeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                image_gather_linear: Default::default(),
                image_gather_extended_modes: Default::default(),
                block_match_extended_clamp_to_edge: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageProcessing3FeaturesQCOM<'a> {
        #[inline]
        pub fn image_gather_linear(mut self, image_gather_linear: bool) -> Self {
            self.image_gather_linear = image_gather_linear.into();
            self
        }

        #[inline]
        pub fn image_gather_extended_modes(mut self, image_gather_extended_modes: bool) -> Self {
            self.image_gather_extended_modes = image_gather_extended_modes.into();
            self
        }

        #[inline]
        pub fn block_match_extended_clamp_to_edge(
            mut self,
            block_match_extended_clamp_to_edge: bool,
        ) -> Self {
            self.block_match_extended_clamp_to_edge = block_match_extended_clamp_to_edge.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceImageProcessing3FeaturesQCOM =
        PhysicalDeviceImageProcessing3FeaturesQCOM<'static>;
    impl PhysicalDeviceImageProcessing3FeaturesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceImageProcessing3FeaturesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
