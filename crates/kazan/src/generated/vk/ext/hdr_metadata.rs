//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_hdr_metadata.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_hdr_metadata";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkXYColorEXT.html>
    #[repr(C)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[derive(Copy, Clone, Default)]
    #[must_use]
    pub struct XYColorEXT {
        pub x: f32,
        pub y: f32,
    }

    impl XYColorEXT {
        #[inline]
        pub fn x(mut self, x: f32) -> Self {
            self.x = x;
            self
        }

        #[inline]
        pub fn y(mut self, y: f32) -> Self {
            self.y = y;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkHdrMetadataEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct HdrMetadataEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub display_primary_red: XYColorEXT,
        pub display_primary_green: XYColorEXT,
        pub display_primary_blue: XYColorEXT,
        pub white_point: XYColorEXT,
        pub max_luminance: f32,
        pub min_luminance: f32,
        pub max_content_light_level: f32,
        pub max_frame_average_light_level: f32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for HdrMetadataEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HdrMetadataEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("display_primary_red", &self.display_primary_red)
                .field("display_primary_green", &self.display_primary_green)
                .field("display_primary_blue", &self.display_primary_blue)
                .field("white_point", &self.white_point)
                .field("max_luminance", &self.max_luminance)
                .field("min_luminance", &self.min_luminance)
                .field("max_content_light_level", &self.max_content_light_level)
                .field(
                    "max_frame_average_light_level",
                    &self.max_frame_average_light_level,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for HdrMetadataEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::HDR_METADATA_EXT;
    }

    impl Default for HdrMetadataEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                display_primary_red: Default::default(),
                display_primary_green: Default::default(),
                display_primary_blue: Default::default(),
                white_point: Default::default(),
                max_luminance: Default::default(),
                min_luminance: Default::default(),
                max_content_light_level: Default::default(),
                max_frame_average_light_level: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> HdrMetadataEXT<'a> {
        #[inline]
        pub fn display_primary_red(mut self, display_primary_red: XYColorEXT) -> Self {
            self.display_primary_red = display_primary_red;
            self
        }

        #[inline]
        pub fn display_primary_green(mut self, display_primary_green: XYColorEXT) -> Self {
            self.display_primary_green = display_primary_green;
            self
        }

        #[inline]
        pub fn display_primary_blue(mut self, display_primary_blue: XYColorEXT) -> Self {
            self.display_primary_blue = display_primary_blue;
            self
        }

        #[inline]
        pub fn white_point(mut self, white_point: XYColorEXT) -> Self {
            self.white_point = white_point;
            self
        }

        #[inline]
        pub fn max_luminance(mut self, max_luminance: f32) -> Self {
            self.max_luminance = max_luminance;
            self
        }

        #[inline]
        pub fn min_luminance(mut self, min_luminance: f32) -> Self {
            self.min_luminance = min_luminance;
            self
        }

        #[inline]
        pub fn max_content_light_level(mut self, max_content_light_level: f32) -> Self {
            self.max_content_light_level = max_content_light_level;
            self
        }

        #[inline]
        pub fn max_frame_average_light_level(mut self, max_frame_average_light_level: f32) -> Self {
            self.max_frame_average_light_level = max_frame_average_light_level;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetHdrMetadataEXT.html>
    pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
        device: Device,
        swapchain_count: u32,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT<'_>,
    );
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkXYColorEXT = XYColorEXT;
    pub type VkHdrMetadataEXT = HdrMetadataEXT<'static>;
    impl HdrMetadataEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkHdrMetadataEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    set_hdr_metadata_ext: PFN_vkSetHdrMetadataEXT,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                set_hdr_metadata_ext: transmute(
                    load(c"vkSetHdrMetadataEXT").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkSetHdrMetadataEXT.html>
    #[inline]
    pub unsafe fn set_hdr_metadata_ext(
        &self,
        device: Device,
        swapchains: &[SwapchainKHR],
        metadata: &[HdrMetadataEXT<'_>],
    ) {
        unsafe {
            (self.set_hdr_metadata_ext)(
                device,
                swapchains.len().try_into().unwrap(),
                swapchains.as_ptr() as _,
                metadata.as_ptr() as _,
            )
        }
    }
}
