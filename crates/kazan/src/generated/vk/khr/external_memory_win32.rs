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
    pub struct ImportMemoryWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub handle: HANDLE,
        pub name: LPCWSTR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImportMemoryWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_MEMORY_WIN32_HANDLE_INFO_KHR;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportMemoryWin32HandleInfoKHR<'a> {}
    impl Default for ImportMemoryWin32HandleInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                handle: Default::default(),
                name: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportMemoryWin32HandleInfoKHR<'a> {
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ExportMemoryWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub p_attributes: *const SECURITY_ATTRIBUTES,
        pub dw_access: DWORD,
        pub name: LPCWSTR,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ExportMemoryWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::EXPORT_MEMORY_WIN32_HANDLE_INFO_KHR;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ExportMemoryWin32HandleInfoKHR<'a> {}
    impl Default for ExportMemoryWin32HandleInfoKHR<'_> {
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
    impl<'a> ExportMemoryWin32HandleInfoKHR<'a> {
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
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryWin32HandlePropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryWin32HandlePropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_WIN32_HANDLE_PROPERTIES_KHR;
    }
    impl Default for MemoryWin32HandlePropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryWin32HandlePropertiesKHR<'a> {
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryGetWin32HandleInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryGetWin32HandleInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_GET_WIN32_HANDLE_INFO_KHR;
    }
    impl Default for MemoryGetWin32HandleInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                memory: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryGetWin32HandleInfoKHR<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    pub type PFN_vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
        device: Device,
        p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR<'_>,
        p_handle: *mut HANDLE,
    ) -> vk::Result;
    pub type PFN_vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
        p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_memory_win32_handle_khr: PFN_vkGetMemoryWin32HandleKHR,
    get_memory_win32_handle_properties_khr: PFN_vkGetMemoryWin32HandlePropertiesKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_memory_win32_handle_khr: transmute(
                    load(c"vkGetMemoryWin32HandleKHR").ok_or(MissingEntryPointError)?,
                ),
                get_memory_win32_handle_properties_khr: transmute(
                    load(c"vkGetMemoryWin32HandlePropertiesKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_win32_handle_khr(
        &self,
        device: Device,
        get_win32_handle_info: &MemoryGetWin32HandleInfoKHR<'_>,
    ) -> crate::Result<HANDLE> {
        unsafe {
            let mut handle = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_win32_handle_khr)(
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
    pub unsafe fn get_memory_win32_handle_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        handle: HANDLE,
    ) -> crate::Result<MemoryWin32HandlePropertiesKHR<'_>> {
        unsafe {
            let mut memory_win32_handle_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_win32_handle_properties_khr)(
                device,
                handle_type,
                handle,
                memory_win32_handle_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory_win32_handle_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
