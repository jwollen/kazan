#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_astc_decode_mode";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageViewASTCDecodeModeEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageViewASTCDecodeModeEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub decode_mode: Format,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageViewASTCDecodeModeEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageViewASTCDecodeModeEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("decode_mode", &self.decode_mode)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageViewASTCDecodeModeEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_VIEW_ASTC_DECODE_MODE_EXT;
    }

    unsafe impl<'a> Extends<ImageViewCreateInfo<'a>> for ImageViewASTCDecodeModeEXT<'a> {}

    impl Default for ImageViewASTCDecodeModeEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                decode_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageViewASTCDecodeModeEXT<'a> {
        pub fn decode_mode(mut self, decode_mode: Format) -> Self {
            self.decode_mode = decode_mode;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceASTCDecodeFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceASTCDecodeFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub decode_mode_shared_exponent: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceASTCDecodeFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceASTCDecodeFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "decode_mode_shared_exponent",
                    &self.decode_mode_shared_exponent,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceASTCDecodeFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ASTC_DECODE_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>> for PhysicalDeviceASTCDecodeFeaturesEXT<'a> {}
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceASTCDecodeFeaturesEXT<'a> {}

    impl Default for PhysicalDeviceASTCDecodeFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                decode_mode_shared_exponent: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceASTCDecodeFeaturesEXT<'a> {
        pub fn decode_mode_shared_exponent(mut self, decode_mode_shared_exponent: bool) -> Self {
            self.decode_mode_shared_exponent = decode_mode_shared_exponent.into();
            self
        }
    }
}
