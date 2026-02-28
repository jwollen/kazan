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
    pub struct PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub attachment_feedback_loop_dynamic_state: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    impl Default for PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
s_type: StructureType::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT
,
p_next: core::ptr::null_mut(),
attachment_feedback_loop_dynamic_state: Default::default(),
_marker: PhantomData
}
        }
    }
    impl<'a> PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'a> {
        pub fn attachment_feedback_loop_dynamic_state(
            mut self,
            attachment_feedback_loop_dynamic_state: Bool32,
        ) -> Self {
            self.attachment_feedback_loop_dynamic_state = attachment_feedback_loop_dynamic_state;
            self
        }
    }
    pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags);
}
pub struct DeviceFn {
    cmd_set_attachment_feedback_loop_enable_ext: PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                cmd_set_attachment_feedback_loop_enable_ext: transmute(
                    load(c"vkCmdSetAttachmentFeedbackLoopEnableEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn cmd_set_attachment_feedback_loop_enable_ext(
        &self,
        command_buffer: CommandBuffer,
        aspect_mask: ImageAspectFlags,
    ) {
        unsafe { (self.cmd_set_attachment_feedback_loop_enable_ext)(command_buffer, aspect_mask) }
    }
}
