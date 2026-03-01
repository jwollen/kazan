#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use bitflags::bitflags;
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::marker::PhantomData;
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct ImportMemoryFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub fd: c_int,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for ImportMemoryFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_MEMORY_FD_INFO_KHR;
    }
    unsafe impl<'a> Extends<MemoryAllocateInfo<'a>> for ImportMemoryFdInfoKHR<'a> {}
    impl Default for ImportMemoryFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null(),
                handle_type: Default::default(),
                fd: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> ImportMemoryFdInfoKHR<'a> {
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
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
    pub struct MemoryFdPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryFdPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_FD_PROPERTIES_KHR;
    }
    impl Default for MemoryFdPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: core::ptr::null_mut(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }
    impl<'a> MemoryFdPropertiesKHR<'a> {
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct MemoryGetFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }
    unsafe impl<'a> TaggedStructure<'a> for MemoryGetFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_GET_FD_INFO_KHR;
    }
    impl Default for MemoryGetFdInfoKHR<'_> {
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
    impl<'a> MemoryGetFdInfoKHR<'a> {
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }
    pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
        p_fd: *mut c_int,
    ) -> vk::Result;
    pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: c_int,
        p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
    ) -> vk::Result;
}
pub struct DeviceFn {
    get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
    get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
}
impl DeviceFn {
    pub unsafe fn load(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, LoadingError> {
        unsafe {
            Ok(Self {
                get_memory_fd_khr: transmute(load(c"vkGetMemoryFdKHR").ok_or(LoadingError)?),
                get_memory_fd_properties_khr: transmute(
                    load(c"vkGetMemoryFdPropertiesKHR").ok_or(LoadingError)?,
                ),
            })
        }
    }
}
impl DeviceFn {
    pub unsafe fn get_memory_fd_khr(
        &self,
        device: Device,
        get_fd_info: &MemoryGetFdInfoKHR<'_>,
    ) -> crate::Result<c_int> {
        unsafe {
            let mut fd = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_fd_khr)(device, get_fd_info, fd.as_mut_ptr());

            match result {
                VkResult::SUCCESS => Ok(fd.assume_init()),
                err => Err(err),
            }
        }
    }
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: c_int,
    ) -> crate::Result<MemoryFdPropertiesKHR<'_>> {
        unsafe {
            let mut memory_fd_properties = core::mem::MaybeUninit::uninit();
            let result = (self.get_memory_fd_properties_khr)(
                device,
                handle_type,
                fd,
                memory_fd_properties.as_mut_ptr(),
            );

            match result {
                VkResult::SUCCESS => Ok(memory_fd_properties.assume_init()),
                err => Err(err),
            }
        }
    }
}
