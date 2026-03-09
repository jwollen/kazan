//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_internally_synchronized_queues.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_internally_synchronized_queues";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub internally_synchronized_queues: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "internally_synchronized_queues",
                    &self.internally_synchronized_queues,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_INTERNALLY_SYNCHRONIZED_QUEUES_FEATURES_KHR;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'a>
    {
    }

    impl Default for PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                internally_synchronized_queues: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'a> {
        #[inline]
        pub fn internally_synchronized_queues(
            mut self,
            internally_synchronized_queues: bool,
        ) -> Self {
            self.internally_synchronized_queues = internally_synchronized_queues.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR =
        PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'static>;
    impl PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
