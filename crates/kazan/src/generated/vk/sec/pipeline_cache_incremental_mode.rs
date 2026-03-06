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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_cache_incremental_mode: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a>
    {
    }
    impl Default for PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                pipeline_cache_incremental_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {
        pub fn pipeline_cache_incremental_mode(
            mut self,
            pipeline_cache_incremental_mode: bool,
        ) -> Self {
            self.pipeline_cache_incremental_mode = pipeline_cache_incremental_mode.into();
            self
        }
    }
}
