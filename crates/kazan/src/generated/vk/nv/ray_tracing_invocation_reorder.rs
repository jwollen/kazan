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
    pub type RayTracingInvocationReorderModeNV = RayTracingInvocationReorderModeEXT;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_invocation_reorder: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                ray_tracing_invocation_reorder: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRayTracingInvocationReorderFeaturesNV<'a> {
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
    pub struct PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub ray_tracing_invocation_reorder_reordering_hint: RayTracingInvocationReorderModeEXT,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                ray_tracing_invocation_reorder_reordering_hint: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRayTracingInvocationReorderPropertiesNV<'a> {
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
