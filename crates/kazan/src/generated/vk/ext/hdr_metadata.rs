#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    pub struct XYColorEXT {
        pub x: f32,
        pub y: f32,
    }
    impl XYColorEXT {
        pub fn x(mut self, x: f32) -> Self {
            self.x = x;
            self
        }
        pub fn y(mut self, y: f32) -> Self {
            self.y = y;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
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
    unsafe impl<'a> TaggedStructure<'a> for HdrMetadataEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::HDR_METADATA_EXT;
    }
    impl Default for HdrMetadataEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
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
        pub fn display_primary_red(mut self, display_primary_red: XYColorEXT) -> Self {
            self.display_primary_red = display_primary_red;
            self
        }
        pub fn display_primary_green(mut self, display_primary_green: XYColorEXT) -> Self {
            self.display_primary_green = display_primary_green;
            self
        }
        pub fn display_primary_blue(mut self, display_primary_blue: XYColorEXT) -> Self {
            self.display_primary_blue = display_primary_blue;
            self
        }
        pub fn white_point(mut self, white_point: XYColorEXT) -> Self {
            self.white_point = white_point;
            self
        }
        pub fn max_luminance(mut self, max_luminance: f32) -> Self {
            self.max_luminance = max_luminance;
            self
        }
        pub fn min_luminance(mut self, min_luminance: f32) -> Self {
            self.min_luminance = min_luminance;
            self
        }
        pub fn max_content_light_level(mut self, max_content_light_level: f32) -> Self {
            self.max_content_light_level = max_content_light_level;
            self
        }
        pub fn max_frame_average_light_level(mut self, max_frame_average_light_level: f32) -> Self {
            self.max_frame_average_light_level = max_frame_average_light_level;
            self
        }
    }
    pub type PFN_vkSetHdrMetadataEXT = unsafe extern "system" fn(
        device: Device,
        swapchain_count: u32,
        p_swapchains: *const SwapchainKHR,
        p_metadata: *const HdrMetadataEXT<'_>,
    );
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
