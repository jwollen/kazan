#![allow(unused_imports)]
use crate::*;
use core::ffi::{c_char, c_int, c_void, CStr};
use kazan_sys::{vk::*, *};
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
