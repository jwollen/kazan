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
    pub struct PhysicalDeviceProvokingVertexFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub provoking_vertex_last: Bool32,
        pub transform_feedback_preserves_provoking_vertex: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                provoking_vertex_last: Default::default(),
                transform_feedback_preserves_provoking_vertex: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceProvokingVertexFeaturesEXT<'a> {
        pub fn provoking_vertex_last(mut self, provoking_vertex_last: bool) -> Self {
            self.provoking_vertex_last = provoking_vertex_last.into();
            self
        }
        pub fn transform_feedback_preserves_provoking_vertex(
            mut self,
            transform_feedback_preserves_provoking_vertex: bool,
        ) -> Self {
            self.transform_feedback_preserves_provoking_vertex =
                transform_feedback_preserves_provoking_vertex.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceProvokingVertexPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub provoking_vertex_mode_per_pipeline: Bool32,
        pub transform_feedback_preserves_triangle_fan_provoking_vertex: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null_mut(),
                provoking_vertex_mode_per_pipeline: Default::default(),
                transform_feedback_preserves_triangle_fan_provoking_vertex: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceProvokingVertexPropertiesEXT<'a> {
        pub fn provoking_vertex_mode_per_pipeline(
            mut self,
            provoking_vertex_mode_per_pipeline: bool,
        ) -> Self {
            self.provoking_vertex_mode_per_pipeline = provoking_vertex_mode_per_pipeline.into();
            self
        }
        pub fn transform_feedback_preserves_triangle_fan_provoking_vertex(
            mut self,
            transform_feedback_preserves_triangle_fan_provoking_vertex: bool,
        ) -> Self {
            self.transform_feedback_preserves_triangle_fan_provoking_vertex =
                transform_feedback_preserves_triangle_fan_provoking_vertex.into();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub provoking_vertex_mode: ProvokingVertexModeEXT,
        pub _marker: PhantomData<&'a ()>,
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
                p_next: core::ptr::null(),
                provoking_vertex_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PipelineRasterizationProvokingVertexStateCreateInfoEXT<'a> {
        pub fn provoking_vertex_mode(
            mut self,
            provoking_vertex_mode: ProvokingVertexModeEXT,
        ) -> Self {
            self.provoking_vertex_mode = provoking_vertex_mode;
            self
        }
    }
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
