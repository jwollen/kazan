#![allow(unused_imports)]
use crate::*;
use kazan_sys::{vk::*, *};
use std::ffi::{c_char, c_int, c_void, CStr};
pub struct DeviceFn {
    get_semaphore_zircon_handle_fuchsia: PFN_vkGetSemaphoreZirconHandleFUCHSIA,
    import_semaphore_zircon_handle_fuchsia: PFN_vkImportSemaphoreZirconHandleFUCHSIA,
}
impl DeviceFn {
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(
        &self,
        device: Device,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
        zircon_handle: &mut zx_handle_t,
    ) -> Result {
        unsafe {
            (self.get_semaphore_zircon_handle_fuchsia)(
                device,
                get_zircon_handle_info,
                zircon_handle,
            )
        }
    }
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(
        &self,
        device: Device,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> Result {
        unsafe {
            (self.import_semaphore_zircon_handle_fuchsia)(
                device,
                import_semaphore_zircon_handle_info,
            )
        }
    }
}
