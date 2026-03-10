//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_image_compression_control_swapchain.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_image_compression_control_swapchain";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

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

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT =
        PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'static>;
    impl PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
