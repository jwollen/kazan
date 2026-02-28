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
    pub struct PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub attachment_feedback_loop_layout: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: StructureType::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_LAYOUT_FEATURES_EXT,
                p_next: core::ptr::null_mut(),
                attachment_feedback_loop_layout: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT<'a> {
        pub fn attachment_feedback_loop_layout(
            mut self,
            attachment_feedback_loop_layout: Bool32,
        ) -> Self {
            self.attachment_feedback_loop_layout = attachment_feedback_loop_layout;
            self
        }
    }
}
