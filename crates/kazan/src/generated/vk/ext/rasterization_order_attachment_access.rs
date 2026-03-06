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
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub rasterization_order_color_attachment_access: Bool32,
        pub rasterization_order_depth_attachment_access: Bool32,
        pub rasterization_order_stencil_attachment_access: Bool32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a>
        for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a>
    {
        const STRUCTURE_TYPE: StructureType =
            StructureType::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT;
    }
    unsafe impl<'a> Extends<PhysicalDeviceFeatures2<'a>>
        for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a>
    {
    }
    unsafe impl<'a> Extends<DeviceCreateInfo<'a>>
        for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a>
    {
    }
    impl Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                rasterization_order_color_attachment_access: Default::default(),
                rasterization_order_depth_attachment_access: Default::default(),
                rasterization_order_stencil_attachment_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'a> {
        pub fn rasterization_order_color_attachment_access(
            mut self,
            rasterization_order_color_attachment_access: bool,
        ) -> Self {
            self.rasterization_order_color_attachment_access =
                rasterization_order_color_attachment_access.into();
            self
        }
        pub fn rasterization_order_depth_attachment_access(
            mut self,
            rasterization_order_depth_attachment_access: bool,
        ) -> Self {
            self.rasterization_order_depth_attachment_access =
                rasterization_order_depth_attachment_access.into();
            self
        }
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
