#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::Result as VkResult, vk::*, *};
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
    pub unsafe fn export_metal_objects_ext(&self, device: Device) -> ExportMetalObjectsInfoEXT<'_> {
        unsafe {
            let mut metal_objects_info = core::mem::MaybeUninit::uninit();
            (self.export_metal_objects_ext)(device, metal_objects_info.as_mut_ptr());
            metal_objects_info.assume_init()
        }
    }
}
