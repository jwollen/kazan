//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_private_data_base_handle.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_private_data_base_handle";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePrivateDataBaseHandleFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePrivateDataBaseHandleFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub private_data_base_handle: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePrivateDataBaseHandleFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePrivateDataBaseHandleFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("private_data_base_handle", &self.private_data_base_handle)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePrivateDataBaseHandleFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRIVATE_DATA_BASE_HANDLE_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePrivateDataBaseHandleFeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDevicePrivateDataBaseHandleFeaturesNV<'_> {}

    impl Default for PhysicalDevicePrivateDataBaseHandleFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                private_data_base_handle: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePrivateDataBaseHandleFeaturesNV<'a> {
        #[inline]
        pub fn private_data_base_handle(mut self, private_data_base_handle: bool) -> Self {
            self.private_data_base_handle = private_data_base_handle.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePrivateDataBaseHandleFeaturesNV =
        PhysicalDevicePrivateDataBaseHandleFeaturesNV<'static>;
    impl PhysicalDevicePrivateDataBaseHandleFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePrivateDataBaseHandleFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
