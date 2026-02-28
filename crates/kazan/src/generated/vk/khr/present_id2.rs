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
    pub struct PhysicalDevicePresentId2FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_id2: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePresentId2FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PRESENT_ID_2_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                present_id2: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePresentId2FeaturesKHR<'a> {
        pub fn present_id2(mut self, present_id2: Bool32) -> Self {
            self.present_id2 = present_id2;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PresentId2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub swapchain_count: u32,
        pub p_present_ids: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PresentId2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PRESENT_ID_2_KHR,
                p_next: core::ptr::null(),
                swapchain_count: Default::default(),
                p_present_ids: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PresentId2KHR<'a> {
        pub fn present_ids(mut self, present_ids: &'a [u64]) -> Self {
            self.swapchain_count = present_ids.len().try_into().unwrap();
            self.p_present_ids = present_ids.as_ptr();
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfaceCapabilitiesPresentId2KHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_id2_supported: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SurfaceCapabilitiesPresentId2KHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::SURFACE_CAPABILITIES_PRESENT_ID_2_KHR,
                p_next: core::ptr::null_mut(),
                present_id2_supported: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SurfaceCapabilitiesPresentId2KHR<'a> {
        pub fn present_id2_supported(mut self, present_id2_supported: Bool32) -> Self {
            self.present_id2_supported = present_id2_supported;
            self
        }
    }
}
