//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_display_stereo.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_display_stereo";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplaySurfaceStereoCreateInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DisplaySurfaceStereoCreateInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub stereo_type: DisplaySurfaceStereoTypeNV,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DisplaySurfaceStereoCreateInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DisplaySurfaceStereoCreateInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("stereo_type", &self.stereo_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DisplaySurfaceStereoCreateInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_SURFACE_STEREO_CREATE_INFO_NV;
    }

    unsafe impl Extends<DisplaySurfaceCreateInfoKHR<'_>> for DisplaySurfaceStereoCreateInfoNV<'_> {}

    impl Default for DisplaySurfaceStereoCreateInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                stereo_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DisplaySurfaceStereoCreateInfoNV<'a> {
        #[inline]
        pub fn stereo_type(mut self, stereo_type: DisplaySurfaceStereoTypeNV) -> Self {
            self.stereo_type = stereo_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplayModeStereoPropertiesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct DisplayModeStereoPropertiesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub hdmi3_d_supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for DisplayModeStereoPropertiesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("DisplayModeStereoPropertiesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("hdmi3_d_supported", &self.hdmi3_d_supported)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for DisplayModeStereoPropertiesNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::DISPLAY_MODE_STEREO_PROPERTIES_NV;
    }

    unsafe impl Extends<DisplayModeProperties2KHR<'_>> for DisplayModeStereoPropertiesNV<'_> {}

    impl Default for DisplayModeStereoPropertiesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                hdmi3_d_supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> DisplayModeStereoPropertiesNV<'a> {
        #[inline]
        pub fn hdmi3_d_supported(mut self, hdmi3_d_supported: bool) -> Self {
            self.hdmi3_d_supported = hdmi3_d_supported.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkDisplaySurfaceStereoTypeNV.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DisplaySurfaceStereoTypeNV(i32);

    impl DisplaySurfaceStereoTypeNV {
        pub const NONE_NV: Self = Self(0);
        pub const ONBOARD_DIN_NV: Self = Self(1);
        pub const HDMI_3D_NV: Self = Self(2);
        pub const INBAND_DISPLAYPORT_NV: Self = Self(3);
    }

    impl fmt::Debug for DisplaySurfaceStereoTypeNV {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::NONE_NV => Some("NONE_NV"),
                Self::ONBOARD_DIN_NV => Some("ONBOARD_DIN_NV"),
                Self::HDMI_3D_NV => Some("HDMI_3D_NV"),
                Self::INBAND_DISPLAYPORT_NV => Some("INBAND_DISPLAYPORT_NV"),
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

    pub type VkDisplaySurfaceStereoCreateInfoNV = DisplaySurfaceStereoCreateInfoNV<'static>;
    pub type VkDisplayModeStereoPropertiesNV = DisplayModeStereoPropertiesNV<'static>;
    pub type VkDisplaySurfaceStereoTypeNV = DisplaySurfaceStereoTypeNV;
    impl DisplaySurfaceStereoCreateInfoNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDisplaySurfaceStereoCreateInfoNV {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl DisplayModeStereoPropertiesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkDisplayModeStereoPropertiesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
