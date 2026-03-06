#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_invocation_reorder: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                ray_tracing_invocation_reorder: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRayTracingInvocationReorderFeaturesEXT<'a> {
        pub fn ray_tracing_invocation_reorder(
            mut self,
            ray_tracing_invocation_reorder: bool,
        ) -> Self {
            self.ray_tracing_invocation_reorder = ray_tracing_invocation_reorder.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
        pub max_shader_binding_table_record_index: u32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                ray_tracing_invocation_reorder_reordering_hint: Default::default(),
                max_shader_binding_table_record_index: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRayTracingInvocationReorderPropertiesEXT<'a> {
        pub fn ray_tracing_invocation_reorder_reordering_hint(
            mut self,
            ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
        ) -> Self {
            self.ray_tracing_invocation_reorder_reordering_hint =
                ray_tracing_invocation_reorder_reordering_hint;
            self
        }
        pub fn max_shader_binding_table_record_index(
            mut self,
            max_shader_binding_table_record_index: u32,
        ) -> Self {
            self.max_shader_binding_table_record_index = max_shader_binding_table_record_index;
            self
        }
    }
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
