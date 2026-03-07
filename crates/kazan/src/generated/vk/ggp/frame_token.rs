//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_GGP_frame_token.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_GGP_frame_token";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPresentFrameTokenGGP.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PresentFrameTokenGGP<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub frame_token: GgpFrameToken,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PresentFrameTokenGGP<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PresentFrameTokenGGP")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("frame_token", &self.frame_token)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PresentFrameTokenGGP<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_FRAME_TOKEN_GGP;
    }

    unsafe impl<'a> Extends<PresentInfoKHR<'a>> for PresentFrameTokenGGP<'a> {}

    impl Default for PresentFrameTokenGGP<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                frame_token: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PresentFrameTokenGGP<'a> {
        #[inline]
        pub fn frame_token(mut self, frame_token: GgpFrameToken) -> Self {
            self.frame_token = frame_token;
            self
        }
    }
}
