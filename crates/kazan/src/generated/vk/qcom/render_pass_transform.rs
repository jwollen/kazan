//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_QCOM_render_pass_transform.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_QCOM_render_pass_transform";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkRenderPassTransformBeginInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct RenderPassTransformBeginInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub transform: SurfaceTransformFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for RenderPassTransformBeginInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("RenderPassTransformBeginInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("transform", &self.transform)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for RenderPassTransformBeginInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM;
    }

    unsafe impl<'a> Extends<RenderPassBeginInfo<'a>> for RenderPassTransformBeginInfoQCOM<'a> {}

    impl Default for RenderPassTransformBeginInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                transform: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> RenderPassTransformBeginInfoQCOM<'a> {
        #[inline]
        pub fn transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.transform = transform;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkCommandBufferInheritanceRenderPassTransformInfoQCOM.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub transform: SurfaceTransformFlagBitsKHR,
        pub render_area: Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for CommandBufferInheritanceRenderPassTransformInfoQCOM<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("CommandBufferInheritanceRenderPassTransformInfoQCOM")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("transform", &self.transform)
                .field("render_area", &self.render_area)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for CommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM;
    }

    unsafe impl<'a> Extends<CommandBufferInheritanceInfo<'a>>
        for CommandBufferInheritanceRenderPassTransformInfoQCOM<'a>
    {
    }

    impl Default for CommandBufferInheritanceRenderPassTransformInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                transform: Default::default(),
                render_area: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> CommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {
        #[inline]
        pub fn transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.transform = transform;
            self
        }

        #[inline]
        pub fn render_area(mut self, render_area: Rect2D) -> Self {
            self.render_area = render_area;
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkRenderPassTransformBeginInfoQCOM = RenderPassTransformBeginInfoQCOM<'static>;
    pub type VkCommandBufferInheritanceRenderPassTransformInfoQCOM =
        CommandBufferInheritanceRenderPassTransformInfoQCOM<'static>;
    impl RenderPassTransformBeginInfoQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkRenderPassTransformBeginInfoQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl CommandBufferInheritanceRenderPassTransformInfoQCOM<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
            unsafe { core::mem::transmute(self) }
        }
    }
}
