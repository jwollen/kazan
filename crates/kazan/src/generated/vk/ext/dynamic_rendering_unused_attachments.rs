//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_dynamic_rendering_unused_attachments.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_dynamic_rendering_unused_attachments";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub dynamic_rendering_unused_attachments: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "dynamic_rendering_unused_attachments",
                    &self.dynamic_rendering_unused_attachments,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT;
    }

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'a>
    {
    }

    impl Default for PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                dynamic_rendering_unused_attachments: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'a> {
        #[inline]
        pub fn dynamic_rendering_unused_attachments(
            mut self,
            dynamic_rendering_unused_attachments: bool,
        ) -> Self {
            self.dynamic_rendering_unused_attachments = dynamic_rendering_unused_attachments.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT =
        PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'static>;
    impl PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
