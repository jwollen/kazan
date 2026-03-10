//! <https://registry.khronos.org/vulkan/specs/latest/man/html/VK_KHR_external_memory_fd.html>
#![allow(unused_imports)]
use crate::{vk::Result as VkResult, vk::*, *};
use core::ffi::{CStr, c_char, c_int, c_void};
use core::mem::transmute;
use core::ptr;

pub const EXTENSION_NAME: &CStr = c"VK_KHR_external_memory_fd";

pub(super) mod defs {
    #![allow(non_camel_case_types, unused_imports)]
    use crate::{vk::*, *};
    use core::ffi::{CStr, c_char, c_int, c_void};
    use core::fmt;
    use core::marker::PhantomData;
    use core::ptr;

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkImportMemoryFdInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct ImportMemoryFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub fd: c_int,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for ImportMemoryFdInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("ImportMemoryFdInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("handle_type", &self.handle_type)
                .field("fd", &self.fd)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for ImportMemoryFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::IMPORT_MEMORY_FD_INFO_KHR;
    }

    unsafe impl Extends<MemoryAllocateInfo<'_>> for ImportMemoryFdInfoKHR<'_> {}

    impl Default for ImportMemoryFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                handle_type: Default::default(),
                fd: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> ImportMemoryFdInfoKHR<'a> {
        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }

        #[inline]
        pub fn fd(mut self, fd: c_int) -> Self {
            self.fd = fd;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryFdPropertiesKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryFdPropertiesKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *mut c_void,
        pub memory_type_bits: u32,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryFdPropertiesKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryFdPropertiesKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory_type_bits", &self.memory_type_bits)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryFdPropertiesKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_FD_PROPERTIES_KHR;
    }

    impl Default for MemoryFdPropertiesKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null_mut(),
                memory_type_bits: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryFdPropertiesKHR<'a> {
        #[inline]
        pub fn memory_type_bits(mut self, memory_type_bits: u32) -> Self {
            self.memory_type_bits = memory_type_bits;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/VkMemoryGetFdInfoKHR.html>
    #[repr(C)]
    #[derive(Copy, Clone)]
    #[must_use]
    pub struct MemoryGetFdInfoKHR<'a> {
        pub s_type: StructureType,
        pub p_next: *const c_void,
        pub memory: DeviceMemory,
        pub handle_type: ExternalMemoryHandleTypeFlagBits,
        pub _marker: PhantomData<&'a ()>,
    }

    #[cfg(feature = "debug")]
    impl fmt::Debug for MemoryGetFdInfoKHR<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("MemoryGetFdInfoKHR")
                .field("s_type", &self.s_type)
                .field("p_next", &self.p_next)
                .field("memory", &self.memory)
                .field("handle_type", &self.handle_type)
                .finish()
        }
    }

    unsafe impl<'a> TaggedStructure<'a> for MemoryGetFdInfoKHR<'a> {
        const STRUCTURE_TYPE: StructureType = StructureType::MEMORY_GET_FD_INFO_KHR;
    }

    impl Default for MemoryGetFdInfoKHR<'_> {
        fn default() -> Self {
            Self {
                s_type: Self::STRUCTURE_TYPE,
                p_next: ptr::null(),
                memory: Default::default(),
                handle_type: Default::default(),
                _marker: PhantomData,
            }
        }
    }

    impl<'a> MemoryGetFdInfoKHR<'a> {
        #[inline]
        pub fn memory(mut self, memory: DeviceMemory) -> Self {
            self.memory = memory;
            self
        }

        #[inline]
        pub fn handle_type(mut self, handle_type: ExternalMemoryHandleTypeFlagBits) -> Self {
            self.handle_type = handle_type;
            self
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryFdKHR.html>
    pub type PFN_vkGetMemoryFdKHR = unsafe extern "system" fn(
        device: Device,
        p_get_fd_info: *const MemoryGetFdInfoKHR<'_>,
        p_fd: *mut c_int,
    ) -> vk::Result;
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryFdPropertiesKHR.html>
    pub type PFN_vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: c_int,
        p_memory_fd_properties: *mut MemoryFdPropertiesKHR<'_>,
    ) -> vk::Result;
}

#[cfg(feature = "ffi")]
pub(super) mod ffi {
    #![allow(non_camel_case_types)]
    use super::defs::*;

    pub type VkImportMemoryFdInfoKHR = ImportMemoryFdInfoKHR<'static>;
    pub type VkMemoryFdPropertiesKHR = MemoryFdPropertiesKHR<'static>;
    pub type VkMemoryGetFdInfoKHR = MemoryGetFdInfoKHR<'static>;
    impl ImportMemoryFdInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkImportMemoryFdInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryFdPropertiesKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryFdPropertiesKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl MemoryGetFdInfoKHR<'_> {
        #[inline]
        pub unsafe fn drop_lifetime_for_ffi(&self) -> &VkMemoryGetFdInfoKHR {
            unsafe { core::mem::transmute(self) }
        }
    }
}

pub struct DeviceFn {
    get_memory_fd_khr: PFN_vkGetMemoryFdKHR,
    get_memory_fd_properties_khr: PFN_vkGetMemoryFdPropertiesKHR,
}

impl LoadDeviceFn for DeviceFn {
    unsafe fn load_with(
        load: impl Fn(&CStr) -> Option<PFN_vkVoidFunction>,
    ) -> core::result::Result<Self, MissingEntryPointError> {
        unsafe {
            Ok(Self {
                get_memory_fd_khr: transmute(
                    load(c"vkGetMemoryFdKHR").ok_or(MissingEntryPointError)?,
                ),
                get_memory_fd_properties_khr: transmute(
                    load(c"vkGetMemoryFdPropertiesKHR").ok_or(MissingEntryPointError)?,
                ),
            })
        }
    }
}

impl DeviceFn {
    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryFdKHR.html>
    #[inline]
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

    /// <https://registry.khronos.org/vulkan/specs/latest/man/html/vkGetMemoryFdPropertiesKHR.html>
    #[inline]
    pub unsafe fn get_memory_fd_properties_khr(
        &self,
        device: Device,
        handle_type: ExternalMemoryHandleTypeFlagBits,
        fd: c_int,
        memory_fd_properties: &mut MemoryFdPropertiesKHR<'_>,
    ) -> crate::Result<()> {
        unsafe {
            let result =
                (self.get_memory_fd_properties_khr)(device, handle_type, fd, memory_fd_properties);

            match result {
                VkResult::SUCCESS => Ok(()),
                err => Err(err),
            }
        }
    }
}
