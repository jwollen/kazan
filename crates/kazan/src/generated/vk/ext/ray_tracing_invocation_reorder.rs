//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_ray_tracing_invocation_reorder.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_ray_tracing_invocation_reorder";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_invocation_reorder: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRayTracingInvocationReorderFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "ray_tracing_invocation_reorder",
                    &self.ray_tracing_invocation_reorder,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                ray_tracing_invocation_reorder: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a> {
        #[inline]
        pub fn ray_tracing_invocation_reorder(
            mut self,
            ray_tracing_invocation_reorder: bool,
        ) -> Self {
            self.ray_tracing_invocation_reorder = ray_tracing_invocation_reorder.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
        pub max_shader_binding_table_record_index: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRayTracingInvocationReorderPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "ray_tracing_invocation_reorder_reordering_hint",
                    &self.ray_tracing_invocation_reorder_reordering_hint,
                )
                .field(
                    "max_shader_binding_table_record_index",
                    &self.max_shader_binding_table_record_index,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAY_TRACING_INVOCATION_REORDER_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                ray_tracing_invocation_reorder_reordering_hint: Default::default(),
                max_shader_binding_table_record_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'a> {
        #[inline]
        pub fn ray_tracing_invocation_reorder_reordering_hint(
            mut self,
            ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
        ) -> Self {
            self.ray_tracing_invocation_reorder_reordering_hint =
                ray_tracing_invocation_reorder_reordering_hint;
            self
        }

        #[inline]
        pub fn max_shader_binding_table_record_index(
            mut self,
            max_shader_binding_table_record_index: u32,
        ) -> Self {
            self.max_shader_binding_table_record_index = max_shader_binding_table_record_index;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRayTracingInvocationReorderModeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RayTracingInvocationReorderModeEXT(i32);

    impl RayTracingInvocationReorderModeEXT {
        pub const NONE_EXT: Self = Self(0);
        pub const REORDER_EXT: Self = Self(1);

        // VK_NV_ray_tracing_invocation_reorder
        pub const NONE_NV: Self = Self::NONE_EXT;
        pub const REORDER_NV: Self = Self::REORDER_EXT;
    }

    impl fmt::Debug for RayTracingInvocationReorderModeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE_EXT => Some("NONE_EXT"),
                Self::REORDER_EXT => Some("REORDER_EXT"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT =
        PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'static>;
    pub type VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT =
        PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'static>;
    pub type VkRayTracingInvocationReorderModeEXT = RayTracingInvocationReorderModeEXT;
    impl PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
