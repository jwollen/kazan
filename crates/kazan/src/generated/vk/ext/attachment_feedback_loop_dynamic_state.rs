//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_attachment_feedback_loop_dynamic_state.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_attachment_feedback_loop_dynamic_state";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub attachment_feedback_loop_dynamic_state: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "attachment_feedback_loop_dynamic_state",
                    &self.attachment_feedback_loop_dynamic_state,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_ATTACHMENT_FEEDBACK_LOOP_DYNAMIC_STATE_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                attachment_feedback_loop_dynamic_state: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'a> {
        #[inline]
        pub fn attachment_feedback_loop_dynamic_state(
            mut self,
            attachment_feedback_loop_dynamic_state: bool,
        ) -> Self {
            self.attachment_feedback_loop_dynamic_state =
                attachment_feedback_loop_dynamic_state.into();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>
    pub type PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT =
        unsafe extern "system" fn(command_buffer: CommandBuffer, aspect_mask: ImageAspectFlags);
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT =
        PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'static>;
    impl PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    cmd_set_attachment_feedback_loop_enable: PFN_vkCmdSetAttachmentFeedbackLoopEnableEXT,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                cmd_set_attachment_feedback_loop_enable: transmute(
                    load(c"vkCmdSetAttachmentFeedbackLoopEnableEXT")
                        .ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>
    #[inline]
    pub unsafe fn cmd_set_attachment_feedback_loop_enable(
        &self,
        command_buffer: CommandBuffer,
        aspect_mask: ImageAspectFlags,
    ) {
        unsafe { (self.cmd_set_attachment_feedback_loop_enable)(command_buffer, aspect_mask) }
    }
}
