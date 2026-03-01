#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub advanced_blend_coherent_operations: Bool32,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn advanced_blend_coherent_operations(
            mut self,
            advanced_blend_coherent_operations: Bool32,
        ) -> Self {
            self.advanced_blend_coherent_operations = advanced_blend_coherent_operations;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
        pub fn advanced_blend_max_color_attachments(
            mut self,
            advanced_blend_max_color_attachments: u32,
        ) -> Self {
            self.advanced_blend_max_color_attachments = advanced_blend_max_color_attachments;
            self
        }
        pub fn advanced_blend_independent_blend(
            mut self,
            advanced_blend_independent_blend: Bool32,
        ) -> Self {
            self.advanced_blend_independent_blend = advanced_blend_independent_blend;
            self
        }
        pub fn advanced_blend_non_premultiplied_src_color(
            mut self,
            advanced_blend_non_premultiplied_src_color: Bool32,
        ) -> Self {
            self.advanced_blend_non_premultiplied_src_color =
                advanced_blend_non_premultiplied_src_color;
            self
        }
        pub fn advanced_blend_non_premultiplied_dst_color(
            mut self,
            advanced_blend_non_premultiplied_dst_color: Bool32,
        ) -> Self {
            self.advanced_blend_non_premultiplied_dst_color =
                advanced_blend_non_premultiplied_dst_color;
            self
        }
        pub fn advanced_blend_correlated_overlap(
            mut self,
            advanced_blend_correlated_overlap: Bool32,
        ) -> Self {
            self.advanced_blend_correlated_overlap = advanced_blend_correlated_overlap;
            self
        }
        pub fn advanced_blend_all_operations(
            mut self,
            advanced_blend_all_operations: Bool32,
        ) -> Self {
            self.advanced_blend_all_operations = advanced_blend_all_operations;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PipelineColorBlendAdvancedStateCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub src_premultiplied: Bool32,
        pub dst_premultiplied: Bool32,
        pub blend_overlap: BlendOverlapEXT,
        pub _marker: PhantomData<&'a ()>,
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
        pub fn src_premultiplied(mut self, src_premultiplied: Bool32) -> Self {
            self.src_premultiplied = src_premultiplied;
            self
        }
        pub fn dst_premultiplied(mut self, dst_premultiplied: Bool32) -> Self {
            self.dst_premultiplied = dst_premultiplied;
            self
        }
        pub fn blend_overlap(mut self, blend_overlap: BlendOverlapEXT) -> Self {
            self.blend_overlap = blend_overlap;
            self
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BlendOverlapEXT(i32);
    impl BlendOverlapEXT {
        pub const UNCORRELATED_EXT: Self = Self(0);
        pub const DISJOINT_EXT: Self = Self(1);
        pub const CONJOINT_EXT: Self = Self(2);
    }
}
