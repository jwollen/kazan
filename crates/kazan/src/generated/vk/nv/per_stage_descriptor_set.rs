#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub per_stage_descriptor_set: Bool32,
        pub dynamic_pipeline_layout: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PER_STAGE_DESCRIPTOR_SET_FEATURES_NV;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePerStageDescriptorSetFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePerStageDescriptorSetFeaturesNV<'a>
    {
    }
    impl Default for PhysicalDevicePerStageDescriptorSetFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                per_stage_descriptor_set: Default::default(),
                dynamic_pipeline_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePerStageDescriptorSetFeaturesNV<'a> {
        pub fn per_stage_descriptor_set(mut self, per_stage_descriptor_set: bool) -> Self {
            self.per_stage_descriptor_set = per_stage_descriptor_set.into();
            self
        }
        pub fn dynamic_pipeline_layout(mut self, dynamic_pipeline_layout: bool) -> Self {
            self.dynamic_pipeline_layout = dynamic_pipeline_layout.into();
            self
        }
    }
}
