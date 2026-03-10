//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_filter_cubic.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_filter_cubic";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceImageViewImageFormatInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_view_type: ImageViewType,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceImageViewImageFormatInfoEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceImageViewImageFormatInfoEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("image_view_type", &self.image_view_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceImageViewImageFormatInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_IMAGE_VIEW_IMAGE_FORMAT_INFO_EXT;
    }

    unsafe impl Extends<PhysicalDeviceImageFormatInfo2<'_>>
        for PhysicalDeviceImageViewImageFormatInfoEXT<'_>
    {
    }

    impl Default for PhysicalDeviceImageViewImageFormatInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                image_view_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceImageViewImageFormatInfoEXT<'a> {
        #[inline]
        pub fn image_view_type(mut self, image_view_type: ImageViewType) -> Self {
            self.image_view_type = image_view_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FilterCubicImageViewImageFormatPropertiesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub filter_cubic: Bool32,
        pub filter_cubic_minmax: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FilterCubicImageViewImageFormatPropertiesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FilterCubicImageViewImageFormatPropertiesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("filter_cubic", &self.filter_cubic)
                .field("filter_cubic_minmax", &self.filter_cubic_minmax)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FilterCubicImageViewImageFormatPropertiesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::FILTER_CUBIC_IMAGE_VIEW_IMAGE_FORMAT_PROPERTIES_EXT;
    }

    unsafe impl Extends<ImageFormatProperties2<'_>>
        for FilterCubicImageViewImageFormatPropertiesEXT<'_>
    {
    }

    impl Default for FilterCubicImageViewImageFormatPropertiesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                filter_cubic: Default::default(),
                filter_cubic_minmax: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FilterCubicImageViewImageFormatPropertiesEXT<'a> {
        #[inline]
        pub fn filter_cubic(mut self, filter_cubic: bool) -> Self {
            self.filter_cubic = filter_cubic.into();
            self
        }

        #[inline]
        pub fn filter_cubic_minmax(mut self, filter_cubic_minmax: bool) -> Self {
            self.filter_cubic_minmax = filter_cubic_minmax.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceImageViewImageFormatInfoEXT =
        PhysicalDeviceImageViewImageFormatInfoEXT<'static>;
    pub type VkFilterCubicImageViewImageFormatPropertiesEXT =
        FilterCubicImageViewImageFormatPropertiesEXT<'static>;
    impl PhysicalDeviceImageViewImageFormatInfoEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceImageViewImageFormatInfoEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl FilterCubicImageViewImageFormatPropertiesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkFilterCubicImageViewImageFormatPropertiesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
