#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_fence_win32";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportFenceWin32HandleInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportFenceWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fence: Fence,
        pub flags: FenceImportFlags,
        pub handle_type: ExternalFenceHandleTypeFlagBits,
        pub handle: HANDLE,
        pub name: LPCWSTR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportFenceWin32HandleInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportFenceWin32HandleInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("fence", &self.fence)
                .field("flags", &self.flags)
                .field("handle_type", &self.handle_type)
                .field("handle", &self.handle)
                .field("name", &self.name)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportFenceWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_FENCE_WIN32_HANDLE_INFO_KHR;
    }

    impl Default for ImportFenceWin32HandleInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                fence: Default::default(),
                flags: Default::default(),
                handle_type: Default::default(),
                handle: Default::default(),
                name: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportFenceWin32HandleInfoKHR<'a> {
        #[inline]
        pub fn fence(mut self, fence: Fence) -> Self {
            self.fence = fence;
            self
        }

        #[inline]
        pub fn flags(mut self, flags: FenceImportFlags) -> Self {
            self.flags = flags;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }

        #[inline]
        pub fn handle(mut self, handle: HANDLE) -> Self {
            self.handle = handle;
            self
        }

        #[inline]
        pub fn name(mut self, name: LPCWSTR) -> Self {
            self.name = name;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkExportFenceWin32HandleInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ExportFenceWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_attributes: *const SECURITY_ATTRIBUTES,
        pub dw_access: DWORD,
        pub name: LPCWSTR,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ExportFenceWin32HandleInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ExportFenceWin32HandleInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("p_attributes", &self.p_attributes)
                .field("dw_access", &self.dw_access)
                .field("name", &self.name)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ExportFenceWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_FENCE_WIN32_HANDLE_INFO_KHR;
    }

    unsafe impl<'a> Extends<FenceCreateInfo<'a>> for ExportFenceWin32HandleInfoKHR<'a> {}

    impl Default for ExportFenceWin32HandleInfoKHR<'_> {
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

    impl<'a> ExportFenceWin32HandleInfoKHR<'a> {
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

        #[inline]
        pub fn name(mut self, name: LPCWSTR) -> Self {
            self.name = name;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkFenceGetWin32HandleInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct FenceGetWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub fence: Fence,
        pub handle_type: ExternalFenceHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for FenceGetWin32HandleInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("FenceGetWin32HandleInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("fence", &self.fence)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for FenceGetWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::FENCE_GET_WIN32_HANDLE_INFO_KHR;
    }

    impl Default for FenceGetWin32HandleInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                fence: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> FenceGetWin32HandleInfoKHR<'a> {
        #[inline]
        pub fn fence(mut self, fence: Fence) -> Self {
            self.fence = fence;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalFenceHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetFenceWin32HandleKHR.html>
    pub type PFN_vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
        device: Device,
        p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportFenceWin32HandleKHR.html>
    pub type PFN_vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
        device: Device,
        p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR<'_>,
    ) -> vk::Result;
}

pub struct DeviceFn {
    import_fence_win32_handle_khr: PFN_vkImportFenceWin32HandleKHR,
    get_fence_win32_handle_khr: PFN_vkGetFenceWin32HandleKHR,
}

impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                import_fence_win32_handle_khr: transmute(
                    load(c"vkImportFenceWin32HandleKHR").ok_or(MissingEntryPointError)?,
                ),
                get_fence_win32_handle_khr: transmute(
                    load(c"vkGetFenceWin32HandleKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkImportFenceWin32HandleKHR.html>
    #[inline]
    pub unsafe fn import_fence_win32_handle_khr(
        &self,
        device: Device,
        import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.import_fence_win32_handle_khr)(device, import_fence_win32_handle_info);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetFenceWin32HandleKHR.html>
    #[inline]
    pub unsafe fn get_fence_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &FenceGetWin32HandleInfoKHR<'_>,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut handle = core::mem::MaybeUninit::uninit();
            let result = (self.get_fence_win32_handle_khr)(
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
