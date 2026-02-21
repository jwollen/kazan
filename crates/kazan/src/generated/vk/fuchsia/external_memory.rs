#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    get_memory_zircon_handle_fuchsia: PFN_vkGetMemoryZirconHandleFUCHSIA,
    get_memory_zircon_handle_properties_fuchsia: PFN_vkGetMemoryZirconHandlePropertiesFUCHSIA,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_zircon_handle_fuchsia: transmute(
                    load(c"vkGetMemoryZirconHandleFUCHSIA").ok_or(LoadingError)?,
                ),
                get_memory_zircon_handle_properties_fuchsia: transmute(
                    load(c"vkGetMemoryZirconHandlePropertiesFUCHSIA").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_zircon_handle_fuchsia(
        &self,
        device: Device,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
        zircon_handle: &mut zx_handle_t,
    ) -> Result {
        unsafe {
            (self.get_memory_zircon_handle_fuchsia)(device, get_zircon_handle_info, zircon_handle)
        }
    }
    pub unsafe fn get_memory_zircon_handle_properties_fuchsia(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        zircon_handle: zx_handle_t,
        memory_zircon_handle_properties: &mut MemoryZirconHandlePropertiesFUCHSIA,
    ) -> Result {
        unsafe {
            (self.get_memory_zircon_handle_properties_fuchsia)(
                device,
                handle_type,
                zircon_handle,
                memory_zircon_handle_properties,
            )
        }
    }
}
