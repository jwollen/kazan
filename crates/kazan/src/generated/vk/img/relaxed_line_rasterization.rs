//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_IMG_relaxed_line_rasterization.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_IMG_relaxed_line_rasterization";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub relaxed_line_rasterization: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRelaxedLineRasterizationFeaturesIMG")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "relaxed_line_rasterization",
                    &self.relaxed_line_rasterization,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RELAXED_LINE_RASTERIZATION_FEATURES_IMG;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'_>
    {
    }

    impl Default for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                relaxed_line_rasterization: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a> {
        #[inline]
        pub fn relaxed_line_rasterization(mut self, relaxed_line_rasterization: bool) -> Self {
            self.relaxed_line_rasterization = relaxed_line_rasterization.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG =
        PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'static>;
    impl PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG {
            unsafe { core::mem::transmute(self) }
        }
    }
}
