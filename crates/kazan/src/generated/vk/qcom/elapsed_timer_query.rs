//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_elapsed_timer_query.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_elapsed_timer_query";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub elapsed_timer_query: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceElapsedTimerQueryFeaturesQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("elapsed_timer_query", &self.elapsed_timer_query)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ELAPSED_TIMER_QUERY_FEATURES_QCOM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'_> {}

    impl Default for PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                elapsed_timer_query: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'a> {
        #[inline]
        pub fn elapsed_timer_query(mut self, elapsed_timer_query: bool) -> Self {
            self.elapsed_timer_query = elapsed_timer_query.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM =
        PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'static>;
    impl PhysicalDeviceElapsedTimerQueryFeaturesQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
