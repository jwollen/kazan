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
    pub struct SurfaceProtectedCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supports_protected: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for SurfaceProtectedCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::SURFACE_PROTECTED_CAPABILITIES_KHR,
                p_next: core::ptr::null_mut(),
                supports_protected: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> SurfaceProtectedCapabilitiesKHR<'a> {
        pub fn supports_protected(mut self, supports_protected: Bool32) -> Self {
            self.supports_protected = supports_protected;
            self
        }
    }
}
