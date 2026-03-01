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
    pub struct CopyCommandTransformInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub transform: SurfaceTransformFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for CopyCommandTransformInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_COMMAND_TRANSFORM_INFO_QCOM;
    }
    unsafe impl<'a> Extends<BufferImageCopy2<'a>> for CopyCommandTransformInfoQCOM<'a> {}
    unsafe impl<'a> Extends<ImageBlit2<'a>> for CopyCommandTransformInfoQCOM<'a> {}
    impl Default for CopyCommandTransformInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                transform: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CopyCommandTransformInfoQCOM<'a> {
        pub fn transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.transform = transform;
            self
        }
    }
}
