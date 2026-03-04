#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportSemaphoreFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub flags: SemaphoreImportFlags,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub fd: c_int,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImportSemaphoreFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_SEMAPHORE_FD_INFO_KHR;
    }
    impl Default for ImportSemaphoreFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                semaphore: Default::default(),
                flags: Default::default(),
                handle_type: Default::default(),
                fd: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportSemaphoreFdInfoKHR<'a> {
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }
        pub fn flags(mut self, flags: SemaphoreImportFlags) -> Self {
            self.flags = flags;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
        pub fn fd(mut self, fd: c_int) -> Self {
            self.fd = fd;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SemaphoreGetFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for SemaphoreGetFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_GET_FD_INFO_KHR;
    }
    impl Default for SemaphoreGetFdInfoKHR<'_> {
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
    impl<'a> SemaphoreGetFdInfoKHR<'a> {
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    pub type PFN_vkGetSemaphoreFdKHR = unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const SemaphoreGetFdInfoKHR<'_>,
        p_fd: *mut c_int,
    ) -> vk::Result;
    pub type PFN_vkImportSemaphoreFdKHR = unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    import_semaphore_fd_khr: PFN_vkImportSemaphoreFdKHR,
    get_semaphore_fd_khr: PFN_vkGetSemaphoreFdKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                import_semaphore_fd_khr: transmute(
                    load(c"vkImportSemaphoreFdKHR").ok_or(MissingEntryPointError)?,
                ),
                get_semaphore_fd_khr: transmute(
                    load(c"vkGetSemaphoreFdKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn import_semaphore_fd_khr(
        &self,
        device: Device,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.import_semaphore_fd_khr)(device, import_semaphore_fd_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_semaphore_fd_khr(
        &self,
        device: Device,
        get_fd_info: &SemaphoreGetFdInfoKHR<'_>,
    ) -> crate::Result<c_int> {
        unsafe {
            let mut fd = core::mem::MaybeUninit::uninit();
            let result = (self.get_semaphore_fd_khr)(device, get_fd_info, fd.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(fd.assume_init()),
                err => Err(err),
            }
        }
    }
}
