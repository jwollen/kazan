#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub dynamic_rendering_unused_attachments: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'_> {
    fn default() -> Self {
        Self {
            s_type:
                StructureType::PHYSICAL_DEVICE_DYNAMIC_RENDERING_UNUSED_ATTACHMENTS_FEATURES_EXT,
            p_next: core::ptr::null_mut(),
            dynamic_rendering_unused_attachments: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT<'a> {
    pub fn dynamic_rendering_unused_attachments(
        mut self,
        dynamic_rendering_unused_attachments: Bool32,
    ) -> Self {
        self.dynamic_rendering_unused_attachments = dynamic_rendering_unused_attachments;
        self
    }
}
