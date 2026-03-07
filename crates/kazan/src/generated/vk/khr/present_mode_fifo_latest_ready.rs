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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_mode_fifo_latest_ready: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a>
    {
    }

    impl Default for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                present_mode_fifo_latest_ready: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a> {
        pub fn present_mode_fifo_latest_ready(
            mut self,
            present_mode_fifo_latest_ready: bool,
        ) -> Self {
            self.present_mode_fifo_latest_ready = present_mode_fifo_latest_ready.into();
            self
        }
    }
}
