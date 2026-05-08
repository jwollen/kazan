//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_SEC_throttle_hint.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_SEC_throttle_hint";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceThrottleHintFeaturesSEC.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceThrottleHintFeaturesSEC<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub throttle_hint: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceThrottleHintFeaturesSEC<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceThrottleHintFeaturesSEC")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("throttle_hint", &self.throttle_hint)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceThrottleHintFeaturesSEC<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_THROTTLE_HINT_FEATURES_SEC;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceThrottleHintFeaturesSEC<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceThrottleHintFeaturesSEC<'_> {}

    impl Default for PhysicalDeviceThrottleHintFeaturesSEC<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                throttle_hint: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceThrottleHintFeaturesSEC<'a> {
        #[inline]
        pub fn throttle_hint(mut self, throttle_hint: bool) -> Self {
            self.throttle_hint = throttle_hint.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkThrottleHintSubmitInfoSEC.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ThrottleHintSubmitInfoSEC<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub throttle_hint: ThrottleHintTypeSEC,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ThrottleHintSubmitInfoSEC<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ThrottleHintSubmitInfoSEC")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("throttle_hint", &self.throttle_hint)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ThrottleHintSubmitInfoSEC<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::THROTTLE_HINT_SUBMIT_INFO_SEC;
    }

    unsafe impl Extends<SubmitInfo<'_>> for ThrottleHintSubmitInfoSEC<'_> {}

    impl Default for ThrottleHintSubmitInfoSEC<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                throttle_hint: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ThrottleHintSubmitInfoSEC<'a> {
        #[inline]
        pub fn throttle_hint(mut self, throttle_hint: ThrottleHintTypeSEC) -> Self {
            self.throttle_hint = throttle_hint;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkThrottleHintTypeSEC.html>
    #[repr(transparent)]
    #[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ThrottleHintTypeSEC(i32);

    impl ThrottleHintTypeSEC {
        pub const DEFAULT_SEC: Self = Self(0);
        pub const LOW_SEC: Self = Self(1);
        pub const HIGH_SEC: Self = Self(2);
    }

    impl fmt::Debug for ThrottleHintTypeSEC {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let name = match *self {
                Self::DEFAULT_SEC => Some("DEFAULT_SEC"),
                Self::LOW_SEC => Some("LOW_SEC"),
                Self::HIGH_SEC => Some("HIGH_SEC"),
                _ => None,
            };
            if let Some(name) = name {
                f.write_str(name)
            } else {
                self.0.fmt(f)
            }
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceThrottleHintFeaturesSEC =
        PhysicalDeviceThrottleHintFeaturesSEC<'static>;
    pub type VkThrottleHintSubmitInfoSEC = ThrottleHintSubmitInfoSEC<'static>;
    pub type VkThrottleHintTypeSEC = ThrottleHintTypeSEC;
    impl PhysicalDeviceThrottleHintFeaturesSEC<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceThrottleHintFeaturesSEC {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl ThrottleHintSubmitInfoSEC<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkThrottleHintSubmitInfoSEC {
            unsafe { core::mem::transmute(self) }
        }
    }
}
