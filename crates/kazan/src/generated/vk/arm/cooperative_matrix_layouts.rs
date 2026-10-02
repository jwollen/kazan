//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_ARM_cooperative_matrix_layouts.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_ARM_cooperative_matrix_layouts";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceCooperativeMatrixLayoutsFeaturesARM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub cooperative_matrix_arm_layouts: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "cooperative_matrix_arm_layouts",
                    &self.cooperative_matrix_arm_layouts,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_COOPERATIVE_MATRIX_LAYOUTS_FEATURES_ARM;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'_>
    {
    }

    impl Default for PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                cooperative_matrix_arm_layouts: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'a> {
        #[inline]
        pub fn cooperative_matrix_arm_layouts(
            mut self,
            cooperative_matrix_arm_layouts: bool,
        ) -> Self {
            self.cooperative_matrix_arm_layouts = cooperative_matrix_arm_layouts.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceCooperativeMatrixLayoutsFeaturesARM =
        PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'static>;
    impl PhysicalDeviceCooperativeMatrixLayoutsFeaturesARM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceCooperativeMatrixLayoutsFeaturesARM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
