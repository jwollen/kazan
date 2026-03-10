//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_raw_access_chains.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_raw_access_chains";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRawAccessChainsFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRawAccessChainsFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_raw_access_chains: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRawAccessChainsFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRawAccessChainsFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("shader_raw_access_chains", &self.shader_raw_access_chains)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceRawAccessChainsFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>> for PhysicalDeviceRawAccessChainsFeaturesNV<'_> {}
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceRawAccessChainsFeaturesNV<'_> {}

    impl Default for PhysicalDeviceRawAccessChainsFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                shader_raw_access_chains: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRawAccessChainsFeaturesNV<'a> {
        #[inline]
        pub fn shader_raw_access_chains(mut self, shader_raw_access_chains: bool) -> Self {
            self.shader_raw_access_chains = shader_raw_access_chains.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceRawAccessChainsFeaturesNV =
        PhysicalDeviceRawAccessChainsFeaturesNV<'static>;
    impl PhysicalDeviceRawAccessChainsFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkPhysicalDeviceRawAccessChainsFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
