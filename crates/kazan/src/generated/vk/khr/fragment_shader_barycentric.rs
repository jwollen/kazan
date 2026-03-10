//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_fragment_shader_barycentric.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_fragment_shader_barycentric";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub fragment_shader_barycentric: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentShaderBarycentricFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "fragment_shader_barycentric",
                    &self.fragment_shader_barycentric,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_>
    {
    }

    impl Default for PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                fragment_shader_barycentric: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'a> {
        #[inline]
        pub fn fragment_shader_barycentric(mut self, fragment_shader_barycentric: bool) -> Self {
            self.fragment_shader_barycentric = fragment_shader_barycentric.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub tri_strip_vertex_order_independent_of_provoking_vertex: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceFragmentShaderBarycentricPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "tri_strip_vertex_order_independent_of_provoking_vertex",
                    &self.tri_strip_vertex_order_independent_of_provoking_vertex,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_PROPERTIES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceProperties2<'_>>
        for PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'_>
    {
    }

    impl Default for PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                tri_strip_vertex_order_independent_of_provoking_vertex: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'a> {
        #[inline]
        pub fn tri_strip_vertex_order_independent_of_provoking_vertex(
            mut self,
            tri_strip_vertex_order_independent_of_provoking_vertex: bool,
        ) -> Self {
            self.tri_strip_vertex_order_independent_of_provoking_vertex =
                tri_strip_vertex_order_independent_of_provoking_vertex.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR =
        PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'static>;
    pub type VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR =
        PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'static>;
    impl PhysicalDeviceFragmentShaderBarycentricFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceFragmentShaderBarycentricPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
