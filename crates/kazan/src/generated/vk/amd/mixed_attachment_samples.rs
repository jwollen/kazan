#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_AMD_mixed_attachment_samples";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAttachmentSampleCountInfoAMD.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct AttachmentSampleCountInfoAMD<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub color_attachment_count: u32,
        pub p_color_attachment_samples: *const SampleCountFlagBits,
        pub depth_stencil_attachment_samples: SampleCountFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for AttachmentSampleCountInfoAMD<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("AttachmentSampleCountInfoAMD")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("color_attachment_count", &self.color_attachment_count)
                .field(
                    "p_color_attachment_samples",
                    &self.p_color_attachment_samples,
                )
                .field(
                    "depth_stencil_attachment_samples",
                    &self.depth_stencil_attachment_samples,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for AttachmentSampleCountInfoAMD<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ATTACHMENT_SAMPLE_COUNT_INFO_AMD;
    }

    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>> for AttachmentSampleCountInfoAMD<'a> {}
    unsafe impl<'a> Extends<GraphicsPipelineCreateInfo<'a>> for AttachmentSampleCountInfoAMD<'a> {}

    impl Default for AttachmentSampleCountInfoAMD<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                color_attachment_count: Default::default(),
                p_color_attachment_samples: core::ptr::null(),
                depth_stencil_attachment_samples: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> AttachmentSampleCountInfoAMD<'a> {
        #[inline]
        pub fn color_attachment_samples(
            mut self,
            color_attachment_samples: &'a [SampleCountFlagBits],
        ) -> Self {
            self.color_attachment_count = color_attachment_samples.len().try_into().unwrap();
            self.p_color_attachment_samples = color_attachment_samples.as_ptr();
            self
        }

        #[inline]
        pub fn depth_stencil_attachment_samples(
            mut self,
            depth_stencil_attachment_samples: SampleCountFlagBits,
        ) -> Self {
            self.depth_stencil_attachment_samples = depth_stencil_attachment_samples;
            self
        }
    }
}
