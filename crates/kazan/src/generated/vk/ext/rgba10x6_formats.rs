//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_rgba10x6_formats.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_rgba10x6_formats";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format_rgba10x6_without_y_cb_cr_sampler: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRGBA10X6FormatsFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "format_rgba10x6_without_y_cb_cr_sampler",
                    &self.format_rgba10x6_without_y_cb_cr_sampler,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RGBA10X6_FORMATS_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                format_rgba10x6_without_y_cb_cr_sampler: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRGBA10X6FormatsFeaturesEXT<'a> {
        #[inline]
        pub fn format_rgba10x6_without_y_cb_cr_sampler(
            mut self,
            format_rgba10x6_without_y_cb_cr_sampler: bool,
        ) -> Self {
            self.format_rgba10x6_without_y_cb_cr_sampler =
                format_rgba10x6_without_y_cb_cr_sampler.into();
            self
        }
    }
}
