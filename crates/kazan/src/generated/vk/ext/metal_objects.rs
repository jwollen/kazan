#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                export_metal_objects_ext: transmute(
                    load(c"vkExportMetalObjectsEXT").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn export_metal_objects_ext(
        &self,
        device: Device,
        metal_objects_info: &mut ExportMetalObjectsInfoEXT,
    ) {
        unsafe { (self.export_metal_objects_ext)(device, metal_objects_info) }
    }
}
