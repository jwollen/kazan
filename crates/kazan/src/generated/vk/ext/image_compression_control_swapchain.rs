//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_image_compression_control_swapchain.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_image_compression_control_swapchain";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_compression_control_swapchain: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "image_compression_control_swapchain",
                    &self.image_compression_control_swapchain,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                image_compression_control_swapchain: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'a> {
        #[inline]
        pub fn image_compression_control_swapchain(
            mut self,
            image_compression_control_swapchain: bool,
        ) -> Self {
            self.image_compression_control_swapchain = image_compression_control_swapchain.into();
            self
        }
    }
}
