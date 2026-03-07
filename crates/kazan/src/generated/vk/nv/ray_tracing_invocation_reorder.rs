//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_ray_tracing_invocation_reorder.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_ray_tracing_invocation_reorder";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRayTracingInvocationReorderModeNV.html>
    pub type RayTracingInvocationReorderModeNV = RayTracingInvocationReorderModeEXT;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_invocation_reorder: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRayTracingInvocationReorderFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "ray_tracing_invocation_reorder",
                    &self.ray_tracing_invocation_reorder,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                ray_tracing_invocation_reorder: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {
        #[inline]
        pub fn ray_tracing_invocation_reorder(
            mut self,
            ray_tracing_invocation_reorder: bool,
        ) -> Self {
            self.ray_tracing_invocation_reorder = ray_tracing_invocation_reorder.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRayTracingInvocationReorderPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "ray_tracing_invocation_reorder_reordering_hint",
                    &self.ray_tracing_invocation_reorder_reordering_hint,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_NV;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a>
    {
    }

    impl Default for PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                ray_tracing_invocation_reorder_reordering_hint: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {
        #[inline]
        pub fn ray_tracing_invocation_reorder_reordering_hint(
            mut self,
            ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
        ) -> Self {
            self.ray_tracing_invocation_reorder_reordering_hint =
                ray_tracing_invocation_reorder_reordering_hint;
            self
        }
    }
}
