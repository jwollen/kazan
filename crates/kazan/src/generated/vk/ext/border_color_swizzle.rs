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
    pub struct SamplerBorderColorComponentMappingCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub components: ComponentMapping,
        pub srgb: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SamplerBorderColorComponentMappingCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SAMPLER_BORDER_COLOR_COMPONENT_MAPPING_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<SamplerCreateInfo<'a>>
        for SamplerBorderColorComponentMappingCreateInfoEXT<'a>
    {
    }
    impl Default for SamplerBorderColorComponentMappingCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                components: Default::default(),
                srgb: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SamplerBorderColorComponentMappingCreateInfoEXT<'a> {
        pub fn components(mut self, components: ComponentMapping) -> Self {
            self.components = components;
            self
        }
        pub fn srgb(mut self, srgb: Bool32) -> Self {
            self.srgb = srgb;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub border_color_swizzle: Bool32,
        pub border_color_swizzle_from_image: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_BORDER_COLOR_SWIZZLE_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                border_color_swizzle: Default::default(),
                border_color_swizzle_from_image: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a> {
        pub fn border_color_swizzle(mut self, border_color_swizzle: Bool32) -> Self {
            self.border_color_swizzle = border_color_swizzle;
            self
        }
        pub fn border_color_swizzle_from_image(
            mut self,
            border_color_swizzle_from_image: Bool32,
        ) -> Self {
            self.border_color_swizzle_from_image = border_color_swizzle_from_image;
            self
        }
    }
}
