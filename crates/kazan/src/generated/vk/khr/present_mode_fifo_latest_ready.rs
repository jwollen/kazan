//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_present_mode_fifo_latest_ready.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_present_mode_fifo_latest_ready";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub present_mode_fifo_latest_ready: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "present_mode_fifo_latest_ready",
                    &self.present_mode_fifo_latest_ready,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'_>
    {
    }

    impl Default for PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                present_mode_fifo_latest_ready: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'a> {
        #[inline]
        pub fn present_mode_fifo_latest_ready(
            mut self,
            present_mode_fifo_latest_ready: bool,
        ) -> Self {
            self.present_mode_fifo_latest_ready = present_mode_fifo_latest_ready.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR =
        PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'static>;
    impl PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
