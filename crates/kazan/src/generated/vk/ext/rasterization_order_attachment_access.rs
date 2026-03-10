//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_EXT_rasterization_order_attachment_access.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_EXT_rasterization_order_attachment_access";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub rasterization_order_color_attachment_access: Bool32,
        pub rasterization_order_depth_attachment_access: Bool32,
        pub rasterization_order_stencil_attachment_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "rasterization_order_color_attachment_access",
                    &self.rasterization_order_color_attachment_access,
                )
                .field(
                    "rasterization_order_depth_attachment_access",
                    &self.rasterization_order_depth_attachment_access,
                )
                .field(
                    "rasterization_order_stencil_attachment_access",
                    &self.rasterization_order_stencil_attachment_access,
                )
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT;
    }

    unsafe impl Extends<PhysicalDeviceFeatures2<'_>>
        for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'_>
    {
    }
    unsafe impl Extends<DeviceCreateInfo<'_>>
        for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'_>
    {
    }

    impl Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                rasterization_order_color_attachment_access: Default::default(),
                rasterization_order_depth_attachment_access: Default::default(),
                rasterization_order_stencil_attachment_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a> {
        #[inline]
        pub fn rasterization_order_color_attachment_access(
            mut self,
            rasterization_order_color_attachment_access: bool,
        ) -> Self {
            self.rasterization_order_color_attachment_access =
                rasterization_order_color_attachment_access.into();
            self
        }

        #[inline]
        pub fn rasterization_order_depth_attachment_access(
            mut self,
            rasterization_order_depth_attachment_access: bool,
        ) -> Self {
            self.rasterization_order_depth_attachment_access =
                rasterization_order_depth_attachment_access.into();
            self
        }

        #[inline]
        pub fn rasterization_order_stencil_attachment_access(
            mut self,
            rasterization_order_stencil_attachment_access: bool,
        ) -> Self {
            self.rasterization_order_stencil_attachment_access =
                rasterization_order_stencil_attachment_access.into();
            self
        }
    }
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT =
        PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'static>;
    impl PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(
            &self,
        ) -> &VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
            unsafe { core::mem::transmute(self) }
        }
    }
}
