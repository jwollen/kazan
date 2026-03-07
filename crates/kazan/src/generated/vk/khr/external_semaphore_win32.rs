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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportSemaphoreWin32HandleInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportSemaphoreWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub flags: SemaphoreImportFlags,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub handle: HANDLE,
        pub name: LPCWSTR,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ImportSemaphoreWin32HandleInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportSemaphoreWin32HandleInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("semaphore", &self.semaphore)
                .field("flags", &self.flags)
                .field("handle_type", &self.handle_type)
                .field("handle", &self.handle)
                .field("name", &self.name)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportSemaphoreWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR;
    }

    impl Default for ImportSemaphoreWin32HandleInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                semaphore: Default::default(),
                flags: Default::default(),
                handle_type: Default::default(),
                handle: Default::default(),
                name: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportSemaphoreWin32HandleInfoKHR<'a> {
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

        pub fn handle(mut self, handle: HANDLE) -> Self {
            self.handle = handle;
            self
        }

        pub fn name(mut self, name: LPCWSTR) -> Self {
            self.name = name;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportSemaphoreWin32HandleInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportSemaphoreWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_attributes: *const SECURITY_ATTRIBUTES,
        pub dw_access: DWORD,
        pub name: LPCWSTR,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for ExportSemaphoreWin32HandleInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportSemaphoreWin32HandleInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_attributes", &self.p_attributes)
                .field("dw_access", &self.dw_access)
                .field("name", &self.name)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportSemaphoreWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_SEMAPHORE_WIN32_HANDLE_INFO_KHR;
    }

    unsafe impl<'a> Extends<SemaphoreCreateInfo<'a>> for ExportSemaphoreWin32HandleInfoKHR<'a> {}

    impl Default for ExportSemaphoreWin32HandleInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_attributes: core::ptr::null(),
                dw_access: Default::default(),
                name: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportSemaphoreWin32HandleInfoKHR<'a> {
        pub fn attributes(mut self, attributes: *const SECURITY_ATTRIBUTES) -> Self {
            self.p_attributes = attributes;
            self
        }

        pub fn dw_access(mut self, dw_access: DWORD) -> Self {
            self.dw_access = dw_access;
            self
        }

        pub fn name(mut self, name: LPCWSTR) -> Self {
            self.name = name;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkD3D12FenceSubmitInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct D3D12FenceSubmitInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub wait_semaphore_values_count: u32,
        pub p_wait_semaphore_values: *const u64,
        pub signal_semaphore_values_count: u32,
        pub p_signal_semaphore_values: *const u64,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for D3D12FenceSubmitInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("D3D12FenceSubmitInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field(
                    "wait_semaphore_values_count",
                    &self.wait_semaphore_values_count,
                )
                .field("p_wait_semaphore_values", &self.p_wait_semaphore_values)
                .field(
                    "signal_semaphore_values_count",
                    &self.signal_semaphore_values_count,
                )
                .field("p_signal_semaphore_values", &self.p_signal_semaphore_values)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for D3D12FenceSubmitInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::D3D12_FENCE_SUBMIT_INFO_KHR;
    }

    unsafe impl<'a> Extends<SubmitInfo<'a>> for D3D12FenceSubmitInfoKHR<'a> {}

    impl Default for D3D12FenceSubmitInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                wait_semaphore_values_count: Default::default(),
                p_wait_semaphore_values: core::ptr::null(),
                signal_semaphore_values_count: Default::default(),
                p_signal_semaphore_values: core::ptr::null(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> D3D12FenceSubmitInfoKHR<'a> {
        pub fn wait_semaphore_values(mut self, wait_semaphore_values: &'a [u64]) -> Self {
            self.wait_semaphore_values_count = wait_semaphore_values.len().try_into().unwrap();
            self.p_wait_semaphore_values = wait_semaphore_values.as_ptr();
            self
        }

        pub fn signal_semaphore_values(mut self, signal_semaphore_values: &'a [u64]) -> Self {
            self.signal_semaphore_values_count = signal_semaphore_values.len().try_into().unwrap();
            self.p_signal_semaphore_values = signal_semaphore_values.as_ptr();
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkSemaphoreGetWin32HandleInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct SemaphoreGetWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub semaphore: Semaphore,
        pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    impl fmt::Debug for SemaphoreGetWin32HandleInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("SemaphoreGetWin32HandleInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("semaphore", &self.semaphore)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for SemaphoreGetWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::SEMAPHORE_GET_WIN32_HANDLE_INFO_KHR;
    }

    impl Default for SemaphoreGetWin32HandleInfoKHR<'_> {
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

    impl<'a> SemaphoreGetWin32HandleInfoKHR<'a> {
        pub fn semaphore(mut self, semaphore: Semaphore) -> Self {
            self.semaphore = semaphore;
            self
        }

        pub fn handle_type(mut self, handle_type: ExternalSemaphoreHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSemaphoreWin32HandleKHR.html>
    pub type PFN_vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
        device: Device,
        p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportSemaphoreWin32HandleKHR.html>
    pub type PFN_vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
        device: Device,
        p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> vk::Result;
}

pub struct DeviceFn {
    import_semaphore_win32_handle_khr: PFN_vkImportSemaphoreWin32HandleKHR,
    get_semaphore_win32_handle_khr: PFN_vkGetSemaphoreWin32HandleKHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                import_semaphore_win32_handle_khr: transmute(
                    load(c"vkImportSemaphoreWin32HandleKHR").ok_or(MissingEntryPointError)?,
                ),
                get_semaphore_win32_handle_khr: transmute(
                    load(c"vkGetSemaphoreWin32HandleKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportSemaphoreWin32HandleKHR.html>
    pub unsafe fn import_semaphore_win32_handle_khr(
        &self,
        device: Device,
        import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result = (self.import_semaphore_win32_handle_khr)(
                device,
                import_semaphore_win32_handle_info,
            );

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetSemaphoreWin32HandleKHR.html>
    pub unsafe fn get_semaphore_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR<'_>,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut handle = core::mem::MaybeUninit::uninit();
            let result = (self.get_semaphore_win32_handle_khr)(
                device,
                get_win32_handle_info,
                handle.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(handle.assume_init()),
                err => Err(err),
            }
        }
    }
}
