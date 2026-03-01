#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub primitive_topology_list_restart: Bool32,
        pub primitive_topology_patch_list_restart: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a>
    {
    }
    impl Default for PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                primitive_topology_list_restart: Default::default(),
                primitive_topology_patch_list_restart: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT<'a> {
        pub fn primitive_topology_list_restart(
            mut self,
            primitive_topology_list_restart: Bool32,
        ) -> Self {
            self.primitive_topology_list_restart = primitive_topology_list_restart;
            self
        }
        pub fn primitive_topology_patch_list_restart(
            mut self,
            primitive_topology_patch_list_restart: Bool32,
        ) -> Self {
            self.primitive_topology_patch_list_restart = primitive_topology_patch_list_restart;
            self
        }
    }
}
