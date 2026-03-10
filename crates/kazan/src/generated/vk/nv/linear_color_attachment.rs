//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_NV_linear_color_attachment.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_NV_linear_color_attachment";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub linear_color_attachment: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceLinearColorAttachmentFeaturesNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceLinearColorAttachmentFeaturesNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("linear_color_attachment", &self.linear_color_attachment)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for PhysicalDeviceLinearColorAttachmentFeaturesNV<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_LINEAR_COLOR_ATTACHMENT_FEATURES_NV;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceLinearColorAttachmentFeaturesNV<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>> for PhysicalDeviceLinearColorAttachmentFeaturesNV<'_> {}

    impl Default for PhysicalDeviceLinearColorAttachmentFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                linear_color_attachment: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceLinearColorAttachmentFeaturesNV<'a> {
        #[inline]
        pub fn linear_color_attachment(mut self, linear_color_attachment: bool) -> Self {
            self.linear_color_attachment = linear_color_attachment.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceLinearColorAttachmentFeaturesNV =
        PhysicalDeviceLinearColorAttachmentFeaturesNV<'static>;
    impl PhysicalDeviceLinearColorAttachmentFeaturesNV<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
            unsafe { core::mem::transmute(self) }
        }
    }
}
