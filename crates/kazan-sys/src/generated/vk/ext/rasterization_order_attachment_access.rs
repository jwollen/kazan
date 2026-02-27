#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
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
impl Default for PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type:
                StructureType::PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT,
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
        rasterization_order_color_attachment_access: Bool32,
    ) -> Self {
        self.rasterization_order_color_attachment_access =
            rasterization_order_color_attachment_access;
        self
    }
    pub fn rasterization_order_depth_attachment_access(
        mut self,
        rasterization_order_depth_attachment_access: Bool32,
    ) -> Self {
        self.rasterization_order_depth_attachment_access =
            rasterization_order_depth_attachment_access;
        self
    }
    pub fn rasterization_order_stencil_attachment_access(
        mut self,
        rasterization_order_stencil_attachment_access: Bool32,
    ) -> Self {
        self.rasterization_order_stencil_attachment_access =
            rasterization_order_stencil_attachment_access;
        self
    }
}
