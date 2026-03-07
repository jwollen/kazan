//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_ray_query.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_ray_query";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRayQueryFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRayQueryFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_query: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRayQueryFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRayQueryFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ray_query", &self.ray_query)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayQueryFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PHYSICAL_DEVICE_RAY_QUERY_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceRayQueryFeaturesKHR<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceRayQueryFeaturesKHR<'a> {}

    impl Default for PhysicalDeviceRayQueryFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                ray_query: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRayQueryFeaturesKHR<'a> {
        #[inline]
        pub fn ray_query(mut self, ray_query: bool) -> Self {
            self.ray_query = ray_query.into();
            self
        }
    }
}
