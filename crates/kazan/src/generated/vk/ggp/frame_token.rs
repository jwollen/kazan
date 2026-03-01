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
    pub struct PresentFrameTokenGGP<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub frame_token: GgpFrameToken,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PresentFrameTokenGGP<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::PRESENT_FRAME_TOKEN_GGP;
    }
    unsafe impl<'a> Extends<PresentInfoKHR<'a>> for PresentFrameTokenGGP<'a> {}
    impl Default for PresentFrameTokenGGP<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                frame_token: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PresentFrameTokenGGP<'a> {
        pub fn frame_token(mut self, frame_token: GgpFrameToken) -> Self {
            self.frame_token = frame_token;
            self
        }
    }
}
