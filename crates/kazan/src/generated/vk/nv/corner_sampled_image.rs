#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceCornerSampledImageFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub corner_sampled_image: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceCornerSampledImageFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_CORNER_SAMPLED_IMAGE_FEATURES_NV,
                p_next: core::ptr::null_mut(),
                corner_sampled_image: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceCornerSampledImageFeaturesNV<'a> {
        pub fn corner_sampled_image(mut self, corner_sampled_image: Bool32) -> Self {
            self.corner_sampled_image = corner_sampled_image;
            self
        }
    }
}
