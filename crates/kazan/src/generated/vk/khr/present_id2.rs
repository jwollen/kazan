//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_present_id2.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_present_id2";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePresentId2FeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePresentId2FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_id2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePresentId2FeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePresentId2FeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_id2", &self.present_id2)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentId2FeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDevicePresentId2FeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevicePresentId2FeaturesKHR<'a> {}

    impl Default for PhysicalDevicePresentId2FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_id2: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePresentId2FeaturesKHR<'a> {
        #[inline]
        pub fn present_id2(mut self, present_id2: bool) -> Self {
            self.present_id2 = present_id2.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentId2KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentId2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_present_ids: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentId2KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentId2KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_present_ids", &self.p_present_ids)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentId2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_ID_2_KHR;
    }

    unsafe impl<'a> Extends<PresentInfoKHR<'a>> for PresentId2KHR<'a> {}

    impl Default for PresentId2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                swapchain_count: Default::default(),
                p_present_ids: ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentId2KHR<'a> {
        #[inline]
        pub fn present_ids(mut self, present_ids: &'a [u64]) -> Self {
            self.swapchain_count = present_ids.len().try_into().unwrap();
            self.p_present_ids = present_ids.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceCapabilitiesPresentId2KHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SurfaceCapabilitiesPresentId2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_id2_supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceCapabilitiesPresentId2KHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceCapabilitiesPresentId2KHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_id2_supported", &self.present_id2_supported)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceCapabilitiesPresentId2KHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_CAPABILITIES_PRESENT_ID_2_KHR;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for SurfaceCapabilitiesPresentId2KHR<'a> {}

    impl Default for SurfaceCapabilitiesPresentId2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_id2_supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceCapabilitiesPresentId2KHR<'a> {
        #[inline]
        pub fn present_id2_supported(mut self, present_id2_supported: bool) -> Self {
            self.present_id2_supported = present_id2_supported.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePresentId2FeaturesKHR = PhysicalDevicePresentId2FeaturesKHR<'static>;
    pub type VkPresentId2KHR = PresentId2KHR<'static>;
    pub type VkSurfaceCapabilitiesPresentId2KHR = SurfaceCapabilitiesPresentId2KHR<'static>;
    impl PhysicalDevicePresentId2FeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDevicePresentId2FeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PresentId2KHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentId2KHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SurfaceCapabilitiesPresentId2KHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSurfaceCapabilitiesPresentId2KHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
