#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRawAccessChainsFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_raw_access_chains: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceRawAccessChainsFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_RAW_ACCESS_CHAINS_FEATURES_NV,
                p_next: core::ptr::null_mut(),
                shader_raw_access_chains: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRawAccessChainsFeaturesNV<'a> {
        pub fn shader_raw_access_chains(mut self, shader_raw_access_chains: Bool32) -> Self {
            self.shader_raw_access_chains = shader_raw_access_chains;
            self
        }
    }
}
