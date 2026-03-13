//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_semaphore_fd.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_semaphore_fd";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportSemaphoreFdInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportSemaphoreFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub flags: SemaphoreImportFlags,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub fd: c_int,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportSemaphoreFdInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportSemaphoreFdInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("semaphore", &self.semaphore)
                .field("flags", &self.flags)
                .field("handle_type", &self.handle_type)
                .field("fd", &self.fd)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportSemaphoreFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR;
    }

    impl Default for ImportSemaphoreFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                semaphore: Default::default(),
                flags: Default::default(),
                handle_type: Default::default(),
                fd: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportSemaphoreFdInfoKHR<'a> {
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
        pub fn fd(mut self, fd: c_int) -> Self {
            self.fd = fd;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSemaphoreGetFdInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct SemaphoreGetFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for SemaphoreGetFdInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SemaphoreGetFdInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("semaphore", &self.semaphore)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SemaphoreGetFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_GET_FD_INFO_KHR;
    }

    impl Default for SemaphoreGetFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                semaphore: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> SemaphoreGetFdInfoKHR<'a> {
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSemaphoreFdKHR.html>
    pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
        p_fd: *mut c_int,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportSemaphoreFdKHR.html>
    pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkImportSemaphoreFdInfoKHR = ImportSemaphoreFdInfoKHR<'static>;
    pub type VkSemaphoreGetFdInfoKHR = SemaphoreGetFdInfoKHR<'static>;
    impl ImportSemaphoreFdInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportSemaphoreFdInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl SemaphoreGetFdInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkSemaphoreGetFdInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    import_semaphore_fd: PFN_vkImportSemaphoreFdKHR,
    get_semaphore_fd: PFN_vkGetSemaphoreFdKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                import_semaphore_fd: transmute(
                    load(c"vkImportSemaphoreFdKHR").ok_or(MissingEntryPointError)?,
                ),
                get_semaphore_fd: transmute(
                    load(c"vkGetSemaphoreFdKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportSemaphoreFdKHR.html>
    #[inline]
    pub unsafe fn import_semaphore_fd(
        &self,
        device: Device,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.import_semaphore_fd)(device, import_semaphore_fd_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSemaphoreFdKHR.html>
    #[inline]
    pub unsafe fn get_semaphore_fd(
        &self,
        device: Device,
        get_fd_info: &SemaphoreGetFdInfoKHR<'_>,
    ) -> crate::Result<c_int> {
        unsafe {
            let mut fd = core::mem::MaybeUninit::uninit();
            let result = (self.get_semaphore_fd)(device, get_fd_info, fd.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(fd.assume_init()),
                err => Err(err),
            }
        }
    }
}
