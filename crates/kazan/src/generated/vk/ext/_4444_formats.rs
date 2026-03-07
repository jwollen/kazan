//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_4444_formats.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_4444_formats";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevice4444FormatsFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub format_a4r4g4b4: Bool32,
        pub format_a4b4g4r4: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevice4444FormatsFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevice4444FormatsFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("format_a4r4g4b4", &self.format_a4r4g4b4)
                .field("format_a4b4g4r4", &self.format_a4b4g4r4)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevice4444FormatsFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_4444_FORMATS_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDevice4444FormatsFeaturesEXT<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDevice4444FormatsFeaturesEXT<'a> {}

    impl Default for PhysicalDevice4444FormatsFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                format_a4r4g4b4: Default::default(),
                format_a4b4g4r4: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevice4444FormatsFeaturesEXT<'a> {
        #[inline]
        pub fn format_a4r4g4b4(mut self, format_a4r4g4b4: bool) -> Self {
            self.format_a4r4g4b4 = format_a4r4g4b4.into();
            self
        }

        #[inline]
        pub fn format_a4b4g4r4(mut self, format_a4b4g4r4: bool) -> Self {
            self.format_a4b4g4r4 = format_a4b4g4r4.into();
            self
        }
    }
}
