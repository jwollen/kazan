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
    pub struct RenderPassTransformBeginInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub transform: SurfaceTransformFlagBitsKHR,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for RenderPassTransformBeginInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::RENDER_PASS_TRANSFORM_BEGIN_INFO_QCOM,
                p_next: core::ptr::null(),
                transform: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> RenderPassTransformBeginInfoQCOM<'a> {
        pub fn transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.transform = transform;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct CommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub transform: SurfaceTransformFlagBitsKHR,
        pub render_area: Rect2D,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for CommandBufferInheritanceRenderPassTransformInfoQCOM<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::COMMAND_BUFFER_INHERITANCE_RENDER_PASS_TRANSFORM_INFO_QCOM,
                p_next: core::ptr::null(),
                transform: Default::default(),
                render_area: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> CommandBufferInheritanceRenderPassTransformInfoQCOM<'a> {
        pub fn transform(mut self, transform: SurfaceTransformFlagBitsKHR) -> Self {
            self.transform = transform;
            self
        }
        pub fn render_area(mut self, render_area: Rect2D) -> Self {
            self.render_area = render_area;
            self
        }
    }
}
