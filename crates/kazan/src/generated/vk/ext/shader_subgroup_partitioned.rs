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
    pub struct PhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_subgroup_partitioned: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_PARTITIONED_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                shader_subgroup_partitioned: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderSubgroupPartitionedFeaturesEXT<'a> {
        pub fn shader_subgroup_partitioned(mut self, shader_subgroup_partitioned: Bool32) -> Self {
            self.shader_subgroup_partitioned = shader_subgroup_partitioned;
            self
        }
    }
}
