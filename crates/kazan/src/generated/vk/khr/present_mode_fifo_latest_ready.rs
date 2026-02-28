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
    pub struct PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_mode_fifo_latest_ready: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                present_mode_fifo_latest_ready: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a> {
        pub fn present_mode_fifo_latest_ready(
            mut self,
            present_mode_fifo_latest_ready: Bool32,
        ) -> Self {
            self.present_mode_fifo_latest_ready = present_mode_fifo_latest_ready;
            self
        }
    }
}
