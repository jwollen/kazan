//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_primitive_topology_list_restart.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_primitive_topology_list_restart";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub primitive_topology_list_restart: Bool32,
        pub primitive_topology_patch_list_restart: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "primitive_topology_list_restart",
                    &self.primitive_topology_list_restart,
                )
                .field(
                    "primitive_topology_patch_list_restart",
                    &self.primitive_topology_patch_list_restart,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                primitive_topology_list_restart: Default::default(),
                primitive_topology_patch_list_restart: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a> {
        #[inline]
        pub fn primitive_topology_list_restart(
            mut self,
            primitive_topology_list_restart: bool,
        ) -> Self {
            self.primitive_topology_list_restart = primitive_topology_list_restart.into();
            self
        }

        #[inline]
        pub fn primitive_topology_patch_list_restart(
            mut self,
            primitive_topology_patch_list_restart: bool,
        ) -> Self {
            self.primitive_topology_patch_list_restart =
                primitive_topology_patch_list_restart.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT =
        PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'static>;
    impl PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
