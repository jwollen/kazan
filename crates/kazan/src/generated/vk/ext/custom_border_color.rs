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
    pub struct SamplerCustomBorderColorCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub custom_border_color: ClearColorValue,
        pub format: Format,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SamplerCustomBorderColorCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SAMPLER_CUSTOM_BORDER_COLOR_CREATE_INFO_EXT;
    }
    unsafe impl<'a> Extends<SamplerCreateInfo<'a>> for SamplerCustomBorderColorCreateInfoEXT<'a> {}
    impl Default for SamplerCustomBorderColorCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                custom_border_color: Default::default(),
                format: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SamplerCustomBorderColorCreateInfoEXT<'a> {
        pub fn custom_border_color(mut self, custom_border_color: ClearColorValue) -> Self {
            self.custom_border_color = custom_border_color;
            self
        }
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCustomBorderColorPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_custom_border_color_samplers: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCustomBorderColorPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_PROPERTIES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceProperties2<'a>>
        for PhysicalDeviceCustomBorderColorPropertiesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceCustomBorderColorPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                max_custom_border_color_samplers: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCustomBorderColorPropertiesEXT<'a> {
        pub fn max_custom_border_color_samplers(
            mut self,
            max_custom_border_color_samplers: u32,
        ) -> Self {
            self.max_custom_border_color_samplers = max_custom_border_color_samplers;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCustomBorderColorFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub custom_border_colors: Bool32,
        pub custom_border_color_without_format: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCustomBorderColorFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_CUSTOM_BORDER_COLOR_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceCustomBorderColorFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceCustomBorderColorFeaturesEXT<'a> {}
    impl Default for PhysicalDeviceCustomBorderColorFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                custom_border_colors: Default::default(),
                custom_border_color_without_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCustomBorderColorFeaturesEXT<'a> {
        pub fn custom_border_colors(mut self, custom_border_colors: Bool32) -> Self {
            self.custom_border_colors = custom_border_colors;
            self
        }
        pub fn custom_border_color_without_format(
            mut self,
            custom_border_color_without_format: Bool32,
        ) -> Self {
            self.custom_border_color_without_format = custom_border_color_without_format;
            self
        }
    }
}
