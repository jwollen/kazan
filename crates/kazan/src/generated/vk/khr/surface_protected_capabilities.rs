#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_surface_protected_capabilities";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSurfaceProtectedCapabilitiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SurfaceProtectedCapabilitiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub supports_protected: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SurfaceProtectedCapabilitiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SurfaceProtectedCapabilitiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("supports_protected", &self.supports_protected)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SurfaceProtectedCapabilitiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SURFACE_PROTECTED_CAPABILITIES_KHR;
    }

    unsafe impl<'a> Extends<SurfaceCapabilities2KHR<'a>> for SurfaceProtectedCapabilitiesKHR<'a> {}

    impl Default for SurfaceProtectedCapabilitiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                supports_protected: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SurfaceProtectedCapabilitiesKHR<'a> {
        pub fn supports_protected(mut self, supports_protected: bool) -> Self {
            self.supports_protected = supports_protected.into();
            self
        }
    }
}
