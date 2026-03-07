#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_linear_color_attachment";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

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

    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceLinearColorAttachmentFeaturesNV<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceLinearColorAttachmentFeaturesNV<'a>
    {
    }

    impl Default for PhysicalDeviceLinearColorAttachmentFeaturesNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
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
