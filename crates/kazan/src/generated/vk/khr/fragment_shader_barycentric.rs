#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_shader_barycentric: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                fragment_shader_barycentric: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'a> {
        pub fn fragment_shader_barycentric(mut self, fragment_shader_barycentric: Bool32) -> Self {
            self.fragment_shader_barycentric = fragment_shader_barycentric;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tri_strip_vertex_order_independent_of_provoking_vertex: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR,
                p_next: core::ptr::null_mut(),
                tri_strip_vertex_order_independent_of_provoking_vertex: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'a> {
        pub fn tri_strip_vertex_order_independent_of_provoking_vertex(
            mut self,
            tri_strip_vertex_order_independent_of_provoking_vertex: Bool32,
        ) -> Self {
            self.tri_strip_vertex_order_independent_of_provoking_vertex =
                tri_strip_vertex_order_independent_of_provoking_vertex;
            self
        }
    }
}
