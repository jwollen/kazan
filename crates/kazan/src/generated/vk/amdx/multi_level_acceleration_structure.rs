#![cfg(feature = "provisional")]
//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_AMDX_multi_level_acceleration_structure.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_AMDX_multi_level_acceleration_structure";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub multi_level_acceleration_structure: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "multi_level_acceleration_structure",
                    &self.multi_level_acceleration_structure,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_MULTI_LEVEL_ACCELERATION_STRUCTURE_FEATURES_AMDX;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'_>
    {
    }

    impl Default for PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                multi_level_acceleration_structure: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'a> {
        #[inline]
        pub fn multi_level_acceleration_structure(
            mut self,
            multi_level_acceleration_structure: bool,
        ) -> Self {
            self.multi_level_acceleration_structure = multi_level_acceleration_structure.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX =
        PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'static>;
    impl PhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceMultiLevelAccelerationStructureFeaturesAMDX {
            unsafe { core::mem::transmute(self) }
        }
    }
}
