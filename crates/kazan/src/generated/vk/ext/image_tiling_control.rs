//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_image_tiling_control.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_image_tiling_control";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageTilingControlFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageTilingControlFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_tiling_control: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageTilingControlFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageTilingControlFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_tiling_control", &self.image_tiling_control)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageTilingControlFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_TILING_CONTROL_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceImageTilingControlFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceImageTilingControlFeaturesEXT<'_> {}

    impl Default for PhysicalDeviceImageTilingControlFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                image_tiling_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageTilingControlFeaturesEXT<'a> {
        #[inline]
        pub fn image_tiling_control(mut self, image_tiling_control: bool) -> Self {
            self.image_tiling_control = image_tiling_control.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageTilingControlCreateInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImageTilingControlCreateInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub tiling_control: ImageTilingControlEXT,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImageTilingControlCreateInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImageTilingControlCreateInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("tiling_control", &self.tiling_control)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImageTilingControlCreateInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMAGE_TILING_CONTROL_CREATE_INFO_EXT;
    }

    unsafe impl Extends<ImageCreateInfo<'_>> for ImageTilingControlCreateInfoEXT<'_> {}

    impl Default for ImageTilingControlCreateInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                tiling_control: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImageTilingControlCreateInfoEXT<'a> {
        #[inline]
        pub fn tiling_control(mut self, tiling_control: ImageTilingControlEXT) -> Self {
            self.tiling_control = tiling_control;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImageTilingControlEXT.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ImageTilingControlEXT(i32);

    impl ImageTilingControlEXT {
        #[inline]
        pub const fn from_raw(x: i32) -> Self {
            Self(x)
        }
        #[inline]
        pub const fn as_raw(self) -> i32 {
            self.0
        }

        pub const DEFAULT_EXT: Self = Self(0);
        pub const MIN_SIZE_EXT: Self = Self(1);
        pub const MAX_PERFORMANCE_EXT: Self = Self(2);
    }

    impl fmt::Debug for ImageTilingControlEXT {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_EXT => Some("DEFAULT_EXT"),
                Self::MIN_SIZE_EXT => Some("MIN_SIZE_EXT"),
                Self::MAX_PERFORMANCE_EXT => Some("MAX_PERFORMANCE_EXT"),
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

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceImageTilingControlFeaturesEXT =
        PhysicalDeviceImageTilingControlFeaturesEXT<'static>;
    pub type VkImageTilingControlCreateInfoEXT = ImageTilingControlCreateInfoEXT<'static>;
    pub type VkImageTilingControlEXT = ImageTilingControlEXT;
    impl PhysicalDeviceImageTilingControlFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceImageTilingControlFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ImageTilingControlCreateInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImageTilingControlCreateInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
