#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_IMG_relaxed_line_rasterization";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
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

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a>
    {
    }

    impl Default for PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                relaxed_line_rasterization: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRelaxedLineRasterizationFeaturesIMG<'a> {
        pub fn relaxed_line_rasterization(mut self, relaxed_line_rasterization: bool) -> Self {
            self.relaxed_line_rasterization = relaxed_line_rasterization.into();
            self
        }
    }
}
