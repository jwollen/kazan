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
    pub struct PhysicalDeviceVideoMaintenance1FeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub video_maintenance1: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceVideoMaintenance1FeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_VIDEO_MAINTENANCE_1_FEATURES_KHR,
                p_next: core::ptr::null_mut(),
                video_maintenance1: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceVideoMaintenance1FeaturesKHR<'a> {
        pub fn video_maintenance1(mut self, video_maintenance1: Bool32) -> Self {
            self.video_maintenance1 = video_maintenance1;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct VideoInlineQueryInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub query_pool: QueryPool,
        pub first_query: u32,
        pub query_count: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for VideoInlineQueryInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::VIDEO_INLINE_QUERY_INFO_KHR,
                p_next: core::ptr::null(),
                query_pool: Default::default(),
                first_query: Default::default(),
                query_count: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> VideoInlineQueryInfoKHR<'a> {
        pub fn query_pool(mut self, query_pool: QueryPool) -> Self {
            self.query_pool = query_pool;
            self
        }
        pub fn first_query(mut self, first_query: u32) -> Self {
            self.first_query = first_query;
            self
        }
        pub fn query_count(mut self, query_count: u32) -> Self {
            self.query_count = query_count;
            self
        }
    }
}
