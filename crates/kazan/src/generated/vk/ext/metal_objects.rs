#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    export_metal_objects_ext: PFN_vkExportMetalObjectsEXT,
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
