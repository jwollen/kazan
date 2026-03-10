//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_ray_tracing_validation.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_ray_tracing_validation";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRayTracingValidationFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRayTracingValidationFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_validation: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRayTracingValidationFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRayTracingValidationFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("ray_tracing_validation", &self.ray_tracing_validation)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingValidationFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_VALIDATION_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceRayTracingValidationFeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceRayTracingValidationFeaturesNV<'_> {}

    impl Default for PhysicalDeviceRayTracingValidationFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                ray_tracing_validation: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRayTracingValidationFeaturesNV<'a> {
        #[inline]
        pub fn ray_tracing_validation(mut self, ray_tracing_validation: bool) -> Self {
            self.ray_tracing_validation = ray_tracing_validation.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceRayTracingValidationFeaturesNV =
        PhysicalDeviceRayTracingValidationFeaturesNV<'static>;
    impl PhysicalDeviceRayTracingValidationFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceRayTracingValidationFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
