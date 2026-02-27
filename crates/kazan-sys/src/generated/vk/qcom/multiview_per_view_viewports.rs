#![allow(non_camel_case_types, unused_imports)]
use crate::{vk::*, *};
use bitflags::bitflags;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::marker::PhantomData;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {
    pub s_type: StructureType,
    pub p_next: *mut c_void,
    pub multiview_per_view_viewports: Bool32,
    pub _marker: PhantomData<&'a ()>,
}
impl Default for PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'_> {
    fn default() -> Self {
        Self {
            s_type: StructureType::PHYSICAL_DEVICE_MULTIVIEW_PER_VIEW_VIEWPORTS_FEATURES_QCOM,
            p_next: core::ptr::null_mut(),
            multiview_per_view_viewports: Default::default(),
            _marker: PhantomData,
        }
    }
}
impl<'a> PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM<'a> {
    pub fn multiview_per_view_viewports(mut self, multiview_per_view_viewports: Bool32) -> Self {
        self.multiview_per_view_viewports = multiview_per_view_viewports;
        self
    }
}
