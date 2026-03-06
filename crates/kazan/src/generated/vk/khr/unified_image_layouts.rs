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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub unified_image_layouts: Bool32,
        pub unified_image_layouts_video: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_UNIFIED_IMAGE_LAYOUTS_FEATURES_KHR;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>> for PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'a> {}
    impl Default for PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                unified_image_layouts: Default::default(),
                unified_image_layouts_video: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceUnifiedImageLayoutsFeaturesKHR<'a> {
        pub fn unified_image_layouts(mut self, unified_image_layouts: bool) -> Self {
            self.unified_image_layouts = unified_image_layouts.into();
            self
        }
        pub fn unified_image_layouts_video(mut self, unified_image_layouts_video: bool) -> Self {
            self.unified_image_layouts_video = unified_image_layouts_video.into();
            self
        }
    }
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkAttachmentFeedbackLoopInfoEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct AttachmentFeedbackLoopInfoEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub feedback_loop_enable: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for AttachmentFeedbackLoopInfoEXT<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::ATTACHMENT_FEEDBACK_LOOP_INFO_EXT;
    }
    unsafe impl<'a> Extends<RenderingAttachmentInfo<'a>> for AttachmentFeedbackLoopInfoEXT<'a> {}
    impl Default for AttachmentFeedbackLoopInfoEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                feedback_loop_enable: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> AttachmentFeedbackLoopInfoEXT<'a> {
        pub fn feedback_loop_enable(mut self, feedback_loop_enable: bool) -> Self {
            self.feedback_loop_enable = feedback_loop_enable.into();
            self
        }
    }
}
