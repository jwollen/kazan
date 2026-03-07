//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_border_color_swizzle.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_border_color_swizzle";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SamplerBorderColorComponentMappingCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub components: ComponentMapping,
        pub srgb: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SamplerBorderColorComponentMappingCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SamplerBorderColorComponentMappingCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("components", &self.components)
                .field("srgb", &self.srgb)
                .finish()
        }
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
                p_next: ptr::null(),
                components: Default::default(),
                srgb: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SamplerBorderColorComponentMappingCreateInfoEXT<'a> {
        #[inline]
        pub fn components(mut self, components: ComponentMapping) -> Self {
            self.components = components;
            self
        }

        #[inline]
        pub fn srgb(mut self, srgb: bool) -> Self {
            self.srgb = srgb.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub border_color_swizzle: Bool32,
        pub border_color_swizzle_from_image: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceBorderColorSwizzleFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceBorderColorSwizzleFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("border_color_swizzle", &self.border_color_swizzle)
                .field(
                    "border_color_swizzle_from_image",
                    &self.border_color_swizzle_from_image,
                )
                .finish()
        }
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
                p_next: ptr::null_mut(),
                border_color_swizzle: Default::default(),
                border_color_swizzle_from_image: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceBorderColorSwizzleFeaturesEXT<'a> {
        #[inline]
        pub fn border_color_swizzle(mut self, border_color_swizzle: bool) -> Self {
            self.border_color_swizzle = border_color_swizzle.into();
            self
        }

        #[inline]
        pub fn border_color_swizzle_from_image(
            mut self,
            border_color_swizzle_from_image: bool,
        ) -> Self {
            self.border_color_swizzle_from_image = border_color_swizzle_from_image.into();
            self
        }
    }
}
