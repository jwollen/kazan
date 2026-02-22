#![allow(unused_imports)]
use crate::*;
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use kazan_sys::{vk::*, *};
pub struct DeviceFn {
    import_semaphore_zircon_handle_fuchsia: PFN_vkImportSemaphoreZirconHandleFUCHSIA,
    get_semaphore_zircon_handle_fuchsia: PFN_vkGetSemaphoreZirconHandleFUCHSIA,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                import_semaphore_zircon_handle_fuchsia: transmute(
                    load(c"vkImportSemaphoreZirconHandleFUCHSIA").ok_or(LoadingError)?,
                ),
                get_semaphore_zircon_handle_fuchsia: transmute(
                    load(c"vkGetSemaphoreZirconHandleFUCHSIA").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(
        &self,
        device: Device,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> crate::Result<()> {
        unsafe {
            result((self.import_semaphore_zircon_handle_fuchsia)(
                device,
                import_semaphore_zircon_handle_info,
            ))
        }
    }
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(
        &self,
        device: Device,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
        zircon_handle: &mut zx_handle_t,
    ) -> crate::Result<()> {
        unsafe {
            result((self.get_semaphore_zircon_handle_fuchsia)(
                device,
                get_zircon_handle_info,
                zircon_handle,
            ))
        }
    }
}
