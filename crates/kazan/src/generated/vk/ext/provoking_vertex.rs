//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_provoking_vertex.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_provoking_vertex";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceProvokingVertexFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceProvokingVertexFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub provoking_vertex_last: Bool32,
        pub transform_feedback_preserves_provoking_vertex: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceProvokingVertexFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceProvokingVertexFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("provoking_vertex_last", &self.provoking_vertex_last)
                .field(
                    "transform_feedback_preserves_provoking_vertex",
                    &self.transform_feedback_preserves_provoking_vertex,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceProvokingVertexFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceProvokingVertexFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceProvokingVertexFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceProvokingVertexFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                provoking_vertex_last: Default::default(),
                transform_feedback_preserves_provoking_vertex: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceProvokingVertexFeaturesEXT<'a> {
        #[inline]
        pub fn provoking_vertex_last(mut self, provoking_vertex_last: bool) -> Self {
            self.provoking_vertex_last = provoking_vertex_last.into();
            self
        }

        #[inline]
        pub fn transform_feedback_preserves_provoking_vertex(
            mut self,
            transform_feedback_preserves_provoking_vertex: bool,
        ) -> Self {
            self.transform_feedback_preserves_provoking_vertex =
                transform_feedback_preserves_provoking_vertex.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceProvokingVertexPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceProvokingVertexPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub provoking_vertex_mode_per_pipeline: Bool32,
        pub transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceProvokingVertexPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceProvokingVertexPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "provoking_vertex_mode_per_pipeline",
                    &self.provoking_vertex_mode_per_pipeline,
                )
                .field(
                    "transform_feedback_preserves_triangle_fan_provoking_vertex",
                    &self.transform_feedback_preserves_triangle_fan_provoking_vertex,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceProvokingVertexPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PROVOKING_VERTEX_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceProvokingVertexPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceProvokingVertexPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                provoking_vertex_mode_per_pipeline: Default::default(),
                transform_feedback_preserves_triangle_fan_provoking_vertex: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceProvokingVertexPropertiesEXT<'a> {
        #[inline]
        pub fn provoking_vertex_mode_per_pipeline(
            mut self,
            provoking_vertex_mode_per_pipeline: bool,
        ) -> Self {
            self.provoking_vertex_mode_per_pipeline = provoking_vertex_mode_per_pipeline.into();
            self
        }

        #[inline]
        pub fn transform_feedback_preserves_triangle_fan_provoking_vertex(
            mut self,
            transform_feedback_preserves_triangle_fan_provoking_vertex: bool,
        ) -> Self {
            self.transform_feedback_preserves_triangle_fan_provoking_vertex =
                transform_feedback_preserves_triangle_fan_provoking_vertex.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineRasterizationProvokingVertexStateCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub provoking_vertex_mode: ProvokingVertexModeEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineRasterizationProvokingVertexStateCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineRasterizationProvokingVertexStateCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("provoking_vertex_mode", &self.provoking_vertex_mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineRasterizationProvokingVertexStateCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_RASTERIZATION_PROVOKING_VERTEX_STATE_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<PipelineRasterizationStateCreateInfo<'a>>
        for PipelineRasterizationProvokingVertexStateCreateInfoEXT<'a>
    {
    }

    impl Default for PipelineRasterizationProvokingVertexStateCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                provoking_vertex_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineRasterizationProvokingVertexStateCreateInfoEXT<'a> {
        #[inline]
        pub fn provoking_vertex_mode(
            mut self,
            provoking_vertex_mode: ProvokingVertexModeEXT,
        ) -> Self {
            self.provoking_vertex_mode = provoking_vertex_mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkProvokingVertexModeEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ProvokingVertexModeEXT(i32);

    impl ProvokingVertexModeEXT {
        pub const FIRST_VERTEX_EXT: Self = Self(0);
        pub const LAST_VERTEX_EXT: Self = Self(1);
    }

    impl fmt::Debug for ProvokingVertexModeEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::FIRST_VERTEX_EXT => Some("FIRST_VERTEX_EXT"),
                Self::LAST_VERTEX_EXT => Some("LAST_VERTEX_EXT"),
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
