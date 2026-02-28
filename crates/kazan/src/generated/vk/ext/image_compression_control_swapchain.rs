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
    pub struct PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub image_compression_control_swapchain: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type:
                    StructureType::PHYSICAL_DEVICE_IMAGE_COMPRESSION_CONTROL_SWAPCHAIN_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                image_compression_control_swapchain: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT<'a> {
        pub fn image_compression_control_swapchain(
            mut self,
            image_compression_control_swapchain: Bool32,
        ) -> Self {
            self.image_compression_control_swapchain = image_compression_control_swapchain;
            self
        }
    }
}
