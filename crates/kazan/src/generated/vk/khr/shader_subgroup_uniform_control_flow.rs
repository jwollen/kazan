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
    pub struct PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub shader_subgroup_uniform_control_flow: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type:
                    StructureType::PHYSICAL_DEVICE_SHADER_SUBGROUP_UNIFORM_CONTROL_FLOW_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                shader_subgroup_uniform_control_flow: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR<'a> {
        pub fn shader_subgroup_uniform_control_flow(
            mut self,
            shader_subgroup_uniform_control_flow: Bool32,
        ) -> Self {
            self.shader_subgroup_uniform_control_flow = shader_subgroup_uniform_control_flow;
            self
        }
    }
}
