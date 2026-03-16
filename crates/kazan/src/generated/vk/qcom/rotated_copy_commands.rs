//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_rotated_copy_commands.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_rotated_copy_commands";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCopyCommandTransformInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CopyCommandTransformInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub transform: SurfaceTransformFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CopyCommandTransformInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CopyCommandTransformInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("transform", &self.transform)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CopyCommandTransformInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::COPY_COMMAND_TRANSFORM_INFO_QCOM;
    }

    unsafe impl Extends<BufferImageCopy2<'_>> for CopyCommandTransformInfoQCOM<'_> {}
    unsafe impl Extends<ImageBlit2<'_>> for CopyCommandTransformInfoQCOM<'_> {}
    unsafe impl Extends<DeviceMemoryImageCopyKHR<'_>> for CopyCommandTransformInfoQCOM<'_> {}

    impl Default for CopyCommandTransformInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                transform: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CopyCommandTransformInfoQCOM<'a> {
        #[inline]
        pub fn transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.transform = transform;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkCopyCommandTransformInfoQCOM = CopyCommandTransformInfoQCOM<'static>;
    impl CopyCommandTransformInfoQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkCopyCommandTransformInfoQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
