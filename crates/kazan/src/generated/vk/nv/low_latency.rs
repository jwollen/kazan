#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_low_latency";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkQueryLowLatencySupportNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct QueryLowLatencySupportNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_queried_low_latency_data: *mut c_void,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for QueryLowLatencySupportNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("QueryLowLatencySupportNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "p_queried_low_latency_data",
                    &self.p_queried_low_latency_data,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for QueryLowLatencySupportNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::QUERY_LOW_LATENCY_SUPPORT_NV;
    }

    unsafe impl<'a> Extends<SemaphoreCreateInfo<'a>> for QueryLowLatencySupportNV<'a> {}

    impl Default for QueryLowLatencySupportNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_queried_low_latency_data: core::ptr::null_mut(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> QueryLowLatencySupportNV<'a> {
        #[inline]
        pub fn queried_low_latency_data(mut self, queried_low_latency_data: *mut c_void) -> Self {
            self.p_queried_low_latency_data = queried_low_latency_data;
            self
        }
    }
}
