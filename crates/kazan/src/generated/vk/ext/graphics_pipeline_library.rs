//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_graphics_pipeline_library.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_graphics_pipeline_library";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub graphics_pipeline_library: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("graphics_pipeline_library", &self.graphics_pipeline_library)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                graphics_pipeline_library: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT<'a> {
        #[inline]
        pub fn graphics_pipeline_library(mut self, graphics_pipeline_library: bool) -> Self {
            self.graphics_pipeline_library = graphics_pipeline_library.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub graphics_pipeline_library_fast_linking: Bool32,
        pub graphics_pipeline_library_independent_interpolation_decoration: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "graphics_pipeline_library_fast_linking",
                    &self.graphics_pipeline_library_fast_linking,
                )
                .field(
                    "graphics_pipeline_library_independent_interpolation_decoration",
                    &self.graphics_pipeline_library_independent_interpolation_decoration,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_GRAPHICS_PIPELINE_LIBRARY_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                graphics_pipeline_library_fast_linking: Default::default(),
                graphics_pipeline_library_independent_interpolation_decoration: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT<'a> {
        #[inline]
        pub fn graphics_pipeline_library_fast_linking(
            mut self,
            graphics_pipeline_library_fast_linking: bool,
        ) -> Self {
            self.graphics_pipeline_library_fast_linking =
                graphics_pipeline_library_fast_linking.into();
            self
        }

        #[inline]
        pub fn graphics_pipeline_library_independent_interpolation_decoration(
            mut self,
            graphics_pipeline_library_independent_interpolation_decoration: bool,
        ) -> Self {
            self.graphics_pipeline_library_independent_interpolation_decoration =
                graphics_pipeline_library_independent_interpolation_decoration.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGraphicsPipelineLibraryCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct GraphicsPipelineLibraryCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub flags: GraphicsPipelineLibraryFlagsEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for GraphicsPipelineLibraryCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("GraphicsPipelineLibraryCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("flags", &self.flags)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for GraphicsPipelineLibraryCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::GRAPHICS_PIPELINE_LIBRARY_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>>
        for GraphicsPipelineLibraryCreateInfoEXT<'a>
    {
    }

    impl Default for GraphicsPipelineLibraryCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                flags: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> GraphicsPipelineLibraryCreateInfoEXT<'a> {
        #[inline]
        pub fn flags(mut self, flags: GraphicsPipelineLibraryFlagsEXT) -> Self {
            self.flags = flags;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGraphicsPipelineLibraryFlagsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct GraphicsPipelineLibraryFlagsEXT(Flags);
    vk_bitflags_wrapped!(GraphicsPipelineLibraryFlagsEXT, Flags);

    impl GraphicsPipelineLibraryFlagsEXT {
        pub const VERTEX_INPUT_INTERFACE_EXT: Self =
            Self(GraphicsPipelineLibraryFlagBitsEXT::VERTEX_INPUT_INTERFACE_EXT.0);
        pub const PRE_RASTERIZATION_SHADERS_EXT: Self =
            Self(GraphicsPipelineLibraryFlagBitsEXT::PRE_RASTERIZATION_SHADERS_EXT.0);
        pub const FRAGMENT_SHADER_EXT: Self =
            Self(GraphicsPipelineLibraryFlagBitsEXT::FRAGMENT_SHADER_EXT.0);
        pub const FRAGMENT_OUTPUT_INTERFACE_EXT: Self =
            Self(GraphicsPipelineLibraryFlagBitsEXT::FRAGMENT_OUTPUT_INTERFACE_EXT.0);
    }

    impl fmt::Debug for GraphicsPipelineLibraryFlagsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            const KNOWN: &[(Flags, &str)] = &[
                (
                    GraphicsPipelineLibraryFlagsEXT::VERTEX_INPUT_INTERFACE_EXT.0,
                    "VERTEX_INPUT_INTERFACE_EXT",
                ),
                (
                    GraphicsPipelineLibraryFlagsEXT::PRE_RASTERIZATION_SHADERS_EXT.0,
                    "PRE_RASTERIZATION_SHADERS_EXT",
                ),
                (
                    GraphicsPipelineLibraryFlagsEXT::FRAGMENT_SHADER_EXT.0,
                    "FRAGMENT_SHADER_EXT",
                ),
                (
                    GraphicsPipelineLibraryFlagsEXT::FRAGMENT_OUTPUT_INTERFACE_EXT.0,
                    "FRAGMENT_OUTPUT_INTERFACE_EXT",
                ),
            ];
            debug_flags(f, KNOWN, self.0)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkGraphicsPipelineLibraryFlagBitsEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
    pub struct GraphicsPipelineLibraryFlagBitsEXT(u32);

    impl GraphicsPipelineLibraryFlagBitsEXT {
        pub const VERTEX_INPUT_INTERFACE_EXT: Self = Self(1 << 0);
        pub const PRE_RASTERIZATION_SHADERS_EXT: Self = Self(1 << 1);
        pub const FRAGMENT_SHADER_EXT: Self = Self(1 << 2);
        pub const FRAGMENT_OUTPUT_INTERFACE_EXT: Self = Self(1 << 3);
    }

    impl fmt::Debug for GraphicsPipelineLibraryFlagBitsEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::VERTEX_INPUT_INTERFACE_EXT => Some("VERTEX_INPUT_INTERFACE_EXT"),
                Self::PRE_RASTERIZATION_SHADERS_EXT => Some("PRE_RASTERIZATION_SHADERS_EXT"),
                Self::FRAGMENT_SHADER_EXT => Some("FRAGMENT_SHADER_EXT"),
                Self::FRAGMENT_OUTPUT_INTERFACE_EXT => Some("FRAGMENT_OUTPUT_INTERFACE_EXT"),
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
