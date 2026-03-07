//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_blend_operation_advanced.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_blend_operation_advanced";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub advanced_blend_coherent_operations: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceBlendOperationAdvancedFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "advanced_blend_coherent_operations",
                    &self.advanced_blend_coherent_operations,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                advanced_blend_coherent_operations: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'a> {
        #[inline]
        pub fn advanced_blend_coherent_operations(
            mut self,
            advanced_blend_coherent_operations: bool,
        ) -> Self {
            self.advanced_blend_coherent_operations = advanced_blend_coherent_operations.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub advanced_blend_max_color_attachments: u32,
        pub advanced_blend_independent_blend: Bool32,
        pub advanced_blend_non_premultiplied_src_color: Bool32,
        pub advanced_blend_non_premultiplied_dst_color: Bool32,
        pub advanced_blend_correlated_overlap: Bool32,
        pub advanced_blend_all_operations: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceBlendOperationAdvancedPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "advanced_blend_max_color_attachments",
                    &self.advanced_blend_max_color_attachments,
                )
                .field(
                    "advanced_blend_independent_blend",
                    &self.advanced_blend_independent_blend,
                )
                .field(
                    "advanced_blend_non_premultiplied_src_color",
                    &self.advanced_blend_non_premultiplied_src_color,
                )
                .field(
                    "advanced_blend_non_premultiplied_dst_color",
                    &self.advanced_blend_non_premultiplied_dst_color,
                )
                .field(
                    "advanced_blend_correlated_overlap",
                    &self.advanced_blend_correlated_overlap,
                )
                .field(
                    "advanced_blend_all_operations",
                    &self.advanced_blend_all_operations,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_BLEND_OPERATION_ADVANCED_PROPERTIES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                advanced_blend_max_color_attachments: Default::default(),
                advanced_blend_independent_blend: Default::default(),
                advanced_blend_non_premultiplied_src_color: Default::default(),
                advanced_blend_non_premultiplied_dst_color: Default::default(),
                advanced_blend_correlated_overlap: Default::default(),
                advanced_blend_all_operations: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceBlendOperationAdvancedPropertiesEXT<'a> {
        #[inline]
        pub fn advanced_blend_max_color_attachments(
            mut self,
            advanced_blend_max_color_attachments: u32,
        ) -> Self {
            self.advanced_blend_max_color_attachments = advanced_blend_max_color_attachments;
            self
        }

        #[inline]
        pub fn advanced_blend_independent_blend(
            mut self,
            advanced_blend_independent_blend: bool,
        ) -> Self {
            self.advanced_blend_independent_blend = advanced_blend_independent_blend.into();
            self
        }

        #[inline]
        pub fn advanced_blend_non_premultiplied_src_color(
            mut self,
            advanced_blend_non_premultiplied_src_color: bool,
        ) -> Self {
            self.advanced_blend_non_premultiplied_src_color =
                advanced_blend_non_premultiplied_src_color.into();
            self
        }

        #[inline]
        pub fn advanced_blend_non_premultiplied_dst_color(
            mut self,
            advanced_blend_non_premultiplied_dst_color: bool,
        ) -> Self {
            self.advanced_blend_non_premultiplied_dst_color =
                advanced_blend_non_premultiplied_dst_color.into();
            self
        }

        #[inline]
        pub fn advanced_blend_correlated_overlap(
            mut self,
            advanced_blend_correlated_overlap: bool,
        ) -> Self {
            self.advanced_blend_correlated_overlap = advanced_blend_correlated_overlap.into();
            self
        }

        #[inline]
        pub fn advanced_blend_all_operations(
            mut self,
            advanced_blend_all_operations: bool,
        ) -> Self {
            self.advanced_blend_all_operations = advanced_blend_all_operations.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPipelineColorBlendAdvancedStateCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_premultiplied: Bool32,
        pub dst_premultiplied: Bool32,
        pub blend_overlap: BlendOverlapEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PipelineColorBlendAdvancedStateCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PipelineColorBlendAdvancedStateCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("src_premultiplied", &self.src_premultiplied)
                .field("dst_premultiplied", &self.dst_premultiplied)
                .field("blend_overlap", &self.blend_overlap)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PIPELINE_COLOR_BLEND_ADVANCED_STATE_CREATE_INFO_EXT;
    }

    unsafe impl<'a> Extends<PipelineColorBlendStateCreateInfo<'a>>
        for PipelineColorBlendAdvancedStateCreateInfoEXT<'a>
    {
    }

    impl Default for PipelineColorBlendAdvancedStateCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                src_premultiplied: Default::default(),
                dst_premultiplied: Default::default(),
                blend_overlap: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
        #[inline]
        pub fn src_premultiplied(mut self, src_premultiplied: bool) -> Self {
            self.src_premultiplied = src_premultiplied.into();
            self
        }

        #[inline]
        pub fn dst_premultiplied(mut self, dst_premultiplied: bool) -> Self {
            self.dst_premultiplied = dst_premultiplied.into();
            self
        }

        #[inline]
        pub fn blend_overlap(mut self, blend_overlap: BlendOverlapEXT) -> Self {
            self.blend_overlap = blend_overlap;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkBlendOverlapEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BlendOverlapEXT(i32);

    impl BlendOverlapEXT {
        pub const UNCORRELATED_EXT: Self = Self(0);
        pub const DISJOINT_EXT: Self = Self(1);
        pub const CONJOINT_EXT: Self = Self(2);
    }

    impl fmt::Debug for BlendOverlapEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::UNCORRELATED_EXT => Some("UNCORRELATED_EXT"),
                Self::DISJOINT_EXT => Some("DISJOINT_EXT"),
                Self::CONJOINT_EXT => Some("CONJOINT_EXT"),
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
