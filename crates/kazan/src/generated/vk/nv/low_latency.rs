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
    pub struct QueryLowLatencySupportNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_queried_low_latency_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for QueryLowLatencySupportNV<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::QUERY_LOW_LATENCY_SUPPORT_NV,
                p_next: core::ptr::null(),
                p_queried_low_latency_data: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> QueryLowLatencySupportNV<'a> {
        pub fn queried_low_latency_data(
            mut self,
            queried_low_latency_data: &'a mut c_void,
        ) -> Self {
            self.p_queried_low_latency_data = queried_low_latency_data;
            self
        }
    }
}
