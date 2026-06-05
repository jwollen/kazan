//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_multisampled_render_to_swapchain.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_multisampled_render_to_swapchain";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSwapchainFlagsSurfaceCapabilitiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SwapchainFlagsSurfaceCapabilitiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub swapchain_supported_flags: SwapchainCreateFlagsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SwapchainFlagsSurfaceCapabilitiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SwapchainFlagsSurfaceCapabilitiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_supported_flags", &self.swapchain_supported_flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SwapchainFlagsSurfaceCapabilitiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SWAPCHAIN_FLAGS_SURFACE_CAPABILITIES_EXT;
    }

    unsafe impl Extends<SurfaceCapabilities2KHR<'_>> for SwapchainFlagsSurfaceCapabilitiesEXT<'_> {}

    impl Default for SwapchainFlagsSurfaceCapabilitiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                swapchain_supported_flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SwapchainFlagsSurfaceCapabilitiesEXT<'a> {
        #[inline]
        pub fn swapchain_supported_flags(
            mut self,
            swapchain_supported_flags: SwapchainCreateFlagsKHR,
        ) -> Self {
            self.swapchain_supported_flags = swapchain_supported_flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multisampled_render_to_swapchain: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "multisampled_render_to_swapchain",
                    &self.multisampled_render_to_swapchain,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTISAMPLED_RENDER_TO_SWAPCHAIN_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                multisampled_render_to_swapchain: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'a> {
        #[inline]
        pub fn multisampled_render_to_swapchain(
            mut self,
            multisampled_render_to_swapchain: bool,
        ) -> Self {
            self.multisampled_render_to_swapchain = multisampled_render_to_swapchain.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSwapchainFlagsSurfaceCapabilitiesEXT = SwapchainFlagsSurfaceCapabilitiesEXT<'static>;
    pub type VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT =
        PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'static>;
    impl SwapchainFlagsSurfaceCapabilitiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSwapchainFlagsSurfaceCapabilitiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
