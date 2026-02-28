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
    pub struct PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub pipeline_cache_incremental_mode: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_PIPELINE_CACHE_INCREMENTAL_MODE_FEATURES_SEC,
                p_next: core::ptr::null_mut(),
                pipeline_cache_incremental_mode: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC<'a> {
        pub fn pipeline_cache_incremental_mode(
            mut self,
            pipeline_cache_incremental_mode: Bool32,
        ) -> Self {
            self.pipeline_cache_incremental_mode = pipeline_cache_incremental_mode;
            self
        }
    }
}
