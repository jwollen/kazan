//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_present_id.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_present_id";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePresentIdFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePresentIdFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_id: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePresentIdFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePresentIdFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("present_id", &self.present_id)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentIdFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_ID_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDevicePresentIdFeaturesKHR<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePresentIdFeaturesKHR<'_> {}

    impl Default for PhysicalDevicePresentIdFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_id: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePresentIdFeaturesKHR<'a> {
        #[inline]
        pub fn present_id(mut self, present_id: bool) -> Self {
            self.present_id = present_id.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentIdKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentIdKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_present_ids: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentIdKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentIdKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("swapchain_count", &self.swapchain_count)
                .field("p_present_ids", &self.p_present_ids)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentIdKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_ID_KHR;
    }

    unsafe impl Extends<PresentInfoKHR<'_>> for PresentIdKHR<'_> {}

    impl Default for PresentIdKHR<'_> {
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

    impl<'a> PresentIdKHR<'a> {
        #[inline]
        pub fn present_ids(mut self, present_ids: &'a [u64]) -> Self {
            self.swapchain_count = present_ids.len().try_into().unwrap();
            self.p_present_ids = present_ids.as_ptr() as _;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePresentIdFeaturesKHR = PhysicalDevicePresentIdFeaturesKHR<'static>;
    pub type VkPresentIdKHR = PresentIdKHR<'static>;
    impl PhysicalDevicePresentIdFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDevicePresentIdFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PresentIdKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPresentIdKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
