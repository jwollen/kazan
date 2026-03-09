//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_custom_border_color.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_custom_border_color";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SamplerCustomBorderColorCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub custom_border_color: ClearColorValue,
        pub format: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SamplerCustomBorderColorCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SamplerCustomBorderColorCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("custom_border_color", &self.custom_border_color)
                .field("format", &self.format)
                .finish()
        }
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
                p_next: ptr::null(),
                custom_border_color: Default::default(),
                format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SamplerCustomBorderColorCreateInfoEXT<'a> {
        #[inline]
        pub fn custom_border_color(mut self, custom_border_color: ClearColorValue) -> Self {
            self.custom_border_color = custom_border_color;
            self
        }

        #[inline]
        pub fn format(mut self, format: Format) -> Self {
            self.format = format;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCustomBorderColorPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub max_custom_border_color_samplers: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCustomBorderColorPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCustomBorderColorPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "max_custom_border_color_samplers",
                    &self.max_custom_border_color_samplers,
                )
                .finish()
        }
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
                p_next: ptr::null_mut(),
                max_custom_border_color_samplers: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCustomBorderColorPropertiesEXT<'a> {
        #[inline]
        pub fn max_custom_border_color_samplers(
            mut self,
            max_custom_border_color_samplers: u32,
        ) -> Self {
            self.max_custom_border_color_samplers = max_custom_border_color_samplers;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCustomBorderColorFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub custom_border_colors: Bool32,
        pub custom_border_color_without_format: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCustomBorderColorFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCustomBorderColorFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("custom_border_colors", &self.custom_border_colors)
                .field(
                    "custom_border_color_without_format",
                    &self.custom_border_color_without_format,
                )
                .finish()
        }
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
                p_next: ptr::null_mut(),
                custom_border_colors: Default::default(),
                custom_border_color_without_format: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCustomBorderColorFeaturesEXT<'a> {
        #[inline]
        pub fn custom_border_colors(mut self, custom_border_colors: bool) -> Self {
            self.custom_border_colors = custom_border_colors.into();
            self
        }

        #[inline]
        pub fn custom_border_color_without_format(
            mut self,
            custom_border_color_without_format: bool,
        ) -> Self {
            self.custom_border_color_without_format = custom_border_color_without_format.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkSamplerCustomBorderColorCreateInfoEXT =
        SamplerCustomBorderColorCreateInfoEXT<'static>;
    pub type VkPhysicalDeviceCustomBorderColorPropertiesEXT =
        PhysicalDeviceCustomBorderColorPropertiesEXT<'static>;
    pub type VkPhysicalDeviceCustomBorderColorFeaturesEXT =
        PhysicalDeviceCustomBorderColorFeaturesEXT<'static>;
    impl SamplerCustomBorderColorCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSamplerCustomBorderColorCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCustomBorderColorPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCustomBorderColorPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl PhysicalDeviceCustomBorderColorFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCustomBorderColorFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
