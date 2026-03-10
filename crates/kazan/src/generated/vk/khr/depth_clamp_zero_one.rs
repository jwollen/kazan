//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_depth_clamp_zero_one.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_depth_clamp_zero_one";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDepthClampZeroOneFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDepthClampZeroOneFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub depth_clamp_zero_one: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDepthClampZeroOneFeaturesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDepthClampZeroOneFeaturesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("depth_clamp_zero_one", &self.depth_clamp_zero_one)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceDepthClampZeroOneFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceDepthClampZeroOneFeaturesKHR<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceDepthClampZeroOneFeaturesKHR<'_> {}

    impl Default for PhysicalDeviceDepthClampZeroOneFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                depth_clamp_zero_one: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDepthClampZeroOneFeaturesKHR<'a> {
        #[inline]
        pub fn depth_clamp_zero_one(mut self, depth_clamp_zero_one: bool) -> Self {
            self.depth_clamp_zero_one = depth_clamp_zero_one.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceDepthClampZeroOneFeaturesKHR =
        PhysicalDeviceDepthClampZeroOneFeaturesKHR<'static>;
    impl PhysicalDeviceDepthClampZeroOneFeaturesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDepthClampZeroOneFeaturesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}
