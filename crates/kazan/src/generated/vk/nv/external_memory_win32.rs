#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_NV_external_memory_win32";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMemoryWin32HandleInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportMemoryWin32HandleInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagsNV,
        pub handle: HANDLE,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportMemoryWin32HandleInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportMemoryWin32HandleInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_type", &self.handle_type)
                .field("handle", &self.handle)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportMemoryWin32HandleInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_NV;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportMemoryWin32HandleInfoNV<'a> {}

    impl Default for ImportMemoryWin32HandleInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                handle: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMemoryWin32HandleInfoNV<'a> {
        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagsNV) -> Self {
            self.handle_type = handle_type;
            self
        }

        #[inline]
        pub fn handle(mut self, handle: HANDLE) -> Self {
            self.handle = handle;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportMemoryWin32HandleInfoNV.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExportMemoryWin32HandleInfoNV<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_attributes: *const SECURITY_ATTRIBUTES,
        pub dw_access: DWORD,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportMemoryWin32HandleInfoNV<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportMemoryWin32HandleInfoNV")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_attributes", &self.p_attributes)
                .field("dw_access", &self.dw_access)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportMemoryWin32HandleInfoNV<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_NV;
    }

    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ExportMemoryWin32HandleInfoNV<'a> {}

    impl Default for ExportMemoryWin32HandleInfoNV<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                p_attributes: core::ptr::null(),
                dw_access: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ExportMemoryWin32HandleInfoNV<'a> {
        #[inline]
        pub fn attributes(mut self, attributes: *const SECURITY_ATTRIBUTES) -> Self {
            self.p_attributes = attributes;
            self
        }

        #[inline]
        pub fn dw_access(mut self, dw_access: DWORD) -> Self {
            self.dw_access = dw_access;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryWin32HandleNV.html>
    pub type PFN_vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
        p_handle: *mut HANDLE,
    ) -> vk::Result;
}

pub struct DeviceFn {
    get_memory_win32_handle_nv: PFN_vkGetMemoryWin32HandleNV,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_memory_win32_handle_nv: transmute(
                    load(c"vkGetMemoryWin32HandleNV").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryWin32HandleNV.html>
    #[inline]
    pub unsafe fn get_memory_win32_handle_nv(
        &self,
        device: Device,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut handle = core::mem::MaybeUninit::uninit();
            let result =
                (self.get_memory_win32_handle_nv)(device, memory, handle_type, handle.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(handle.assume_init()),
                err => Err(err),
            }
        }
    }
}
