#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_FUCHSIA_external_semaphore";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportSemaphoreZirconHandleInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportSemaphoreZirconHandleInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub flags: SemaphoreImportFlags,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub zircon_handle: zx_handle_t,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportSemaphoreZirconHandleInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportSemaphoreZirconHandleInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("semaphore", &self.semaphore)
                .field("flags", &self.flags)
                .field("handle_type", &self.handle_type)
                .field("zircon_handle", &self.zircon_handle)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportSemaphoreZirconHandleInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::IMPORT_SEMAPHORE_ZIRCON_HANDLE_INFO_FUCHSIA;
    }

    impl Default for ImportSemaphoreZirconHandleInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                semaphore: Default::default(),
                flags: Default::default(),
                handle_type: Default::default(),
                zircon_handle: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportSemaphoreZirconHandleInfoFUCHSIA<'a> {
        #[inline]
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: SemaphoreImportFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }

        #[inline]
        pub fn zircon_handle(mut self, zircon_handle: zx_handle_t) -> Self {
            self.zircon_handle = zircon_handle;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSemaphoreGetZirconHandleInfoFUCHSIA.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SemaphoreGetZirconHandleInfoFUCHSIA<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SemaphoreGetZirconHandleInfoFUCHSIA<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SemaphoreGetZirconHandleInfoFUCHSIA")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("semaphore", &self.semaphore)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SemaphoreGetZirconHandleInfoFUCHSIA<'a> {
        const STRUCTURE_TYPE: StructureType =
            StructureType::SEMAPHORE_GET_ZIRCON_HANDLE_INFO_FUCHSIA;
    }

    impl Default for SemaphoreGetZirconHandleInfoFUCHSIA<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                semaphore: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SemaphoreGetZirconHandleInfoFUCHSIA<'a> {
        #[inline]
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html>
    pub type PFN_vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
        device: Device,
        p_get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
        p_zircon_handle: *mut zx_handle_t,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html>
    pub type PFN_vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
    )
        -> vk::Result;
}

pub struct DeviceFn {
    import_semaphore_zircon_handle_fuchsia: PFN_vkImportSemaphoreZirconHandleFUCHSIA,
    get_semaphore_zircon_handle_fuchsia: PFN_vkGetSemaphoreZirconHandleFUCHSIA,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                import_semaphore_zircon_handle_fuchsia: transmute(
                    load(c"vkImportSemaphoreZirconHandleFUCHSIA").ok_or(MissingEntryPointError)?,
                ),
                get_semaphore_zircon_handle_fuchsia: transmute(
                    load(c"vkGetSemaphoreZirconHandleFUCHSIA").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportSemaphoreZirconHandleFUCHSIA.html>
    #[inline]
    pub unsafe fn import_semaphore_zircon_handle_fuchsia(
        &self,
        device: Device,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.import_semaphore_zircon_handle_fuchsia)(
                device,
                import_semaphore_zircon_handle_info,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSemaphoreZirconHandleFUCHSIA.html>
    #[inline]
    pub unsafe fn get_semaphore_zircon_handle_fuchsia(
        &self,
        device: Device,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA<'_>,
    ) -> crate::Result<zx_handle_t> {
        unsafe {
            let mut zircon_handle = core::mem::MaybeUninit::uninit();
            let result = (self.get_semaphore_zircon_handle_fuchsia)(
                device,
                get_zircon_handle_info,
                zircon_handle.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(zircon_handle.assume_init()),
                err => Err(err),
            }
        }
    }
}
