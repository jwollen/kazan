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
    pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub graphics_pipeline_library: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                graphics_pipeline_library: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'a> {
        pub fn graphics_pipeline_library(mut self, graphics_pipeline_library: Bool32) -> Self {
            self.graphics_pipeline_library = graphics_pipeline_library;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub graphics_pipeline_library_fast_linking: Bool32,
        pub graphics_pipeline_library_independent_interpolation_decoration: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT,
                p_next: core::ptr::null_mut(),
                graphics_pipeline_library_fast_linking: Default::default(),
                graphics_pipeline_library_independent_interpolation_decoration: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'a> {
        pub fn graphics_pipeline_library_fast_linking(
            mut self,
            graphics_pipeline_library_fast_linking: Bool32,
        ) -> Self {
            self.graphics_pipeline_library_fast_linking = graphics_pipeline_library_fast_linking;
            self
        }
        pub fn graphics_pipeline_library_independent_interpolation_decoration(
            mut self,
            graphics_pipeline_library_independent_interpolation_decoration: Bool32,
        ) -> Self {
            self.graphics_pipeline_library_independent_interpolation_decoration =
                graphics_pipeline_library_independent_interpolation_decoration;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GraphicsPipelineLibraryCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: GraphicsPipelineLibraryFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for GraphicsPipelineLibraryCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> GraphicsPipelineLibraryCreateInfoEXT<'a> {
        pub fn flags(mut self, flags: GraphicsPipelineLibraryFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
    }
    bitflags! {
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct GraphicsPipelineLibraryFlagsEXT: Flags {
            const VERTEX_INPUT_INTERFACE_EXT = GraphicsPipelineLibraryFlagBitsEXT::VERTEX_INPUT_INTERFACE_EXT.0;
            const PRE_RASTERIZATION_SHADERS_EXT = GraphicsPipelineLibraryFlagBitsEXT::PRE_RASTERIZATION_SHADERS_EXT.0;
            const FRAGMENT_SHADER_EXT = GraphicsPipelineLibraryFlagBitsEXT::FRAGMENT_SHADER_EXT.0;
            const FRAGMENT_OUTPUT_INTERFACE_EXT = GraphicsPipelineLibraryFlagBitsEXT::FRAGMENT_OUTPUT_INTERFACE_EXT.0;
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct GraphicsPipelineLibraryFlagBitsEXT(u32);
    impl GraphicsPipelineLibraryFlagBitsEXT {
        pub const VERTEX_INPUT_INTERFACE_EXT: Self = Self(1 << 0);
        pub const PRE_RASTERIZATION_SHADERS_EXT: Self = Self(1 << 1);
        pub const FRAGMENT_SHADER_EXT: Self = Self(1 << 2);
        pub const FRAGMENT_OUTPUT_INTERFACE_EXT: Self = Self(1 << 3);
    }
}
